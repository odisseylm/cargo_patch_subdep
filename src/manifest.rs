use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
//--------------------------------------------------------------------------------------------------



pub fn gather_manifest_files(dir: &Path) -> Result<Vec<PathBuf>, anyhow::Error> {
    let mut manifests = Vec::<PathBuf>::with_capacity(10);
    gather_manifest_files_impl(dir, &mut manifests) ?;
    Ok(manifests)
}


fn gather_manifest_files_impl(dir: &Path, manifests: &mut Vec<PathBuf>) -> Result<(), anyhow::Error> {
    use core::str::FromStr;
    let cargo_toml = OsString::from_str("Cargo.toml") ?;
    let cargo_toml_opt = Some(cargo_toml.as_os_str());

    for entry in fs::read_dir(dir)? {
        let entry = entry ?;
        let path = entry.path();
        let metadata = fs::metadata(&path) ?;

        if metadata.is_symlink() {
            // skip
        } else if metadata.is_file() {
            if path.file_name() == cargo_toml_opt {
                manifests.push(path.to_path_buf())
            }
        } else if metadata.is_dir() {
            gather_manifest_files_impl(&path, manifests) ?
        }
    }

    Ok(())
}
