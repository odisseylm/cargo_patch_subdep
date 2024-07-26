use std::fs;
use std::path::Path;
use mvv_cargo_patch_subdep::{
    conf::{ OverrideSubDepConfig, str_override_entry },
    deps::gather_all_dep_ver_from_dir,
    patch::replace_deps_version_in_file_tree,
    load_src::load_dep_sources,
    io::{ clear_dir, copy_dir },
    util::{ string_hash_map, string_hash_set },
};
//--------------------------------------------------------------------------------------------------



#[test]
fn replace_deps_version_in_file_tree_real_test() -> Result<(), anyhow::Error> {
    let conf = OverrideSubDepConfig::new([
        str_override_entry(
            "*", // not used on phase of overriding
            "reqwest", "0.11.27", "0.12.5"),
        str_override_entry(
            "*", // not used on phase of overriding
            "reqwest", "0.11", "0.12",
        ),
    ]);

    let test_temp_dir = "target/tmp_test_resources/replace_deps_version_in_file_tree_real_test";
    let example_project_path = &format!("{test_temp_dir}/project_ok_real_case_deps_to_path");
    clear_dir(Path::new(example_project_path)) ?;
    fs::create_dir_all(example_project_path) ?;

    copy_dir(
        Path::new("./test_resources/project_ok_real_case_deps_to_path"),
        Path::new(test_temp_dir),
    ) ?;

    let all_deps = gather_all_dep_ver_from_dir(
        Path::new(example_project_path)) ?;
    let req_versions = all_deps.get("reqwest");
    assert_eq!(
            req_versions,
            Some(&string_hash_set(["", "0.11.27", "0.12.5"])),
        );

    replace_deps_version_in_file_tree(
        Path::new(example_project_path), &conf) ?;

    let all_deps = gather_all_dep_ver_from_dir(
        Path::new(example_project_path)) ?;
    let req_versions = all_deps.get("reqwest");
    assert_eq!(
            req_versions,
            Some(&string_hash_set(["", "0.12.5"])),
        );

    Ok(())
}


#[test]
fn replace_deps_version_in_file_tree_synthetic_test() -> Result<(), anyhow::Error> {
    let conf = OverrideSubDepConfig::new([
        str_override_entry(
            "*", // not used on phase of overriding
            "reqwest", "0.12.1", "0.1012.1"),
        str_override_entry(
            "*", // not used on phase of overriding
            "reqwest", "0.12.2", "0.1012.2"),
        str_override_entry(
            "*", // not used on phase of overriding
            "reqwest", "0.12.3", "0.1012.3"),
        str_override_entry(
            "*", // not used on phase of overriding
            "reqwest", "0.12.4", "0.1012.4"),
        str_override_entry(
            "*", // not used on phase of overriding
            "reqwest", "0.12.5", "0.1012.5"),
    ]);

    let test_temp_dir = "target/tmp_test_resources/replace_deps_version_in_file_tree_synthetic_test";
    let example_project_path = &format!("{test_temp_dir}/project_ok_1");
    clear_dir(Path::new(example_project_path)) ?;
    fs::create_dir_all(example_project_path) ?;

    copy_dir(
        Path::new("./test_resources/project_ok_1"),
        Path::new(test_temp_dir),
    ) ?;

    let all_deps = gather_all_dep_ver_from_dir(
        Path::new(example_project_path)) ?;
    let req_versions = all_deps.get("reqwest");
    assert_eq!(
            req_versions,
            Some(&string_hash_set(["0.11.20", "0.12.1", "0.12.3", "0.12.4", "0.12.5"])),
        );

    replace_deps_version_in_file_tree(
        Path::new(example_project_path), &conf) ?;

    let all_deps = gather_all_dep_ver_from_dir(
        Path::new(example_project_path)) ?;
    let req_versions = all_deps.get("reqwest");
    assert_eq!(
            req_versions,
            Some(&string_hash_set([
                // not changed because there was no rule for "0.11.20"
                "0.11.20",
                // changed
                "0.1012.1", "0.1012.4", "0.1012.3", "0.1012.5"])),
        );

    Ok(())
}


#[test]
fn load_dep_sources_test() -> Result<(), anyhow::Error> {

    let test_temp_dir = "target/tmp_test_resources/load_dep_sources_test";
    let example_project_path = format!("{test_temp_dir}/project_load_dep_src");
    let example_project_path = Path::new(&example_project_path);
    clear_dir(Path::new(example_project_path)) ?;
    fs::create_dir_all(test_temp_dir) ?;

    let src_path = example_project_path.join("src");
    fs::create_dir_all(&src_path) ?;
    fs::write(&src_path.join("lib.rs"), "") ?;

    let progenitor_ver = if cfg!(any(cargo_core_ver_prefix = "05x")) {
        "0.1.1"
    } else if cfg!(any(cargo_core_ver_prefix = "06x")) {
        // "0.7.0", "0.6.0", "0.5.0", "0.4.1", "0.3.0" => deps errors like:
        //   Error: failed to select a version for the requirement `clap = "^4.2.5"`
        //   candidate versions found which didn't match: 3.2.25, 3.2.24, 3.2.23, ...
        "0.2.0" // "0.1.1";
    } else {
        "0.7.0"
    };

    load_dep_sources(
        &example_project_path, &string_hash_map([
            ("progenitor", progenitor_ver),
            ("progenitor-client", progenitor_ver),
            ("progenitor-impl", progenitor_ver),
            ("progenitor-macro", progenitor_ver),
        ])) ?;

    let base_load_path = Path::new(
        "./target/tmp_test_resources/load_dep_sources_test/project_load_dep_src/target/patch-override-sub-dep");

    assert!(base_load_path.join(&format!("progenitor/progenitor-{progenitor_ver}")).exists());
    assert!(base_load_path.join(&format!("progenitor-client/progenitor-client-{progenitor_ver}")).exists());
    assert!(base_load_path.join(&format!("progenitor-impl/progenitor-impl-{progenitor_ver}")).exists());
    assert!(base_load_path.join(&format!("progenitor-macro/progenitor-macro-{progenitor_ver}")).exists());

    Ok(())
}
