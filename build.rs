use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let simd128_c = manifest.join("src/wasm32/simd/simd128/simd128.c");
    let relaxed_c = manifest.join("src/wasm32/simd/relaxed_simd/relaxed_simd.c");
    println!("cargo:rerun-if-changed={}", simd128_c.display());
    println!("cargo:rerun-if-changed={}", relaxed_c.display());
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_FEATURE");

    let target = env::var("TARGET").unwrap_or_default();
    if !target.starts_with("wasm32") {
        return;
    }
    let features = env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or_default();
    let simd128 = has_feature(&features, "simd128");
    let relaxed = has_feature(&features, "relaxed-simd");
    if !simd128 {
        return;
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    compile_archive(&out_dir, &simd128_c, "rust_isa_simd128", &["-msimd128"]);
    if relaxed {
        compile_archive(
            &out_dir,
            &relaxed_c,
            "rust_isa_relaxed_simd",
            &["-msimd128", "-mrelaxed-simd"],
        );
    }
}

fn has_feature(features: &str, name: &str) -> bool {
    features.split([',', ' ']).any(|feature| feature == name)
}

fn compile_archive(out_dir: &Path, c_path: &Path, name: &str, flags: &[&str]) {
    let obj_path = out_dir.join(format!("{name}.o"));
    let mut command = Command::new("clang");
    command.args([
        "--target=wasm32-unknown-unknown",
        "-c",
        "-O2",
        "-flto=thin",
        "-ffreestanding",
        "-fno-builtin",
        "-fno-vectorize",
        "-fno-slp-vectorize",
        "-Wall",
        "-Werror",
    ]);
    command.args(flags);
    command.arg("-o").arg(&obj_path).arg(c_path);
    let status = command
        .status()
        .unwrap_or_else(|_| panic!("clang was not found on PATH"));
    if !status.success() {
        panic!("clang failed to compile {}", c_path.display());
    }

    let archive = out_dir.join(format!("lib{name}.a"));
    let status = Command::new("llvm-ar")
        .arg("rcs")
        .arg(&archive)
        .arg(&obj_path)
        .status()
        .unwrap_or_else(|_| panic!("llvm-ar was not found on PATH"));
    if !status.success() {
        panic!("llvm-ar failed to archive {}", obj_path.display());
    }
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static={name}");
}
