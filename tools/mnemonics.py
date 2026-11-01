#!/usr/bin/env python3
"""Check that documented instruction mnemonics are accepted by llvm-mc.

The check reads the first code span of each rustdoc block, normalizes it to a
mnemonic, and probes the target assembler without operands. llvm-mc reports
"too few operands" for a known mnemonic and "invalid instruction" for an
unknown one.

Names that are intentionally not single mnemonics are listed in
tools/mnemonics.toml under [allow.<arch>].
"""

import argparse
import re
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CONFIG_PATH = ROOT / "tools" / "mnemonics.toml"
SOURCE = ROOT / "src"


def load_config():
    with CONFIG_PATH.open("rb") as handle:
        return tomllib.load(handle)


def normalize(raw, arch):
    text = raw.strip()
    if any(ch in text for ch in " +-*=/="):
        return None
    if text.startswith(("v128.", "v256.", "v512.")):
        return None
    text = re.sub(r"::<[^>]*>", "", text)
    text = re.sub(r"<[^>]*>", "", text)
    while re.search(r"_v(128|256|512)$", text):
        text = re.sub(r"_v(128|256|512)$", "", text)
    if arch in {"aarch64", "arm"}:
        while re.search(r"_[iusf](8|16|32|64)$", text) or text.endswith("_bf16"):
            text = re.sub(r"_[iusf](8|16|32|64)$", "", text)
            text = re.sub(r"_bf16$", "", text)
    if ".." in text:
        text = text.split("..")[0]
    if not re.fullmatch(r"[a-z][a-z0-9._]*", text):
        return None
    return text


OPEN_DELIMITERS = {"(": ")", "[": "]", "{": "}"}
MACRO_INVOCATION = re.compile(r"\b([A-Za-z_][A-Za-z0-9_]*)!\s*([\(\[\{])")


def skip_literal(text, index):
    """Skip a string, character literal, or comment starting at ``index``."""
    if text.startswith("//", index):
        end = text.find("\n", index)
        return len(text) if end < 0 else end
    if text.startswith("/*", index):
        depth = 1
        index += 2
        while index < len(text) and depth:
            if text.startswith("/*", index):
                depth += 1
                index += 2
            elif text.startswith("*/", index):
                depth -= 1
                index += 2
            else:
                index += 1
        return index
    if text[index] in ('"', "'"):
        quote = text[index]
        index += 1
        while index < len(text):
            if text[index] == "\\":
                index += 2
            elif text[index] == quote:
                return index + 1
            else:
                index += 1
        return index
    return index


def macro_line_ranges(text):
    """Return line ranges covered by macro invocations (not macro_rules)."""
    ranges = []
    for match in MACRO_INVOCATION.finditer(text):
        if match.group(1) == "macro_rules":
            continue
        line_start = text.rfind("\n", 0, match.start()) + 1
        if line_start != match.start():
            # Nested macros such as asm! and assert_eq! are implementation
            # details, not wrapper generators. Generated wrappers are invoked
            # at the start of a line.
            continue
        stack = [OPEN_DELIMITERS[match.group(2)]]
        index = match.start(2) + 1
        while index < len(text) and stack:
            if text.startswith("//", index):
                newline = text.find("\n", index)
                index = len(text) if newline < 0 else newline + 1
                continue
            if text.startswith("/*", index):
                close = text.find("*/", index + 2)
                index = len(text) if close < 0 else close + 2
                continue
            if text[index] in ('"', "'"):
                index = skip_literal(text, index)
                continue
            char = text[index]
            if char in OPEN_DELIMITERS:
                stack.append(OPEN_DELIMITERS[char])
            elif stack and char == stack[-1]:
                stack.pop()
                if not stack:
                    start_line = text.count("\n", 0, match.start())
                    end_line = text.count("\n", 0, index)
                    ranges.append((start_line, end_line))
            index += 1
    return ranges


def candidates(arch):
    found = {}
    for path in (SOURCE / arch).rglob("*.rs"):
        text = path.read_text()
        lines = text.splitlines()
        macro_lines = set()
        for start, end in macro_line_ranges(text):
            macro_lines.update(range(start, end + 1))
        i = 0
        while i < len(lines):
            if not lines[i].lstrip().startswith("///"):
                i += 1
                continue
            start = i
            while i < len(lines) and lines[i].lstrip().startswith("///"):
                i += 1
            # Keep the historical column-0 docs and add docs inside macro
            # invocations. The first line is the mnemonic; later lines are prose.
            if not lines[start].startswith("///") and start not in macro_lines:
                continue
            match = re.match(r"^/// `([^`]+)`", lines[start].lstrip())
            if match:
                token = normalize(match.group(1), arch)
                if token:
                    found.setdefault(token, (str(path.relative_to(ROOT)), start + 1))
    return found


def target_for(arch, path, target):
    parts = Path(path).parts
    overrides = target.get("overrides", {})
    if len(parts) >= 5:
        family, extension = parts[2], parts[3]
        if extension in overrides:
            return overrides[extension]
        if family in overrides:
            return overrides[family]
    return target


def probe(target, tokens):
    command = ["llvm-mc", "-triple=" + target["triple"], "-mattr=" + target["features"]]
    process = subprocess.run(
        command, input="\n".join(tokens) + "\n", text=True, capture_output=True
    )
    if process.returncode != 0 and not (
        "invalid instruction" in process.stderr or "too few operands" in process.stderr
    ):
        print("llvm-mc failed: " + " ".join(command), file=sys.stderr)
        print(process.stderr, file=sys.stderr)
        return None

    errors = {}
    for line in process.stderr.splitlines():
        match = re.match(r"<stdin>:(\d+):\d+: (?:error|warning): (.*)", line)
        if match:
            errors.setdefault(int(match.group(1)) - 1, []).append(match.group(2))

    result = {}
    for index, token in enumerate(tokens):
        messages = errors.get(index, [])
        if not messages:
            result[token] = "ok"
            continue
        joined = " ".join(messages)
        if "too few operands" in joined or "any one of the following" in joined:
            result[token] = "ok"
        elif (
            "invalid instruction" in joined
            or "unrecognized instruction mnemonic" in joined
            or "invalid instruction mnemonic" in joined
        ):
            result[token] = "invalid"
        else:
            result[token] = "ok"
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--arch", action="append", help="arch to check (default: all configured)")
    parser.add_argument("--verbose", action="store_true", help="print allow and baseline entries")
    parser.add_argument("--no-baseline", action="store_true", help="fail on baseline entries too")
    args = parser.parse_args()

    config = load_config()
    targets = config.get("targets", {})
    allow = config.get("allow", {})
    baseline = config.get("baseline", {})
    archs = args.arch or sorted(targets)

    failed = False
    baseline_count = 0
    for arch in archs:
        if arch not in targets:
            print("no target configured for " + arch, file=sys.stderr)
            return 2

        found = candidates(arch)
        groups = {}
        for token, (path, line) in found.items():
            target = target_for(arch, path, targets[arch])
            key = (target["triple"], target["features"])
            groups.setdefault(key, []).append((token, path, line))

        for (triple, features), entries in sorted(groups.items()):
            tokens = [entry[0] for entry in entries]
            result = probe({"triple": triple, "features": features}, tokens)
            if result is None:
                return 2
            for token, path, line in entries:
                verdict = result[token]
                if verdict == "invalid":
                    reason = allow.get(arch, {}).get(token)
                    if reason:
                        if args.verbose:
                            print("allow", arch, token, "(" + reason + ")")
                    elif token in baseline.get(arch, {}) and not args.no_baseline:
                        baseline_count += 1
                        if args.verbose:
                            print("baseline", arch, token, "(" + baseline[arch][token] + ")")
                    else:
                        print("INVALID " + arch + " " + token + " " + path + ":" + str(line))
                        failed = True
                elif verdict == "review":
                    print("REVIEW " + arch + " " + token + " " + path + ":" + str(line))
                    failed = True

    if failed:
        return 1
    print("mnemonics: ok (" + str(baseline_count) + " baseline)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
