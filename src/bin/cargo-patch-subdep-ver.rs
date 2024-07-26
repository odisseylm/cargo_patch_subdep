use mvv_cargo_patch_subdep::patch::do_patch_project;

fn main() -> anyhow::Result<()> {
    let project_dir = std::env::current_dir() ?;
    do_patch_project(&project_dir)
}
