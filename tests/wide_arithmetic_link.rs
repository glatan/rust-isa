#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::*;

#[test]
fn wide_arithmetic_pipeline() {
    let dir = nested_dir("wide-arithmetic-link");
    let output = cargo(&dir)
        .args([
            "build",
            "-p",
            "wide-arithmetic-link",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let wasm = dir.join("wasm32-unknown-unknown/release/wide_arithmetic_link.wasm");
    assert!(wasm.is_file(), "missing {}", wasm.display());
    let opt = dir.join("wide-arithmetic-link.opt.wasm");
    let wat = dir.join("wide-arithmetic-link.wat");
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

    for (kernel, opcode) in [
        ("kernel_add", "i64.add128"),
        ("kernel_sub", "i64.sub128"),
        ("kernel_mul_u", "i64.mul_wide_u"),
        ("kernel_mul_s", "i64.mul_wide_s"),
    ] {
        let body = function_body(&text, kernel);
        reject_call(kernel, body);
        assert_eq!(
            body.matches(opcode).count(),
            1,
            "{kernel} does not contain exactly one {opcode}:\n{body}"
        );
    }

    assert_undefined_symbols(&newest_archive(&dir, "librust_isa_wide_arithmetic.a"));
}

fn assert_undefined_symbols(archive: &Path) {
    let nm = Command::new("llvm-nm")
        .args(["-u"])
        .arg(archive)
        .output()
        .expect("llvm-nm");
    assert!(nm.status.success(), "{}", output_text(&nm));
    let undefined = String::from_utf8_lossy(&nm.stdout);
    for line in undefined.lines() {
        let mut parts = line.split_whitespace();
        if matches!(parts.next(), Some("U")) {
            let symbol = parts.next().unwrap_or("");
            assert_eq!(
                symbol, "__stack_pointer",
                "unexpected undefined symbol {line}"
            );
        }
    }
}
