use std::path::Path;
use mvv_cargo_patch_subdep_ver::{
    conf::{ ReplaceSubDepVersConfig, str_override_entry },
    deps::gather_patching_deps_from_dir,
    util::string_hash_map_1,
};
//--------------------------------------------------------------------------------------------------



fn test_override_config() -> ReplaceSubDepVersConfig {
    ReplaceSubDepVersConfig::new([
        str_override_entry("progenitor-client", "reqwest", "0.11.27", "0.12.5"),
        str_override_entry("progenitor-client", "reqwest", "0.11", "0.12"),
    ])
}



#[test]
fn gather_patching_dependencies_test_if_ok() -> Result<(), anyhow::Error> {
    let conf = test_override_config();
    let deps = gather_patching_deps_from_dir(
        Path::new("./test_resources/project_ok_1"),
        &conf,
    ) ?;

    println!("### deps: {deps:?}");
    assert_eq!(
            deps,
            string_hash_map_1("progenitor-client", "0.7.0"),
        );

    // assert!(false, "Test error to see output");
    Ok(())
}

#[test]
#[should_panic(expected = "Different versions of [progenitor-client] are found [0.6.0] and [0.7.0]")]
fn gather_patching_dependencies_test_if_dif_versions() {
    let conf = test_override_config();
    gather_patching_deps_from_dir(
        Path::new("./test_resources/project_err_with_dif_dep_versions"),
        &conf,
    ).unwrap();
}
