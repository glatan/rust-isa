#!/usr/bin/env python3
"""Check relative Markdown links, heading anchors, and docs reachability."""

from __future__ import annotations

import argparse
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DOCS = ROOT / "docs"
INDEX = DOCS / "README.md"
FILES = [ROOT / "README.md", ROOT / "CONTRIBUTING.md"]
FILES.extend(sorted(DOCS.glob("**/*.md")))
LINK = re.compile(r"\]\(([^)]+)\)")
HEADING = re.compile(r"^#{1,6}\s+(.*?)\s*#*\s*$")


def slug(text: str) -> str:
    text = re.sub(r"<[^>]+>", "", text)
    text = text.replace("`", "").strip().lower()
    text = re.sub(r"[^\w\- ]", "", text, flags=re.UNICODE)
    return text.replace(" ", "-")


def anchors(path: Path) -> set[str]:
    found: set[str] = set()
    for line in path.read_text(encoding="utf-8").splitlines():
        match = HEADING.match(line)
        if match:
            found.add(slug(match.group(1)))
    return found


def linked_docs(path: Path):
    for match in LINK.finditer(path.read_text(encoding="utf-8")):
        target = match.group(1).strip()
        if not target or "://" in target or target.startswith("mailto:"):
            continue
        location, _, _ = target.partition("#")
        destination = path if not location else (path.parent / location).resolve()
        if (
            destination.is_file()
            and destination.suffix == ".md"
            and DOCS in destination.parents
        ):
            yield destination


def reachable_docs() -> set[Path]:
    seen = {INDEX}
    queue = [INDEX]
    while queue:
        for destination in linked_docs(queue.pop()):
            if destination not in seen:
                seen.add(destination)
                queue.append(destination)
    return seen


def check() -> list[str]:
    problems: list[str] = []
    for path in sorted(set(DOCS.glob("**/*.md")) - reachable_docs()):
        problems.append(
            f"{path.relative_to(ROOT)}: not reachable from docs/README.md"
        )
    for path in FILES:
        text = path.read_text(encoding="utf-8")
        for match in LINK.finditer(text):
            target = match.group(1).strip()
            if not target or "://" in target or target.startswith("mailto:"):
                continue
            location, _, anchor = target.partition("#")
            destination = path if not location else (path.parent / location).resolve()
            if not destination.exists():
                problems.append(f"{path.relative_to(ROOT)}: missing {target}")
                continue
            if anchor and destination.suffix == ".md":
                if anchor not in anchors(destination):
                    problems.append(
                        f"{path.relative_to(ROOT)}: missing anchor {target}"
                    )
    return problems


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--check",
        action="store_true",
        help="validate links and exit nonzero on errors",
    )
    args = parser.parse_args()
    problems = check()
    if args.check:
        if problems:
            for problem in problems:
                print(problem)
            raise SystemExit(1)
        print("doc links: ok")
        return
    for problem in problems:
        print(problem)


if __name__ == "__main__":
    main()
