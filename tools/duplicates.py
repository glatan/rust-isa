#!/usr/bin/env python3
"""Find instruction names that appear in more than one extension of one arch."""

from __future__ import annotations

import argparse
import re
from collections import defaultdict
from pathlib import Path

BACKTICK = re.compile(r"`([^`]+)`")


def rows(root: Path):
    for readme in sorted((root / "docs").glob("*/README.md")):
        arch = readme.parent.name
        for raw in readme.read_text(encoding="utf-8").splitlines():
            if not raw.startswith("|"):
                continue
            cells = [cell.strip() for cell in raw.strip().strip("|").split("|")]
            if len(cells) < 4 or not cells[0].startswith("`"):
                continue
            extension = cells[0].replace("`", "").strip()
            names = BACKTICK.findall(cells[1]) + BACKTICK.findall(cells[2])
            yield arch, extension, names


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--root",
        type=Path,
        default=Path(__file__).resolve().parent.parent,
        help="repository root (default: parent of tools/)",
    )
    parser.add_argument("--arch", help="only show this arch")
    args = parser.parse_args()

    by_arch: dict[str, dict[str, set[str]]] = defaultdict(lambda: defaultdict(set))
    for arch, extension, names in rows(args.root):
        for name in names:
            by_arch[arch][name].add(extension)

    found = False
    for arch in sorted(by_arch):
        if args.arch and arch != args.arch:
            continue
        for name in sorted(by_arch[arch]):
            extensions = sorted(by_arch[arch][name])
            if len(extensions) < 2:
                continue
            found = True
            print(f"{arch:10} {name:28} {', '.join(extensions)}")
    if not found:
        print("no duplicate instruction names")


if __name__ == "__main__":
    main()
