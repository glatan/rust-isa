#!/usr/bin/env python3
"""Report instruction coverage from the tables in docs/<arch>/README.md.

The arch README tables are the source of truth. This script only reads them;
it does not generate the tables or the Rust API.
"""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Iterable

BACKTICK = re.compile(r"`([^`]+)`")
NUMBERED = re.compile(r"^(\d+)\s*命令")


def count_items(cell: str) -> int:
    cell = cell.strip()
    if not cell or cell == "なし":
        return 0
    numbered = NUMBERED.match(cell)
    if numbered:
        return int(numbered.group(1))
    return len(BACKTICK.findall(cell))


def read_rows(root: Path) -> Iterable[dict[str, int | str]]:
    for readme in sorted((root / "docs").glob("*/README.md")):
        arch = readme.parent.name
        for raw in readme.read_text(encoding="utf-8").splitlines():
            if not raw.startswith("|"):
                continue
            cells = [cell.strip() for cell in raw.strip().strip("|").split("|")]
            if len(cells) < 3 or not cells[0].startswith("`"):
                continue
            extension = cells[0].replace("`", "").strip()
            yield {
                "arch": arch,
                "extension": extension,
                "implemented": count_items(cells[1]),
                "unimplemented": count_items(cells[2]),
            }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--root",
        type=Path,
        default=Path(__file__).resolve().parent.parent,
        help="repository root (default: parent of tools/)",
    )
    parser.add_argument("--arch", help="only show this arch")
    parser.add_argument(
        "--todo",
        action="store_true",
        help="only show extensions with unimplemented items",
    )
    parser.add_argument("--json", action="store_true", help="print JSON")
    parser.add_argument(
        "--manifest",
        type=Path,
        help="also write the full inventory as JSON to this path",
    )
    args = parser.parse_args()

    rows = list(read_rows(args.root))
    if args.manifest:
        args.manifest.write_text(
            json.dumps(rows, ensure_ascii=False, indent=2) + "\n",
            encoding="utf-8",
        )

    if args.arch:
        rows = [row for row in rows if row["arch"] == args.arch]
    if args.todo:
        rows = [row for row in rows if row["unimplemented"] > 0]

    if args.json:
        print(json.dumps(rows, ensure_ascii=False, indent=2))
        return

    print(f"{'arch':10} {'extension':36} {'done':>6} {'todo':>6}")
    for row in rows:
        print(
            f"{row['arch']:10} {row['extension'][:36]:36} "
            f"{row['implemented']:6} {row['unimplemented']:6}"
        )
    print(
        f"{'total':10} {'':36} "
        f"{sum(int(row['implemented']) for row in rows):6} "
        f"{sum(int(row['unimplemented']) for row in rows):6}"
    )


if __name__ == "__main__":
    main()
