use std::path::Path;
use std::fs;
use std::io::ErrorKind;
use fs_extra::dir::{ copy, CopyOptions };
//--------------------------------------------------------------------------------------------------



pub fn load_cargo_manifest(manifest_file_path: &Path) -> Result<cargo_toml::Manifest, anyhow::Error> {
    let as_str = fs::read_to_string(&manifest_file_path) ?;
    let manifest = cargo_toml::Manifest::from_str(&as_str) ?;
    Ok(manifest)
}


pub fn save_manifest(manifest: &cargo_toml::Manifest, manifest_file_path: &Path) -> Result<(), anyhow::Error> {
    let new_as_str = toml::to_string(&manifest) ?;
    fs::write(manifest_file_path, &new_as_str) ?;
    Ok(())
}


pub fn clear_dir(dir: &Path) -> Result<(), anyhow::Error> {
    match fs::remove_dir_all(dir) {
        Ok(_) => Ok(()),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => Ok(()),
            _ => Err(err.into()),
        },
    }
}


pub fn copy_dir(from_dir: &Path, to_dir: &Path) -> Result<(), anyhow::Error> {
    fs::create_dir_all(to_dir) ?;
    let options = CopyOptions::new();
    copy(from_dir, to_dir, &options) ?;
    Ok(())
}
