use std::fs;
use std::path::Path;
use mvv_cargo_patch_subdep::{
    cargo_core::{ fetch_cargo_core_workspace, setup_cargo_core_config },
    patch::do_patch_project,
    io::{ clear_dir, copy_dir },
};
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

    // assert!(false, "Test qwerty");

    do_patch_project(Path::new(example_project_path)) ?;
    open_workspace(example_project_path) ?;

    // TODO: add asserts

    // Do it again
    do_patch_project(Path::new(&example_project_path)) ?;
    open_workspace(example_project_path) ?;

    // TODO: add asserts

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
