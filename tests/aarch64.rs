#![cfg(not(target_arch = "wasm32"))]

mod common;

use common::*;

#[test]
fn aarch64_pipeline() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
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

    let fp16 = function_at(&text, "<kernel_fp16>:");
    assert!(
        !fp16
            .lines()
            .any(|line| line.split_whitespace().any(|word| word == "bl")),
        "kernel_fp16 calls C:\n{fp16}"
    );
    let lines: Vec<_> = fp16.lines().collect();
    let fadd_count: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("fadd"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(fadd_count.len(), 2, "kernel_fp16 body:\n{fp16}");
    assert_eq!(
        fadd_count[1],
        fadd_count[0] + 1,
        "fadd is not adjacent:\n{fp16}"
    );

    let sha3 = function_at(&text, "<kernel_sha3>:");
    assert!(
        !sha3
            .lines()
            .any(|line| line.split_whitespace().any(|word| word == "bl")),
        "kernel_sha3 calls C:\n{sha3}"
    );
    let lines: Vec<_> = sha3.lines().collect();
    let eor3_count: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("eor3"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(eor3_count.len(), 2, "kernel_sha3 body:\n{sha3}");
    assert_eq!(
        eor3_count[1],
        eor3_count[0] + 1,
        "eor3 is not adjacent:\n{sha3}"
    );

    let sm4 = function_at(&text, "<kernel_sm4>:");
    assert!(
        !sm4.lines()
            .any(|line| line.split_whitespace().any(|word| word == "bl")),
        "kernel_sm4 calls C:\n{sm4}"
    );
    let lines: Vec<_> = sm4.lines().collect();
    let sm4e_count: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("sm4e"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(sm4e_count.len(), 2, "kernel_sm4 body:\n{sm4}");
    assert_eq!(
        sm4e_count[1],
        sm4e_count[0] + 1,
        "sm4e is not adjacent:\n{sm4}"
    );

    let i8mm = function_at(&text, "<kernel_i8mm>:");
    assert!(
        !i8mm
            .lines()
            .any(|line| line.split_whitespace().any(|word| word == "bl")),
        "kernel_i8mm calls C:\n{i8mm}"
    );
    let lines: Vec<_> = i8mm.lines().collect();
    let smmla_count: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("smmla"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(smmla_count.len(), 2, "kernel_i8mm body:\n{i8mm}");
    assert_eq!(
        smmla_count[1],
        smmla_count[0] + 1,
        "smmla is not adjacent:\n{i8mm}"
    );

    let bf16 = function_at(&text, "<kernel_bf16>:");
    assert!(
        !bf16
            .lines()
            .any(|line| line.split_whitespace().any(|word| word == "bl")),
        "kernel_bf16 calls C:\n{bf16}"
    );
    let lines: Vec<_> = bf16.lines().collect();
    let bfdot_count: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("bfdot"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(bfdot_count.len(), 2, "kernel_bf16 body:\n{bf16}");
    assert_eq!(
        bfdot_count[1],
        bfdot_count[0] + 1,
        "bfdot is not adjacent:\n{bf16}"
    );

    let fhm = function_at(&text, "<kernel_fhm>:");
    assert!(
        !fhm.lines()
            .any(|line| line.split_whitespace().any(|word| word == "bl")),
        "kernel_fhm calls C:\n{fhm}"
    );
    let lines: Vec<_> = fhm.lines().collect();
    let fmla_count: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("fmla"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(fmla_count.len(), 2, "kernel_fhm body:\n{fhm}");
    assert_eq!(
        fmla_count[1],
        fmla_count[0] + 1,
        "fmla is not adjacent:\n{fhm}"
    );

    let jsconv = function_at(&text, "<kernel_jsconv>:");
    assert!(
        !jsconv
            .lines()
            .any(|line| line.split_whitespace().any(|word| word == "bl")),
        "kernel_jsconv calls C:\n{jsconv}"
    );
    let lines: Vec<_> = jsconv.lines().collect();
    let fjcvtzs_count: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("fjcvtzs"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(fjcvtzs_count.len(), 2, "kernel_jsconv body:\n{jsconv}");
    assert_eq!(
        fjcvtzs_count[1],
        fjcvtzs_count[0] + 1,
        "fjcvtzs is not adjacent:\n{jsconv}"
    );

    let sve = function_at(&text, "<kernel_sve_add>:");
    assert!(
        !sve.lines()
            .any(|line| line.split_whitespace().any(|word| word == "bl")),
        "kernel_sve_add calls C:\n{sve}"
    );
    assert!(
        sve.lines().any(|line| line.contains("add\tz")),
        "kernel_sve_add has no SVE add:\n{sve}"
    );

    let f32mm = function_at(&text, "<kernel_f32mm>:");
    assert!(
        !f32mm
            .lines()
            .any(|line| line.split_whitespace().any(|word| word == "bl")),
        "kernel_f32mm calls C:\n{f32mm}"
    );
    assert!(
        f32mm.lines().any(|line| line.contains("fmmla")),
        "kernel_f32mm has no fmmla:\n{f32mm}"
    );

    let f64mm = function_at(&text, "<kernel_f64mm>:");
    assert!(
        !f64mm
            .lines()
            .any(|line| line.split_whitespace().any(|word| word == "bl")),
        "kernel_f64mm calls C:\n{f64mm}"
    );
    assert!(
        f64mm.lines().any(|line| line.contains("fmmla")),
        "kernel_f64mm has no fmmla:\n{f64mm}"
    );

    let fcma = function_at(&text, "<kernel_fcma>:");
    assert!(
        !fcma
            .lines()
            .any(|line| line.split_whitespace().any(|word| word == "bl")),
        "kernel_fcma calls C:\n{fcma}"
    );
    let lines: Vec<_> = fcma.lines().collect();
    let fcmla_count: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains("fcmla"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(fcmla_count.len(), 2, "kernel_fcma body:\n{fcma}");
    assert_eq!(
        fcmla_count[1],
        fcmla_count[0] + 1,
        "fcmla is not adjacent:\n{fcma}"
    );
}
