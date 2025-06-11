// Licensed under the Apache-2.0 license

extern crate cbindgen;

use std::path::PathBuf;
use std::{env, str::FromStr};

use cbindgen::ExportConfig;

fn main() {
    // Get Crate dir
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let expansion_config = cbindgen::ParseExpandConfig {
        all_features: true,
        crates: vec![
            "caliptra-api-c-binding".to_owned(),
            "caliptra-api".to_owned(),
        ],
        ..Default::default()
    };

    let parse_config = cbindgen::ParseConfig {
        expand: expansion_config,
        ..Default::default()
    };

    let export_conf = ExportConfig {
        include: vec![
            "mailbox_command".to_owned(),
            "MailboxReqHeader".to_owned(),
            "MailboxRespHeader".to_owned(),
            "mailbox_response_header".to_owned(),
        ],
        ..Default::default()
    };

    // Generate Config
    let config = cbindgen::Config {
        header: Some(String::from_str("// Licensed under the Apache-2.0 license").unwrap()),
        language: cbindgen::Language::C,
        include_guard: Some("CALIPTRA_API_C_BINDING_H".to_string()),
        cpp_compat: true,
        parse: parse_config,
        export: export_conf,
        ..Default::default()
    };

    // Generate Output file
    let out_file = PathBuf::from(&crate_dir)
        .join("include")
        .join("caliptra_api.h");

    // Generate caliptra_model.h
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_file);
}
