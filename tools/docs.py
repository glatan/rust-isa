#!/usr/bin/env python3
"""Build the local rustdoc site for enabled targets.

The output is docs-build/<target>/rust_isa. It is not committed and is never
deployed to an external service.
"""

from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TARGETS = ROOT / "docs" / "targets.toml"
SITE = ROOT / "docs-build"
CARGO_TARGET = ROOT / "target" / "docs-cargo"


def load_targets() -> list[dict]:
    with TARGETS.open("rb") as f:
        data = tomllib.load(f)
    return [target for target in data.get("target", []) if target.get("enabled")]


def build_target(target: dict) -> None:
    triple = target["triple"]
    target_dir = CARGO_TARGET / triple
    env = os.environ.copy()
    env["RUST_ISA_DOCS"] = "1"
    env["RUSTDOCFLAGS"] = target.get("rustdocflags", "")
    command = ["cargo"]
    if target.get("build_std", False):
        env["RUSTC_BOOTSTRAP"] = "1"
        command.extend(["-Z", "build-std=core"])
    command.extend(
        [
            "doc",
            "--no-deps",
            "--target",
            triple,
            "--target-dir",
            str(target_dir),
        ]
    )
    for feature in target.get("features", []):
        command.extend(["--features", feature])
    print(f"docs: {triple}")
    subprocess.run(command, cwd=ROOT, env=env, check=True)

    source = target_dir / triple / "doc" / "rust_isa"
    if not source.is_dir():
        raise SystemExit(f"docs: missing rustdoc output for {triple}: {source}")
    destination = SITE / triple / "rust_isa"
    if destination.exists():
        shutil.rmtree(destination)
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copytree(source, destination)


def write_index(targets: list[dict]) -> None:
    SITE.mkdir(parents=True, exist_ok=True)
    rows = "\n".join(
        f'<li><a href="{target["triple"]}/rust_isa/index.html">'
        f'{target["triple"]}</a> ({target["arch"]})</li>'
        for target in targets
    )
    html = f"""<!doctype html>
<html lang="ja">
<meta charset="utf-8">
<title>rust-isa local docs</title>
<h1>rust-isa local docs</h1>
<ul>
{rows}
</ul>
</html>
"""
    (SITE / "index.html").write_text(html, encoding="utf-8")


def check(targets: list[dict]) -> int:
    errors = []
    index = SITE / "index.html"
    if not index.is_file():
        errors.append(f"missing {index.relative_to(ROOT)}")
    for target in targets:
        page = SITE / target["triple"] / "rust_isa" / "index.html"
        if not page.is_file():
            errors.append(f"missing {page.relative_to(ROOT)}")
    for error in errors:
        print(error, file=sys.stderr)
    return 1 if errors else 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    targets = load_targets()
    if args.check:
        return check(targets)
    for target in targets:
        build_target(target)
    write_index(targets)
    return check(targets)


if __name__ == "__main__":
    raise SystemExit(main())
