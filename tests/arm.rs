#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::*;

#[test]
fn arm_values() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    let dir = nested_dir("arm-values");
    let rustflags = "-C target-feature=+neon -C linker-plugin-lto -C linker=clang -C link-arg=--target=armv7-unknown-linux-gnueabihf -C link-arg=-fuse-ld=lld -C link-arg=-static -C link-arg=-nostdlib -C relocation-model=static";
    let output = cargo(&dir)
        .env("RUSTC_BOOTSTRAP", "1")
        .env("RUSTFLAGS", rustflags)
        .args([
            "build",
            "--manifest-path",
            "fixtures/arm-values/Cargo.toml",
            "-Z",
            "build-std=core",
            "--target",
            "armv7-unknown-linux-gnueabihf",
            "--release",
        ])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("armv7-unknown-linux-gnueabihf/release/arm-values");
    assert!(binary.is_file(), "missing {}", binary.display());
    let output = Command::new("qemu-arm-static")
        .arg("-cpu")
        .arg("max")
        .arg(&binary)
        .output()
        .expect("qemu-arm-static");
    assert!(
        output.status.success(),
        "arm value test failed: {}\n{}",
        output.status,
        output_text(&output)
    );
}

#[test]
fn arm_mve_values() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    let dir = nested_dir("arm-mve-values");
    let rustflags = "-C target-feature=+thumb-mode -C linker-plugin-lto -C linker=clang -C link-arg=--target=armv7-unknown-linux-gnueabihf -C link-arg=-fuse-ld=lld -C link-arg=-static -C link-arg=-nostdlib -C relocation-model=static";
    let output = cargo(&dir)
        .env("RUSTC_BOOTSTRAP", "1")
        .env("RUSTFLAGS", rustflags)
        .args([
            "build",
            "--manifest-path",
            "fixtures/arm-values/Cargo.toml",
            "--features",
            "mve",
            "-Z",
            "build-std=core",
            "--target",
            "armv7-unknown-linux-gnueabihf",
            "--release",
        ])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("armv7-unknown-linux-gnueabihf/release/arm-values");
    assert!(binary.is_file(), "missing {}", binary.display());
    let output = Command::new("qemu-arm-static")
        .arg("-cpu")
        .arg("cortex-m55")
        .arg(&binary)
        .output()
        .expect("qemu-arm-static");
    assert!(
        output.status.success(),
        "arm MVE value test failed: {}\n{}",
        output.status,
        output_text(&output)
    );
}
