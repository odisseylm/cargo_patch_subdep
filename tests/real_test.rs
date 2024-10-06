use std::path::PathBuf;
use mvv_cargo_patch_subdep_ver::patch::do_patch_project;



fn init_logger() {

    // Set environment for logging configuration
    // if std::env::var("RUST_LOG").is_err() {
    //     std::env::set_var("RUST_LOG", "info");
    // }

    // env_logger::init();
    env_logger::builder()
        .filter(None, log::LevelFilter::Debug)
        .init();
}


#[test]
#[ignore]
fn apply_for_rust_study_project_01() {
    init_logger();

    do_patch_project(&PathBuf::from("/home/volodymyr/projects/rust/rust-study-project-01"))
        .unwrap()
}
