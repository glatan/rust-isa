#!/usr/bin/env python3
"""Validate extension manifests.

This is the pilot validator. It checks the schema and source references, not
whether every upstream instruction has a public wrapper yet.
"""

from __future__ import annotations

import argparse
import hashlib
import re
import sys
import tomllib
from collections import Counter
from pathlib import Path

from doc_names import arch_names

ROOT = Path(__file__).resolve().parents[1]
MANIFEST_DIR = ROOT / "tools" / "isa"
SOURCE_FILE = ROOT / "tools" / "upstream" / "SOURCE.toml"

TOP_LEVEL = {
    "schema",
    "arch",
    "extension",
    "module",
    "kind",
    "source",
    "coverage",
}
INSTRUCTION = {"mnemonic", "api", "status"}
STATUS = {"implemented", "missing", "excluded"}
KIND = {"fixed", "scalable", "stateful", "subextension"}


ARCH_NAMES: dict[str, set[str]] = {}


def load_sources() -> dict[str, dict]:
    with SOURCE_FILE.open("rb") as f:
        data = tomllib.load(f)
    return {entry["name"]: entry for entry in data.get("source", [])}


def check_source_data(sources: dict[str, dict]) -> list[str]:
    errors: list[str] = []
    for name, source in sources.items():
        for entry in source.get("data", []):
            path = ROOT / entry["path"]
            if not path.is_file():
                errors.append(f"{name}: missing data file: {entry['path']}")
                continue
            digest = hashlib.sha256(path.read_bytes()).hexdigest()
            if digest != entry["sha256"]:
                errors.append(f"{name}: sha256 mismatch: {entry['path']}")
    return errors


def source_path(module: str) -> Path:
    return ROOT / "src" / Path(*module.removeprefix("crate::").split("::"))


def public_api_exists(module: str, api: str) -> bool:
    directory = source_path(module)
    if not directory.is_dir():
        return False
    pattern = re.compile(rf"pub\s+(?:unsafe\s+)?fn\s+{re.escape(api)}\b")
    for path in directory.rglob("*.rs"):
        if pattern.search(path.read_text(encoding="utf-8")):
            return True
    return False


def test_id_exists(test_id: str) -> bool:
    path_text, _, function = test_id.partition("::")
    path = ROOT / path_text
    if not path.exists():
        return False
    if function:
        if not path.is_file():
            return False
        text = path.read_text(encoding="utf-8")
        return re.search(rf"\bfn\s+{re.escape(function)}\b", text) is not None
    return True


def manifest_files() -> list[Path]:
    return sorted(
        p
        for p in MANIFEST_DIR.rglob("*.toml")
        if p.is_file() and p.name != "queue.toml"
    )


def check_manifest(path: Path, sources: dict[str, dict]) -> tuple[str, Counter]:
    with path.open("rb") as f:
        data = tomllib.load(f)

    errors: list[str] = []
    missing = TOP_LEVEL - set(data)
    if missing:
        errors.append(f"missing top-level keys: {sorted(missing)}")
    if data.get("schema") != 1:
        errors.append("schema must be 1")
    if data.get("kind") not in KIND:
        errors.append(f"unknown kind: {data.get('kind')!r}")
    if data.get("source") not in sources:
        errors.append(f"unknown source: {data.get('source')!r}")
    if data.get("coverage") not in {"pilot", "complete"}:
        errors.append(f"unknown coverage: {data.get('coverage')!r}")

    instructions = data.get("instruction", [])
    if not instructions:
        errors.append("no [[instruction]] entries")

    seen_api: set[str] = set()
    counts: Counter = Counter()
    for index, instruction in enumerate(instructions, start=1):
        missing = INSTRUCTION - set(instruction)
        if missing:
            errors.append(f"instruction {index}: missing keys: {sorted(missing)}")
            continue
        api = instruction["api"]
        if api:
            if api in seen_api:
                errors.append(f"instruction {index}: duplicate api: {api!r}")
            seen_api.add(api)
        status = instruction["status"]
        if status not in STATUS:
            errors.append(f"instruction {index}: unknown status: {status!r}")
        if status == "excluded":
            if not instruction.get("exclusion_class"):
                errors.append(f"instruction {index}: excluded without exclusion_class")
            if not instruction.get("reason"):
                errors.append(f"instruction {index}: excluded without reason")
        if "test_ids" in instruction and not isinstance(instruction["test_ids"], list):
            errors.append(f"instruction {index}: test_ids must be a list")
        if status == "implemented" and not api:
            errors.append(f"instruction {index}: implemented without api")
        test_ids = instruction.get("test_ids", [])
        if status == "implemented" and not test_ids:
            errors.append(f"instruction {index}: implemented without test_ids")
        for test_id in test_ids:
            if not test_id_exists(test_id):
                errors.append(
                    f"instruction {index}: test id not found: {test_id}"
                )
        if "generated" in instruction and not isinstance(instruction["generated"], bool):
            errors.append(f"instruction {index}: generated must be a boolean")
        if status == "implemented" and api:
            module = data.get("module", "")
            arch = module.removeprefix("crate::").split("::")[0]
            names = ARCH_NAMES.setdefault(arch, arch_names(arch))
            if not public_api_exists(module, api) and api not in names:
                errors.append(
                    f"instruction {index}: public API not found: "
                    f"{module}::{api}"
                )
        counts[status] += 1

    label = f"{data.get('arch')}::{data.get('extension')}"
    if errors:
        for error in errors:
            print(f"{path.relative_to(ROOT)}: {error}", file=sys.stderr)
        raise SystemExit(1)
    return label, counts



def check_acle_sve_inventory() -> list[str]:
    """Verify every ACLE SVE/SVE2 instruction has a manifest entry."""
    import json

    path = ROOT / "tools/upstream/acle/sve-instructions.json"
    if not path.is_file():
        return [f"missing ACLE inventory: {path.relative_to(ROOT)}"]
    upstream = [
        entry["instruction"].upper()
        for entry in json.loads(path.read_text(encoding="utf-8"))["instructions"]
    ]
    names: dict[str, str] = {}
    for path in sorted((MANIFEST_DIR / "aarch64").glob("*.toml")):
        if path.name == "queue.toml":
            continue
        data = tomllib.loads(path.read_text(encoding="utf-8"))
        for instruction in data["instruction"]:
            if instruction["status"] == "missing":
                continue
            names[instruction["mnemonic"].upper()] = instruction["status"]
            for form in instruction.get("forms", []):
                names[form.upper()] = instruction["status"]
    missing = sorted(set(upstream) - names.keys())
    if missing:
        return [f"ACLE instruction not implemented or excluded: {name}" for name in missing]
    print(f"aarch64 ACLE inventory: {len(set(upstream))}/{len(set(upstream))}")
    return []

def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--pilot",
        action="store_true",
        help="allow status=missing (pilot manifests)",
    )
    parser.add_argument(
        "--require-complete",
        action="store_true",
        help="fail if any instruction is still missing",
    )
    args = parser.parse_args()

    sources = load_sources()
    data_errors = check_source_data(sources)
    if data_errors:
        for error in data_errors:
            print(error, file=sys.stderr)
        return 1

    inventory_errors = check_acle_sve_inventory()
    if inventory_errors:
        for error in inventory_errors:
            print(error, file=sys.stderr)
        return 1

    total: Counter = Counter()
    for path in manifest_files():
        label, counts = check_manifest(path, sources)
        total.update(counts)
        print(
            f"{label}: "
            f"implemented={counts['implemented']} "
            f"missing={counts['missing']} "
            f"excluded={counts['excluded']}"
        )

    print(
        "total: "
        f"implemented={total['implemented']} "
        f"missing={total['missing']} "
        f"excluded={total['excluded']}"
    )

    if args.require_complete and total["missing"]:
        print("missing instructions remain", file=sys.stderr)
        return 1
    if not args.pilot and total["missing"]:
        print("use --pilot while manifests are incomplete", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
