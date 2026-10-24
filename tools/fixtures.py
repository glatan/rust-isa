#!/usr/bin/env python3
"""Check that fixtures/ and docs/fixture-matrix.md agree.

The check also compares the target-nested table with the nested_dir() names
used by the integration tests.
"""

from __future__ import annotations

import argparse
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DOC = ROOT / "docs" / "fixture-matrix.md"
FIXTURE = re.compile(r"`(fixtures/[^`]+)`")
TEST_NESTED = re.compile(r'nested_dir\("([^"]+)"\)')
MATRIX_NESTED = re.compile(r"`target-nested/([^`]+)`")


def check() -> list[str]:
    problems: list[str] = []
    listed = {
        match
        for match in FIXTURE.findall(DOC.read_text(encoding="utf-8"))
        if "<" not in match and ">" not in match
    }
    actual = {
        path.relative_to(ROOT).as_posix()
        for path in (ROOT / "fixtures").iterdir()
        if path.is_dir()
    }
    for fixture in sorted(listed - actual):
        problems.append(f"fixture-matrix.md lists missing {fixture}")
    for fixture in sorted(actual - listed):
        problems.append(f"fixture-matrix.md does not list {fixture}")

    tests = "\n".join(
        path.read_text(encoding="utf-8")
        for path in sorted((ROOT / "tests").rglob("*.rs"))
    )
    used_nested = set(TEST_NESTED.findall(tests))
    listed_nested = set(MATRIX_NESTED.findall(DOC.read_text(encoding="utf-8")))
    for name in sorted(used_nested - listed_nested):
        problems.append(f"tests use target-nested/{name} but fixture-matrix.md does not list it")
    for name in sorted(listed_nested - used_nested):
        problems.append(f"fixture-matrix.md lists target-nested/{name} but no test uses it")
    return problems


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--check",
        action="store_true",
        help="validate the fixture matrix and exit nonzero on errors",
    )
    args = parser.parse_args()
    problems = check()
    if args.check:
        if problems:
            for problem in problems:
                print(problem)
            raise SystemExit(1)
        print("fixtures: ok")
        return
    for problem in problems:
        print(problem)


if __name__ == "__main__":
    main()
