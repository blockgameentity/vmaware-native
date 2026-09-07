#!/usr/bin/env python3
"""Offline vendor consistency check for CI.

Verifies vendor/vmaware.hpp matches vendor/vmaware.sha256, then
regenerates src/technique.rs from the vendored header into a temp
directory and diffs it against the committed file.

Needs a C++ compiler to build ci/dump_techniques.cpp. Honors CXX.
Exit nonzero on any drift so pull requests fail loudly.
"""

from __future__ import annotations

import difflib
import hashlib
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
HEADER = ROOT / "vendor" / "vmaware.hpp"
CHECKSUM_FILE = ROOT / "vendor" / "vmaware.sha256"
DUMPER_SOURCE = ROOT / "ci" / "dump_techniques.cpp"
GENERATOR = ROOT / "ci" / "generate_techniques.py"
COMMITTED = ROOT / "src" / "technique.rs"
SHA_RE = re.compile(r"^[0-9a-f]{64}$")


def fail(message: str) -> int:
    print(f"check-vendor: error: {message}", file=sys.stderr)
    return 1


def main() -> int:
    try:
        expected = CHECKSUM_FILE.read_text(encoding="utf-8").strip().split()[0]
    except OSError as exc:
        return fail(f"cannot read {CHECKSUM_FILE}: {exc}")
    if not SHA_RE.match(expected):
        return fail(f"invalid checksum in {CHECKSUM_FILE}")

    try:
        digest = hashlib.sha256(HEADER.read_bytes()).hexdigest()
    except OSError as exc:
        return fail(f"cannot read {HEADER}: {exc}")
    if digest != expected:
        return fail(f"{HEADER.name} does not match {CHECKSUM_FILE.name}")

    cxx = os.environ.get("CXX", "c++").split()
    if not cxx:
        return fail("CXX must name a C++ compiler")
    if shutil.which(cxx[0]) is None:
        return fail(f"C++ compiler not found: {cxx[0]}")

    try:
        committed = COMMITTED.read_text(encoding="utf-8")
    except OSError as exc:
        return fail(f"cannot read {COMMITTED}: {exc}")

    with tempfile.TemporaryDirectory(prefix="check-vendor-") as tmp:
        tmpdir = Path(tmp)
        dumper = tmpdir / ("dump_techniques.exe" if os.name == "nt" else "dump_techniques")
        try:
            subprocess.run(
                [*cxx, "-std=c++20", f"-I{HEADER.parent}", str(DUMPER_SOURCE), "-o", str(dumper)],
                check=True,
                capture_output=True,
                text=True,
            )
        except subprocess.CalledProcessError as exc:
            print(exc.stdout, end="")
            print(exc.stderr, end="", file=sys.stderr)
            return fail("failed to compile dump_techniques.cpp")

        try:
            proc = subprocess.run([str(dumper)], check=True, capture_output=True, text=True)
        except subprocess.CalledProcessError as exc:
            print(exc.stdout, end="")
            print(exc.stderr, end="", file=sys.stderr)
            return fail("dump_techniques failed")

        tsv = tmpdir / "techniques.tsv"
        tsv.write_text(proc.stdout, encoding="utf-8")
        regenerated = tmpdir / "technique.rs"
        try:
            subprocess.run(
                [sys.executable, str(GENERATOR), str(tsv), str(regenerated)],
                check=True,
                capture_output=True,
                text=True,
            )
        except subprocess.CalledProcessError as exc:
            print(exc.stdout, end="")
            print(exc.stderr, end="", file=sys.stderr)
            return fail("generate_techniques.py failed")

        fresh = regenerated.read_text(encoding="utf-8")
        if fresh == committed:
            print(f"check-vendor: ok ({digest[:12]}..., {COMMITTED.name} in sync)")
            return 0

        diff = "".join(
            difflib.unified_diff(
                committed.splitlines(keepends=True),
                fresh.splitlines(keepends=True),
                fromfile=f"a/{COMMITTED.relative_to(ROOT).as_posix()}",
                tofile=f"b/{COMMITTED.relative_to(ROOT).as_posix()}",
            )
        )
        print(diff, end="")
        return fail(
            f"{COMMITTED.name} is out of sync with {HEADER.name}; run ci/update-vmaware.sh"
        )


if __name__ == "__main__":
    raise SystemExit(main())
