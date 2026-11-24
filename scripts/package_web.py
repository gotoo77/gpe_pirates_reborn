#!/usr/bin/env python3
"""Build the locked game and package its web entry; no server or deployment."""
import pathlib
import shutil
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[1]


def main():
    version = subprocess.check_output(["wasm-bindgen", "--version"], text=True).strip()
    if version != "wasm-bindgen 0.2.127":
        raise SystemExit(f"wasm-bindgen 0.2.127 required; found {version}")
    subprocess.run([
        "cargo", "build", "--locked", "--release", "--example", "pirates_web",
        "--target", "wasm32-unknown-unknown",
    ], cwd=ROOT, check=True)
    target = pathlib.Path(__import__("os").environ.get("CARGO_TARGET_DIR", ROOT / "target"))
    if not target.is_absolute():
        target = ROOT / target
    destination = ROOT / "dist"
    destination.mkdir(exist_ok=True)
    subprocess.run([
        "wasm-bindgen", "--target", "web", "--out-dir", str(destination),
        "--out-name", "pirates",
        str(target / "wasm32-unknown-unknown/release/examples/pirates_web.wasm"),
    ], cwd=ROOT, check=True)
    shutil.copyfile(ROOT / "web/index.html", destination / "index.html")
    shutil.copyfile(ROOT / "web/boot.mjs", destination / "boot.mjs")
    print(f"Web package: {destination}")


if __name__ == "__main__":
    main()
