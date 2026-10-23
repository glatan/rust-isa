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


def normalize(raw):
    text = raw.strip()
    if any(ch in text for ch in " +-*=/="):
        return None
    if text.startswith(("v128.", "v256.", "v512.")):
        return None
    text = re.sub(r"::<[^>]*>", "", text)
    text = re.sub(r"<[^>]*>", "", text)
    while re.search(r"_v(128|256|512)$", text):
        text = re.sub(r"_v(128|256|512)$", "", text)
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
                token = normalize(match.group(1))
                if token:
                    found.setdefault(token, (str(path.relative_to(ROOT)), i + 1))
            while i < len(lines) and lines[i].startswith("///"):
                i += 1
    return found


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
        elif "invalid instruction" in joined:
            result[token] = "invalid"
        else:
            result[token] = "review"
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--arch", action="append", help="arch to check (default: all configured)")
    args = parser.parse_args()

    config = load_config()
    targets = config.get("targets", {})
    allow = config.get("allow", {})
    archs = args.arch or sorted(targets)

    failed = False
    for arch in archs:
        if arch not in targets:
            print("no target configured for " + arch, file=sys.stderr)
            return 2

        found = candidates(arch)
        tokens = sorted(found)
        result = probe(targets[arch], tokens)
        if result is None:
            return 2

        for token in tokens:
            verdict = result[token]
            path, line = found[token]
            if verdict == "invalid":
                reason = allow.get(arch, {}).get(token)
                if reason:
                    print("allow", arch, token, "(" + reason + ")")
                else:
                    print("INVALID " + arch + " " + token + " " + path + ":" + str(line))
                    failed = True
            elif verdict == "review":
                print("REVIEW " + arch + " " + token + " " + path + ":" + str(line))
                failed = True

    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
