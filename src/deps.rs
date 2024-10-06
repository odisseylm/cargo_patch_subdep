use std::collections::{ HashMap, HashSet };
use std::path::Path;
use crate::{
    conf::ReplaceSubDepVersConfig,
    manifest::gather_manifest_files,
    io::load_cargo_manifest,
    util::string_hash_set_1,
};
//--------------------------------------------------------------------------------------------------


#[inline]
pub fn is_implicit_ver(ver: &str) -> bool {
    ver == "" || ver == "*"
}




pub fn gather_patching_deps_from_dir(project_dir: &Path, config: &ReplaceSubDepVersConfig)
                                     -> Result<HashMap<String, String>, anyhow::Error> {

    let manifests = gather_manifest_files(project_dir) ?;
    let mut dependencies_to_fix = HashMap::new();

    for m_path in manifests {
        gather_patching_deps_from_manifest(&m_path, config, &mut dependencies_to_fix) ?;
    }

    Ok(dependencies_to_fix)
}

fn gather_patching_deps_from_manifest<'a>(
    manifest_path: &Path,
    config: &ReplaceSubDepVersConfig,
    dependencies_to_fix: &mut HashMap<String, String>,
)
    -> Result<(), anyhow::Error> {

    fn gather_patching_deps_from_deps<'a>(
        dependencies: &'a cargo_toml::DepsSet, config: &ReplaceSubDepVersConfig)
        -> Result<HashMap<&'a String, &'a cargo_toml::Dependency>, anyhow::Error> {

        let deps_with_ver = dependencies.iter()
            .filter(|entry| config.is_dep_to_fix(entry.0))
            .collect::<HashMap<_,_>>();

        Ok(deps_with_ver)
    }

    if to_ignore_manifest(manifest_path, &config.ignore_cargos) {
        return Ok(());
    }

    let m = load_cargo_manifest(&manifest_path) ?;

    let deps = gather_patching_deps_from_deps(&m.dependencies, config) ?;
    merge_deps(dependencies_to_fix, &deps) ?;

    let deps = gather_patching_deps_from_deps(&m.build_dependencies, config) ?;
    merge_deps(dependencies_to_fix, &deps) ?;

    let deps = gather_patching_deps_from_deps(&m.dev_dependencies, config) ?;
    merge_deps(dependencies_to_fix, &deps) ?;

    if let Some(workspace) = m.workspace {
        let deps = gather_patching_deps_from_deps(&workspace.dependencies, config) ?;
        merge_deps(dependencies_to_fix, &deps) ?;
    }

    Ok(())
}


pub fn to_ignore_manifest(manifest_path: &Path, to_ignore_cargos: &Vec<String>) -> bool {
    let manifest_path_str = manifest_path.to_string_lossy();
    let manifest_path_str = manifest_path_str.as_ref();

    for ignore_cargo in to_ignore_cargos.iter() {
        if manifest_path_str.contains(ignore_cargo) {
            return true;
        }
    }
    false
}

fn merge_deps(summary: &mut HashMap<String, String>, merge_with: &HashMap<&String, &cargo_toml::Dependency>)
              -> Result<(), anyhow::Error> {

    for entry in merge_with {
        let dep_name: &String = entry.0;
        let dep: &cargo_toml::Dependency = entry.1;
        let prev_version = summary.get(dep_name);
        let next_ver = dep.try_req().unwrap_or("");

        match prev_version {
            None => {
                summary.insert(dep_name.to_owned(), next_ver.to_owned());
            }
            Some(ref current_version) => {
                let prev_ver = current_version.as_str();
                if is_implicit_ver(next_ver) {
                    // skip, no need to merge/update
                } else if !is_implicit_ver(next_ver) && is_implicit_ver(prev_ver) {
                    summary.insert(dep_name.to_owned(), next_ver.to_owned());
                } else if !is_implicit_ver(next_ver) && !is_implicit_ver(prev_ver) {
                    if next_ver != prev_ver {
                        return Err(anyhow::anyhow!("Different versions of [{dep_name}] are found [{next_ver}] and [{prev_ver}]."));
                    }
                }
            }
        }
    }

    Ok(())
}


// for testing only
pub fn gather_all_dep_ver_from_dir(project_dir: &Path)
                                   -> Result<HashMap<String, HashSet<String>>, anyhow::Error> {

    let manifests = gather_manifest_files(project_dir) ?;
    let mut all_deps_ver = HashMap::<String, HashSet<String>>::new();

    for m_path in manifests {
        gather_all_dep_ver_from_manifest(&m_path, &mut all_deps_ver) ?;
    }

    Ok(all_deps_ver)
}


// for testing only
fn gather_all_dep_ver_from_manifest<'a>(
    manifest_path: &Path,
    all_deps_ver: &mut HashMap<String, HashSet<String>>,
) -> Result<(), anyhow::Error> {

    fn merge_deps_ver(summary: &mut HashMap<String, HashSet<String>>, merge_with: &cargo_toml::DepsSet) {
        for entry in merge_with {
            let dep_name: &String = entry.0;
            let dep: &cargo_toml::Dependency = entry.1;
            let mut prev_version = summary.get_mut(dep_name);
            let next_ver = dep.try_req().unwrap_or("");

            match prev_version {
                None => {
                    summary.insert(dep_name.to_owned(), string_hash_set_1(next_ver));
                }
                Some(ref mut current_version) => {
                    current_version.insert(next_ver.to_owned());
                }
            }
        }
    }


    let m = load_cargo_manifest(&manifest_path) ?;

    merge_deps_ver(all_deps_ver, &m.dependencies);
    merge_deps_ver(all_deps_ver, &m.build_dependencies);
    merge_deps_ver(all_deps_ver, &m.dev_dependencies);

    if let Some(workspace) = m.workspace {
        merge_deps_ver(all_deps_ver, &workspace.dependencies);
    }

    Ok(())
}
