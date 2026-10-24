#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::*;

#[test]
fn loongarch_pipeline() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    let dir = nested_dir("loongarch-values");
    let rustflags = "-C linker-plugin-lto -C linker=clang -C link-arg=--target=loongarch64-unknown-linux-gnu -C link-arg=-fuse-ld=lld -C link-arg=-static -C link-arg=-nostdlib -C relocation-model=static";
    let output = cargo(&dir)
        .env("RUSTC_BOOTSTRAP", "1")
        .env("RUSTFLAGS", rustflags)
        .args([
            "build",
            "--manifest-path",
            "fixtures/loongarch-values/Cargo.toml",
            "-Z",
            "build-std=core",
            "--target",
            "loongarch64-unknown-linux-gnu",
            "--release",
        ])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("loongarch64-unknown-linux-gnu/release/loongarch-values");
    assert!(binary.is_file(), "missing {}", binary.display());
    let output = Command::new("qemu-loongarch64-static")
        .arg("-cpu")
        .arg("max")
        .arg(&binary)
        .output()
        .expect("qemu-loongarch64-static");
    assert!(
        output.status.success(),
        "loongarch value test failed: {}\n{}",
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

    let add = function_at(&text, "<kernel_add>:");
    assert!(
        !add.lines()
            .any(|line| line.split_whitespace().any(|word| word == "bl")),
        "kernel_add calls C:\n{add}"
    );
    assert!(add.contains("vadd.w"), "kernel_add body:\n{add}");

    let mul = function_at(&text, "<kernel_mul>:");
    assert!(
        !mul.lines()
            .any(|line| line.split_whitespace().any(|word| word == "bl")),
        "kernel_mul calls C:\n{mul}"
    );
    let lines: Vec<_> = mul.lines().collect();
    let found: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("vmul.w"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(found.len(), 2, "kernel_mul body:\n{mul}");
    assert_eq!(found[1], found[0] + 1, "vmul.w is not adjacent:\n{mul}");

    for (symbol, mnemonic) in [
        ("<kernel_xvadd>:", "xvadd.w"),
        ("<kernel_xvmul>:", "xvmul.w"),
    ] {
        let body = function_at(&text, symbol);
        assert!(
            !body
                .lines()
                .any(|line| line.split_whitespace().any(|word| word == "bl")),
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
