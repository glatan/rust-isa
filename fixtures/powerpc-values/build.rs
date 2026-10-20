use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let source = manifest.join("toc.s");
    let object = out.join("toc.o");
    println!("cargo:rerun-if-changed={}", source.display());
    let status = Command::new("clang")
        .args(["--target=powerpc64le-unknown-linux-gnu", "-c"])
        .arg(&source)
        .arg("-o")
        .arg(&object)
        .status()
        .expect("clang");
    assert!(status.success());
    println!("cargo:rustc-link-arg={}", object.display());
}
