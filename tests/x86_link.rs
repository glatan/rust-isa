#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::*;

#[test]
fn sse2_pipeline() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    let dir = nested_dir("sse2-link");
    let rustflags =
        "-C target-feature=+sse2 -C linker-plugin-lto -C linker=clang -C link-arg=-fuse-ld=lld";
    let output = cargo(&dir)
        .env("RUSTFLAGS", rustflags)
        .args(["build", "-p", "sse2-link", "--release"])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("release/sse2-link");
    assert!(binary.is_file(), "missing {}", binary.display());
    let output = Command::new("llvm-objdump")
        .args(["-d", "--no-show-raw-insn"])
        .arg(&binary)
        .output()
        .expect("llvm-objdump");
    assert!(output.status.success(), "{}", output_text(&output));
    let text = String::from_utf8_lossy(&output.stdout);
    let start = text.find("<kernel>:").expect("kernel symbol");
    let body = &text[start..];
    let body = &body[..body.find("\n\n").unwrap_or(body.len())];
    assert!(!body.contains("call"), "kernel calls C:\n{body}");
    let lines: Vec<_> = body.lines().collect();
    let paddd: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("paddd"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(paddd.len(), 2, "kernel body:\n{body}");
    assert_eq!(
        paddd[1],
        paddd[0] + 1,
        "store or load between paddd:\n{body}"
    );
}

#[test]
fn avx2_pipeline() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    let dir = nested_dir("avx2-link");
    let rustflags =
        "-C target-feature=+avx2 -C linker-plugin-lto -C linker=clang -C link-arg=-fuse-ld=lld";
    let output = cargo(&dir)
        .env("RUSTFLAGS", rustflags)
        .args(["build", "-p", "avx2-link", "--release"])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("release/avx2-link");
    assert!(binary.is_file(), "missing {}", binary.display());
    let output = Command::new("llvm-objdump")
        .args(["-d", "--no-show-raw-insn"])
        .arg(&binary)
        .output()
        .expect("llvm-objdump");
    assert!(output.status.success(), "{}", output_text(&output));
    let text = String::from_utf8_lossy(&output.stdout);
    let start = text.find("<kernel>:").expect("kernel symbol");
    let body = &text[start..];
    let body = &body[..body.find("\n\n").unwrap_or(body.len())];
    assert!(!body.contains("call"), "kernel calls C:\n{body}");
    let add = body
        .lines()
        .position(|line| line.contains("vpaddd"))
        .unwrap_or_else(|| panic!("missing vpaddd:\n{body}"));
    let mul = body
        .lines()
        .position(|line| line.contains("vpmulld"))
        .unwrap_or_else(|| panic!("missing vpmulld:\n{body}"));
    assert_eq!(
        mul,
        add + 1,
        "store or load between vpaddd and vpmulld:\n{body}"
    );
}

#[test]
fn avx512f_pipeline() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    let dir = nested_dir("avx512f-link");
    let rustflags =
        "-C target-feature=+avx512f -C linker-plugin-lto -C linker=clang -C link-arg=-fuse-ld=lld";
    let output = cargo(&dir)
        .env("RUSTFLAGS", rustflags)
        .args(["build", "-p", "avx512f-link", "--release"])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("release/avx512f-link");
    assert!(binary.is_file(), "missing {}", binary.display());
    let output = Command::new("llvm-objdump")
        .args(["-d", "--no-show-raw-insn"])
        .arg(&binary)
        .output()
        .expect("llvm-objdump");
    assert!(output.status.success(), "{}", output_text(&output));
    let text = String::from_utf8_lossy(&output.stdout);
    let start = text.find("<kernel>:").expect("kernel symbol");
    let body = &text[start..];
    let body = &body[..body.find("\n\n").unwrap_or(body.len())];
    assert!(!body.contains("call"), "kernel calls C:\n{body}");
    let lines: Vec<_> = body.lines().collect();
    let paddd: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("vpaddd"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(paddd.len(), 2, "kernel body:\n{body}");
    assert_eq!(
        paddd[1],
        paddd[0] + 1,
        "store or load between vpaddd:\n{body}"
    );
}

#[test]
fn aes_pipeline() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    let dir = nested_dir("aes-link");
    let rustflags =
        "-C target-feature=+aes -C linker-plugin-lto -C linker=clang -C link-arg=-fuse-ld=lld";
    let output = cargo(&dir)
        .env("RUSTFLAGS", rustflags)
        .args(["build", "-p", "aes-link", "--release"])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("release/aes-link");
    assert!(binary.is_file(), "missing {}", binary.display());
    let output = Command::new("llvm-objdump")
        .args(["-d", "--no-show-raw-insn"])
        .arg(&binary)
        .output()
        .expect("llvm-objdump");
    assert!(output.status.success(), "{}", output_text(&output));
    let text = String::from_utf8_lossy(&output.stdout);

    let aes = function_at(&text, "<kernel_aes>:");
    assert!(!aes.contains("call"), "kernel_aes calls C:\n{aes}");
    let lines: Vec<_> = aes.lines().collect();
    let aesenc: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("aesenc"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(aesenc.len(), 2, "kernel_aes body:\n{aes}");
    assert_eq!(
        aesenc[1],
        aesenc[0] + 1,
        "store or load between aesenc:\n{aes}"
    );
}

#[test]
fn fma_pipeline() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    let dir = nested_dir("fma-link");
    let rustflags = "-C target-feature=+avx2,+fma -C linker-plugin-lto -C linker=clang -C link-arg=-fuse-ld=lld";
    let output = cargo(&dir)
        .env("RUSTFLAGS", rustflags)
        .args(["build", "-p", "fma-link", "--release"])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("release/fma-link");
    assert!(binary.is_file(), "missing {}", binary.display());
    let output = Command::new("llvm-objdump")
        .args(["-d", "--no-show-raw-insn"])
        .arg(&binary)
        .output()
        .expect("llvm-objdump");
    assert!(output.status.success(), "{}", output_text(&output));
    let text = String::from_utf8_lossy(&output.stdout);

    let fma = function_at(&text, "<kernel_fma>:");
    assert!(!fma.contains("call"), "kernel_fma calls C:\n{fma}");
    let lines: Vec<_> = fma.lines().collect();
    let vfmadd: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("vfmadd"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(vfmadd.len(), 2, "kernel_fma body:\n{fma}");
    assert_eq!(
        vfmadd[1],
        vfmadd[0] + 1,
        "store or load between vfmadd:\n{fma}"
    );
}

#[test]
fn avx512vl_pipeline() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    let dir = nested_dir("avx512vl-link");
    let rustflags = "-C target-feature=+avx512f,+avx512bw,+avx512vl,+avx512vbmi -C linker-plugin-lto -C linker=clang -C link-arg=-fuse-ld=lld";
    let output = cargo(&dir)
        .env("RUSTFLAGS", rustflags)
        .args(["build", "-p", "avx512vl-link", "--release"])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("release/avx512vl-link");
    assert!(binary.is_file(), "missing {}", binary.display());
    let output = Command::new("llvm-objdump")
        .args(["-d", "--no-show-raw-insn"])
        .arg(&binary)
        .output()
        .expect("llvm-objdump");
    assert!(output.status.success(), "{}", output_text(&output));
    let text = String::from_utf8_lossy(&output.stdout);

    let add = function_at(&text, "<kernel_add>:");
    assert!(!add.contains("call"), "kernel_add calls C:\n{add}");
    let add_lines: Vec<_> = add.lines().collect();
    let vpaddb: Vec<_> = add_lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("vpaddb"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(vpaddb.len(), 2, "kernel_add body:\n{add}");
    assert_eq!(
        vpaddb[1],
        vpaddb[0] + 1,
        "store or load between vpaddb:\n{add}"
    );

    let perm = function_at(&text, "<kernel_perm>:");
    assert!(!perm.contains("call"), "kernel_perm calls C:\n{perm}");
    let perm_lines: Vec<_> = perm.lines().collect();
    let vpermb: Vec<_> = perm_lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("vpermb"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(vpermb.len(), 2, "kernel_perm body:\n{perm}");
    assert_eq!(
        vpermb[1],
        vpermb[0] + 1,
        "store or load between vpermb:\n{perm}"
    );
}

#[test]
fn x86_zero_cost_pipeline() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    let dir = nested_dir("x86-zero-cost-link");
    let rustflags = "-C target-feature=+sse3,+ssse3,+sse4.1,+sse4.2,+avx,+sse4a,+sha,+avx512cd,+avx512vbmi,+avx512vpopcntdq,+avx512bitalg,+avx512vbmi2,+avx512bf16,+avx512fp16,+avx512vp2intersect,+avx,+avx2,+avx512f,+avx512bw,+avx512dq,+avx512ifma,+avx512vnni,+avx512vl,+gfni,+vaes,+vpclmulqdq,+avx10.1 -C linker-plugin-lto -C linker=clang -C link-arg=-fuse-ld=lld";
    let output = cargo(&dir)
        .env("RUSTFLAGS", rustflags)
        .args(["build", "-p", "x86-zero-cost-link", "--release"])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("release/x86-zero-cost-link");
    assert!(binary.is_file(), "missing {}", binary.display());
    let output = Command::new("llvm-objdump")
        .args(["-d", "--no-show-raw-insn"])
        .arg(&binary)
        .output()
        .expect("llvm-objdump");
    assert!(output.status.success(), "{}", output_text(&output));
    let text = String::from_utf8_lossy(&output.stdout);

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

fn assert_adjacent(text: &str, symbol: &str, mnemonic: &str) {
    let body = function_at(text, symbol);
    assert!(!body.contains("call"), "{symbol} calls C:\n{body}");
    let lines: Vec<_> = body.lines().collect();
    let found: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains(mnemonic))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(found.len(), 2, "{symbol} {mnemonic} body:\n{body}");
    assert_eq!(
        found[1],
        found[0] + 1,
        "store or load between {mnemonic} in {symbol}:\n{body}"
    );
}
