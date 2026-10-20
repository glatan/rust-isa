#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::*;

#[test]
fn s390x_values() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    let dir = nested_dir("s390x-values");
    let rustflags = "-C target-feature=+vector -C linker-plugin-lto -C linker=clang -C link-arg=--target=s390x-unknown-linux-gnu -C link-arg=-fuse-ld=lld -C link-arg=-static -C link-arg=-nostdlib -C relocation-model=static";
    let output = cargo(&dir)
        .env("RUSTC_BOOTSTRAP", "1")
        .env("RUSTFLAGS", rustflags)
        .args([
            "build",
            "--manifest-path",
            "fixtures/s390x-values/Cargo.toml",
            "-Z",
            "build-std=core",
            "--target",
            "s390x-unknown-linux-gnu",
            "--release",
        ])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("s390x-unknown-linux-gnu/release/s390x-values");
    assert!(binary.is_file(), "missing {}", binary.display());
    let output = Command::new("qemu-s390x-static")
        .arg(&binary)
        .output()
        .expect("qemu-s390x-static");
    assert!(
        output.status.success(),
        "s390x value test failed: {}\n{}",
        output.status,
        output_text(&output)
    );
}
