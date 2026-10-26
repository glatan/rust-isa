#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::*;
use std::io::Write;
use std::process::Stdio;

#[test]
fn mips_values() {
    let dir = nested_dir("mips-values");
    let rustflags = "-C target-feature=+msa,+fp64 -C linker-plugin-lto -C linker=clang -C link-arg=--target=mips64-unknown-linux-gnuabi64 -C link-arg=-fuse-ld=lld -C link-arg=-static -C link-arg=-nostdlib -C link-arg=-Wl,-e,_start -C relocation-model=static";
    let output = cargo(&dir)
        .env("RUSTC_BOOTSTRAP", "1")
        .env("RUSTFLAGS", rustflags)
        .args([
            "build",
            "--manifest-path",
            "fixtures/mips-values/Cargo.toml",
            "-Z",
            "build-std=core",
            "--target",
            "mips64-unknown-linux-gnuabi64",
            "--release",
        ])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("mips64-unknown-linux-gnuabi64/release/mips-values");
    assert!(binary.is_file(), "missing {}", binary.display());
    let output = Command::new("qemu-mips64-static")
        .args(["-cpu", "Loongson-3A4000"])
        .arg(&binary)
        .output()
        .expect("qemu-mips64-static");
    assert!(
        output.status.success(),
        "mips value test failed: {}\n{}",
        output.status,
        output_text(&output)
    );

    let output = Command::new("llvm-objdump")
        .args(["-d"])
        .arg(&binary)
        .output()
        .expect("llvm-objdump");
    assert!(output.status.success(), "{}", output_text(&output));
    let text = String::from_utf8_lossy(&output.stdout);
    for (symbol, mnemonic) in [("<kernel_add>:", "addv.w"), ("<kernel_mul>:", "mulv.w")] {
        let decoded = decode_msa(&text, symbol);
        assert!(
            !body_has_call(&decoded, &["jal", "jalr"]),
            "{symbol} calls C:\n{decoded}"
        );
        assert_eq!(
            opcode_positions(&decoded, mnemonic).len(),
            2,
            "{symbol} {mnemonic} body:\n{decoded}"
        );
        assert!(
            !decoded.lines().any(|line| {
                let op = line.split_whitespace().next().unwrap_or("");
                matches!(op, "lw" | "sw" | "ld" | "sd" | "st")
            }),
            "{symbol} has a memory operation:\n{decoded}"
        );
    }
}

fn decode_msa(text: &str, symbol: &str) -> String {
    let body = function_at(text, symbol);
    let mut hex = String::new();
    for line in body.lines() {
        let Some((_, rest)) = line.split_once(':') else {
            continue;
        };
        let mut count = 0;
        for token in rest.split_whitespace() {
            if token.len() != 2 || !token.chars().all(|ch| ch.is_ascii_hexdigit()) {
                break;
            }
            hex.push_str("0x");
            hex.push_str(token);
            hex.push(' ');
            count += 1;
            if count == 4 {
                break;
            }
        }
    }
    let mut child = Command::new("llvm-mc")
        .args([
            "--disassemble",
            "--triple=mips64-unknown-linux-gnuabi64",
            "--mattr=+msa",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("llvm-mc");
    child
        .stdin
        .as_mut()
        .expect("llvm-mc stdin")
        .write_all(hex.as_bytes())
        .expect("write llvm-mc stdin");
    let output = child.wait_with_output().expect("llvm-mc");
    assert!(output.status.success(), "llvm-mc failed");
    String::from_utf8_lossy(&output.stdout).into_owned()
}
