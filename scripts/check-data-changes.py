#!/usr/bin/env python3
"""Rejeita arquivos fora do escopo de um refresh de dados."""

import argparse
import os
import subprocess
import sys


def changed_paths(payload):
    records = iter(payload.split(b"\0"))
    for record in records:
        if not record:
            continue
        if len(record) < 4 or record[2:3] != b" ":
            raise ValueError("status Git inválido")
        yield os.fsdecode(record[3:])
        if any(kind in record[:2] for kind in (ord("R"), ord("C"))):
            original = next(records, b"")
            if not original:
                raise ValueError("origem da renomeação ausente")
            yield os.fsdecode(original)


def allowed(path, after_bump=False):
    if path in ("docs/DATA-FRESHNESS.md",):
        return True
    if after_bump and path in ("Cargo.toml", "Cargo.lock", "CHANGELOG.md"):
        return True
    parts = path.split("/")
    if any(part in ("", ".", "..") for part in parts):
        return False
    return path.startswith(("crates/br-validators/data/", "data/refresh-reports/"))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--after-bump", action="store_true")
    args = parser.parse_args()
    payload = subprocess.check_output(["git", "status", "--porcelain=v1", "-z", "--untracked-files=all"])
    unexpected = sorted(set(path for path in changed_paths(payload) if not allowed(path, args.after_bump)))
    for path in unexpected:
        print(f"Arquivo inesperado no refresh: {path!r}", file=sys.stderr)
    return 1 if unexpected else 0


if __name__ == "__main__":
    sys.exit(main())
