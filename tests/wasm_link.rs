#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::*;

#[test]
fn wasm_pipeline() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    check_link();
    check_without_simd128();
}

fn check_link() {
    let dir = nested_dir("link");
    let output = cargo(&dir)
        .args([
            "build",
            "-p",
            "simd128-link",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let wasm = dir.join("wasm32-unknown-unknown/release/simd128_link.wasm");
    assert!(wasm.is_file(), "missing {}", wasm.display());
    let opt = dir.join("simd128-link.opt.wasm");
    let wat = dir.join("simd128-link.wat");
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
    check_wat(&fs::read_to_string(&wat).unwrap());

    assert_undefined_symbols(&newest_archive(&dir, "librust_isa_simd128.a"));
    assert_undefined_symbols(&newest_archive(&dir, "librust_isa_relaxed_simd.a"));
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
        if parts.next() == Some("U") {
            let symbol = parts.next().unwrap_or("");
            assert_eq!(
                symbol, "__stack_pointer",
                "unexpected undefined symbol {line}"
            );
        }
    }
}

fn check_without_simd128() {
    let dir = nested_dir("no-simd");
    let output = cargo(&dir)
        .env("RUSTFLAGS", "")
        .args([
            "build",
            "-p",
            "rust-isa",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));
    for path in files_under(&dir) {
        if path.extension().and_then(|ext| ext.to_str()) != Some("rlib") {
            continue;
        }
        let nm = Command::new("llvm-nm")
            .arg(&path)
            .output()
            .expect("llvm-nm");
        if !nm.status.success() {
            continue;
        }
        let text = String::from_utf8_lossy(&nm.stdout);
        assert!(
            !text.contains("rust_isa_simd128_") && !text.contains("rust_isa_relaxed_simd_"),
            "{} references a SIMD symbol:\n{text}",
            path.display()
        );
    }

    let output = cargo(&dir)
        .env("RUSTFLAGS", "")
        .args([
            "build",
            "-p",
            "simd128-link",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .output()
        .expect("cargo");
    let text = output_text(&output);
    assert!(
        !output.status.success(),
        "simd128-link built without simd128:\n{text}"
    );
    assert!(
        text.contains("simd128-link requires target_feature simd128 and relaxed-simd"),
        "{text}"
    );
}
