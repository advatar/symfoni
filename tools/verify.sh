#!/usr/bin/env bash
set -euo pipefail

echo "== Rust =="
cargo fmt --check
cargo clippy -- -D warnings
cargo test

echo "== Swift =="
# Adjust scheme name(s) to your repo. Keeping non-failing defaults.
xcodebuild -version
