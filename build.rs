
fn main() {
    let cargo_core_ver = get_cargo_crate_version();
    if let Some(ref cargo_core_ver) = cargo_core_ver {
        let cargo_core_feature = get_cargo_core_ver_feature(cargo_core_ver);
        if !cargo_core_feature.is_empty() {
            let cargo_core_ver_prefix = cargo_core_feature.strip_prefix("cargo_core_")
                .expect(&format!("Unexpected format of cargo_core_feature [{cargo_core_feature}]."));
            println!(r#"cargo::rustc-cfg=cargo_core_ver_prefix="{cargo_core_ver_prefix}""#);
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


fn get_cargo_core_ver_feature(cargo_core_ver: &str) -> &'static str {
    if cargo_core_ver.starts_with("0.5") {
        "cargo_core_05x"
    } else if cargo_core_ver.starts_with("0.6") {
        "cargo_core_06x"
    } else if cargo_core_ver.starts_with("0.70") {
        "cargo_core_07x"
    } else if cargo_core_ver.starts_with("0.79") {
        "cargo_core_07next_x"
    // } else if cargo_core_ver.starts_with(0.78 - 0.79) { Deps are not compiled now
    //     "cargo_core_07next_x"
    } else if cargo_core_ver.starts_with("0.8") {
        "cargo_core_08x"
    } else if cargo_core_ver.starts_with("0.9") {
        "cargo_core_09x"
    } else if cargo_core_ver.starts_with("0.10") {
        "cargo_core_010x"
    } else if cargo_core_ver.starts_with("0.11") {
        "cargo_core_011x"
    } else if cargo_core_ver.starts_with("0.12") {
        "cargo_core_012x"
    } else if cargo_core_ver.starts_with("0.13") {
        "cargo_core_013x"
    } else if cargo_core_ver.starts_with("0.14") {
        "cargo_core_014x"
    } else if cargo_core_ver.starts_with("1.0") {
        "cargo_core_1_0x"
    } else if cargo_core_ver.starts_with("1.1") {
        "cargo_core_1_1x"
    } else if cargo_core_ver.starts_with("1.2") {
        "cargo_core_1_2x"
    } else if cargo_core_ver.starts_with("1.3") {
        "cargo_core_1_3x"
    } else if cargo_core_ver.starts_with("1.4") {
        "cargo_core_1_4x"
    } else if cargo_core_ver.starts_with("1.5") {
        "cargo_core_1_5x"
    } else {
        ""
    }
}
