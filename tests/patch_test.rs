use std::fs;
use std::path::Path;
use mvv_cargo_patch_subdep::{
    cargo_core::{ fetch_cargo_core_workspace, setup_cargo_core_config },
    patch::do_patch_project,
    io::{ clear_dir, copy_dir },
};
use mvv_cargo_patch_subdep::deps::gather_all_dep_ver_from_dir;
use mvv_cargo_patch_subdep::util::string_hash_set;
//--------------------------------------------------------------------------------------------------


#[test]
fn do_patch_project_test() -> anyhow::Result<()> {

    let test_temp_dir = "target/tmp_test_resources/do_patch_project_test";
    clear_dir(Path::new(test_temp_dir)) ?;

    let example_project_path = format!("{test_temp_dir}/project_ok_1");
    let example_project_path = Path::new(&example_project_path);

    copy_dir(
        Path::new("./test_resources/project_ok_1"),
        Path::new(test_temp_dir),
    ) ?;

    let all_deps = gather_all_dep_ver_from_dir(example_project_path) ?;
    let req_versions = all_deps.get("reqwest");
    assert_eq!(
            req_versions,
            Some(&string_hash_set(["0.11.20", "0.12.1", "0.12.3", "0.12.4", "0.12.5"])),
        );

    do_patch_project(Path::new(example_project_path)) ?;
    open_workspace(example_project_path) ?;

    let all_deps = gather_all_dep_ver_from_dir(example_project_path) ?;
    let req_versions = all_deps.get("reqwest");
    assert_eq!(
            req_versions,
            Some(&string_hash_set(["0.11.20", "0.12.1", "0.12.3", "0.12.4", "0.12.5"])),
        );

    let patched_dep_path = example_project_path.join("target/patch-override-sub-dep/progenitor-client");
    let patched_dep_all_sub_deps = gather_all_dep_ver_from_dir(&patched_dep_path) ?;
    let req_versions = patched_dep_all_sub_deps.get("reqwest");
    // patched from "0.11.27" to "0.12.5"
    assert_eq!(req_versions, Some(&string_hash_set(["0.12.5"])));

    // Do it again
    do_patch_project(Path::new(&example_project_path)) ?;
    open_workspace(example_project_path) ?;

    let all_deps = gather_all_dep_ver_from_dir(example_project_path) ?;
    let req_versions = all_deps.get("reqwest");
    assert_eq!(
            req_versions,
            Some(&string_hash_set(["0.11.20", "0.12.1", "0.12.3", "0.12.4", "0.12.5"])),
        );

    let patched_dep_path = example_project_path.join("target/patch-override-sub-dep/progenitor-client");
    let patched_dep_all_sub_deps = gather_all_dep_ver_from_dir(&patched_dep_path) ?;
    let req_versions = patched_dep_all_sub_deps.get("reqwest");
    // patched from "0.11.27" to "0.12.5"
    assert_eq!(req_versions, Some(&string_hash_set(["0.12.5"])));

    Ok(())
}


fn open_workspace(project_dir: &Path) -> anyhow::Result<()> {

    // In rust should be applied only for existent file.
    let manifest_path = fs::canonicalize(project_dir.join("Cargo.toml")) ?;

    let config = setup_cargo_core_config() ?;
    let _lock = config.acquire_package_cache_lock(cargo::util::cache_lock::CacheLockMode::Shared) ?;
    let workspace = fetch_cargo_core_workspace(&config, &manifest_path) ?;
    let (_pkg_set, _resolve) = cargo::ops::resolve_ws(&workspace) ?;
    Ok(())
}
