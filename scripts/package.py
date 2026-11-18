"""Verify a native dist build's version and DOM execution before packaging it."""

import argparse
import shutil
import subprocess
import tempfile
import tomllib
from pathlib import Path

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("target")
args = parser.parse_args()

target = args.target
name = f"limo-{target}"
executable = "limo.exe" if "windows" in target else "limo"
binary = Path("target") / target / "dist" / executable
with open("Cargo.toml", "rb") as manifest:
    version = tomllib.load(manifest)["package"]["version"]

Path("dist").mkdir(exist_ok=True)
with tempfile.TemporaryDirectory(prefix="limo-release-") as temporary:
    package = Path(temporary) / name
    package.mkdir()
    packaged_binary = package / executable
    shutil.copy2(binary, packaged_binary)
    shutil.copy2("LICENSE", package)
    shutil.copy2("README.md", package)

    reported_version = subprocess.check_output(
        [str(packaged_binary), "--version"], text=True, timeout=30
    ).strip()
    assert reported_version == f"limo {version}", reported_version
    dom_output = subprocess.check_output(
        [str(packaged_binary), "-e", "console.log(document.createElement('p').tagName)"],
        cwd=package, text=True, timeout=30,
    )
    if dom_output != "P\n":
        raise SystemExit(f"Unexpected DOM output: {dom_output!r}")

    archive = shutil.make_archive(
        str(Path("dist") / name), "zip" if "windows" in target else "gztar",
        root_dir=temporary, base_dir=name,
    )
    print(f"Created {archive}")
