#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::*;

#[test]
fn i686_values() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    let dir = nested_dir("i686-values");
    let rustflags = "-C linker-plugin-lto -C linker=clang -C link-arg=--target=i686-unknown-linux-gnu -C link-arg=-fuse-ld=lld -C link-arg=-static -C link-arg=-nostdlib -C relocation-model=static";
    let output = cargo(&dir)
        .env("RUSTC_BOOTSTRAP", "1")
        .env("RUSTFLAGS", rustflags)
        .args([
            "build",
            "--manifest-path",
            "fixtures/i686-values/Cargo.toml",
            "-Z",
            "build-std=core",
            "--target",
            "i686-unknown-linux-gnu",
            "--release",
        ])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("i686-unknown-linux-gnu/release/i686-values");
    assert!(binary.is_file(), "missing {}", binary.display());
    let sde = std::env::var("RUST_ISA_SDE32").unwrap_or_else(|_| "sde".to_string());
    let flags = std::env::var("RUST_ISA_SDE_FLAGS").unwrap_or_else(|_| "-future".to_string());
    let mut command = Command::new(&sde);
    for flag in flags.split_whitespace() {
        command.arg(flag);
    }
    let output = command
        .arg("--")
        .arg(&binary)
        .output()
        .unwrap_or_else(|error| panic!("failed to run {sde}: {error}"));
    assert!(
        output.status.success(),
        "i686 value test failed under SDE: {}\n{}",
        output.status,
        output_text(&output)
    );

    let output = Command::new("llvm-objdump")
        .args(["-d", "--no-show-raw-insn"])
        .arg(&binary)
        .output()
        .expect("llvm-objdump");
    assert!(output.status.success(), "{}", output_text(&output));
    let text = String::from_utf8_lossy(&output.stdout);

    let sse2 = function_at(&text, "<kernel_sse2>:");
    assert!(!sse2.contains("call"), "kernel_sse2 calls C:\n{sse2}");
    let lines: Vec<_> = sse2.lines().collect();
    let paddd: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("paddd"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(paddd.len(), 2, "kernel_sse2 body:\n{sse2}");
    assert_eq!(paddd[1], paddd[0] + 1, "paddd is not adjacent:\n{sse2}");
    let pmuludq = lines
        .iter()
        .position(|line| line.contains("pmuludq"))
        .unwrap_or_else(|| panic!("missing pmuludq:\n{sse2}"));
    assert_eq!(pmuludq, paddd[1] + 1, "pmuludq is not adjacent:\n{sse2}");

    let avx2 = function_at(&text, "<kernel_avx2>:");
    assert!(!avx2.contains("call"), "kernel_avx2 calls C:\n{avx2}");
    let lines: Vec<_> = avx2.lines().collect();
    let vpaddd: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("vpaddd"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(vpaddd.len(), 2, "kernel_avx2 body:\n{avx2}");
    assert_eq!(vpaddd[1], vpaddd[0] + 1, "vpaddd is not adjacent:\n{avx2}");
    let vpmulld = lines
        .iter()
        .position(|line| line.contains("vpmulld"))
        .unwrap_or_else(|| panic!("missing vpmulld:\n{avx2}"));
    assert_eq!(vpmulld, vpaddd[1] + 1, "vpmulld is not adjacent:\n{avx2}");
}
