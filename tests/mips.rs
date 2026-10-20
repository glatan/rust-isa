#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::*;

#[test]
fn mips_values() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    let dir = nested_dir("mips-values");
    let rustflags = "-C target-feature=+msa,+fp64 -C linker-plugin-lto -C linker=clang -C link-arg=--target=mips64-unknown-linux-gnuabi64 -C link-arg=-fuse-ld=lld -C link-arg=-static -C link-arg=-nostdlib -C link-arg=-Wl,-e,_start -C relocation-model=static";
    let output = cargo(&dir)
        .env("RUSTC_BOOTSTRAP", "1")
        .env("RUSTFLAGS", rustflags)
        .args([
            "build",
            "--manifest-path",
            "fixtures/mips-values/Cargo.toml",
            "-Z",
            "build-std=core",
            "--target",
            "mips64-unknown-linux-gnuabi64",
            "--release",
        ])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("mips64-unknown-linux-gnuabi64/release/mips-values");
    assert!(binary.is_file(), "missing {}", binary.display());
    let output = Command::new("qemu-mips64-static")
        .arg(&binary)
        .output()
        .expect("qemu-mips64-static");
    assert!(
        output.status.success(),
        "mips value test failed: {}\n{}",
        output.status,
        output_text(&output)
    );
}
