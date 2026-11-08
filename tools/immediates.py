#!/usr/bin/env python3
"""Report immediate-taking public functions without a compile_fail doctest.

An immediate-taking function is a public function whose signature carries a
const generic parameter (for example ``const IMM: u8`` or ``const LANE: usize``).
The check is name based within each arch: a function counts as covered when its
name appears as a call expression inside a `````compile_fail`` doctest in that
arch's ``src/`` tree. Functions whose const asserts cannot fail (a bound
covering the whole type, such as ``IMM <= 255`` on a ``u8``) are reported
as uncoverable with a vacuous bound; functions without any const assert
are reported as uncoverable without one. Only rejectable functions fail
``--check``. This catches missing range-rejection tests without pretending
that every invalid value is rejected.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

PUBLIC_FN_WITH_CONST = re.compile(
    r"pub\s+(?:unsafe\s+)?fn\s+([A-Za-z0-9_]+)\s*(?:::)?\s*<([^;{]*?)>"
)
CONST_PARAM = re.compile(r"\bconst\s+([A-Z_][A-Z0-9_]*)\s*:\s*([A-Za-z0-9_]+)")
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


TYPE_MAX = {
    "u8": 255,
    "u16": 65535,
    "u32": 4294967295,
    "u64": 18446744073709551615,
    "usize": 18446744073709551615,
    "i8": 127,
    "i16": 32767,
    "i32": 2147483647,
    "i64": 9223372036854775807,
    "isize": 9223372036854775807,
}

TYPE_MIN = {
    "u8": 0,
    "u16": 0,
    "u32": 0,
    "u64": 0,
    "usize": 0,
    "i8": -128,
    "i16": -32768,
    "i32": -2147483648,
    "i64": -9223372036854775808,
    "isize": -9223372036854775808,
}

COMPARE = re.compile(r"([A-Za-z_][A-Za-z0-9_]*)\s*(<=|>=|<|>)\s*(-?\d+)")


def assert_is_vacuous(condition: str, param: str, types: dict[str, str]) -> bool:
    """Check whether a const assert can never fail for the parameter type.

    Asserts such as ``IMM <= 255`` on a ``u8`` accept every value, so no
    out-of-range input exists to reject with ``compile_fail``.
    """
    maximum = TYPE_MAX.get(types.get(param, ""), None)
    minimum = TYPE_MIN.get(types.get(param, ""), None)
    if maximum is None or minimum is None:
        return False
    parts = [part.strip(" ()") for part in condition.split("&&")]
    if not parts:
        return False
    for part in parts:
        match = COMPARE.fullmatch(part.strip())
        if match is None or match.group(1) != param:
            return False
        op, value = match.group(2), int(match.group(3))
        if op == "<=" and value < maximum:
            return False
        if op == "<" and value <= maximum:
            return False
        if op == ">=" and value > minimum:
            return False
        if op == ">" and value >= minimum:
            return False
    return True


def source_files(path: Path) -> list[Path]:
    if path.is_file():
        return [path]
    return sorted(path.rglob("*.rs"))


def assert_conditions(span: str) -> list[str]:
    """Extract the condition of every ``assert!`` in the span."""
    conditions: list[str] = []
    for match in re.finditer(r"assert!\s*\(", span):
        depth = 0
        argument: list[str] = []
        for char in span[match.end() :]:
            if char == "(":
                depth += 1
            elif char == ")":
                if depth == 0:
                    break
                depth -= 1
            argument.append(char)
        text = "".join(argument)
        depth = 0
        condition: list[str] = []
        for char in text:
            if char == "(":
                depth += 1
            elif char == ")":
                depth -= 1
            elif char == "," and depth == 0:
                break
            condition.append(char)
        conditions.append("".join(condition))
    return conditions


def immediate_functions(root: Path, arch: str) -> dict[str, dict[str, object]]:
    """Map function name to const params and their assert coverage.

    A parameter is testable when a ``const`` assert constrains it to fewer
    values than its type allows (for example ``const { assert!(IMM <= 15,
    ...) };``). Full-range immediates (any ``u8`` is encodable) and boolean
    switches have no invalid value. A function is rejectable when at least
    one parameter is testable.
    """
    found: dict[str, dict[str, object]] = {}
    for path in source_files(root / "src" / arch):
        text = path.read_text(encoding="utf-8")
        for match in PUBLIC_FN_WITH_CONST.finditer(text):
            typed = CONST_PARAM.findall(match.group(2))
            if not typed:
                continue
            types = dict(typed)
            start = match.start()
            following = text[start + 10 :]
            end = ITEM_END.search(following)
            span = following[: end.start()] if end else following[:3000]
            conditions = assert_conditions(span)
            mentioned: set[str] = set()
            testable: set[str] = set()
            for param in types:
                relevant = [
                    condition
                    for condition in conditions
                    if re.search(rf"\b{re.escape(param)}\b", condition)
                ]
                if relevant:
                    mentioned.add(param)
                    if not all(
                        assert_is_vacuous(condition, param, types)
                        for condition in relevant
                    ):
                        testable.add(param)
            entry = found.setdefault(
                match.group(1),
                {"params": set(), "mentioned": set(), "testable": set()},
            )
            for key, values in (
                ("params", set(types)),
                ("mentioned", mentioned),
                ("testable", testable),
            ):
                stored = entry[key]
                assert isinstance(stored, set)
                stored.update(values)
    result: dict[str, dict[str, object]] = {}
    for name, entry in found.items():
        params = entry["params"]
        assert isinstance(params, set)
        mentioned = entry["mentioned"]
        assert isinstance(mentioned, set)
        testable = entry["testable"]
        assert isinstance(testable, set)
        result[name] = {
            "params": sorted(params),
            "asserted": bool(mentioned),
            "testable": bool(testable),
        }
    return result


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
        if name not in covered and info["testable"]
    )
    uncovered = sorted(
        name for name, info in functions.items() if name not in covered
    )
    vacuous = sorted(
        name
        for name in uncovered
        if functions[name]["asserted"] and not functions[name]["testable"]
    )
    no_assert = sorted(
        name for name in uncovered if not functions[name]["asserted"]
    )
    return {
        "arch": arch,
        "immediate": len(functions),
        "missing": missing,
        "uncoverable_vacuous": vacuous,
        "uncoverable_no_assert": no_assert,
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
        vacuous = entry["uncoverable_vacuous"]
        assert isinstance(vacuous, list)
        no_assert = entry["uncoverable_no_assert"]
        assert isinstance(no_assert, list)
        print(
            f"{entry['arch']}: {len(missing)} missing / "
            f"{entry['immediate']} immediate "
            f"({len(vacuous)} vacuous, {len(no_assert)} without assert)"
        )
        if missing:
            failed = True
        if args.names or args.check:
            for name in missing:
                print(f"  {name}")
        if args.names:
            for name in vacuous:
                print(f"  uncoverable vacuous bound: {name}")
            for name in no_assert:
                print(f"  uncoverable no assert: {name}")
    if args.check and failed:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
