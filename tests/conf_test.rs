use std::path::Path;
use mvv_cargo_patch_subdep::conf::{
    gather_override_patch_conf_from_dir, ReplaceSubDepVersConfig, str_override_entry,
};
//--------------------------------------------------------------------------------------------------



#[test]
fn gather_override_patch_conf_from_dir_test() -> anyhow::Result<()> {
    let conf = gather_override_patch_conf_from_dir(Path::new("./test_resources/project_workspace_with_root_patch_conf")) ?;
    println!("\n\n### conf: {conf:?}");

    let expected_conf = ReplaceSubDepVersConfig::new([
        str_override_entry("progenitor-client", "reqwest", "0.11.27", "0.12.5"),
        str_override_entry("progenitor-client", "url", "2.5.0", "2.5.1"),
        str_override_entry("progenitor-client-macro", "http", "0.2.6", "0.2.9"),
        str_override_entry("progenitor-client-macro", "reqwest", "0.11.27", "0.12.5"),
    ]);

    assert_eq!(conf, expected_conf);
    // assert!(false, "Fail to see output");
    Ok(())
}
