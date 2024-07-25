use std::path::Path;
use if_chain::if_chain;
use toml::Value;
use crate::manifest::gather_manifest_files;
use crate::io::load_cargo_manifest;
//--------------------------------------------------------------------------------------------------



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverrideSubDepConfig {
    // Since there should not be a lot of entries, we can use list instead of map (with complex key)
    pub entries: Vec<OverrideEntry>,
}

impl OverrideSubDepConfig {
    pub fn new<const N: usize>(entries: [OverrideEntry; N]) -> Self {
        OverrideSubDepConfig { entries: entries.into_iter().collect::<Vec<_>>() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct OverrideEntry {
    pub dependency: String,
    pub sub_dependency: String,
    pub version_to_fix: String,
    pub version_required: String,
}

// mainly for test
pub fn str_override_entry(dep: &str, sub_dep: &str, ver_to_fix: &str, ver_req: &str) -> OverrideEntry {
    OverrideEntry {
        dependency: dep.to_owned(),
        sub_dependency: sub_dep.to_owned(),
        version_to_fix: ver_to_fix.to_owned(),
        version_required: ver_req.to_owned(),
    }
}

impl OverrideSubDepConfig {
    pub fn find_override_for_sub_dep(&self, sub_dep_name: &str, version: &str) -> Option<&OverrideEntry> {
        self.entries.iter()
            .find(|e| e.sub_dependency == sub_dep_name && e.version_to_fix == version)
    }
    pub fn is_dep_to_fix(&self, dep_name: &str) -> bool {
        self.entries.iter()
            .find(|e| e.dependency == dep_name)
            .is_some()
    }
}


pub fn gather_override_patch_conf_from_dir(project_dir: &Path) -> Result<OverrideSubDepConfig, anyhow::Error>{

    let manifests = gather_manifest_files(project_dir) ?;
    let mut all_conf_entries = Vec::<OverrideEntry>::new();

    for m_path in manifests {
        let m = load_cargo_manifest(&m_path) ?;

        let workspace_metadata = m.workspace
            .and_then(|w| w.metadata);

        let w_conf = parse_conf_metadata(workspace_metadata)?;
        all_conf_entries.extend(w_conf.entries);

        let package_metadata = m.package
            .and_then(|w| w.metadata);
        let p_conf = parse_conf_metadata(package_metadata)?;
        all_conf_entries.extend(p_conf.entries);
    }

    all_conf_entries.sort_unstable();
    all_conf_entries.dedup();

    Ok(OverrideSubDepConfig { entries: all_conf_entries })
}


pub fn parse_conf_metadata(metadata: Option<Value>) -> Result<OverrideSubDepConfig, anyhow::Error>{
    let mut conf_entries = Vec::<OverrideEntry>::new();

    if_chain! {
        if let Some(Value::Table(ref table)) = metadata;
        let patch_override_sub_dependencies = table.get("patch-override-sub-dependencies");
        if let Some(Value::Table(ref patch_override_sub_dependencies)) = patch_override_sub_dependencies;

        then {
            for dep_name in patch_override_sub_dependencies {
                let override_opt = dep_name.1.get("override");
                let dep_name = dep_name.0.as_str();
                if let Some(Value::Array(ref array)) = override_opt {

                    let override_params = split_override_params(dep_name, array) ?;
                    conf_entries.extend(override_params);
                }
            }
        }
    }

    Ok(OverrideSubDepConfig { entries: conf_entries })
}

fn split_override_params(dep_name: &str, array: &toml::value::Array) -> anyhow::Result<Vec<OverrideEntry>> {
    const MSG: &str = r#"Expected 3*N params (format like: "reqwest", "0.11.27", "0.12.5",)"#;

    if (array.len() % 3) != 0 {
        return Err(anyhow::anyhow!(MSG));
    }

    let mut conf_entries = Vec::<OverrideEntry>::new();

    for i in (0..array.len()).step_by(3) {
        let sub_dep_name = toml_val_str(array.get(i), MSG)?;
        let ver_from = toml_val_str(array.get(i + 1), MSG)?;
        let ver_to = toml_val_str(array.get(i + 2), MSG)?;

        let conf_entry = OverrideEntry {
            dependency: dep_name.to_owned(),
            sub_dependency: sub_dep_name.to_owned(),
            version_to_fix: ver_from.to_owned(),
            version_required: ver_to.to_owned(),
        };
        conf_entries.push(conf_entry);
    }

    Ok(conf_entries)
}


fn toml_val_str(v: Option<&Value>, err_msg: &'static str) -> anyhow::Result<String> {
    let v = v.ok_or_else(||anyhow::anyhow!(err_msg)) ?;
    if let Value::String(ref str_value) = v {
        Ok(str_value.to_owned())
    } else {
        anyhow::bail!(err_msg)
    }
}
