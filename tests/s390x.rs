#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::*;
use std::io::Write;
use std::process::Stdio;

#[test]
fn s390x_values() {
    let dir = nested_dir("s390x-values");
    let rustflags = "-C target-feature=+vector -C linker-plugin-lto -C linker=clang -C link-arg=--target=s390x-unknown-linux-gnu -C link-arg=-fuse-ld=lld -C link-arg=-static -C link-arg=-nostdlib -C relocation-model=static";
    let output = cargo(&dir)
        .env("RUSTC_BOOTSTRAP", "1")
        .env("RUSTFLAGS", rustflags)
        .args([
            "build",
            "--manifest-path",
            "fixtures/s390x-values/Cargo.toml",
            "-Z",
            "build-std=core",
            "--target",
            "s390x-unknown-linux-gnu",
            "--release",
        ])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("s390x-unknown-linux-gnu/release/s390x-values");
    assert!(binary.is_file(), "missing {}", binary.display());
    let output = Command::new("qemu-s390x-static")
        .arg("-cpu")
        .arg("max")
        .arg(&binary)
        .output()
        .expect("qemu-s390x-static");
    assert!(
        output.status.success(),
        "s390x value test failed: {}\n{}",
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
    for (symbol, mnemonic) in [
        ("<kernel_add>:", "vaf"),
        ("<kernel_mul>:", "vmlf"),
        ("<kernel_perm>:", "vperm"),
    ] {
        let decoded = decode_vector(&text, symbol);
        let lines: Vec<_> = decoded.lines().collect();
        assert!(
            !body_has_call(&decoded, &["brasl"]),
            "{symbol} calls C:\n{decoded}"
        );
        let positions = opcode_positions(&decoded, mnemonic);
        assert_eq!(positions.len(), 2, "{symbol} {mnemonic} body:\n{decoded}");
        assert!(
            !lines[positions[0] + 1..positions[1]].iter().any(|line| {
                let op = line.split_whitespace().next().unwrap_or("");
                matches!(op, "vl" | "vst" | "lg" | "stg" | "mvc")
            }),
            "{symbol} has a memory operation between {mnemonic}:\n{decoded}"
        );
    }
}

fn decode_vector(text: &str, symbol: &str) -> String {
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
            if count == 6 {
                break;
            }
        }
    }
    let mut child = Command::new("llvm-mc")
        .args([
            "--disassemble",
            "--triple=s390x-unknown-linux-gnu",
            "--mattr=+vector",
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
