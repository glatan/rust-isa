#![allow(dead_code)]

pub use std::collections::BTreeSet;
pub use std::fs;
pub use std::path::{Path, PathBuf};
pub use std::process::{Command, Output};
pub use std::time::SystemTime;

pub fn check_x86_symbols(module: &str, c_file: &str, prefix: &str) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/x86")
        .join(module);
    check_symbols(&root, c_file, prefix);
}

pub fn check_symbols(root: &Path, c_file: &str, prefix: &str) {
    let mut rust_symbols = BTreeSet::new();
    for path in rust_sources(root) {
        rust_symbols.extend(rust_symbols_in(&fs::read_to_string(&path).unwrap(), prefix));
    }
    let c_symbols = c_symbols_in(&fs::read_to_string(root.join(c_file)).unwrap(), prefix);
    assert_eq!(rust_symbols, c_symbols, "{}", root.display());
    assert!(!rust_symbols.is_empty(), "{}", root.display());
}

pub fn rust_sources(dir: &Path) -> Vec<PathBuf> {
    files_under(dir)
        .into_iter()
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("rs"))
        .collect()
}

pub fn is_japanese(ch: char) -> bool {
    matches!(ch, '\u{3040}'..='\u{30ff}' | '\u{3400}'..='\u{9fff}')
}

pub fn function_at<'a>(text: &'a str, symbol: &str) -> &'a str {
    let start = text
        .find(symbol)
        .unwrap_or_else(|| panic!("missing {symbol}"));
    let body = &text[start..];
    &body[..body.find("\n\n").unwrap_or(body.len())]
}

pub fn cargo(target_dir: &Path) -> Command {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut command = Command::new(cargo);
    command
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("CARGO_TARGET_DIR", target_dir)
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("RUSTFLAGS");
    command
}

pub fn nested_dir(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target-nested")
        .join(name)
}

pub fn output_text(output: &Output) -> String {
    format!(
        "status: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

pub fn rust_symbols_in(text: &str, prefix: &str) -> BTreeSet<String> {
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

pub fn c_symbols_in(text: &str, prefix: &str) -> BTreeSet<String> {
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

pub fn lane_functions(text: &str) -> Vec<(&str, &str)> {
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

pub fn expected_lanes(name: &str) -> u32 {
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

pub fn assert_limit(name: &str, body: &str) -> u32 {
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

pub fn arm_indices(body: &str) -> Vec<u32> {
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

pub fn check_wat(wat: &str) {
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

pub fn reject_call(name: &str, body: &str) {
    for word in body.split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_') {
        assert!(
            word != "call" && word != "call_indirect",
            "{name} still contains call"
        );
    }
    assert!(!body.contains("rust_isa_"), "{name} still names a C symbol");
}

pub fn contains_opcode(body: &str, opcode: &str) -> bool {
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

pub fn function_body<'a>(wat: &'a str, name: &str) -> &'a str {
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

pub fn body_at(wat: &str, start: usize) -> &str {
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

pub fn newest_archive(dir: &Path, name: &str) -> PathBuf {
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

pub fn files_under(dir: &Path) -> Vec<PathBuf> {
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
