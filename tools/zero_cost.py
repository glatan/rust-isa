#!/usr/bin/env python3
"""Check the zero-cost measurement table in docs/zero-cost.md.

Each row is keyed by (arch, fixture, kernel, extension, instructions).
Fixtures must exist. Kernel names must appear in the fixture source. The
``manual`` fixture marks a measurement made with a temporary binary, so its
kernel is ``—`` and is not checked against the repository. Every manual row
must have an explicit reason in ``MANUAL``. LTO, call, store/load, and result
values are restricted to known values, and a zero-cost row must have no call
and no store/load.
"""

from __future__ import annotations

import argparse
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DOC = ROOT / "docs" / "zero-cost.md"
LTO_VALUES = {"thin", "fat", "thin/fat", "off"}
CALL_VALUES = {"なし", "—"}
STORE_LOAD_VALUES = {"なし", "あり", "—"}
RESULT_VALUES = {
    "ゼロコスト",
    "往復が残る",
    "8-byte table のみ",
    "スカラー C フォールバック",
    "wat で確認",
}
BACKTICK = re.compile(r"`([^`]+)`")

# Manual rows must have an explicit reason. The key is
# (arch, extension, instructions) as written in the table.
MANUAL = {
    ("RISC-V", "`v`", "`vrgather`"): "store/load remains",
    ("RISC-V", "`v`", "`vcompress`"): "store/load remains",
    ("LoongArch", "`lasx`", "`xvshuf.b`"): "store/load remains",
    ("32-bit ARM", "`neon`", "`vadd.i32`、`vmul.i32`"): "value fixture does not inline C",
    ("32-bit ARM", "`neon`", "`vtbl1_s8`、`vtbl2_s8`"): "8-byte table only",
    ("MIPS", "`msa`", "`vshf.w`"): "scalar C fallback",
}


def table_rows() -> list[list[str]]:
    lines = DOC.read_text(encoding="utf-8").splitlines()
    rows: list[list[str]] = []
    in_table = False
    for line in lines:
        if line.strip() == "## arch別の測定結果":
            in_table = True
            continue
        if in_table and line.startswith("## "):
            break
        if not in_table or not line.startswith("|"):
            continue
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
        if cells[0] == "arch" or set(cells[0]) <= set("-: "):
            continue
        rows.append(cells)
    return rows


def names(cell: str) -> list[str]:
    return BACKTICK.findall(cell)


def fixture_text(path: Path) -> str:
    parts = []
    for suffix in ("*.rs", "*.c", "*.h"):
        for file in path.rglob(suffix):
            parts.append(file.read_text(encoding="utf-8", errors="ignore"))
    return "\n".join(parts)


def check() -> list[str]:
    problems: list[str] = []
    seen: set[tuple[str, ...]] = set()
    seen_manual: set[tuple[str, ...]] = set()
    for number, cells in enumerate(table_rows(), 2):
        if len(cells) != 9:
            problems.append(f"row {number}: expected 9 columns, found {len(cells)}")
            continue
        arch, fixture, kernel, lto, extension, instructions, call, store_load, result = cells
        fixture = fixture.strip("`")
        if lto not in LTO_VALUES:
            problems.append(f"row {number}: unknown LTO value {lto!r}")
        if call not in CALL_VALUES:
            problems.append(f"row {number}: unknown call value {call!r}")
        if store_load not in STORE_LOAD_VALUES:
            problems.append(
                f"row {number}: unknown store/load value {store_load!r}"
            )
        if result not in RESULT_VALUES:
            problems.append(f"row {number}: unknown result value {result!r}")
        if result == "ゼロコスト" and (call != "なし" or store_load != "なし"):
            problems.append(
                f"row {number}: zero-cost row must have no call and no store/load"
            )
        if result == "往復が残る" and store_load != "あり":
            problems.append(
                f"row {number}: round-trip row must record store/load as あり"
            )
        key = (arch, fixture, kernel, extension, instructions)
        if key in seen:
            problems.append(f"row {number}: duplicate measurement {key}")
        seen.add(key)

        if fixture == "manual":
            if kernel != "—":
                problems.append(f"row {number}: manual measurement must use kernel —")
            manual_key = (arch, extension, instructions)
            if manual_key not in MANUAL:
                problems.append(
                    f"row {number}: manual measurement has no reason: {manual_key}"
                )
            else:
                seen_manual.add(manual_key)
            continue

        path = ROOT / fixture
        if not path.is_dir():
            problems.append(f"row {number}: missing fixture {fixture}")
            continue
        text = fixture_text(path)
        for name in names(kernel):
            if not re.search(rf"\b{re.escape(name)}\b", text):
                problems.append(
                    f"row {number}: kernel {name!r} not found in {fixture}"
                )

    used = {
        row[1].strip("`")
        for row in table_rows()
        if len(row) == 9 and row[1].strip("`") != "manual"
    }
    for path in sorted((ROOT / "fixtures").glob("*-link")):
        fixture = path.relative_to(ROOT).as_posix()
        if path.is_dir() and fixture not in used:
            problems.append(f"link fixture {fixture} is not in the zero-cost table")
    for key in sorted(set(MANUAL) - seen_manual):
        problems.append(f"manual reason for a missing row: {key}")
    return problems


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--check",
        action="store_true",
        help="validate the zero-cost measurement table and exit nonzero on errors",
    )
    args = parser.parse_args()
    problems = check()
    if args.check:
        if problems:
            for problem in problems:
                print(problem)
            raise SystemExit(1)
        print("zero-cost measurements: ok")
        return
    for problem in problems:
        print(problem)


if __name__ == "__main__":
    main()
