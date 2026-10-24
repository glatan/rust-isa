#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::*;

#[test]
fn aarch64_pipeline() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    let dir = nested_dir("aarch64-values");
    let rustflags = "-C target-feature=+aes,+sha2,+crc -C linker-plugin-lto -C linker=clang -C link-arg=--target=aarch64-unknown-linux-gnu -C link-arg=-fuse-ld=lld -C link-arg=-static -C link-arg=-nostdlib -C relocation-model=static";
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

    let add = function_at(&text, "<kernel_add>:");
    assert!(
        !add.lines()
            .any(|line| line.split_whitespace().any(|word| word == "bl")),
        "kernel_add calls C:\n{add}"
    );
    let lines: Vec<_> = add.lines().collect();
    let adds: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("add\tv"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(adds.len(), 2, "kernel_add body:\n{add}");
    assert_eq!(adds[1], adds[0] + 1, "add is not adjacent:\n{add}");

    let mul = function_at(&text, "<kernel_mul>:");
    assert!(
        !mul.lines()
            .any(|line| line.split_whitespace().any(|word| word == "bl")),
        "kernel_mul calls C:\n{mul}"
    );
    let lines: Vec<_> = mul.lines().collect();
    let muls: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("mul\tv"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(muls.len(), 2, "kernel_mul body:\n{mul}");
    assert_eq!(muls[1], muls[0] + 1, "mul is not adjacent:\n{mul}");

    let tbl = function_at(&text, "<kernel_tbl>:");
    assert!(
        !tbl.lines()
            .any(|line| line.split_whitespace().any(|word| word == "bl")),
        "kernel_tbl calls C:\n{tbl}"
    );
    let lines: Vec<_> = tbl.lines().collect();
    let tbls: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("tbl\tv"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(tbls.len(), 2, "kernel_tbl body:\n{tbl}");
    assert_eq!(tbls[1], tbls[0] + 1, "tbl is not adjacent:\n{tbl}");

    let dot = function_at(&text, "<kernel_dot>:");
    assert!(
        !dot.lines()
            .any(|line| line.split_whitespace().any(|word| word == "bl")),
        "kernel_dot calls C:\n{dot}"
    );
    let lines: Vec<_> = dot.lines().collect();
    let sdots: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("sdot\tv"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(sdots.len(), 2, "kernel_dot body:\n{dot}");
    assert_eq!(sdots[1], sdots[0] + 1, "sdot is not adjacent:\n{dot}");

    let aes = function_at(&text, "<kernel_aes>:");
    assert!(
        !aes.lines()
            .any(|line| line.split_whitespace().any(|word| word == "bl")),
        "kernel_aes calls C:\n{aes}"
    );
    let lines: Vec<_> = aes.lines().collect();
    let aes_count: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("aese"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(aes_count.len(), 2, "kernel_aes body:\n{aes}");
    assert_eq!(
        aes_count[1],
        aes_count[0] + 1,
        "aese is not adjacent:\n{aes}"
    );

    let sha2 = function_at(&text, "<kernel_sha2>:");
    assert!(
        !sha2
            .lines()
            .any(|line| line.split_whitespace().any(|word| word == "bl")),
        "kernel_sha2 calls C:\n{sha2}"
    );
    let lines: Vec<_> = sha2.lines().collect();
    let sha2_count: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("sha256h"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(sha2_count.len(), 2, "kernel_sha2 body:\n{sha2}");
    assert_eq!(
        sha2_count[1],
        sha2_count[0] + 1,
        "sha256h is not adjacent:\n{sha2}"
    );

    let crc = function_at(&text, "<kernel_crc>:");
    assert!(
        !crc.lines()
            .any(|line| line.split_whitespace().any(|word| word == "bl")),
        "kernel_crc calls C:\n{crc}"
    );
    let lines: Vec<_> = crc.lines().collect();
    let crc_count: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("crc32b"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(crc_count.len(), 2, "kernel_crc body:\n{crc}");
    assert_eq!(
        crc_count[1],
        crc_count[0] + 1,
        "crc32b is not adjacent:\n{crc}"
    );
}
