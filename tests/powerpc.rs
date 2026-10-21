#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::*;

#[test]
fn powerpc_pipeline() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    let dir = nested_dir("powerpc-values");
    let rustflags = "-C target-feature=+altivec,+vsx -C linker-plugin-lto -C linker=clang -C link-arg=--target=powerpc64le-unknown-linux-gnu -C link-arg=-fuse-ld=lld -C link-arg=-static -C link-arg=-nostdlib -C relocation-model=static";
    let output = cargo(&dir)
        .env("RUSTC_BOOTSTRAP", "1")
        .env("RUSTFLAGS", rustflags)
        .args([
            "build",
            "--manifest-path",
            "fixtures/powerpc-values/Cargo.toml",
            "-Z",
            "build-std=core",
            "--target",
            "powerpc64le-unknown-linux-gnu",
            "--release",
        ])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let binary = dir.join("powerpc64le-unknown-linux-gnu/release/powerpc-values");
    assert!(binary.is_file(), "missing {}", binary.display());
    let output = Command::new("qemu-ppc64le-static")
        .arg("-cpu")
        .arg("power10")
        .arg(&binary)
        .output()
        .expect("qemu-ppc64le-static");
    assert!(
        output.status.success(),
        "powerpc value test failed: {}\n{}",
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

    for (symbol, mnemonic) in [("<kernel_add>:", "vadduwm"), ("<kernel_mul>:", "vmuluwm")] {
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
