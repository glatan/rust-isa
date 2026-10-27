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
    let c_path = root.join(c_file);
    assert!(c_path.is_file(), "missing C source {}", c_path.display());
    let mut rust_symbols = BTreeSet::new();
    for path in rust_sources(root) {
        rust_symbols.extend(rust_symbols_in(&fs::read_to_string(&path).unwrap(), prefix));
    }
    let c_symbols = c_symbols_in(&fs::read_to_string(c_path).unwrap(), prefix);
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
    matches!(
        ch,
        '\u{3000}'..='\u{303f}' | '\u{3040}'..='\u{30ff}' | '\u{3400}'..='\u{9fff}' | '\u{ff00}'..='\u{ffef}'
    )
}

pub fn function_at<'a>(text: &'a str, symbol: &str) -> &'a str {
    let name = symbol.trim_end_matches(':');
    let header = if name.starts_with('<') && name.ends_with('>') {
        format!("{name}:")
    } else {
        format!("<{name}>:")
    };
    let mut start = None;
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        if line.trim_end().ends_with(&header) {
            start = Some(offset);
            break;
        }
        offset += line.len();
    }
    let start = start.unwrap_or_else(|| panic!("missing {symbol}"));

    let body_start = start
        + text[start..]
            .find('\n')
            .map_or(text.len() - start, |end| end + 1);
    let mut end = text.len();
    let mut offset = body_start;
    for line in text[body_start..].split_inclusive('\n') {
        if is_function_header(line) {
            end = offset;
            break;
        }
        offset += line.len();
    }
    &text[start..end]
}

fn is_function_header(line: &str) -> bool {
    let mut parts = line.splitn(2, char::is_whitespace);
    let Some(address) = parts.next() else {
        return false;
    };
    let Some(rest) = parts.next() else {
        return false;
    };
    !address.is_empty()
        && address.chars().all(|ch| ch.is_ascii_hexdigit())
        && rest.trim_start().starts_with('<')
        && rest.trim_end().ends_with(">:")
}

/// Check whether a disassembly line contains `opcode` as a standalone token.
///
/// This avoids matching `fadd` when looking for `add`, or `xvadd.w` when
/// looking for `vadd.w`.
pub fn line_has_opcode(line: &str, opcode: &str) -> bool {
    line.split_whitespace().any(|word| word == opcode)
}

/// Check whether any line of `body` contains `opcode` as a standalone token.
pub fn body_has_opcode(body: &str, opcode: &str) -> bool {
    body.lines().any(|line| line_has_opcode(line, opcode))
}

/// Return the line indices that contain `opcode` as a standalone token.
pub fn opcode_positions(body: &str, opcode: &str) -> Vec<usize> {
    body.lines()
        .enumerate()
        .filter(|(_, line)| line_has_opcode(line, opcode))
        .map(|(index, _)| index)
        .collect()
}

/// Check an x86 mnemonic, accepting the VEX/EVEX form as well as the legacy
/// form. For example, `pshufb` also matches `vpshufb`.
pub fn x86_line_has_opcode(line: &str, opcode: &str) -> bool {
    line.split_whitespace()
        .any(|word| x86_word_has_opcode(word.trim_end_matches([',', ';']), opcode))
}

fn x86_word_has_opcode(word: &str, opcode: &str) -> bool {
    let legacy = word.strip_prefix('v').unwrap_or(word);
    [word, legacy].into_iter().any(|candidate| {
        candidate.strip_prefix(opcode).is_some_and(|rest| {
            rest.is_empty()
                || rest
                    .chars()
                    .next()
                    .is_some_and(|ch| !ch.is_ascii_alphanumeric() && ch != '_')
        })
    })
}

pub fn x86_opcode_positions(body: &str, opcode: &str) -> Vec<usize> {
    body.lines()
        .enumerate()
        .filter(|(_, line)| x86_line_has_opcode(line, opcode))
        .map(|(index, _)| index)
        .collect()
}

pub fn body_has_call(body: &str, mnemonics: &[&str]) -> bool {
    body.lines().any(|line| {
        line.split_whitespace()
            .any(|word| mnemonics.iter().any(|mnemonic| word == *mnemonic))
    })
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
    let root = std::env::var_os("RUST_ISA_NESTED")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target-nested"));
    root.join(name)
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
    let filtered = text
        .lines()
        .filter(|line| {
            let line = line.trim_start();
            !line.starts_with("//") && !line.starts_with("/*") && !line.starts_with('*')
        })
        .collect::<Vec<_>>()
        .join("\n");
    let mut rest = filtered.as_str();
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
        if line.starts_with("//") || line.starts_with("/*") || line.starts_with('*') {
            continue;
        }
        let Some(start) = line.find(prefix) else {
            continue;
        };
        let prefix = line[..start].trim();
        if !matches!(
            prefix,
            "void"
                | "bool"
                | "int"
                | "size_t"
                | "int8_t"
                | "int16_t"
                | "int32_t"
                | "int64_t"
                | "uint8_t"
                | "uint16_t"
                | "uint32_t"
                | "uint64_t"
                | "float"
                | "double"
        ) {
            continue;
        }
        let tail = &line[start..];
        let end = tail.find('(').expect("c symbol");
        let name = &tail[..end];
        if name.contains('#') {
            continue;
        }
        symbols.insert(name.to_string());
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
    check_wat_function(wat, "kernel_one", "f32x4.add", 1);
    check_wat_function(wat, "kernel_two", "f32x4.add", 2);
    check_wat_function(wat, "kernel_relaxed", "f32x4.relaxed_madd", 2);
}

fn check_wat_function(wat: &str, name: &str, arithmetic: &str, expected: usize) {
    let body = function_body(wat, name);
    reject_call(name, body);
    let lines: Vec<_> = body.lines().collect();
    let operations: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| contains_opcode(line, arithmetic))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(
        operations.len(),
        expected,
        "{name} {arithmetic} count:\n{body}"
    );
    let last = *operations.last().unwrap();

    let stores: Vec<_> = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| contains_opcode(line, "v128.store"))
        .map(|(index, _)| index)
        .collect();
    assert!(
        stores.len() <= 1,
        "{name} has more than one v128.store:\n{body}"
    );
    if let Some(&store) = stores.first() {
        assert!(store > last, "{name} stores an intermediate value:\n{body}");
    }

    for (index, line) in lines.iter().enumerate() {
        if contains_opcode(line, "v128.load") {
            let previous = lines[..index]
                .iter()
                .rev()
                .find(|line| !line.trim().is_empty());
            assert!(
                previous.is_none_or(|line| !contains_opcode(line, "v128.store")),
                "{name} reloads a stored intermediate value:\n{body}"
            );
        }
    }
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
