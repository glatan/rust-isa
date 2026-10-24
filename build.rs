use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let simd128_c = manifest.join("src/wasm32/simd/simd128/simd128.c");
    let relaxed_c = manifest.join("src/wasm32/simd/relaxed_simd/relaxed_simd.c");
    let wide_arithmetic_c = manifest.join("src/wasm32/wide_arithmetic/wide_arithmetic.c");
    let sse2_c = manifest.join("src/x86/simd/sse2/sse2.c");
    let sse3_c = manifest.join("src/x86/simd/sse3/sse3.c");
    let sse4_1_c = manifest.join("src/x86/simd/sse4_1/sse4_1.c");
    let sse4_2_c = manifest.join("src/x86/simd/sse4_2/sse4_2.c");
    let sse4a_c = manifest.join("src/x86/simd/sse4a/sse4a.c");
    let ssse3_c = manifest.join("src/x86/simd/ssse3/ssse3.c");
    let avx2_c = manifest.join("src/x86/simd/avx2/avx2.c");
    let avx_c = manifest.join("src/x86/simd/avx/avx.c");
    let avxifma_c = manifest.join("src/x86/simd/avxifma/avxifma.c");
    let avx10_1_c = manifest.join("src/x86/simd/avx10_1/avx10_1.c");
    let avx10_2_c = manifest.join("src/x86/simd/avx10_2/avx10_2.c");
    let avxneconvert_c = manifest.join("src/x86/simd/avxneconvert/avxneconvert.c");
    let avxvnni_c = manifest.join("src/x86/simd/avxvnni/avxvnni.c");
    let avxvnniint8_c = manifest.join("src/x86/simd/avxvnniint8/avxvnniint8.c");
    let avxvnniint16_c = manifest.join("src/x86/simd/avxvnniint16/avxvnniint16.c");
    let f16c_c = manifest.join("src/x86/simd/f16c/f16c.c");
    let fma4_c = manifest.join("src/x86/simd/fma4/fma4.c");
    let xop_c = manifest.join("src/x86/simd/xop/xop.c");
    let avx512f_c = manifest.join("src/x86/simd/avx512f/avx512f.c");
    let avx512bw_c = manifest.join("src/x86/simd/avx512bw/avx512bw.c");
    let avx512bitalg_c = manifest.join("src/x86/simd/avx512bitalg/avx512bitalg.c");
    let avx512bf16_c = manifest.join("src/x86/simd/avx512bf16/avx512bf16.c");
    let avx512cd_c = manifest.join("src/x86/simd/avx512cd/avx512cd.c");
    let avx512fp16_c = manifest.join("src/x86/simd/avx512fp16/avx512fp16.c");
    let avx512vbmi2_c = manifest.join("src/x86/simd/avx512vbmi2/avx512vbmi2.c");
    let avx512vp2intersect_c =
        manifest.join("src/x86/simd/avx512vp2intersect/avx512vp2intersect.c");
    let avx512vpopcntdq_c = manifest.join("src/x86/simd/avx512vpopcntdq/avx512vpopcntdq.c");
    let avx512dq_c = manifest.join("src/x86/simd/avx512dq/avx512dq.c");
    let avx512ifma_c = manifest.join("src/x86/simd/avx512ifma/avx512ifma.c");
    let avx512vbmi_c = manifest.join("src/x86/simd/avx512vbmi/avx512vbmi.c");
    let avx512vnni_c = manifest.join("src/x86/simd/avx512vnni/avx512vnni.c");
    let avx512vl_c = manifest.join("src/x86/simd/avx512vl/avx512vl.c");
    let aes_c = manifest.join("src/x86/crypto/aes/aes.c");
    let sha_c = manifest.join("src/x86/crypto/sha/sha.c");
    let sha512_c = manifest.join("src/x86/crypto/sha512/sha512.c");
    let sm3_c = manifest.join("src/x86/crypto/sm3/sm3.c");
    let sm4_c = manifest.join("src/x86/crypto/sm4/sm4.c");
    let gfni_c = manifest.join("src/x86/crypto/gfni/gfni.c");
    let vaes_c = manifest.join("src/x86/crypto/vaes/vaes.c");
    let vpclmulqdq_c = manifest.join("src/x86/crypto/vpclmulqdq/vpclmulqdq.c");
    let fma_c = manifest.join("src/x86/simd/fma/fma.c");
    let bmi1_c = manifest.join("src/x86/integer/bmi1/bmi1.c");
    let bmi2_c = manifest.join("src/x86/integer/bmi2/bmi2.c");
    let lzcnt_c = manifest.join("src/x86/integer/lzcnt/lzcnt.c");
    let movbe_c = manifest.join("src/x86/integer/movbe/movbe.c");
    let rdrand_c = manifest.join("src/x86/integer/rdrand/rdrand.c");
    let rdseed_c = manifest.join("src/x86/integer/rdseed/rdseed.c");
    let popcnt_c = manifest.join("src/x86/integer/popcnt/popcnt.c");
    let neon_c = manifest.join("src/aarch64/simd/neon/neon.c");
    let dotprod_c = manifest.join("src/aarch64/simd/dotprod/dotprod.c");
    let a64_fp16_c = manifest.join("src/aarch64/simd/fp16/fp16.c");
    let a64_bf16_c = manifest.join("src/aarch64/simd/bf16/bf16.c");
    let a64_i8mm_c = manifest.join("src/aarch64/simd/i8mm/i8mm.c");
    let a64_rdm_c = manifest.join("src/aarch64/simd/rdm/rdm.c");
    let a64_fcma_c = manifest.join("src/aarch64/simd/fcma/fcma.c");
    let a64_fhm_c = manifest.join("src/aarch64/simd/fhm/fhm.c");
    let a64_jsconv_c = manifest.join("src/aarch64/simd/jsconv/jsconv.c");
    let a64_f32mm_c = manifest.join("src/aarch64/simd/f32mm/f32mm.c");
    let a64_f64mm_c = manifest.join("src/aarch64/simd/f64mm/f64mm.c");
    let a64_sve_c = manifest.join("src/aarch64/simd/sve/sve.c");
    let a64_sme_c = manifest.join("src/aarch64/simd/sme/sme.c");
    let a64_aes_c = manifest.join("src/aarch64/crypto/aes/aes.c");
    let a64_sha2_c = manifest.join("src/aarch64/crypto/sha2/sha2.c");
    let a64_sha3_c = manifest.join("src/aarch64/crypto/sha3/sha3.c");
    let a64_sm4_c = manifest.join("src/aarch64/crypto/sm4/sm4.c");
    let a64_crc_c = manifest.join("src/aarch64/crypto/crc/crc.c");
    let rvv_c = manifest.join("src/riscv/simd/rvv/rvv.c");
    let rvv_slice_c = manifest.join("src/riscv/simd/rvv_slice/rvv_slice.c");
    let zvkn_c = manifest.join("src/riscv/simd/zv/zvkn/zvkn.c");
    let zvks_c = manifest.join("src/riscv/simd/zv/zvks/zvks.c");
    let zvkg_c = manifest.join("src/riscv/simd/zv/zvkg/zvkg.c");
    let zvbc_c = manifest.join("src/riscv/simd/zv/zvbc/zvbc.c");
    let rv_zba_c = manifest.join("src/riscv/integer/zba/zba.c");
    let rv_zbb_c = manifest.join("src/riscv/integer/zbb/zbb.c");
    let rv_zbc_c = manifest.join("src/riscv/integer/zbc/zbc.c");
    let rv_zbkb_c = manifest.join("src/riscv/integer/zbkb/zbkb.c");
    let rv_zbkx_c = manifest.join("src/riscv/integer/zbkx/zbkx.c");
    let rv_zbs_c = manifest.join("src/riscv/integer/zbs/zbs.c");
    let rv_zkn_c = manifest.join("src/riscv/crypto/zkn/zkn.c");
    let rv_scalar_c = manifest.join("src/riscv/float/scalar/scalar.c");
    let rv_zfa_c = manifest.join("src/riscv/float/zfa/zfa.c");
    let rv_fp_helpers_h = manifest.join("src/riscv/float/fp_helpers.h");
    let rv_zks_c = manifest.join("src/riscv/crypto/zks/zks.c");
    let lsx_c = manifest.join("src/loongarch/simd/lsx/lsx.c");
    let lasx_c = manifest.join("src/loongarch/simd/lasx/lasx.c");
    let s390x_vector_c = manifest.join("src/s390x/simd/vector/vector.c");
    let s390x_enh_c = manifest.join("src/s390x/simd/vector_enhancements_1/vector_enhancements_1.c");
    let arm_neon_c = manifest.join("src/arm/simd/neon/neon.c");
    let arm_dotprod_c = manifest.join("src/arm/simd/dotprod/dotprod.c");
    let arm_fp16_c = manifest.join("src/arm/simd/fp16/fp16.c");
    let arm_i8mm_c = manifest.join("src/arm/simd/i8mm/i8mm.c");
    let arm_mve_c = manifest.join("src/arm/simd/mve/mve.c");
    let arm_mve_fp_c = manifest.join("src/arm/simd/mve_fp/mve_fp.c");
    let arm_aes_c = manifest.join("src/arm/crypto/aes/aes.c");
    let arm_sha2_c = manifest.join("src/arm/crypto/sha2/sha2.c");
    let arm_crc_c = manifest.join("src/arm/crypto/crc/crc.c");
    let altivec_c = manifest.join("src/powerpc/simd/altivec/altivec.c");
    let ppc_vsx_c = manifest.join("src/powerpc/simd/vsx/vsx.c");
    let ppc_power8_c = manifest.join("src/powerpc/simd/power8_vector/power8_vector.c");
    let ppc_power9_c = manifest.join("src/powerpc/simd/power9_vector/power9_vector.c");
    let ppc_power10_c = manifest.join("src/powerpc/simd/power10_vector/power10_vector.c");
    let ppc_crypto_c = manifest.join("src/powerpc/crypto/power8_crypto/power8_crypto.c");
    let ppc_mma_c = manifest.join("src/powerpc/matrix/mma/mma.c");
    let msa_c = manifest.join("src/mips/simd/msa/msa.c");
    let amx_tile_c = manifest.join("src/x86/matrix/amx_tile/amx_tile.c");
    for path in [
        &simd128_c,
        &relaxed_c,
        &wide_arithmetic_c,
        &sse2_c,
        &sse3_c,
        &sse4_1_c,
        &sse4_2_c,
        &sse4a_c,
        &ssse3_c,
        &avx2_c,
        &avx_c,
        &avxifma_c,
        &avx10_1_c,
        &avx10_2_c,
        &avxneconvert_c,
        &avxvnni_c,
        &avxvnniint8_c,
        &avxvnniint16_c,
        &f16c_c,
        &fma4_c,
        &xop_c,
        &avx512f_c,
        &avx512bw_c,
        &avx512bitalg_c,
        &avx512bf16_c,
        &avx512cd_c,
        &avx512fp16_c,
        &avx512vbmi2_c,
        &avx512vp2intersect_c,
        &avx512vpopcntdq_c,
        &avx512dq_c,
        &avx512ifma_c,
        &avx512vbmi_c,
        &avx512vnni_c,
        &avx512vl_c,
        &aes_c,
        &sha_c,
        &sha512_c,
        &sm3_c,
        &sm4_c,
        &gfni_c,
        &vaes_c,
        &vpclmulqdq_c,
        &fma_c,
        &bmi1_c,
        &bmi2_c,
        &lzcnt_c,
        &movbe_c,
        &rdrand_c,
        &rdseed_c,
        &popcnt_c,
        &neon_c,
        &dotprod_c,
        &a64_fp16_c,
        &a64_bf16_c,
        &a64_i8mm_c,
        &a64_rdm_c,
        &a64_fcma_c,
        &a64_fhm_c,
        &a64_jsconv_c,
        &a64_f32mm_c,
        &a64_f64mm_c,
        &a64_sve_c,
        &a64_sme_c,
        &a64_aes_c,
        &a64_sha2_c,
        &a64_sha3_c,
        &a64_sm4_c,
        &a64_crc_c,
        &rvv_c,
        &rvv_slice_c,
        &zvkn_c,
        &zvks_c,
        &zvkg_c,
        &zvbc_c,
        &rv_zba_c,
        &rv_zbb_c,
        &rv_zbc_c,
        &rv_zbkb_c,
        &rv_zbkx_c,
        &rv_zbs_c,
        &rv_zkn_c,
        &rv_scalar_c,
        &rv_zfa_c,
        &rv_fp_helpers_h,
        &rv_zks_c,
        &lsx_c,
        &lasx_c,
        &s390x_vector_c,
        &s390x_enh_c,
        &arm_neon_c,
        &arm_dotprod_c,
        &arm_fp16_c,
        &arm_i8mm_c,
        &arm_mve_c,
        &arm_mve_fp_c,
        &arm_aes_c,
        &arm_sha2_c,
        &arm_crc_c,
        &altivec_c,
        &ppc_vsx_c,
        &ppc_power8_c,
        &ppc_power9_c,
        &ppc_power10_c,
        &ppc_crypto_c,
        &ppc_mma_c,
        &msa_c,
        &amx_tile_c,
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
        for (path, name, flags) in [
            (&ppc_vsx_c, "rust_isa_vsx", &["-maltivec", "-mvsx"][..]),
            (
                &ppc_power8_c,
                "rust_isa_power8_vector",
                &["-mcpu=power8", "-maltivec", "-mvsx"][..],
            ),
            (
                &ppc_power9_c,
                "rust_isa_power9_vector",
                &["-mcpu=power9", "-maltivec", "-mvsx"][..],
            ),
            (
                &ppc_power10_c,
                "rust_isa_power10_vector",
                &["-mcpu=power10", "-maltivec", "-mvsx"][..],
            ),
            (
                &ppc_crypto_c,
                "rust_isa_power8_crypto",
                &["-mcpu=power8", "-maltivec", "-mvsx"][..],
            ),
            (
                &ppc_mma_c,
                "rust_isa_mma",
                &["-mcpu=power10", "-mmma", "-maltivec", "-mvsx"][..],
            ),
        ] {
            compile_archive(
                &out_dir,
                path,
                name,
                flags,
                Some("powerpc64le-unknown-linux-gnu"),
            );
        }
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
                "-mfpu=neon-vfpv4",
                "-mfloat-abi=hard",
            ],
            Some("armv7-unknown-linux-gnueabihf"),
        );
        for (path, name, flags) in [
            (
                &arm_dotprod_c,
                "rust_isa_arm_dotprod",
                &[
                    "-mthumb",
                    "-march=armv8.2-a+dotprod",
                    "-mfpu=neon",
                    "-mfloat-abi=hard",
                ][..],
            ),
            (
                &arm_fp16_c,
                "rust_isa_arm_fp16",
                &[
                    "-mthumb",
                    "-march=armv8.2-a+fp16",
                    "-mfpu=neon-fp-armv8",
                    "-mfloat-abi=hard",
                ][..],
            ),
            (
                &arm_i8mm_c,
                "rust_isa_arm_i8mm",
                &[
                    "-mthumb",
                    "-march=armv8.6-a+i8mm",
                    "-mfpu=neon",
                    "-mfloat-abi=hard",
                ][..],
            ),
            (
                &arm_mve_c,
                "rust_isa_arm_mve",
                &[
                    "-mthumb",
                    "-march=armv8.1-m.main+mve",
                    "-mfpu=fp-armv8",
                    "-mfloat-abi=hard",
                ][..],
            ),
            (
                &arm_mve_fp_c,
                "rust_isa_arm_mve_fp",
                &[
                    "-mthumb",
                    "-march=armv8.1-m.main+mve.fp+fp.dp",
                    "-mfpu=fp-armv8",
                    "-mfloat-abi=hard",
                ][..],
            ),
            (
                &arm_aes_c,
                "rust_isa_arm_aes",
                &[
                    "-mthumb",
                    "-march=armv8-a+crypto",
                    "-mfpu=crypto-neon-fp-armv8",
                    "-mfloat-abi=hard",
                ][..],
            ),
            (
                &arm_sha2_c,
                "rust_isa_arm_sha2",
                &[
                    "-mthumb",
                    "-march=armv8-a+crypto",
                    "-mfpu=crypto-neon-fp-armv8",
                    "-mfloat-abi=hard",
                ][..],
            ),
            (
                &arm_crc_c,
                "rust_isa_arm_crc",
                &[
                    "-mthumb",
                    "-march=armv8-a+crc",
                    "-mfpu=neon",
                    "-mfloat-abi=hard",
                ][..],
            ),
        ] {
            compile_archive(
                &out_dir,
                path,
                name,
                flags,
                Some("armv7-unknown-linux-gnueabihf"),
            );
        }
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
        compile_archive(
            &out_dir,
            &s390x_enh_c,
            "rust_isa_s390x_enh",
            &["-march=z14", "-mzvector"],
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
        compile_archive(
            &out_dir,
            &lasx_c,
            "rust_isa_lasx",
            &["-mlasx"],
            Some("loongarch64-unknown-linux-gnu"),
        );
        return;
    }

    if target == "riscv64gc-unknown-linux-gnu" {
        compile_archive(
            &out_dir,
            &rvv_c,
            "rust_isa_rvv",
            &[
                "-march=rv64gcv",
                "-mabi=lp64d",
                "-mno-relax",
                "-fno-stack-protector",
            ],
            Some("riscv64-unknown-linux-gnu"),
        );
        compile_archive(
            &out_dir,
            &rvv_slice_c,
            "rust_isa_rvv_slice",
            &["-march=rv64gcv", "-mabi=lp64d", "-mno-relax"],
            Some("riscv64-unknown-linux-gnu"),
        );
        compile_archive(
            &out_dir,
            &zvkn_c,
            "rust_isa_rvv_zvkn",
            &["-march=rv64gcv", "-mabi=lp64d", "-mno-relax"],
            Some("riscv64-unknown-linux-gnu"),
        );
        compile_archive(
            &out_dir,
            &zvks_c,
            "rust_isa_rvv_zvks",
            &["-march=rv64gcv", "-mabi=lp64d", "-mno-relax"],
            Some("riscv64-unknown-linux-gnu"),
        );
        compile_archive(
            &out_dir,
            &zvkg_c,
            "rust_isa_rvv_zvkg",
            &["-march=rv64gcv", "-mabi=lp64d", "-mno-relax"],
            Some("riscv64-unknown-linux-gnu"),
        );
        compile_archive(
            &out_dir,
            &zvbc_c,
            "rust_isa_rvv_zvbc",
            &["-march=rv64gcv", "-mabi=lp64d", "-mno-relax"],
            Some("riscv64-unknown-linux-gnu"),
        );
        for (path, name, march) in [
            (&rv_zba_c, "rust_isa_rv_zba", &["-march=rv64gc_zba"][..]),
            (&rv_zbb_c, "rust_isa_rv_zbb", &["-march=rv64gc_zbb"][..]),
            (&rv_zbc_c, "rust_isa_rv_zbc", &["-march=rv64gc_zbc"][..]),
            (&rv_zbkb_c, "rust_isa_rv_zbkb", &["-march=rv64gc_zbkb"][..]),
            (&rv_zbkx_c, "rust_isa_rv_zbkx", &["-march=rv64gc_zbkx"][..]),
            (&rv_zbs_c, "rust_isa_rv_zbs", &["-march=rv64gc_zbs"][..]),
            (&rv_zkn_c, "rust_isa_rv_zkn", &["-march=rv64gc_zkn"][..]),
            (
                &rv_scalar_c,
                "rust_isa_rv_scalar",
                &["-march=rv64gc_zfh"][..],
            ),
            (&rv_zfa_c, "rust_isa_rv_zfa", &["-march=rv64gc_zfh_zfa"][..]),
            (&rv_zks_c, "rust_isa_rv_zks", &["-march=rv64gc_zks"][..]),
        ] {
            let mut flags = march.to_vec();
            flags.push("-mabi=lp64d");
            flags.push("-mno-relax");
            compile_archive(
                &out_dir,
                path,
                name,
                &flags,
                Some("riscv64-unknown-linux-gnu"),
            );
        }
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
        for (path, name, flags) in [
            (
                &a64_fp16_c,
                "rust_isa_a64_fp16",
                &["-march=armv8.2-a+fp16+fp16fml"][..],
            ),
            (
                &a64_bf16_c,
                "rust_isa_a64_bf16",
                &["-march=armv8.2-a+bf16"][..],
            ),
            (
                &a64_i8mm_c,
                "rust_isa_a64_i8mm",
                &["-march=armv8.6-a+i8mm"][..],
            ),
            (&a64_rdm_c, "rust_isa_a64_rdm", &["-march=armv8.1-a"][..]),
            (
                &a64_fcma_c,
                "rust_isa_a64_fcma",
                &["-march=armv8.3-a+fcma"][..],
            ),
            (
                &a64_fhm_c,
                "rust_isa_a64_fhm",
                &["-march=armv8.2-a+fp16"][..],
            ),
            (
                &a64_f32mm_c,
                "rust_isa_a64_f32mm",
                &["-march=armv8.6-a+sve+f32mm"][..],
            ),
            (
                &a64_f64mm_c,
                "rust_isa_a64_f64mm",
                &["-march=armv8.6-a+sve+f64mm"][..],
            ),
            (
                &a64_jsconv_c,
                "rust_isa_a64_jsconv",
                &[
                    "-march=armv8.3-a",
                    "-Xclang",
                    "-target-feature",
                    "-Xclang",
                    "+jsconv",
                ][..],
            ),
            (&a64_sve_c, "rust_isa_a64_sve", &["-march=armv8-a+sve"][..]),
            (&a64_sme_c, "rust_isa_a64_sme", &["-march=armv9-a+sme"][..]),
            (
                &a64_aes_c,
                "rust_isa_a64_aes",
                &["-march=armv8-a+crypto"][..],
            ),
            (
                &a64_sha2_c,
                "rust_isa_a64_sha2",
                &["-march=armv8-a+crypto"][..],
            ),
            (
                &a64_sha3_c,
                "rust_isa_a64_sha3",
                &["-march=armv8.2-a+sha3"][..],
            ),
            (
                &a64_sm4_c,
                "rust_isa_a64_sm4",
                &["-march=armv8.2-a+sm4"][..],
            ),
            (&a64_crc_c, "rust_isa_a64_crc", &["-march=armv8-a+crc"][..]),
        ] {
            compile_archive(
                &out_dir,
                path,
                name,
                flags,
                Some("aarch64-unknown-linux-gnu"),
            );
        }
        return;
    }

    if target == "x86_64-unknown-linux-gnu" || target == "i686-unknown-linux-gnu" {
        compile_native(&out_dir, &sse2_c, "rust_isa_sse2", &["-msse2"]);
        compile_native(&out_dir, &sse3_c, "rust_isa_sse3", &["-msse3"]);
        compile_native(&out_dir, &sse4_1_c, "rust_isa_sse41", &["-msse4.1"]);
        compile_native(&out_dir, &sse4_2_c, "rust_isa_sse42", &["-msse4.2"]);
        compile_native(&out_dir, &sse4a_c, "rust_isa_sse4a", &["-msse4a"]);
        compile_native(&out_dir, &ssse3_c, "rust_isa_ssse3", &["-mssse3"]);
        compile_native(
            &out_dir,
            &avx2_c,
            "rust_isa_avx2",
            &["-mavx2", "-mno-avx512f"],
        );
        compile_native(&out_dir, &avx_c, "rust_isa_avx", &["-mavx"]);
        compile_native(
            &out_dir,
            &avxifma_c,
            "rust_isa_avxifma",
            &["-mavxifma", "-mavx2"],
        );
        compile_native(&out_dir, &avx10_1_c, "rust_isa_avx10_1", &["-mavx10.1"]);
        compile_native(&out_dir, &avx10_2_c, "rust_isa_avx10_2", &["-mavx10.2"]);
        compile_native(
            &out_dir,
            &avxneconvert_c,
            "rust_isa_avxneconvert",
            &["-mavxneconvert", "-mavx2"],
        );
        compile_native(
            &out_dir,
            &avxvnni_c,
            "rust_isa_avxvnni",
            &["-mavxvnni", "-mavx2"],
        );
        compile_native(
            &out_dir,
            &avxvnniint8_c,
            "rust_isa_avxvnniint8",
            &["-mavxvnniint8", "-mavx2"],
        );
        compile_native(
            &out_dir,
            &avxvnniint16_c,
            "rust_isa_avxvnniint16",
            &["-mavxvnniint16", "-mavx2"],
        );
        compile_native(&out_dir, &f16c_c, "rust_isa_f16c", &["-mf16c", "-mavx"]);
        compile_native(&out_dir, &fma4_c, "rust_isa_fma4", &["-mfma4"]);
        compile_native(&out_dir, &xop_c, "rust_isa_xop", &["-mxop"]);
        compile_native(&out_dir, &avx512f_c, "rust_isa_avx512f", &["-mavx512f"]);
        compile_native(
            &out_dir,
            &avx512bw_c,
            "rust_isa_avx512bw",
            &["-mavx512f", "-mavx512bw"],
        );
        compile_native(
            &out_dir,
            &avx512bitalg_c,
            "rust_isa_avx512bitalg",
            &["-mavx512f", "-mavx512bw", "-mavx512bitalg"],
        );
        compile_native(
            &out_dir,
            &avx512bf16_c,
            "rust_isa_avx512bf16",
            &["-mavx512f", "-mavx512bf16"],
        );
        compile_native(
            &out_dir,
            &avx512cd_c,
            "rust_isa_avx512cd",
            &["-mavx512f", "-mavx512cd"],
        );
        compile_native(
            &out_dir,
            &avx512fp16_c,
            "rust_isa_avx512fp16",
            &["-mavx512f", "-mavx512fp16"],
        );
        compile_native(
            &out_dir,
            &avx512vbmi2_c,
            "rust_isa_avx512vbmi2",
            &["-mavx512f", "-mavx512bw", "-mavx512vbmi2"],
        );
        compile_native(
            &out_dir,
            &avx512vp2intersect_c,
            "rust_isa_avx512vp2intersect",
            &["-mavx512f", "-mavx512vp2intersect"],
        );
        compile_native(
            &out_dir,
            &avx512vpopcntdq_c,
            "rust_isa_avx512vpopcntdq",
            &["-mavx512f", "-mavx512vpopcntdq"],
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
            // The C file uses per-function target attributes for BW, DQ,
            // and the optional subextensions (IFMA, VBMI, VBMI2, VNNI,
            // BITALG, VPOPCNTDQ, VP2INTERSECT). Compile it with the minimal
            // F+VL baseline so each wrapper's C features are a subset of
            // the Rust target-feature set and LTO can inline it.
            &["-mavx512f", "-mavx512vl"],
        );
        compile_native(&out_dir, &aes_c, "rust_isa_aes", &["-maes"]);
        compile_native(&out_dir, &sha_c, "rust_isa_sha", &["-msha"]);
        compile_native(&out_dir, &sha512_c, "rust_isa_sha512", &["-msha512"]);
        compile_native(&out_dir, &sm3_c, "rust_isa_sm3", &["-msm3"]);
        compile_native(&out_dir, &sm4_c, "rust_isa_sm4", &["-msm4"]);
        compile_native(&out_dir, &gfni_c, "rust_isa_gfni", &["-mgfni"]);
        compile_native(
            &out_dir,
            &vaes_c,
            "rust_isa_vaes",
            &["-mvaes", "-mavx2", "-mavx512f"],
        );
        compile_native(
            &out_dir,
            &vpclmulqdq_c,
            "rust_isa_vpclmulqdq",
            &["-mvpclmulqdq", "-mavx2", "-mavx512f", "-mavx512vl"],
        );
        compile_native(&out_dir, &fma_c, "rust_isa_fma", &["-mavx2", "-mfma"]);
        compile_native(&out_dir, &bmi1_c, "rust_isa_bmi1", &["-mbmi"]);
        compile_native(&out_dir, &bmi2_c, "rust_isa_bmi2", &["-mbmi2"]);
        compile_native(&out_dir, &lzcnt_c, "rust_isa_lzcnt", &["-mlzcnt"]);
        compile_native(&out_dir, &movbe_c, "rust_isa_movbe", &["-mmovbe"]);
        compile_native(&out_dir, &rdrand_c, "rust_isa_rdrand", &["-mrdrnd"]);
        compile_native(&out_dir, &rdseed_c, "rust_isa_rdseed", &["-mrdseed"]);
        compile_native(&out_dir, &amx_tile_c, "rust_isa_amx_tile", &["-mamx-tile"]);
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
