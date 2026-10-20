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
fn avx512_pipeline() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    let dir = nested_dir("avx512-link");
    let rustflags =
        "-C target-feature=+avx512f -C linker-plugin-lto -C linker=clang -C link-arg=-fuse-ld=lld";
    let output = cargo(&dir)
        .env("RUSTFLAGS", rustflags)
        .args(["build", "-p", "avx512-link", "--release"])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("release/avx512-link");
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
fn crypto_pipeline() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    let dir = nested_dir("crypto-link");
    let rustflags = "-C target-feature=+aes,+avx2,+fma -C linker-plugin-lto -C linker=clang -C link-arg=-fuse-ld=lld";
    let output = cargo(&dir)
        .env("RUSTFLAGS", rustflags)
        .args(["build", "-p", "crypto-link", "--release"])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("release/crypto-link");
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
