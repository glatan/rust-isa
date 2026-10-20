#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::*;

#[test]
fn wide_arithmetic_pipeline() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    let dir = nested_dir("wide-link");
    let output = cargo(&dir)
        .args([
            "build",
            "-p",
            "wide-link",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let wasm = dir.join("wasm32-unknown-unknown/release/wide_link.wasm");
    assert!(wasm.is_file(), "missing {}", wasm.display());
    let opt = dir.join("wide-link.opt.wasm");
    let wat = dir.join("wide-link.wat");
    let status = Command::new("wasm-opt")
        .args(["-O3"])
        .arg(&wasm)
        .arg("-o")
        .arg(&opt)
        .status()
        .expect("wasm-opt");
    assert!(status.success(), "wasm-opt failed");
    let status = Command::new("wasm2wat")
        .arg(&opt)
        .arg("-o")
        .arg(&wat)
        .status()
        .expect("wasm2wat");
    assert!(status.success(), "wasm2wat failed");
    let text = fs::read_to_string(&wat).unwrap();
    for opcode in [
        "i64.add128",
        "i64.sub128",
        "i64.mul_wide_u",
        "i64.mul_wide_s",
    ] {
        assert!(text.contains(opcode), "missing {opcode}");
    }
}
