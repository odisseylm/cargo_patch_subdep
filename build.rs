
fn main() {
    let cargo_core_ver = get_cargo_crate_version();
    if let Some(ref cargo_core_ver) = cargo_core_ver {
        let cargo_core_ver_id = get_cargo_core_ver_id(cargo_core_ver);
        if !cargo_core_ver_id.is_empty() {
            println!(r#"cargo::rustc-cfg=cargo_core_ver_prefix="{cargo_core_ver_id}""#);
        }
    }
}


fn get_cargo_crate_version() -> Option<String> {

    let cmd = cargo_metadata::MetadataCommand::new();
    let cargo_metadata_res = cmd.exec();
    let resolve1 = cargo_metadata_res.expect("Cargo-metadata failed.")
        .resolve.expect("Problem with Cargo-metadata 'resolve' section.");
    let cargo_core_ver = resolve1.nodes.iter()
        .find_map(|el|{
            let repr = el.id.repr.as_str();
            let index = repr.find("crates.io-index#cargo@");
            match index {
                None => None,
                Some(index) => {
                    let version = repr[index + "crates.io-index#cargo@".len()..].trim();
                    if version.is_empty() {
                        None
                    } else {
                        Some(version)
                    }
                }
            }
        });

    cargo_core_ver.map(|v|v.to_owned())
}


fn get_cargo_core_ver_id(cargo_core_ver: &str) -> &'static str {
    if cargo_core_ver.starts_with("0.5") {
        "05x"
    } else if cargo_core_ver.starts_with("0.6") {
        "06x"
    } else if cargo_core_ver.starts_with("0.70") {
        "07x"
    } else if cargo_core_ver.starts_with("0.79") {
        "07next_x"
    // } else if cargo_core_ver.starts_with(0.78 - 0.79) { Deps are not compiled now
    //     "07next_x"
    } else if cargo_core_ver.starts_with("0.8") {
        "08x"
    } else if cargo_core_ver.starts_with("0.9") {
        "09x"
    } else if cargo_core_ver.starts_with("0.10") {
        "010x"
    } else if cargo_core_ver.starts_with("0.11") {
        "011x"
    } else if cargo_core_ver.starts_with("0.12") {
        "012x"
    } else if cargo_core_ver.starts_with("0.13") {
        "013x"
    } else if cargo_core_ver.starts_with("0.14") {
        "014x"
    } else if cargo_core_ver.starts_with("1.0") {
        "1_0x"
    } else if cargo_core_ver.starts_with("1.1") {
        "1_1x"
    } else if cargo_core_ver.starts_with("1.2") {
        "1_2x"
    } else if cargo_core_ver.starts_with("1.3") {
        "1_3x"
    } else if cargo_core_ver.starts_with("1.4") {
        "1_4x"
    } else if cargo_core_ver.starts_with("1.5") {
        "1_5x"
    } else {
        ""
    }
}
