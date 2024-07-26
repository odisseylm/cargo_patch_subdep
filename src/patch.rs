use std::io::Write;
use std::path::Path;
use cargo_toml::Dependency;
use crate::{
    conf::{ gather_override_patch_conf_from_dir, OverrideEntry, OverrideSubDepConfig },
    deps::gather_patching_deps_from_dir,
    manifest::gather_manifest_files,
    io::{ load_cargo_manifest, save_manifest },
    load_src::load_dep_sources,
};
//--------------------------------------------------------------------------------------------------



pub fn replace_deps_version_in_file_tree(root_dir: &Path, override_config: &OverrideSubDepConfig)
    -> Result<(), anyhow::Error> {

    let manifests = gather_manifest_files(root_dir) ?;
    for manifest in manifests {
        replace_deps_version_in_file(&manifest, override_config) ?
    }
    Ok(())
}

fn replace_deps_version_in_file(manifest_file_path: &Path, override_config: &OverrideSubDepConfig) -> Result<(), anyhow::Error> {
    let mut manifest = load_cargo_manifest(manifest_file_path) ?;
    let mut fixed = false;

    fixed |= replace_deps_version(&mut manifest.dependencies, &override_config);
    fixed |= replace_deps_version(&mut manifest.build_dependencies, &override_config);
    fixed |= replace_deps_version(&mut manifest.dev_dependencies, &override_config);

    if let Some(ref mut w) = manifest.workspace {
        fixed |= replace_deps_version(&mut w.dependencies, &override_config);
    }

    if fixed {
        save_manifest(&manifest, manifest_file_path) ?;
    }

    Ok(())
}


fn replace_deps_version(deps: &mut cargo_toml::DepsSet, override_config: &OverrideSubDepConfig) -> bool {
    let mut fixed = false;

    for dep in deps {
        let dep_version = dep.1.try_req().unwrap_or("");
        let override_entry = override_config
            .find_override_for_sub_dep(&dep.0, dep_version);

        if let Some(ref override_entry) = override_entry {
            let dep: &mut cargo_toml::Dependency = dep.1;
            fixed |= replace_dep_version_if_needed(dep, override_entry);
        }
    }

    fixed
}

fn replace_dep_version_if_needed(dep: &mut cargo_toml::Dependency, override_entry: &OverrideEntry) -> bool {
    use cargo_toml::Dependency;

    let version = dep.try_req().unwrap_or("");
    if version == override_entry.version_to_fix {
        match dep {

            // Version requirement (e. g. ^1.5)
            Dependency::Simple(ref mut dep) => {
                *dep = override_entry.version_required.clone(); // "0.12.5".to_owned();
                true
            }

            Dependency::Inherited(ref _dep) => {
                false
            }

            // { version = "^1.5", features = ["a", "b"] } etc.
            Dependency::Detailed(ref mut dep) => {
                dep.version = Some(override_entry.version_required.clone()); // "0.12.5".to_owned());
                true
            }
        }
    } else {
        false
    }
}


// real high-level action code
pub fn do_patch_project(project_dir: &Path) -> Result<(), anyhow::Error> {

    let conf = gather_override_patch_conf_from_dir(project_dir) ?;
    if conf.entries.is_empty() {
        eprintln!("Hm... Nothing to path. Config keys [workspace.metadata.patch-override-sub-dependencies.*] or [package.metadata.patch-override-sub-dependencies.*] are not found");
        return Ok(());
    }

    let deps_to_patch = gather_patching_deps_from_dir(project_dir, &conf) ?;
    let copied_src = load_dep_sources(project_dir, &deps_to_patch) ?;

    for d in &deps_to_patch {
        let dep_name = d.0.as_str();

        let downloaded_dep_src_to_patch = copied_src.get(dep_name)
            .ok_or_else(||anyhow::anyhow!("Dependency [{dep_name}] is not loaded.")) ?;

        let this_dep_patch_rules = conf.entries
            .iter()
            .filter(|e| e.dependency == dep_name)
            .map(|e| e.clone())
            .collect::<Vec<_>>();
        let this_dep_conf = OverrideSubDepConfig { entries: this_dep_patch_rules };

        replace_deps_version_in_file_tree(&downloaded_dep_src_to_patch, &this_dep_conf) ?;
    }

    let root_m_path = project_dir.join("Cargo.toml");

    for d in &deps_to_patch {
        let dep_name = d.0.as_str();
        let dep_version = d.1.as_str();

        let downloaded_dep_src_to_patch = copied_src.get(dep_name)
            .ok_or_else(|| anyhow::anyhow!("Dependency [{dep_name}] is not loaded.")) ?;

        add_patch_entry_if_no(&root_m_path, dep_name, dep_version, &downloaded_dep_src_to_patch) ?;
    }

    Ok(())
}


pub fn add_patch_entry_if_no(manifest_path: &Path,
                             dep_name: &str, dep_version: &str, dep_src_path: &Path,
    ) -> anyhow::Result<()> {

    let m = load_cargo_manifest(manifest_path) ?;

    let is_patch_entry_already_present = m.patch.iter()
        .find(|e|e.0 == dep_name)
        .is_some();

    if is_patch_entry_already_present {
        return Ok(());
    }

    let mut f = std::fs::OpenOptions::new()
        // .write(true)
        .append(true)
        .open(manifest_path) ?;

    let base_path = manifest_path.parent().ok_or_else(||anyhow::anyhow!(
        "No parent for [{manifest_path:?}]")) ?;
    let base_path = std::fs::canonicalize(base_path) ?;

    let dep_src_path = std::fs::canonicalize(dep_src_path) ?;
    let dep_src_relative_path = dep_src_path.strip_prefix(&base_path) ?;
    let dep_src_relative_path_str = dep_src_relative_path
        .as_os_str().to_string_lossy();
    let dep_src_relative_path_str = dep_src_relative_path_str.as_ref();

    let current_patch_entry/*: Vec<&cargo_toml::Dependency>*/ = m.patch.iter()
        .flat_map(|e| e.1.iter().filter(|e|
                e.0.as_str() == dep_name
            )
            .map(|e| e.1)
        )
        .collect::<Vec<_>>();

    let dep_patches_count = current_patch_entry.len();
    if dep_patches_count > 1 {
        anyhow::bail!("Seems manifest [{manifest_path:?}] already contains [{dep_patches_count}]. How is it possible?");
    }

    if let Some(current_patch_entry) = current_patch_entry.first() {
        match current_patch_entry {
            Dependency::Simple(_) => {
                anyhow::bail!("Unexpected patch entry type [Simple] for [{dep_name}] in [{manifest_path:?}].");
            }
            Dependency::Inherited(_) => {
                anyhow::bail!("Unexpected patch entry type [Inherited] for [{dep_name}] in [{manifest_path:?}].");
            }
            Dependency::Detailed(ref current_patch_entry) => {
                if let Some(ref patch_path) = current_patch_entry.path {
                    if patch_path == dep_src_relative_path_str {
                        println!("Patch entry for [{dep_name} = {dep_src_relative_path:?}] already exists in [{manifest_path:?}].");
                        return Ok(());
                    } else {
                        anyhow::bail!("Patch entry for [{dep_name}] already exists in [{manifest_path:?}],\
                               and has another custom patch path [{patch_path}].\n\
                               Do not use 'patch sub-dependency' or remove this custom patch entry.");

                    }
                }

                anyhow::bail!("Patch entry for [{dep_name}] already exists in [{manifest_path:?}] and has custom parameters.\n\
                               Do not use 'patch sub-dependency' or remove this custom patch entry.");
            }
        }
    }

    let patch_entry: String = format!("\n\
                      [patch.crates-io.{dep_name}]\n\
                      version = \"{dep_version}\"\n\
                      path = \"{dep_src_relative_path_str}\"\n");

    f.write(patch_entry.as_bytes()) ?;
    f.flush() ?;

    Ok(())
}
