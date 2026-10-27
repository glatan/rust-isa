#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::*;

#[test]
fn aarch64_pipeline() {
    let dir = nested_dir("aarch64-values");
    let rustflags = "-C target-feature=+aes,+sha2,+crc,+fp16,+fhm,+sha3,+sm4,+i8mm,+bf16,+jsconv,+sve,+f32mm,+f64mm,+fcma -C linker-plugin-lto -C linker=clang -C link-arg=--target=aarch64-unknown-linux-gnu -C link-arg=-fuse-ld=lld -C link-arg=-static -C link-arg=-nostdlib -C relocation-model=static";
    let output = cargo(&dir)
        .env("RUSTC_BOOTSTRAP", "1")
        .env("RUSTFLAGS", rustflags)
        .args([
            "build",
            "--manifest-path",
            "fixtures/aarch64-values/Cargo.toml",
            "-Z",
            "build-std=core",
            "--target",
            "aarch64-unknown-linux-gnu",
            "--release",
        ])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("aarch64-unknown-linux-gnu/release/aarch64-values");
    assert!(binary.is_file(), "missing {}", binary.display());
    let output = Command::new("qemu-aarch64-static")
        .arg("-cpu")
        .arg("max")
        .arg(&binary)
        .output()
        .expect("qemu-aarch64-static");
    assert!(
        output.status.success(),
        "aarch64 value test failed: {}\n{}",
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

    for (symbol, mnemonic) in [
        ("<kernel_add>:", "add"),
        ("<kernel_mul>:", "mul"),
        ("<kernel_tbl>:", "tbl"),
        ("<kernel_dot>:", "sdot"),
        ("<kernel_aes>:", "aese"),
        ("<kernel_sha2>:", "sha256h"),
        ("<kernel_crc>:", "crc32b"),
        ("<kernel_fp16>:", "fadd"),
        ("<kernel_sha3>:", "eor3"),
        ("<kernel_sm4>:", "sm4e"),
        ("<kernel_i8mm>:", "smmla"),
        ("<kernel_bf16>:", "bfdot"),
        ("<kernel_fhm>:", "fmla"),
        ("<kernel_jsconv>:", "fjcvtzs"),
        ("<kernel_fcma>:", "fcmla"),
    ] {
        assert_adjacent(&text, symbol, mnemonic);
    }

    for (symbol, mnemonic) in [
        ("<kernel_sve_add>:", "fadd"),
        ("<kernel_sve_mul>:", "fmul"),
        ("<kernel_sve_sub>:", "fsub"),
        ("<kernel_f32mm>:", "fmmla"),
        ("<kernel_f64mm>:", "fmmla"),
    ] {
        assert_present(&text, symbol, mnemonic);
    }
}

fn assert_adjacent(text: &str, symbol: &str, mnemonic: &str) {
    let body = function_at(text, symbol);
    assert!(!body_has_call(body, &["bl"]), "{symbol} calls C:\n{body}");
    let positions = opcode_positions(body, mnemonic);
    assert_eq!(positions.len(), 2, "{symbol} {mnemonic} body:\n{body}");
    assert_eq!(
        positions[1],
        positions[0] + 1,
        "{mnemonic} is not adjacent:\n{body}"
    );
}

fn assert_present(text: &str, symbol: &str, mnemonic: &str) {
    let body = function_at(text, symbol);
    assert!(!body_has_call(body, &["bl"]), "{symbol} calls C:\n{body}");
    assert!(
        body_has_opcode(body, mnemonic),
        "{symbol} has no {mnemonic}:\n{body}"
    );
}
