#!/usr/bin/env python3
"""Empacota a CLI, README e licença com metadados reproduzíveis."""

import argparse
import datetime
import gzip
import io
from pathlib import Path
import re
import subprocess
import tarfile
import zipfile


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--tag", required=True)
    parser.add_argument("--target", required=True, choices=[
        "x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu",
        "aarch64-apple-darwin", "x86_64-apple-darwin", "x86_64-pc-windows-msvc",
    ])
    parser.add_argument("--binary", required=True, type=Path)
    parser.add_argument("--out", default=Path("dist"), type=Path)
    args = parser.parse_args()
    if not re.fullmatch(r"v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)", args.tag):
        parser.error("tag deve ser vMAJOR.MINOR.PATCH")
    root = Path(__file__).resolve().parent.parent
    epoch = int(subprocess.check_output(["git", "log", "-1", "--format=%ct"], cwd=root, text=True).strip())
    windows = args.target.endswith("windows-msvc")
    binary_name = "br-validators.exe" if windows else "br-validators"
    files = [("LICENSE", (root / "LICENSE").read_bytes(), 0o644),
             ("README.md", (root / "README.md").read_bytes(), 0o644),
             (binary_name, args.binary.read_bytes(), 0o755)]
    args.out.mkdir(parents=True, exist_ok=True)
    name = f"br-validators-{args.tag}-{args.target}"
    if windows:
        date = datetime.datetime.fromtimestamp(epoch, datetime.timezone.utc)
        with zipfile.ZipFile(args.out / f"{name}.zip", "w", compression=zipfile.ZIP_DEFLATED) as archive:
            for path, data, mode in files:
                info = zipfile.ZipInfo(path, date.timetuple()[:6])
                info.create_system = 3
                info.external_attr = (0o100000 | mode) << 16
                info.compress_type = zipfile.ZIP_DEFLATED
                archive.writestr(info, data)
    else:
        with (args.out / f"{name}.tar.gz").open("wb") as output:
            with gzip.GzipFile(fileobj=output, mode="wb", filename="", mtime=epoch) as compressed:
                with tarfile.open(fileobj=compressed, mode="w", format=tarfile.GNU_FORMAT) as archive:
                    for path, data, mode in files:
                        info = tarfile.TarInfo(path)
                        info.size = len(data)
                        info.mode = mode
                        info.mtime = epoch
                        archive.addfile(info, io.BytesIO(data))


if __name__ == "__main__":
    main()
