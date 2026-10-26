#!/usr/bin/env python3
"""Check that API names in the scalable mapping table exist in src.

The mapping table is a representative list, not an inventory. This script
checks only that the API names in its tables and prose exist somewhere in
`src`; it does not verify that each name belongs to the mapped module.
"""

from __future__ import annotations

import argparse
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DOC = ROOT / "docs" / "scalable-mapping.md"
SOURCE = ROOT / "src"
BACKTICK = re.compile(r"`([^`]+)`")
API_NAME = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)?$")


def source_text() -> str:
    return "\n".join(path.read_text(encoding="utf-8") for path in SOURCE.rglob("*.rs"))


def api_candidates(text: str) -> list[str]:
    candidates: list[str] = []
    for line in text.splitlines():
        if not line.startswith("|"):
            continue
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
        if len(cells) < 2:
            continue
        for item in BACKTICK.findall(cells[0]):
            for part in item.split("/"):
                candidates.append(part.strip())
    return candidates


def prose_types(text: str) -> list[str]:
    names: list[str] = []
    for item in BACKTICK.findall(text):
        if "::" not in item and not item[:1].isupper():
            continue
        if item == item.upper():
            continue
        if item.startswith(("http://", "https://")):
            continue
        names.append(item.split("::")[0].strip())
    return names


def check() -> list[str]:
    text = DOC.read_text(encoding="utf-8")
    src = source_text()
    problems: list[str] = []
    seen: set[str] = set()
    for item in api_candidates(text) + prose_types(text):
        if not item or item in seen:
            continue
        seen.add(item)
        base = item.split("::")[0].split("<")[0].split("(")[0].strip()
        if not API_NAME.match(base):
            continue
        if not re.search(rf"\b{re.escape(base)}\b", src):
            problems.append(f"{DOC}: API name has no source definition: {item}")
    return problems


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="check the mapping table")
    args = parser.parse_args()
    problems = check()
    if args.check:
        if problems:
            for problem in problems:
                print(problem)
            raise SystemExit(1)
        print("scalable mapping: ok")
        return
    for problem in problems:
        print(problem)


if __name__ == "__main__":
    main()
