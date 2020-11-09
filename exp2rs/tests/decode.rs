use std::{env, fs, io::Write, path::*};

#[ignore]
#[test]
fn decode() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // load EXPRESS
    let code = fs::read_to_string(root.join("express/test.exp")).unwrap();
    let schemas = exp2rs::decode(&code).unwrap();

    // Generate Rust code
    let out_dir = root.join("generated");
    if !out_dir.exists() {
        fs::create_dir_all(&out_dir).unwrap();
    }
    let dest = out_dir.join("decoded.rs");
    let mut file = std::fs::File::create(&dest).unwrap();
    file.write(b"#![allow(dead_code)]\n").unwrap();
    for schema in schemas {
        file.write(schema.rust_code().as_bytes()).unwrap();
    }

    // Test the generate Rust code is compile-able
    let t = trybuild::TestCases::new();
    t.pass(dest);
}