#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."
cargo fmt --all -- --check
cargo check --locked --all-targets
cargo test --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --release
cargo check --locked --all-targets --target wasm32-unknown-unknown
cargo clippy --locked --all-targets --target wasm32-unknown-unknown -- -D warnings
python3 -B scripts/package_web.py
bash -n up.sh scripts/check.sh
python3 -B -m unittest discover -s tests -v
node --test web/boot.test.mjs
git diff --check
