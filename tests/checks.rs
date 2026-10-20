#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::*;

const SYMBOL_COUNT: usize = 374;

#[test]
fn check_rust_and_c_symbols() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/wasm32/simd/simd128");
    let mut rust_symbols = BTreeSet::new();
    for path in rust_sources(&root) {
        rust_symbols.extend(rust_symbols_in(
            &fs::read_to_string(&path).unwrap(),
            "rust_isa_simd128_",
        ));
    }
    let c_symbols = c_symbols_in(
        &fs::read_to_string(root.join("simd128.c")).unwrap(),
        "rust_isa_simd128_",
    );
    let missing_in_c: Vec<_> = rust_symbols.difference(&c_symbols).collect();
    let missing_in_rust: Vec<_> = c_symbols.difference(&rust_symbols).collect();
    assert!(
        missing_in_c.is_empty() && missing_in_rust.is_empty(),
        "missing in C: {missing_in_c:?}\nmissing in Rust: {missing_in_rust:?}"
    );
    assert_eq!(rust_symbols.len(), SYMBOL_COUNT);
}

#[test]
fn check_relaxed_symbols() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/wasm32/simd/relaxed_simd");
    let mut rust_symbols = BTreeSet::new();
    for path in rust_sources(&root) {
        rust_symbols.extend(rust_symbols_in(
            &fs::read_to_string(&path).unwrap(),
            "rust_isa_relaxed_simd_",
        ));
    }
    let c_symbols = c_symbols_in(
        &fs::read_to_string(root.join("relaxed_simd.c")).unwrap(),
        "rust_isa_relaxed_simd_",
    );
    let missing_in_c: Vec<_> = rust_symbols.difference(&c_symbols).collect();
    let missing_in_rust: Vec<_> = c_symbols.difference(&rust_symbols).collect();
    assert!(
        missing_in_c.is_empty() && missing_in_rust.is_empty(),
        "missing in C: {missing_in_c:?}\nmissing in Rust: {missing_in_rust:?}"
    );
    assert_eq!(rust_symbols.len(), 20);
}

#[test]
fn check_x86_symbols_match() {
    check_x86_symbols("simd/sse2", "sse2.c", "rust_isa_sse2_");
    check_x86_symbols("simd/avx2", "avx2.c", "rust_isa_avx2_");
    check_x86_symbols("simd/avx512f", "avx512f.c", "rust_isa_avx512f_");
    check_x86_symbols("simd/avx512bw", "avx512bw.c", "rust_isa_avx512bw_");
    check_x86_symbols("simd/avx512dq", "avx512dq.c", "rust_isa_avx512dq_");
    check_x86_symbols("simd/avx512ifma", "avx512ifma.c", "rust_isa_avx512ifma_");
    check_x86_symbols("simd/avx512vbmi", "avx512vbmi.c", "rust_isa_avx512vbmi_");
    check_x86_symbols("simd/avx512vnni", "avx512vnni.c", "rust_isa_avx512vnni_");
    check_x86_symbols("simd/avx512vl", "avx512vl.c", "rust_isa_avx512vl_");
    check_x86_symbols("crypto/aes", "aes.c", "rust_isa_aes_");
    check_x86_symbols("crypto/sha", "sha.c", "rust_isa_sha_");
    check_x86_symbols("crypto/gfni", "gfni.c", "rust_isa_gfni_");
    check_x86_symbols("crypto/vaes", "vaes.c", "rust_isa_vaes_");
    check_x86_symbols("crypto/vpclmulqdq", "vpclmulqdq.c", "rust_isa_vpclmulqdq_");
    check_x86_symbols("simd/fma", "fma.c", "rust_isa_fma_");
    check_x86_symbols("integer/bmi1", "bmi1.c", "rust_isa_bmi1_");
    check_x86_symbols("integer/bmi2", "bmi2.c", "rust_isa_bmi2_");
    check_x86_symbols("integer/popcnt", "popcnt.c", "rust_isa_popcnt_");
}

#[test]
fn check_native_symbols() {
    for (module, c_file, prefix) in [
        ("aarch64/simd/neon", "neon.c", "rust_isa_neon_"),
        ("aarch64/simd/dotprod", "dotprod.c", "rust_isa_dotprod_"),
        ("arm/simd/neon", "neon.c", "rust_isa_arm_neon_"),
        ("riscv/simd/rv_vector", "rv_vector.c", "rust_isa_rv_vector_"),
        ("loongarch/simd/lsx", "lsx.c", "rust_isa_lsx_"),
        ("s390x/simd/vector", "vector.c", "rust_isa_s390x_vector_"),
        ("powerpc/simd/altivec", "altivec.c", "rust_isa_altivec_"),
        ("mips/simd/msa", "msa.c", "rust_isa_msa_"),
    ] {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join(module);
        check_symbols(&root, c_file, prefix);
    }
}

#[test]
fn check_wide_arithmetic_symbols() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/wasm32/wide_arithmetic");
    check_symbols(&root, "wide_arithmetic.c", "rust_isa_wide_arithmetic_");
}

#[test]
fn check_rustdoc_is_english() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    for path in rust_sources(&root) {
        let text = fs::read_to_string(&path).unwrap();
        for (number, line) in text.lines().enumerate() {
            let trimmed = line.trim_start();
            if !trimmed.starts_with("///") && !trimmed.starts_with("//!") {
                continue;
            }
            assert!(
                !line.chars().any(is_japanese),
                "{}:{}: {line}",
                path.display(),
                number + 1
            );
        }
    }
}

#[test]
fn check_lane_arms_match_width() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/wasm32/simd/simd128");
    let mut seen = 0;
    for entry in fs::read_dir(&root).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }
        let text = fs::read_to_string(&path).unwrap();
        for (name, body) in lane_functions(&text) {
            seen += 1;
            let limit = assert_limit(name, body);
            let expected = expected_lanes(name);
            assert_eq!(limit, expected, "{name}");
            let mut arms = arm_indices(body);
            arms.sort();
            let want: Vec<u32> = (0..expected).collect();
            assert_eq!(arms, want, "{name}");
        }
    }
    assert_eq!(seen, 24);
}
