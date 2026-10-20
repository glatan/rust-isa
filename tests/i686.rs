#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::*;

#[test]
fn i686_values() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    let dir = nested_dir("i686-values");
    let rustflags = "-C linker-plugin-lto -C linker=clang -C link-arg=--target=i686-unknown-linux-gnu -C link-arg=-fuse-ld=lld -C link-arg=-static -C link-arg=-nostdlib -C relocation-model=static";
    let output = cargo(&dir)
        .env("RUSTC_BOOTSTRAP", "1")
        .env("RUSTFLAGS", rustflags)
        .args([
            "build",
            "--manifest-path",
            "fixtures/i686-values/Cargo.toml",
            "-Z",
            "build-std=core",
            "--target",
            "i686-unknown-linux-gnu",
            "--release",
        ])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("i686-unknown-linux-gnu/release/i686-values");
    assert!(binary.is_file(), "missing {}", binary.display());
    let output = Command::new("qemu-i386-static")
        .arg("-cpu")
        .arg("max")
        .arg(&binary)
        .output()
        .expect("qemu-i386-static");
    assert!(
        output.status.success(),
        "i686 value test failed: {}\n{}",
        output.status,
        output_text(&output)
    );
}
