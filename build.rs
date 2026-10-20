use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let simd128_c = manifest.join("src/wasm32/simd/simd128/simd128.c");
    let relaxed_c = manifest.join("src/wasm32/simd/relaxed_simd/relaxed_simd.c");
    let wide_arithmetic_c = manifest.join("src/wasm32/wide_arithmetic/wide_arithmetic.c");
    let sse2_c = manifest.join("src/x86/simd/sse2/sse2.c");
    let avx2_c = manifest.join("src/x86/simd/avx2/avx2.c");
    let avx512f_c = manifest.join("src/x86/simd/avx512f/avx512f.c");
    let avx512bw_c = manifest.join("src/x86/simd/avx512bw/avx512bw.c");
    let avx512dq_c = manifest.join("src/x86/simd/avx512dq/avx512dq.c");
    let avx512ifma_c = manifest.join("src/x86/simd/avx512ifma/avx512ifma.c");
    let avx512vbmi_c = manifest.join("src/x86/simd/avx512vbmi/avx512vbmi.c");
    let avx512vnni_c = manifest.join("src/x86/simd/avx512vnni/avx512vnni.c");
    let avx512vl_c = manifest.join("src/x86/simd/avx512vl/avx512vl.c");
    let aes_c = manifest.join("src/x86/crypto/aes/aes.c");
    let sha_c = manifest.join("src/x86/crypto/sha/sha.c");
    let gfni_c = manifest.join("src/x86/crypto/gfni/gfni.c");
    let vaes_c = manifest.join("src/x86/crypto/vaes/vaes.c");
    let vpclmulqdq_c = manifest.join("src/x86/crypto/vpclmulqdq/vpclmulqdq.c");
    let fma_c = manifest.join("src/x86/simd/fma/fma.c");
    let bmi1_c = manifest.join("src/x86/integer/bmi1/bmi1.c");
    let bmi2_c = manifest.join("src/x86/integer/bmi2/bmi2.c");
    let popcnt_c = manifest.join("src/x86/integer/popcnt/popcnt.c");
    let neon_c = manifest.join("src/aarch64/simd/neon/neon.c");
    let dotprod_c = manifest.join("src/aarch64/simd/dotprod/dotprod.c");
    let rv_vector_c = manifest.join("src/riscv/simd/rv_vector/rv_vector.c");
    let lsx_c = manifest.join("src/loongarch/simd/lsx/lsx.c");
    let s390x_vector_c = manifest.join("src/s390x/simd/vector/vector.c");
    let arm_neon_c = manifest.join("src/arm/simd/neon/neon.c");
    let altivec_c = manifest.join("src/powerpc/simd/altivec/altivec.c");
    let msa_c = manifest.join("src/mips/simd/msa/msa.c");
    for path in [
        &simd128_c,
        &relaxed_c,
        &wide_arithmetic_c,
        &sse2_c,
        &avx2_c,
        &avx512f_c,
        &avx512bw_c,
        &avx512dq_c,
        &avx512ifma_c,
        &avx512vbmi_c,
        &avx512vnni_c,
        &avx512vl_c,
        &aes_c,
        &sha_c,
        &gfni_c,
        &vaes_c,
        &vpclmulqdq_c,
        &fma_c,
        &bmi1_c,
        &bmi2_c,
        &popcnt_c,
        &neon_c,
        &dotprod_c,
        &rv_vector_c,
        &lsx_c,
        &s390x_vector_c,
        &arm_neon_c,
        &altivec_c,
        &msa_c,
    ] {
        println!("cargo:rerun-if-changed={}", path.display());
    }
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_FEATURE");

    let target = env::var("TARGET").unwrap_or_default();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    if target == "mips64-unknown-linux-gnuabi64" {
        compile_archive(
            &out_dir,
            &msa_c,
            "rust_isa_msa",
            &["-march=mips64r2", "-mmsa", "-mabicalls", "-mxgot"],
            Some("mips64-unknown-linux-gnuabi64"),
        );
        return;
    }

    if target == "powerpc64le-unknown-linux-gnu" {
        compile_archive(
            &out_dir,
            &altivec_c,
            "rust_isa_altivec",
            &["-maltivec", "-mvsx"],
            Some("powerpc64le-unknown-linux-gnu"),
        );
        return;
    }

    if target == "armv7-unknown-linux-gnueabihf" {
        compile_archive(
            &out_dir,
            &arm_neon_c,
            "rust_isa_arm_neon",
            &[
                "-mthumb",
                "-march=armv7-a",
                "-mfpu=neon",
                "-mfloat-abi=hard",
            ],
            Some("armv7-unknown-linux-gnueabihf"),
        );
        return;
    }

    if target == "s390x-unknown-linux-gnu" {
        compile_archive(
            &out_dir,
            &s390x_vector_c,
            "rust_isa_s390x_vector",
            &["-march=z13", "-mzvector"],
            Some("s390x-unknown-linux-gnu"),
        );
        return;
    }

    if target == "loongarch64-unknown-linux-gnu" {
        compile_archive(
            &out_dir,
            &lsx_c,
            "rust_isa_lsx",
            &["-mlsx"],
            Some("loongarch64-unknown-linux-gnu"),
        );
        return;
    }

    if target == "riscv64gc-unknown-linux-gnu" {
        compile_archive(
            &out_dir,
            &rv_vector_c,
            "rust_isa_rv_vector",
            &["-march=rv64gcv", "-mabi=lp64d", "-mno-relax"],
            Some("riscv64-unknown-linux-gnu"),
        );
        return;
    }

    if target == "aarch64-unknown-linux-gnu" {
        compile_archive(
            &out_dir,
            &neon_c,
            "rust_isa_neon",
            &["-march=armv8-a+simd"],
            Some("aarch64-unknown-linux-gnu"),
        );
        compile_archive(
            &out_dir,
            &dotprod_c,
            "rust_isa_dotprod",
            &["-march=armv8-a+dotprod"],
            Some("aarch64-unknown-linux-gnu"),
        );
        return;
    }

    if target == "x86_64-unknown-linux-gnu" || target == "i686-unknown-linux-gnu" {
        compile_native(&out_dir, &sse2_c, "rust_isa_sse2", &["-msse2"]);
        compile_native(
            &out_dir,
            &avx2_c,
            "rust_isa_avx2",
            &["-mavx2", "-mno-avx512f"],
        );
        compile_native(&out_dir, &avx512f_c, "rust_isa_avx512f", &["-mavx512f"]);
        compile_native(
            &out_dir,
            &avx512bw_c,
            "rust_isa_avx512bw",
            &["-mavx512f", "-mavx512bw"],
        );
        compile_native(
            &out_dir,
            &avx512dq_c,
            "rust_isa_avx512dq",
            &["-mavx512f", "-mavx512dq"],
        );
        compile_native(
            &out_dir,
            &avx512ifma_c,
            "rust_isa_avx512ifma",
            &["-mavx512f", "-mavx512ifma"],
        );
        compile_native(
            &out_dir,
            &avx512vbmi_c,
            "rust_isa_avx512vbmi",
            &["-mavx512f", "-mavx512vbmi"],
        );
        compile_native(
            &out_dir,
            &avx512vnni_c,
            "rust_isa_avx512vnni",
            &["-mavx512f", "-mavx512vnni"],
        );
        compile_native(
            &out_dir,
            &avx512vl_c,
            "rust_isa_avx512vl",
            &["-mavx512f", "-mavx512dq", "-mavx512vl"],
        );
        compile_native(&out_dir, &aes_c, "rust_isa_aes", &["-maes"]);
        compile_native(&out_dir, &sha_c, "rust_isa_sha", &["-msha"]);
        compile_native(&out_dir, &gfni_c, "rust_isa_gfni", &["-mgfni"]);
        compile_native(&out_dir, &vaes_c, "rust_isa_vaes", &["-mvaes", "-mavx2"]);
        compile_native(
            &out_dir,
            &vpclmulqdq_c,
            "rust_isa_vpclmulqdq",
            &["-mvpclmulqdq", "-mavx2", "-mavx512f", "-mavx512vl"],
        );
        compile_native(&out_dir, &fma_c, "rust_isa_fma", &["-mavx2", "-mfma"]);
        compile_native(&out_dir, &bmi1_c, "rust_isa_bmi1", &["-mbmi"]);
        compile_native(&out_dir, &bmi2_c, "rust_isa_bmi2", &["-mbmi2"]);
        compile_native(&out_dir, &popcnt_c, "rust_isa_popcnt", &["-mpopcnt"]);
        return;
    }

    if !target.starts_with("wasm32") {
        return;
    }
    let features = env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or_default();
    let simd128 = has_feature(&features, "simd128");
    let relaxed = has_feature(&features, "relaxed-simd");

    if simd128 {
        compile_archive(
            &out_dir,
            &simd128_c,
            "rust_isa_simd128",
            &["-msimd128"],
            Some("wasm32-unknown-unknown"),
        );
        if relaxed {
            compile_archive(
                &out_dir,
                &relaxed_c,
                "rust_isa_relaxed_simd",
                &["-msimd128", "-mrelaxed-simd"],
                Some("wasm32-unknown-unknown"),
            );
        }
    }
    if env::var_os("CARGO_FEATURE_WIDE_ARITHMETIC").is_some() {
        compile_archive(
            &out_dir,
            &wide_arithmetic_c,
            "rust_isa_wide_arithmetic",
            &["-mwide-arithmetic"],
            Some("wasm32-unknown-unknown"),
        );
    }
}

fn has_feature(features: &str, name: &str) -> bool {
    features.split([',', ' ']).any(|feature| feature == name)
}

fn compile_native(out_dir: &Path, c_path: &Path, name: &str, flags: &[&str]) {
    let target = env::var("TARGET").unwrap_or_default();
    let clang_target = if target == "i686-unknown-linux-gnu" {
        Some("i686-unknown-linux-gnu")
    } else {
        None
    };
    let mut all_flags = vec!["-mno-mmx", "-mno-x87", "-mno-popcnt", "-mno-xsave"];
    if clang_target.is_some() {
        all_flags.push("-mstackrealign");
    }
    all_flags.extend_from_slice(flags);
    compile_archive(out_dir, c_path, name, &all_flags, clang_target);
}

fn compile_archive(
    out_dir: &Path,
    c_path: &Path,
    name: &str,
    flags: &[&str],
    clang_target: Option<&str>,
) {
    let obj_path = out_dir.join(format!("{name}.o"));
    let mut command = Command::new("clang");
    if let Some(target) = clang_target {
        command.arg(format!("--target={target}"));
    }
    command.args([
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
