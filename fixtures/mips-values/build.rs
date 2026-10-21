use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let source = manifest.join("start.c");
    let object = out.join("start.o");
    println!("cargo:rerun-if-changed={}", source.display());
    let status = Command::new("clang")
        .args([
            "--target=mips64-unknown-linux-gnuabi64",
            "-march=mips64r2",
            "-mmsa",
            "-mabi=64",
            "-mno-abicalls",
            "-G0",
            "-ffreestanding",
            "-O2",
            "-c",
        ])
        .arg(&source)
        .arg("-o")
        .arg(&object)
        .status()
        .expect("clang");
    assert!(status.success());
    println!("cargo:rustc-link-arg={}", object.display());
}
