#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::*;

#[test]
fn riscv_pipeline() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    let dir = nested_dir("riscv-values");
    let rustflags = "-C target-feature=+v -C linker-plugin-lto -C linker=clang -C link-arg=--target=riscv64-unknown-linux-gnu -C link-arg=-fuse-ld=lld -C link-arg=-static -C link-arg=-nostdlib -C relocation-model=static";
    let output = cargo(&dir)
        .env("RUSTC_BOOTSTRAP", "1")
        .env("RUSTFLAGS", rustflags)
        .args([
            "build",
            "--manifest-path",
            "fixtures/riscv-values/Cargo.toml",
            "-Z",
            "build-std=core",
            "--target",
            "riscv64gc-unknown-linux-gnu",
            "--release",
        ])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("riscv64gc-unknown-linux-gnu/release/riscv-values");
    assert!(binary.is_file(), "missing {}", binary.display());
    let output = Command::new("qemu-riscv64-static")
        .arg(&binary)
        .output()
        .expect("qemu-riscv64-static");
    assert!(
        output.status.success(),
        "riscv value test failed: {}\n{}",
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

    for (symbol, mnemonic) in [("<kernel_add>:", "vadd.vv"), ("<kernel_mul>:", "vmul.vv")] {
        let body = function_at(&text, symbol);
        assert!(
            !body
                .lines()
                .any(|line| line.split_whitespace().any(|word| word == "jal")),
            "{symbol} calls C:\n{body}"
        );
        let lines: Vec<_> = body.lines().collect();
        let found: Vec<_> = lines
            .iter()
            .enumerate()
            .filter(|(_, line)| line.contains(mnemonic))
            .map(|(index, _)| index)
            .collect();
        assert_eq!(found.len(), 2, "{symbol} body:\n{body}");
        assert_eq!(
            found[1],
            found[0] + 1,
            "{mnemonic} is not adjacent:\n{body}"
        );
    }
}
