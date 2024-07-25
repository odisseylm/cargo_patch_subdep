use std::fs;
use std::path::{ Path, PathBuf };
use std::collections::HashMap;
use anyhow::anyhow;
use fs_extra::dir::{ copy, CopyOptions };
use crate::{
    io::{ save_manifest, clear_dir },
    cargo_core::{ fetch_cargo_core_workspace, setup_cargo_core_config },
};
//--------------------------------------------------------------------------------------------------



pub fn copy_package(dir: &Path, pkg_name: &str, pkg_root: &Path) -> Result<PathBuf, anyhow::Error> {
    let options = CopyOptions::new();
    let to = dir.join("target/patch-override-sub-dep").join(pkg_name);

    clear_dir(&to) ?;
    fs::create_dir_all(&to) ?;

    let _res = copy(pkg_root, &to, &options) ?;
    if let Some(name) = pkg_root.file_name() {
        let downloaded_dep_src_path = to.join(name).canonicalize() ?;
        Ok(downloaded_dep_src_path)
    } else {
        Err(anyhow!("Dependency Folder does not have a name"))
    }
}


fn create_temp_project_with_deps(project_dir: &Path, deps: &HashMap<String, String>) -> Result<PathBuf, anyhow::Error> {

    let temp_w_prj_dir = project_dir.join(".~tmp_patch_sub_deps_prj");
    clear_dir(&temp_w_prj_dir) ?;

    let temp_w_manifest_file = temp_w_prj_dir.join("Cargo.toml");
    let temp_prj_dir = temp_w_prj_dir.join("tmp_01");
    let temp_manifest_file = temp_prj_dir.join("Cargo.toml");
    let temp_src_dir = temp_prj_dir.join("src");

    fs::create_dir_all(&temp_prj_dir) ?;

    fs::write(&temp_w_manifest_file,
              "[workspace]\nmembers = [ \"tmp_01\" ]\n"
    ) ?;

    let mut m = cargo_toml::Manifest::from_str("") ?;

    let mut pkg = cargo_toml::Package::new("mvv-temp-for-loading-sources", "0.0.1");
    pkg.edition = cargo_toml::Inheritable::Set(cargo_toml::Edition::E2021);
    m.package = Some(pkg);

    for dep in deps {
        m.dependencies.insert(dep.0.to_owned(), cargo_toml::Dependency::Simple(dep.1.to_owned()));
    }

    save_manifest(&m, &temp_manifest_file) ?;

    fs::create_dir_all(&temp_src_dir) ?;
    fs::write(temp_src_dir.join("lib.rs"), "") ?;

    Ok(temp_prj_dir)
}

pub fn load_dep_sources(project_dir: &Path, deps: &HashMap<String, String>)
    -> Result<HashMap<String, PathBuf>, anyhow::Error> {

    let temp_project_dir = create_temp_project_with_deps(project_dir, deps) ?;
    let temp_manifest_path = temp_project_dir.join("Cargo.toml");

    // In rust should be applied only for existent file.
    let temp_manifest_path = fs::canonicalize(temp_manifest_path) ?;

    let config = setup_cargo_core_config() ?;
    let _lock = config.acquire_package_cache_lock(cargo::util::cache_lock::CacheLockMode::Shared) ?;
    let workspace = fetch_cargo_core_workspace(&config, &temp_manifest_path) ?;
    let (pkg_set, _resolve) = cargo::ops::resolve_ws(&workspace) ?;

    /*
    let workspace_state = resolve_cargo_core_sub_packages(&temp_manifest_path) ?;
    let pkg_set = &workspace_state.packages;
    */

    let mut copied = HashMap::<String, PathBuf>::new();

    for dep in deps {
        let dep_name = dep.0.as_str();
        let pkg_id = pkg_set.package_ids()
            .find(|e| e.name() == dep_name);
        let pkg_id = pkg_id.ok_or_else(||anyhow::anyhow!(
            "Package ID [{dep_name}] is not found in [{temp_manifest_path:?}].")) ?;

        let pkg = pkg_set.get_one(pkg_id) ?;
        let pkg_name = pkg.name().as_str();
        let copied_path = copy_package(project_dir, pkg_name, pkg.root()) ?;

        copied.insert(pkg_name.to_owned(), copied_path);
    }

    Ok(copied)
}
