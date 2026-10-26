#!/usr/bin/env python3
"""Check that `make` targets named in the docs exist in the Makefile."""

from __future__ import annotations

import argparse
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MAKEFILE = ROOT / "Makefile"
DOCS = [ROOT / "README.md", ROOT / "CONTRIBUTING.md", *sorted((ROOT / "docs").rglob("*.md"))]
TARGET = re.compile(r"^([A-Za-z0-9_.-]+):", re.MULTILINE)
MAKE = re.compile(r"\bmake\s+([A-Za-z0-9_.-]+)")
SKIP = {"target"}


def makefile_targets() -> set[str]:
    text = MAKEFILE.read_text(encoding="utf-8")
    targets = set(TARGET.findall(text))
    for line in text.splitlines():
        if line.startswith(".PHONY:"):
            targets.update(line.removeprefix(".PHONY:").split())
    return targets


def check() -> list[str]:
    targets = makefile_targets()
    problems: list[str] = []
    for path in DOCS:
        text = path.read_text(encoding="utf-8")
        for number, line in enumerate(text.splitlines(), 1):
            for name in MAKE.findall(line):
                if name in SKIP or name in targets:
                    continue
                problems.append(f"{path}:{number}: make target not in Makefile: {name}")
    return problems


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="check documented make targets")
    args = parser.parse_args()
    problems = check()
    if args.check:
        if problems:
            for problem in problems:
                print(problem)
            raise SystemExit(1)
        print("make targets: ok")
        return
    for problem in problems:
        print(problem)


if __name__ == "__main__":
    main()
