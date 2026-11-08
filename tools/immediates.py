#!/usr/bin/env python3
"""Report immediate-taking public functions without a compile_fail doctest.

An immediate-taking function is a public function whose signature carries a
const generic parameter (for example ``const IMM: u8`` or ``const LANE: usize``).
The check is name based within each arch: a function counts as covered when its
name appears as a call expression inside a `````compile_fail`` doctest in that
arch's ``src/`` tree. Functions without a const assert (full-range immediates,
boolean switches) cannot fail to compile and are reported as uncoverable
rather than missing. This catches missing range-rejection tests without
pretending to prove that every invalid value is rejected.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

PUBLIC_FN_WITH_CONST = re.compile(
    r"pub\s+(?:unsafe\s+)?fn\s+([A-Za-z0-9_]+)\s*(?:::)?\s*<([^;{]*?)>"
)
CONST_PARAM = re.compile(r"\bconst\s+([A-Z_][A-Z0-9_]*)\s*:")
CALL = re.compile(r"\b([A-Za-z_][A-Za-z0-9_]*)\s*(?:::<[^>]*>)?\s*\(")
FENCE = re.compile(r"^```(\w*).*$")
ITEM_END = re.compile(r"\n(?:pub\s|macro_rules!|unsafe extern)")

ARCHES = [
    "x86",
    "aarch64",
    "arm",
    "riscv",
    "loongarch",
    "s390x",
    "powerpc",
    "mips",
    "wasm32",
]


def source_files(path: Path) -> list[Path]:
    if path.is_file():
        return [path]
    return sorted(path.rglob("*.rs"))


def immediate_functions(root: Path, arch: str) -> dict[str, dict[str, object]]:
    """Map function name to const params and whether a range is asserted.

    A function is rejectable only when its body carries a ``const`` assert on
    the const generic (for example ``const { assert!(IMM <= 15, ...) };``).
    Full-range immediates (any ``u8`` is encodable) and boolean switches have
    no invalid value and are reported as uncoverable instead of missing.
    """
    found: dict[str, dict[str, object]] = {}
    for path in source_files(root / "src" / arch):
        text = path.read_text(encoding="utf-8")
        for match in PUBLIC_FN_WITH_CONST.finditer(text):
            params = sorted(set(CONST_PARAM.findall(match.group(2))))
            if not params:
                continue
            start = match.start()
            following = text[start + 10 :]
            end = ITEM_END.search(following)
            span = following[: end.start()] if end else following[:3000]
            asserted = "const {" in span and "assert!" in span
            entry = found.setdefault(
                match.group(1), {"params": set(), "asserted": False}
            )
            params_set = entry["params"]
            assert isinstance(params_set, set)
            params_set.update(params)
            entry["asserted"] = bool(entry["asserted"]) or asserted
    return {
        name: {"params": sorted(entry["params"]), "asserted": entry["asserted"]}  # type: ignore[arg-type]
        for name, entry in found.items()
    }


def compile_fail_names(root: Path, arch: str) -> set[str]:
    """Collect call names appearing inside compile_fail doctests."""
    names: set[str] = set()
    for path in source_files(root / "src" / arch):
        in_block = False
        for line in path.read_text(encoding="utf-8").splitlines():
            code = re.sub(r"^\s*(?:///|//!|#)?\s*", "", line)
            fence = FENCE.match(code)
            if fence is not None:
                in_block = fence.group(1).startswith("compile_fail")
                continue
            if in_block:
                names.update(CALL.findall(line))
    return names


def report(root: Path, arch: str) -> dict[str, object]:
    functions = immediate_functions(root, arch)
    covered = compile_fail_names(root, arch)
    missing = sorted(
        name
        for name, info in functions.items()
        if name not in covered and info["asserted"]
    )
    uncoverable = sorted(
        name
        for name, info in functions.items()
        if name not in covered and not info["asserted"]
    )
    return {
        "arch": arch,
        "immediate": len(functions),
        "missing": missing,
        "uncoverable": uncoverable,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--root",
        type=Path,
        default=Path(__file__).resolve().parent.parent,
        help="repository root (default: parent of tools/)",
    )
    parser.add_argument("--arch", choices=ARCHES, help="only report one arch")
    parser.add_argument("--json", action="store_true", help="print JSON")
    parser.add_argument(
        "--names",
        action="store_true",
        help="print missing function names, not only counts",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="exit nonzero when any rejectable immediate function lacks a compile_fail doctest",
    )
    args = parser.parse_args()

    arches = [args.arch] if args.arch else ARCHES
    reports = [report(args.root, arch) for arch in arches]
    if args.json:
        print(json.dumps(reports, indent=2))
        return

    failed = False
    for entry in reports:
        missing = entry["missing"]
        assert isinstance(missing, list)
        uncoverable = entry["uncoverable"]
        assert isinstance(uncoverable, list)
        print(
            f"{entry['arch']}: {len(missing)} missing / "
            f"{entry['immediate']} immediate ({len(uncoverable)} uncoverable)"
        )
        if missing:
            failed = True
        if args.names or args.check:
            for name in missing:
                print(f"  {name}")
            for name in uncoverable:
                print(f"  uncoverable: {name}")
    if args.check and failed:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
