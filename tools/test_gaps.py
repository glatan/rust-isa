#!/usr/bin/env python3
"""Report public functions that are not referenced by value tests.

The check is call based. A function is counted as covered only when its name
appears as a call expression in the arch's test sources or value fixture;
imports and bare mentions do not count. This catches functions that are never called by any test,
without pretending to prove semantic coverage.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

PUBLIC_FN = re.compile(r"pub\s+(?:unsafe\s+)?fn\s+([A-Za-z0-9_]+)")
CALL = re.compile(r"\b([A-Za-z_][A-Za-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(")

ARCHES = {
    "x86": (
        "src/x86",
        ["tests/x86"],
    ),
    "aarch64": (
        "src/aarch64",
        ["fixtures/aarch64-values/src/main.rs"],
    ),
    "arm": (
        "src/arm",
        ["fixtures/arm-values/src/main.rs"],
    ),
    "riscv": (
        "src/riscv",
        ["fixtures/riscv-values/src/main.rs"],
    ),
    "loongarch": (
        "src/loongarch",
        ["fixtures/loongarch-values/src/main.rs"],
    ),
    "s390x": (
        "src/s390x",
        ["fixtures/s390x-values/src/main.rs"],
    ),
    "powerpc": (
        "src/powerpc",
        ["fixtures/powerpc-values/src/main.rs"],
    ),
    "mips": (
        "src/mips",
        ["fixtures/mips-values/src/main.rs"],
    ),
    "wasm32": (
        "src/wasm32",
        ["tests/wasm32"],
    ),
}


def source_files(path: Path) -> list[Path]:
    if path.is_file():
        return [path]
    return sorted(path.rglob("*.rs"))


def public_functions(root: Path, source: str) -> set[str]:
    names: set[str] = set()
    for path in source_files(root / source):
        names.update(PUBLIC_FN.findall(path.read_text(encoding="utf-8")))
    return {name for name in names if not name.startswith("rust_isa_")}


ALIAS = re.compile(r"([A-Za-z_][A-Za-z0-9_]*)\s+as\s+([A-Za-z_][A-Za-z0-9_]*)")


def used_names(root: Path, tests: list[str]) -> set[str]:
    names: set[str] = set()
    for entry in tests:
        for path in source_files(root / entry):
            text = path.read_text(encoding="utf-8")
            aliases = {alias: original for original, alias in ALIAS.findall(text)}
            for called in CALL.findall(text):
                names.add(aliases.get(called, called))
    return names


def report(root: Path, arch: str) -> dict[str, object]:
    source, tests = ARCHES[arch]
    functions = public_functions(root, source)
    used = used_names(root, tests)
    missing = sorted(functions - used)
    return {
        "arch": arch,
        "public": len(functions),
        "missing": missing,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--root",
        type=Path,
        default=Path(__file__).resolve().parent.parent,
        help="repository root (default: parent of tools/)",
    )
    parser.add_argument("--arch", choices=sorted(ARCHES), help="only report one arch")
    parser.add_argument("--json", action="store_true", help="print JSON")
    parser.add_argument(
        "--names",
        action="store_true",
        help="print missing function names, not only counts",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="exit nonzero when any public function is unreferenced",
    )
    args = parser.parse_args()

    arches = [args.arch] if args.arch else sorted(ARCHES)
    reports = [report(args.root, arch) for arch in arches]
    if args.json:
        print(json.dumps(reports, indent=2))
        return

    failed = False
    for entry in reports:
        missing = entry["missing"]
        assert isinstance(missing, list)
        print(f"{entry['arch']}: {len(missing)} missing / {entry['public']} public")
        if missing:
            failed = True
        if args.names or args.check:
            for name in missing:
                print(f"  {name}")
    if args.check and failed:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
