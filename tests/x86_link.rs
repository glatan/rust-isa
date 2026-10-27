#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::*;

const CALLS: &[&str] = &["call", "callq", "calll", "callw"];

#[test]
fn sse2_pipeline() {
    let text = build(
        nested_dir("sse2-link"),
        "sse2-link",
        "-C target-feature=+sse2 -C linker-plugin-lto -C linker=clang -C link-arg=-fuse-ld=lld",
    );
    assert_adjacent(&text, "<kernel>:", "paddd");
}

#[test]
fn avx2_pipeline() {
    let text = build(
        nested_dir("avx2-link"),
        "avx2-link",
        "-C target-feature=+avx2 -C linker-plugin-lto -C linker=clang -C link-arg=-fuse-ld=lld",
    );
    assert_sequence(&text, "<kernel>:", "vpaddd", "vpmulld");
}

#[test]
fn avx512f_pipeline() {
    let text = build(
        nested_dir("avx512f-link"),
        "avx512f-link",
        "-C target-feature=+avx512f -C linker-plugin-lto -C linker=clang -C link-arg=-fuse-ld=lld",
    );
    assert_adjacent(&text, "<kernel>:", "vpaddd");
}

#[test]
fn aes_pipeline() {
    let text = build(
        nested_dir("aes-link"),
        "aes-link",
        "-C target-feature=+aes -C linker-plugin-lto -C linker=clang -C link-arg=-fuse-ld=lld",
    );
    assert_adjacent(&text, "<kernel_aes>:", "aesenc");
}

#[test]
fn fma_pipeline() {
    let text = build(
        nested_dir("fma-link"),
        "fma-link",
        "-C target-feature=+avx2,+fma -C linker-plugin-lto -C linker=clang -C link-arg=-fuse-ld=lld",
    );
    assert_adjacent(&text, "<kernel_fma>:", "vfmadd132ps");
}

#[test]
fn avx512vl_pipeline() {
    let text = build(
        nested_dir("avx512vl-link"),
        "avx512vl-link",
        "-C target-feature=+avx512f,+avx512bw,+avx512vl,+avx512vbmi -C linker-plugin-lto -C linker=clang -C link-arg=-fuse-ld=lld",
    );
    assert_adjacent(&text, "<kernel_add>:", "vpaddb");
    assert_adjacent(&text, "<kernel_perm>:", "vpermb");
}

#[test]
fn x86_zero_cost_pipeline() {
    let text = build(
        nested_dir("x86-zero-cost-link"),
        "x86-zero-cost-link",
        "-C target-feature=+sse3,+ssse3,+sse4.1,+sse4.2,+avx,+sse4a,+sha,+avx512cd,+avx512vbmi,+avx512vpopcntdq,+avx512bitalg,+avx512vbmi2,+avx512bf16,+avx512fp16,+avx512vp2intersect,+avx,+avx2,+avx512f,+avx512bw,+avx512dq,+avx512ifma,+avx512vnni,+avx512vl,+gfni,+vaes,+vpclmulqdq,+avx10.1 -C linker-plugin-lto -C linker=clang -C link-arg=-fuse-ld=lld",
    );
    for (symbol, mnemonic) in [
        ("<kernel_sse3>:", "addsubps"),
        ("<kernel_ssse3>:", "pshufb"),
        ("<kernel_sse41>:", "pmulld"),
        ("<kernel_sse42>:", "pcmpgtq"),
        ("<kernel_sse4a>:", "insertq"),
        ("<kernel_avx>:", "vaddps"),
        ("<kernel_avx>:", "vmulps"),
        ("<kernel_cd>:", "vplzcntd"),
        ("<kernel_vbmi>:", "vpermb"),
        ("<kernel_vpopcnt>:", "vpopcntd"),
        ("<kernel_bitalg>:", "vpopcntb"),
        ("<kernel_vbmi2>:", "vpcompressb"),
        ("<kernel_bf16>:", "vdpbf16ps"),
        ("<kernel_fp16>:", "vaddph"),
        ("<kernel_vp2intersect>:", "vp2intersectd"),
        ("<kernel_sha>:", "sha256rnds2"),
        ("<kernel_bw>:", "vpaddb"),
        ("<kernel_dq>:", "vpmullq"),
        ("<kernel_ifma>:", "vpmadd52luq"),
        ("<kernel_vnni>:", "vpdpbusd"),
        ("<kernel_gfni>:", "gf2p8affineqb"),
        ("<kernel_pshufb>:", "vpshufb"),
        ("<kernel_vaes>:", "vaesenc"),
        ("<kernel_vpclmul>:", "vpclmulqdq"),
        ("<kernel_avx10_1>:", "vpaddd"),
        ("<kernel_avx10_1>:", "vpmulld"),
    ] {
        assert_adjacent(&text, symbol, mnemonic);
    }
}

fn build(dir: PathBuf, package: &str, rustflags: &str) -> String {
    let output = cargo(&dir)
        .env("RUSTFLAGS", rustflags)
        .args(["build", "-p", package, "--release"])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("release").join(package);
    assert!(binary.is_file(), "missing {}", binary.display());
    let output = Command::new("llvm-objdump")
        .args(["-d", "--no-show-raw-insn"])
        .arg(&binary)
        .output()
        .expect("llvm-objdump");
    assert!(output.status.success(), "{}", output_text(&output));
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn assert_adjacent(text: &str, symbol: &str, mnemonic: &str) {
    let body = function_at(text, symbol);
    assert!(!body_has_call(body, CALLS), "{symbol} calls C:\n{body}");
    let found = x86_opcode_positions(body, mnemonic);
    assert_eq!(found.len(), 2, "{symbol} {mnemonic} body:\n{body}");
    assert_eq!(
        found[1],
        found[0] + 1,
        "store or load between {mnemonic} in {symbol}:\n{body}"
    );
}

fn assert_sequence(text: &str, symbol: &str, first: &str, second: &str) {
    let body = function_at(text, symbol);
    assert!(!body_has_call(body, CALLS), "{symbol} calls C:\n{body}");
    let first_at = x86_opcode_positions(body, first);
    let second_at = x86_opcode_positions(body, second);
    assert_eq!(first_at.len(), 1, "{symbol} body:\n{body}");
    assert_eq!(second_at.len(), 1, "{symbol} body:\n{body}");
    assert_eq!(
        second_at[0],
        first_at[0] + 1,
        "store or load between {first} and {second} in {symbol}:\n{body}"
    );
}
