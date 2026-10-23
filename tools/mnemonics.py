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


def candidates(arch):
    found = {}
    for path in (SOURCE / arch).rglob("*.rs"):
        lines = path.read_text().splitlines()
        i = 0
        while i < len(lines):
            if not lines[i].startswith("///"):
                i += 1
                continue
            match = re.match(r"^/// `([^`]+)`", lines[i])
            if match:
                token = normalize(match.group(1), arch)
                if token:
                    found.setdefault(token, (str(path.relative_to(ROOT)), i + 1))
            while i < len(lines) and lines[i].startswith("///"):
                i += 1
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
