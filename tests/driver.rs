#![cfg(not(target_arch = "wasm32"))]

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::SystemTime;

const SYMBOL_COUNT: usize = 374;

#[test]
fn rust_and_c_symbols_match() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/wasm32/simd/simd128");
    let mut rust_symbols = BTreeSet::new();
    for path in rust_sources(&root) {
        rust_symbols.extend(rust_symbols_in(
            &fs::read_to_string(&path).unwrap(),
            "rust_isa_simd128_",
        ));
    }
    let c_symbols = c_symbols_in(
        &fs::read_to_string(root.join("simd128.c")).unwrap(),
        "rust_isa_simd128_",
    );
    let missing_in_c: Vec<_> = rust_symbols.difference(&c_symbols).collect();
    let missing_in_rust: Vec<_> = c_symbols.difference(&rust_symbols).collect();
    assert!(
        missing_in_c.is_empty() && missing_in_rust.is_empty(),
        "missing in C: {missing_in_c:?}\nmissing in Rust: {missing_in_rust:?}"
    );
    assert_eq!(rust_symbols.len(), SYMBOL_COUNT);
}

#[test]
fn relaxed_symbols_match() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/wasm32/simd/relaxed_simd");
    let mut rust_symbols = BTreeSet::new();
    for path in rust_sources(&root) {
        rust_symbols.extend(rust_symbols_in(
            &fs::read_to_string(&path).unwrap(),
            "rust_isa_relaxed_simd_",
        ));
    }
    let c_symbols = c_symbols_in(
        &fs::read_to_string(root.join("relaxed_simd.c")).unwrap(),
        "rust_isa_relaxed_simd_",
    );
    let missing_in_c: Vec<_> = rust_symbols.difference(&c_symbols).collect();
    let missing_in_rust: Vec<_> = c_symbols.difference(&rust_symbols).collect();
    assert!(
        missing_in_c.is_empty() && missing_in_rust.is_empty(),
        "missing in C: {missing_in_c:?}\nmissing in Rust: {missing_in_rust:?}"
    );
    assert_eq!(rust_symbols.len(), 20);
}

#[test]
fn x86_symbols_match() {
    check_x86_symbols("sse2", "sse2.c", "rust_isa_sse2_");
    check_x86_symbols("avx2", "avx2.c", "rust_isa_avx2_");
    check_x86_symbols("avx512f", "avx512f.c", "rust_isa_avx512f_");
    check_x86_symbols("avx512bw", "avx512bw.c", "rust_isa_avx512bw_");
    check_x86_symbols("avx512dq", "avx512dq.c", "rust_isa_avx512dq_");
    check_x86_symbols("avx512ifma", "avx512ifma.c", "rust_isa_avx512ifma_");
    check_x86_symbols("avx512vbmi", "avx512vbmi.c", "rust_isa_avx512vbmi_");
    check_x86_symbols("avx512vnni", "avx512vnni.c", "rust_isa_avx512vnni_");
    check_x86_symbols("avx512vl", "avx512vl.c", "rust_isa_avx512vl_");
}

fn check_x86_symbols(module: &str, c_file: &str, prefix: &str) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/x86/simd")
        .join(module);
    let mut rust_symbols = BTreeSet::new();
    for path in rust_sources(&root) {
        rust_symbols.extend(rust_symbols_in(&fs::read_to_string(&path).unwrap(), prefix));
    }
    let c_symbols = c_symbols_in(&fs::read_to_string(root.join(c_file)).unwrap(), prefix);
    assert_eq!(rust_symbols, c_symbols, "{module}");
    assert!(!rust_symbols.is_empty(), "{module}");
}

#[test]
fn rustdoc_is_english() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    for path in rust_sources(&root) {
        let text = fs::read_to_string(&path).unwrap();
        for (number, line) in text.lines().enumerate() {
            let trimmed = line.trim_start();
            if !trimmed.starts_with("///") && !trimmed.starts_with("//!") {
                continue;
            }
            assert!(
                !line.chars().any(is_japanese),
                "{}:{}: {line}",
                path.display(),
                number + 1
            );
        }
    }
}

fn rust_sources(dir: &Path) -> Vec<PathBuf> {
    files_under(dir)
        .into_iter()
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("rs"))
        .collect()
}

fn is_japanese(ch: char) -> bool {
    matches!(ch, '\u{3040}'..='\u{30ff}' | '\u{3400}'..='\u{9fff}')
}

#[test]
fn lane_arms_match_width() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/wasm32/simd/simd128");
    let mut seen = 0;
    for entry in fs::read_dir(&root).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            continue;
        }
        let text = fs::read_to_string(&path).unwrap();
        for (name, body) in lane_functions(&text) {
            seen += 1;
            let limit = assert_limit(name, body);
            let expected = expected_lanes(name);
            assert_eq!(limit, expected, "{name}");
            let mut arms = arm_indices(body);
            arms.sort();
            let want: Vec<u32> = (0..expected).collect();
            assert_eq!(arms, want, "{name}");
        }
    }
    assert_eq!(seen, 24);
}

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
fn wasm_pipeline() {
    if std::env::var_os("RUST_ISA_NESTED").is_some() {
        return;
    }
    check_link();
    check_without_simd128();
    check_wasip1_tests();
}

fn check_link() {
    let dir = nested_dir("link");
    let output = cargo(&dir)
        .args([
            "build",
            "-p",
            "simd128-link",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));

    let wasm = dir.join("wasm32-unknown-unknown/release/simd128_link.wasm");
    assert!(wasm.is_file(), "missing {}", wasm.display());
    let opt = dir.join("simd128-link.opt.wasm");
    let wat = dir.join("simd128-link.wat");
    let status = Command::new("wasm-opt")
        .args(["-O3"])
        .arg(&wasm)
        .arg("-o")
        .arg(&opt)
        .status()
        .expect("wasm-opt");
    assert!(status.success(), "wasm-opt failed");
    let status = Command::new("wasm2wat")
        .arg(&opt)
        .arg("-o")
        .arg(&wat)
        .status()
        .expect("wasm2wat");
    assert!(status.success(), "wasm2wat failed");
    check_wat(&fs::read_to_string(&wat).unwrap());

    assert_undefined_symbols(&newest_archive(&dir, "librust_isa_simd128.a"));
    assert_undefined_symbols(&newest_archive(&dir, "librust_isa_relaxed_simd.a"));
}

fn assert_undefined_symbols(archive: &Path) {
    let nm = Command::new("llvm-nm")
        .args(["-u"])
        .arg(archive)
        .output()
        .expect("llvm-nm");
    assert!(nm.status.success(), "{}", output_text(&nm));
    let undefined = String::from_utf8_lossy(&nm.stdout);
    for line in undefined.lines() {
        let mut parts = line.split_whitespace();
        if parts.next() == Some("U") {
            let symbol = parts.next().unwrap_or("");
            assert_eq!(
                symbol, "__stack_pointer",
                "unexpected undefined symbol {line}"
            );
        }
    }
}

fn check_without_simd128() {
    let dir = nested_dir("no-simd");
    let output = cargo(&dir)
        .env("RUSTFLAGS", "")
        .args([
            "build",
            "-p",
            "rust-isa",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));
    for path in files_under(&dir) {
        if path.extension().and_then(|ext| ext.to_str()) != Some("rlib") {
            continue;
        }
        let nm = Command::new("llvm-nm")
            .arg(&path)
            .output()
            .expect("llvm-nm");
        if !nm.status.success() {
            continue;
        }
        let text = String::from_utf8_lossy(&nm.stdout);
        assert!(
            !text.contains("rust_isa_simd128_") && !text.contains("rust_isa_relaxed_simd_"),
            "{} references a SIMD symbol:\n{text}",
            path.display()
        );
    }

    let output = cargo(&dir)
        .env("RUSTFLAGS", "")
        .args([
            "build",
            "-p",
            "simd128-link",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .output()
        .expect("cargo");
    let text = output_text(&output);
    assert!(
        !output.status.success(),
        "simd128-link built without simd128:\n{text}"
    );
    assert!(
        text.contains("simd128-link requires target_feature simd128 and relaxed-simd"),
        "{text}"
    );
}

fn check_wasip1_tests() {
    let dir = nested_dir("wasip1");
    let output = cargo(&dir)
        .env("RUST_ISA_NESTED", "1")
        .args(["test", "--target", "wasm32-wasip1"])
        .output()
        .expect("cargo");
    assert!(output.status.success(), "{}", output_text(&output));
}

fn cargo(target_dir: &Path) -> Command {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut command = Command::new(cargo);
    command
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("CARGO_TARGET_DIR", target_dir)
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("RUSTFLAGS");
    command
}

fn nested_dir(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target-nested")
        .join(name)
}

fn output_text(output: &Output) -> String {
    format!(
        "status: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn rust_symbols_in(text: &str, prefix: &str) -> BTreeSet<String> {
    let mut symbols = BTreeSet::new();
    let needle = format!("fn {prefix}");
    let mut rest = text;
    while let Some(index) = rest.find(needle.as_str()) {
        let start = index + "fn ".len();
        let tail = &rest[start..];
        let end = tail.find('(').expect("rust symbol");
        symbols.insert(tail[..end].to_string());
        rest = &tail[end..];
    }
    symbols
}

fn c_symbols_in(text: &str, prefix: &str) -> BTreeSet<String> {
    let mut symbols = BTreeSet::new();
    for line in text.lines() {
        let line = line.trim();
        let Some(start) = line.find(prefix) else {
            continue;
        };
        let prefix = line[..start].trim();
        if !matches!(
            prefix,
            "void" | "int32_t" | "int64_t" | "uint32_t" | "uint64_t" | "float" | "double"
        ) {
            continue;
        }
        let tail = &line[start..];
        let end = tail.find('(').expect("c symbol");
        symbols.insert(tail[..end].to_string());
    }
    symbols
}

fn lane_functions(text: &str) -> Vec<(&str, &str)> {
    let mut found = Vec::new();
    let marker = "<const LANE: u32>";
    let mut rest = text;
    while let Some(index) = rest.find(marker) {
        let head = &rest[..index];
        let fn_at = head.rfind("fn ").expect("lane fn");
        let name = head[fn_at + 3..].split_whitespace().last().expect("name");
        let body_and_after = &rest[index + marker.len()..];
        let body_end = ["\npub fn ", "\npub unsafe fn ", "\nunsafe extern "]
            .into_iter()
            .filter_map(|needle| body_and_after.find(needle))
            .min()
            .unwrap_or(body_and_after.len());
        found.push((name, &body_and_after[..body_end]));
        rest = &body_and_after[body_end..];
    }
    found
}

fn expected_lanes(name: &str) -> u32 {
    const RULES: &[(&str, u32)] = &[
        ("i8x16", 16),
        ("load8_", 16),
        ("store8_", 16),
        ("i16x8", 8),
        ("load16_", 8),
        ("store16_", 8),
        ("i32x4", 4),
        ("f32x4", 4),
        ("load32_", 4),
        ("store32_", 4),
        ("i64x2", 2),
        ("f64x2", 2),
        ("load64_", 2),
        ("store64_", 2),
    ];
    RULES
        .iter()
        .find(|(needle, _)| name.contains(needle))
        .map(|(_, lanes)| *lanes)
        .unwrap_or_else(|| panic!("no lane width for {name}"))
}

fn assert_limit(name: &str, body: &str) -> u32 {
    let marker = "assert!(LANE < ";
    let at = body
        .find(marker)
        .unwrap_or_else(|| panic!("{name} has no lane assert"));
    let after = &body[at + marker.len()..];
    let end = after.find(')').expect("assert end");
    after[..end]
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("{name} lane assert is not an integer"))
}

fn arm_indices(body: &str) -> Vec<u32> {
    let mut indices = Vec::new();
    for line in body.lines() {
        let trimmed = line.trim();
        let Some(arrow) = trimmed.find("=>") else {
            continue;
        };
        let head = trimmed[..arrow].trim();
        if let Ok(index) = head.parse() {
            indices.push(index);
        }
    }
    indices
}

fn check_wat(wat: &str) {
    let one = function_body(wat, "kernel_one");
    reject_call("kernel_one", one);
    assert_eq!(one.matches("f32x4.add").count(), 1, "kernel_one");

    let two = function_body(wat, "kernel_two");
    reject_call("kernel_two", two);
    let parts: Vec<&str> = two.split("f32x4.add").collect();
    assert_eq!(parts.len(), 3, "kernel_two add count");
    assert!(
        !contains_opcode(parts[1], "v128.store"),
        "intermediate result store remains:\n{}",
        parts[1]
    );

    let relaxed = function_body(wat, "kernel_relaxed");
    reject_call("kernel_relaxed", relaxed);
    let parts: Vec<&str> = relaxed.split("f32x4.relaxed_madd").collect();
    assert_eq!(parts.len(), 3, "kernel_relaxed madd count");
    assert!(
        !contains_opcode(parts[1], "v128.store"),
        "intermediate result store remains:\n{}",
        parts[1]
    );
}

fn reject_call(name: &str, body: &str) {
    for word in body.split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_') {
        assert!(
            word != "call" && word != "call_indirect",
            "{name} still contains call"
        );
    }
    assert!(!body.contains("rust_isa_"), "{name} still names a C symbol");
}

fn contains_opcode(body: &str, opcode: &str) -> bool {
    let mut rest = body;
    while let Some(index) = rest.find(opcode) {
        let after = index + opcode.len();
        let boundary = after == rest.len()
            || (!rest.as_bytes()[after].is_ascii_alphanumeric() && rest.as_bytes()[after] != b'_');
        if boundary {
            return true;
        }
        rest = &rest[after..];
    }
    false
}

fn function_body<'a>(wat: &'a str, name: &str) -> &'a str {
    let export_key = format!("(export \"{name}\" (func ");
    let export_at = wat
        .find(&export_key)
        .unwrap_or_else(|| panic!("missing export {name}"));
    let after = export_at + export_key.len();
    let end = wat[after..].find(')').expect("export func");
    let target = &wat[after..after + end];
    if let Some(func_name) = target.strip_prefix('$') {
        let marker = format!("(func ${func_name}");
        let start = wat
            .find(&marker)
            .unwrap_or_else(|| panic!("missing func {marker}"));
        return body_at(wat, start);
    }
    let index: usize = target
        .parse()
        .unwrap_or_else(|_| panic!("bad func index {target}"));
    let mut starts = Vec::new();
    let mut search = 0;
    while let Some(rel) = wat[search..].find("(func ") {
        let at = search + rel;
        if wat[at..].starts_with("(func $") || wat[at..].starts_with("(func (;") {
            starts.push(at);
        }
        search = at + "(func ".len();
    }
    let start = *starts
        .get(index)
        .unwrap_or_else(|| panic!("missing func index {index}"));
    body_at(wat, start)
}

fn body_at(wat: &str, start: usize) -> &str {
    let mut depth = 0;
    for (index, ch) in wat[start..].char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return &wat[start..start + index + ch.len_utf8()];
                }
            }
            _ => {}
        }
    }
    panic!("unbalanced func");
}

fn newest_archive(dir: &Path, name: &str) -> PathBuf {
    files_under(dir)
        .into_iter()
        .filter(|path| path.file_name().and_then(|file| file.to_str()) == Some(name))
        .max_by_key(|path| {
            fs::metadata(path)
                .and_then(|meta| meta.modified())
                .unwrap_or(SystemTime::UNIX_EPOCH)
        })
        .unwrap_or_else(|| panic!("missing {name}"))
}

fn files_under(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        let Ok(read) = fs::read_dir(&current) else {
            continue;
        };
        for entry in read.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                files.push(path);
            }
        }
    }
    files
}
