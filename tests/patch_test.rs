use std::fs;
use std::path::Path;
use mvv_cargo_patch_subdep::{
    cargo_core::{ fetch_cargo_core_workspace, setup_cargo_core_config, acquire_cargo_core_package_cache_lock },
    deps::gather_all_dep_ver_from_dir,
    patch::do_patch_project,
    io::{ clear_dir, copy_dir },
    util::string_hash_set,
};
//--------------------------------------------------------------------------------------------------


#[test]
fn do_patch_project_test() -> anyhow::Result<()> {

    let test_temp_dir = "target/tmp_test_resources/do_patch_project_test";
    clear_dir(Path::new(test_temp_dir)) ?;

    let prj_name = if cfg!(any(cargo_core_ver_prefix = "05x")) {
        "project_ok_1_OLD_2018"
    } else if cfg!(any(cargo_core_ver_prefix = "06x")) {
        "project_ok_1_OLD"
    } else {
        "project_ok_1"
    };

    let example_project_path = format!("{test_temp_dir}/{prj_name}");
    let example_project_path = Path::new(&example_project_path);

    copy_dir(
        Path::new(&format!("./test_resources/{prj_name}")),
        Path::new(test_temp_dir),
    ) ?;

    #[allow(unused_variables, dead_code)]
    let patch_sub_dep_reqwest_to = "0.12.5";
    #[cfg(any(cargo_core_ver_prefix = "05x", cargo_core_ver_prefix = "06x"))]
    let patch_sub_dep_reqwest_to = "0.11.23";

    #[allow(unused_variables, dead_code)]
    let expected_req_versions = ["0.11.20", "0.12.1", "0.12.3", "0.12.4", "0.12.5"];
    #[cfg(any(cargo_core_ver_prefix = "05x", cargo_core_ver_prefix = "06x"))]
    let expected_req_versions = ["0.11.20", "0.11.21", "0.11.22"];

    let all_deps = gather_all_dep_ver_from_dir(example_project_path) ?;
    let req_versions = all_deps.get("reqwest");
    assert_eq!(
            req_versions,
            Some(&string_hash_set(expected_req_versions)),
        );

    do_patch_project(Path::new(example_project_path)) ?;
    open_workspace(example_project_path) ?;

    #[allow(unused_variables, dead_code)]
    let expected_req_versions = ["0.11.20", "0.12.1", "0.12.3", "0.12.4", "0.12.5"];
    #[cfg(any(cargo_core_ver_prefix = "05x", cargo_core_ver_prefix = "06x"))]
    let expected_req_versions = ["0.11.20", "0.11.21", "0.11.22", "0.11.23"];

    let all_deps = gather_all_dep_ver_from_dir(example_project_path) ?;
    let req_versions = all_deps.get("reqwest");
    assert_eq!(
            req_versions,
            Some(&string_hash_set(expected_req_versions)),
        );

    let patched_dep_path = example_project_path.join("target/patch-override-sub-dep/progenitor-client");
    let patched_dep_all_sub_deps = gather_all_dep_ver_from_dir(&patched_dep_path) ?;
    let req_versions = patched_dep_all_sub_deps.get("reqwest");
    // patched from "0.11.27" to "0.12.5" (or from "0.11" to "0.11.23")
    assert_eq!(req_versions, Some(&string_hash_set([patch_sub_dep_reqwest_to])));

    // Do it again
    do_patch_project(Path::new(&example_project_path)) ?;
    open_workspace(example_project_path) ?;

    #[allow(unused_variables, dead_code)]
    let expected_req_versions = ["0.11.20", "0.12.1", "0.12.3", "0.12.4", "0.12.5"];
    #[cfg(any(cargo_core_ver_prefix = "05x", cargo_core_ver_prefix = "06x"))]
    let expected_req_versions = ["0.11.20", "0.11.21", "0.11.22", "0.11.23"];

    let all_deps = gather_all_dep_ver_from_dir(example_project_path) ?;
    let req_versions = all_deps.get("reqwest");
    assert_eq!(
            req_versions,
            Some(&string_hash_set(expected_req_versions)),
        );

    let patched_dep_path = example_project_path.join("target/patch-override-sub-dep/progenitor-client");
    let patched_dep_all_sub_deps = gather_all_dep_ver_from_dir(&patched_dep_path) ?;
    let req_versions = patched_dep_all_sub_deps.get("reqwest");
    // patched from "0.11.27" to "0.12.5" (or from "0.11" to "0.11.23")
    assert_eq!(req_versions, Some(&string_hash_set([patch_sub_dep_reqwest_to])));

    Ok(())
}


fn open_workspace(project_dir: &Path) -> anyhow::Result<()> {
    if cfg!(any(cargo_core_ver_prefix = "05x")) {
        // feature `resolver` is required
        //
        // this Cargo does not support nightly features, but if you
        // switch to nightly channel you can add
        // `cargo-features = ["resolver"]` to enable this feature
        return Ok(());
    }

    // In rust should be applied only for existent file.
    let manifest_path = fs::canonicalize(project_dir.join("Cargo.toml")) ?;

    let config = setup_cargo_core_config() ?;
    let _lock = acquire_cargo_core_package_cache_lock(&config) ?;
    let workspace = fetch_cargo_core_workspace(&config, &manifest_path) ?;
    let (_pkg_set, _resolve) = cargo::ops::resolve_ws(&workspace) ?;
    Ok(())
}
