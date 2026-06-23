#!/usr/bin/env python3

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "src" / "technique.rs"
TEMPLATE = ROOT / "ci" / "technique_rs.txt"
SECTION_COMMENTS = {
    "GPU_CAPABILITIES": "Windows",
    "TASK_SEGMENT": "Linux + Windows",
    "SMBIOS_VM_BIT": "Linux",
    "THREAD_COUNT": "Linux + macOS",
    "MAC_MEMSIZE": "macOS",
    "HYPERVISOR_BIT": "Cross-platform",
}


def snake_to_pascal(name: str) -> str:
    overrides = {
        "VMID": "Vmid",
        "DLL": "Dll",
        "MSR": "Msr",
        "EDID": "Edid",
    }

    if name in overrides:
        return overrides[name]

    return "".join(part.lower().capitalize() for part in name.split("_"))


def read_techniques(path: Path) -> list[tuple[int, str, str]]:
    techniques: list[tuple[int, str, str]] = []

    for line_no, line in enumerate(path.read_text().splitlines(), start=1):
        line = line.strip()

        if not line:
            continue

        try:
            value_raw, canonical = line.split("\t", 1)
        except ValueError:
            raise SystemExit(f"{path}:{line_no}: expected '<number>\\t<NAME>'")

        value = int(value_raw)

        if value < 0 or value > 255:
            raise SystemExit(f"{path}:{line_no}: value {value} does not fit repr(u8)")

        if canonical == "Unknown flag":
            raise SystemExit(f"{path}:{line_no}: got unknown VMAware flag")

        variant = snake_to_pascal(canonical)
        techniques.append((value, canonical, variant))

    techniques.sort(key=lambda item: item[0])

    values = [value for value, _, _ in techniques]
    names = [canonical for _, canonical, _ in techniques]
    variants = [variant for _, _, variant in techniques]

    if len(values) != len(set(values)):
        raise SystemExit("duplicate VMAware technique values")

    if len(names) != len(set(names)):
        raise SystemExit("duplicate VMAware technique names")

    if len(variants) != len(set(variants)):
        raise SystemExit("duplicate generated Rust variant names")

    return techniques


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit("usage: generate_techniques.py /path/to/vmaware-techniques.tsv")

    techniques = read_techniques(Path(sys.argv[1]))
    template = TEMPLATE.read_text(encoding="utf-8")

    for marker in ("%tech%", "%tech_all%"):
        if template.count(marker) != 1:
            raise SystemExit(f"{TEMPLATE}: expected exactly one {marker} marker")

    tech_lines: list[str] = []
    tech_all_lines: list[str] = []

    for value, canonical, variant in techniques:
        if canonical in SECTION_COMMENTS:
            comment = SECTION_COMMENTS[canonical]
            tech_lines.append(f"    // {comment}")
            tech_all_lines.append(f"        // {comment}")

        tech_lines.append(f"    {variant} = {value},")
        tech_all_lines.append(f"        Self::{variant},")

    tech = "\n".join(tech_lines)
    tech_all = "\n".join(tech_all_lines)
    generated = template.replace("    %tech%", tech)
    generated = generated.replace("        %tech_all%", tech_all)

    if "%tech%" in generated or "%tech_all%" in generated:
        raise SystemExit(f"{TEMPLATE}: failed to replace template markers")

    OUT.write_text(generated, encoding="utf-8")
    print(f"generated {OUT} with {len(techniques)} techniques")


if __name__ == "__main__":
    main()
