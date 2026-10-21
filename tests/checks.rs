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
    check_x86_symbols("simd/sse3", "sse3.c", "rust_isa_sse3_");
    check_x86_symbols("simd/ssse3", "ssse3.c", "rust_isa_ssse3_");
    check_x86_symbols("simd/sse4_1", "sse4_1.c", "rust_isa_sse41_");
    check_x86_symbols("simd/sse4_2", "sse4_2.c", "rust_isa_sse42_");
    check_x86_symbols("simd/sse4a", "sse4a.c", "rust_isa_sse4a_");
    check_x86_symbols("simd/avx", "avx.c", "rust_isa_avx_");
    check_x86_symbols("simd/avx2", "avx2.c", "rust_isa_avx2_");
    check_x86_symbols("simd/f16c", "f16c.c", "rust_isa_f16c_");
    check_x86_symbols("simd/avxifma", "avxifma.c", "rust_isa_avxifma_");
    check_x86_symbols(
        "simd/avxneconvert",
        "avxneconvert.c",
        "rust_isa_avxneconvert_",
    );
    check_x86_symbols("simd/avxvnni", "avxvnni.c", "rust_isa_avxvnni_");
    check_x86_symbols("simd/avxvnniint8", "avxvnniint8.c", "rust_isa_avxvnniint8_");
    check_x86_symbols(
        "simd/avxvnniint16",
        "avxvnniint16.c",
        "rust_isa_avxvnniint16_",
    );
    check_x86_symbols("simd/avx10_1", "avx10_1.c", "rust_isa_avx10_1_");
    check_x86_symbols("simd/avx10_2", "avx10_2.c", "rust_isa_avx10_2_");
    check_x86_symbols("simd/avx512f", "avx512f.c", "rust_isa_avx512f_");
    check_x86_symbols("simd/avx512bw", "avx512bw.c", "rust_isa_avx512bw_");
    check_x86_symbols(
        "simd/avx512bitalg",
        "avx512bitalg.c",
        "rust_isa_avx512bitalg_",
    );
    check_x86_symbols("simd/avx512bf16", "avx512bf16.c", "rust_isa_avx512bf16_");
    check_x86_symbols("simd/avx512cd", "avx512cd.c", "rust_isa_avx512cd_");
    check_x86_symbols("simd/avx512dq", "avx512dq.c", "rust_isa_avx512dq_");
    check_x86_symbols("simd/avx512fp16", "avx512fp16.c", "rust_isa_avx512fp16_");
    check_x86_symbols("simd/avx512ifma", "avx512ifma.c", "rust_isa_avx512ifma_");
    check_x86_symbols("simd/avx512vbmi", "avx512vbmi.c", "rust_isa_avx512vbmi_");
    check_x86_symbols("simd/avx512vbmi2", "avx512vbmi2.c", "rust_isa_avx512vbmi2_");
    check_x86_symbols("simd/avx512vnni", "avx512vnni.c", "rust_isa_avx512vnni_");
    check_x86_symbols("simd/avx512vl", "avx512vl.c", "rust_isa_avx512vl_");
    check_x86_symbols(
        "simd/avx512vp2intersect",
        "avx512vp2intersect.c",
        "rust_isa_avx512vp2intersect_",
    );
    check_x86_symbols(
        "simd/avx512vpopcntdq",
        "avx512vpopcntdq.c",
        "rust_isa_avx512vpopcntdq_",
    );
    check_x86_symbols("crypto/aes", "aes.c", "rust_isa_aes_");
    check_x86_symbols("crypto/sha", "sha.c", "rust_isa_sha_");
    check_x86_symbols("crypto/sha512", "sha512.c", "rust_isa_sha512_");
    check_x86_symbols("crypto/sm3", "sm3.c", "rust_isa_sm3_");
    check_x86_symbols("crypto/sm4", "sm4.c", "rust_isa_sm4_");
    check_x86_symbols("crypto/gfni", "gfni.c", "rust_isa_gfni_");
    check_x86_symbols("crypto/vaes", "vaes.c", "rust_isa_vaes_");
    check_x86_symbols("crypto/vpclmulqdq", "vpclmulqdq.c", "rust_isa_vpclmulqdq_");
    check_x86_symbols("simd/fma", "fma.c", "rust_isa_fma_");
    check_x86_symbols("simd/fma4", "fma4.c", "rust_isa_fma4_");
    check_x86_symbols("simd/xop", "xop.c", "rust_isa_xop_");
    check_x86_symbols("integer/bmi1", "bmi1.c", "rust_isa_bmi1_");
    check_x86_symbols("integer/bmi2", "bmi2.c", "rust_isa_bmi2_");
    check_x86_symbols("integer/lzcnt", "lzcnt.c", "rust_isa_lzcnt_");
    check_x86_symbols("integer/movbe", "movbe.c", "rust_isa_movbe_");
    check_x86_symbols("integer/popcnt", "popcnt.c", "rust_isa_popcnt_");
    check_x86_symbols("integer/rdrand", "rdrand.c", "rust_isa_rdrand_");
    check_x86_symbols("integer/rdseed", "rdseed.c", "rust_isa_rdseed_");
    check_x86_symbols("matrix/amx_tile", "amx_tile.c", "rust_isa_amx_tile_");
}

#[test]
fn check_native_symbols() {
    for (module, c_file, prefix) in [
        ("aarch64/simd/neon", "neon.c", "rust_isa_neon_"),
        ("aarch64/simd/dotprod", "dotprod.c", "rust_isa_dotprod_"),
        ("aarch64/simd/fp16", "fp16.c", "rust_isa_fp16_"),
        ("aarch64/simd/bf16", "bf16.c", "rust_isa_bf16_"),
        ("aarch64/simd/i8mm", "i8mm.c", "rust_isa_i8mm_"),
        ("aarch64/simd/rdm", "rdm.c", "rust_isa_rdm_"),
        ("aarch64/simd/fcma", "fcma.c", "rust_isa_fcma_"),
        ("aarch64/simd/fhm", "fhm.c", "rust_isa_fhm_"),
        ("aarch64/simd/sve", "sve.c", "rust_isa_sve_"),
        ("aarch64/simd/sme", "sme.c", "rust_isa_sme_"),
        ("aarch64/crypto/aes", "aes.c", "rust_isa_aes_"),
        ("aarch64/crypto/sha2", "sha2.c", "rust_isa_sha2_"),
        ("aarch64/crypto/sha3", "sha3.c", "rust_isa_sha3_"),
        ("aarch64/crypto/sm4", "sm4.c", "rust_isa_sm4_"),
        ("aarch64/crypto/crc", "crc.c", "rust_isa_crc_"),
        ("arm/simd/neon", "neon.c", "rust_isa_arm_neon_"),
        ("arm/simd/dotprod", "dotprod.c", "rust_isa_arm_dotprod_"),
        ("arm/simd/fp16", "fp16.c", "rust_isa_arm_fp16_"),
        ("arm/simd/i8mm", "i8mm.c", "rust_isa_arm_i8mm_"),
        ("arm/crypto/aes", "aes.c", "rust_isa_arm_aes_"),
        ("arm/crypto/sha2", "sha2.c", "rust_isa_arm_sha2_"),
        ("arm/crypto/crc", "crc.c", "rust_isa_arm_crc_"),
        ("riscv/simd/rv_vector", "rv_vector.c", "rust_isa_rv_vector_"),
        ("riscv/simd/rvv_slice", "rvv_slice.c", "rust_isa_rvv_slice_"),
        ("riscv/integer/zba", "zba.c", "rust_isa_riscv_zba_"),
        ("riscv/integer/zbb", "zbb.c", "rust_isa_riscv_zbb_"),
        ("riscv/integer/zbc", "zbc.c", "rust_isa_riscv_zbc_"),
        ("riscv/integer/zbs", "zbs.c", "rust_isa_riscv_zbs_"),
        ("riscv/crypto/zkn", "zkn.c", "rust_isa_riscv_zkn_"),
        ("riscv/crypto/zks", "zks.c", "rust_isa_riscv_zks_"),
        ("loongarch/simd/lsx", "lsx.c", "rust_isa_lsx_"),
        ("loongarch/simd/lasx", "lasx.c", "rust_isa_lasx_"),
        ("s390x/simd/vector", "vector.c", "rust_isa_s390x_vector_"),
        (
            "s390x/simd/vector_enhancements",
            "vector_enhancements.c",
            "rust_isa_s390x_enh_",
        ),
        ("powerpc/simd/altivec", "altivec.c", "rust_isa_altivec_"),
        ("powerpc/simd/vsx", "vsx.c", "rust_isa_vsx_"),
        (
            "powerpc/simd/power8_vector",
            "power8_vector.c",
            "rust_isa_power8_vector_",
        ),
        (
            "powerpc/simd/power9_vector",
            "power9_vector.c",
            "rust_isa_power9_vector_",
        ),
        (
            "powerpc/simd/power10_vector",
            "power10_vector.c",
            "rust_isa_power10_vector_",
        ),
        (
            "powerpc/crypto/power8_crypto",
            "power8_crypto.c",
            "rust_isa_power8_crypto_",
        ),
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
