use std::path::Path;
use if_chain::if_chain;
use toml::Value;
use crate::deps::to_ignore_manifest;
use crate::manifest::gather_manifest_files;
use crate::io::load_cargo_manifest;
//--------------------------------------------------------------------------------------------------



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplaceSubDepVersConfig {
    // Since there should not be a lot of entries, we can use list instead of map (with complex key)
    pub entries: Vec<ReplaceSubDepVerEntry>,
    pub ignore_cargos: Vec<String>,
}

impl ReplaceSubDepVersConfig {
    pub fn new<const N: usize, const M: usize>(
        entries: [ReplaceSubDepVerEntry; N],
        ignore_cargos: [&str; M],
    ) -> Self {
        ReplaceSubDepVersConfig {
            entries: entries.into_iter().collect::<Vec<_>>(),
            ignore_cargos: ignore_cargos.into_iter().map(|s|s.to_owned()).collect::<Vec<_>>(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReplaceSubDepVerEntry {
    pub dependency: String,
    pub sub_dependency: String,
    pub from_ver: String,
    pub to_ver: String,
}

// mainly for test
pub fn str_override_entry(dep: &str, sub_dep: &str, ver_to_fix: &str, ver_req: &str) -> ReplaceSubDepVerEntry {
    ReplaceSubDepVerEntry {
        dependency: dep.to_owned(),
        sub_dependency: sub_dep.to_owned(),
        from_ver: ver_to_fix.to_owned(),
        to_ver: ver_req.to_owned(),
    }
}

impl ReplaceSubDepVersConfig {
    pub fn find_override_for_sub_dep(&self, sub_dep_name: &str, version: &str) -> Option<&ReplaceSubDepVerEntry> {
        self.entries.iter()
            .find(|e| e.sub_dependency == sub_dep_name && e.from_ver == version)
    }
    pub fn is_dep_to_fix(&self, dep_name: &str) -> bool {
        self.entries.iter()
            .find(|e| e.dependency == dep_name)
            .is_some()
    }
}


pub fn gather_override_patch_conf_from_dir(project_dir: &Path)
    -> Result<ReplaceSubDepVersConfig, anyhow::Error>{

    let manifests = gather_manifest_files(project_dir) ?;
    let mut all_conf_entries = Vec::<ReplaceSubDepVerEntry>::new();
    // TODO: We should not collect/aggregate them without relation to dependency,
    //       but let's live now with this simple solution/impl.
    let mut ignore_cargos = Vec::<String>::new();

    for m_path in manifests {

        if to_ignore_manifest(&m_path, &ignore_cargos) {
            continue;
        }

        let m = load_cargo_manifest(&m_path) ?;

        let workspace_metadata = m.workspace
            .and_then(|w| w.metadata);

        let w_conf = parse_conf_metadata(workspace_metadata)?;
        all_conf_entries.extend(w_conf.entries);
        ignore_cargos.extend(w_conf.ignore_cargos);

        let package_metadata = m.package
            .and_then(|w| w.metadata);
        let p_conf = parse_conf_metadata(package_metadata)?;
        all_conf_entries.extend(p_conf.entries);
        ignore_cargos.extend(p_conf.ignore_cargos);
    }

    all_conf_entries.sort_unstable();
    all_conf_entries.dedup();

    ignore_cargos.sort_unstable();
    ignore_cargos.dedup();

    Ok(ReplaceSubDepVersConfig { entries: all_conf_entries, ignore_cargos })
}


pub fn parse_conf_metadata(metadata: Option<Value>) -> Result<ReplaceSubDepVersConfig, anyhow::Error>{
    let mut conf_entries = Vec::<ReplaceSubDepVerEntry>::new();
    let mut ignore_cargos = Vec::<String>::new();

    if_chain! {
        if let Some(Value::Table(ref table)) = metadata;
        let patch_override_sub_dependencies = table.get("patch-replace-sub-dependencies");
        if let Some(Value::Table(ref patch_override_sub_dependencies)) = patch_override_sub_dependencies;

        then {
            for dep_name in patch_override_sub_dependencies {
                let override_opt = &dep_name.1.get("replace");
                let ignore_opt = &dep_name.1.get("ignore_cargos");

                let dep_name = dep_name.0.as_str();
                if let Some(Value::Array(ref array)) = override_opt {
                    let replace_attrs = parse_override_params(dep_name, array) ?;
                    conf_entries.extend(replace_attrs);
                }

                if let Some(Value::Array(ref array)) = ignore_opt {
                    let ignore_attrs = parse_ignore_cargos(array) ?;
                    ignore_cargos.extend(ignore_attrs);
                }
            }
        }
    }

    Ok(ReplaceSubDepVersConfig { entries: conf_entries, ignore_cargos })
}


fn parse_override_params(dep_name: &str, array: &toml::value::Array) -> anyhow::Result<Vec<ReplaceSubDepVerEntry>> {
    const ERR_MSG: &str = r#"Expected format like: [ sub_dep = "reqwest", from_ver = "0.11.27", to_ver = "0.12.5",)"#;
    let mut conf_entries = Vec::<ReplaceSubDepVerEntry>::new();

    for array_item in array.iter() {
        if let Value::Table(ref table) = array_item {
            let sub_dependency = table.get("sub_dep").string_value(ERR_MSG) ?;
            let from_ver  = table.get("from_ver").string_value(ERR_MSG) ?;
            let to_ver = table.get("to_ver").string_value(ERR_MSG) ?;

            conf_entries.push(ReplaceSubDepVerEntry {
                dependency: dep_name.to_owned(), sub_dependency,
                from_ver, to_ver,
            });
        } else {
            anyhow::bail!(ERR_MSG);
        }
    }

    Ok(conf_entries)
}


fn parse_ignore_cargos(array: &toml::value::Array) -> anyhow::Result<Vec<String>> {
    const ERR_MSG: &str = r#"Expected format like: [ "thirdparty/some-crate-to-ignore/Cargo.toml", ]"#;
    let mut ignore_entries = Vec::<String>::new();

    for array_item in array.iter() {
        if let Value::String(ref to_ignore_path_part) = array_item {
            ignore_entries.push(to_ignore_path_part.to_string());
        } else {
            anyhow::bail!(ERR_MSG);
        }
    }

    Ok(ignore_entries)
}


fn toml_val_str(v: &Option<&Value>, err_msg: &'static str) -> anyhow::Result<String> {
    let v = v.ok_or_else(||anyhow::anyhow!(err_msg)) ?;
    if let Value::String(ref str_value) = v {
        Ok(str_value.to_owned())
    } else {
        anyhow::bail!(err_msg)
    }
}


#[extension_trait::extension_trait]
pub impl TomlValueExt for Option<&Value> {
    fn string_value(&self, err_msg: &'static str) -> anyhow::Result<String> {
        toml_val_str(self, err_msg)
    }
}
