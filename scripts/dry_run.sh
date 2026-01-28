#!/usr/bin/env bash
set -euo pipefail

echo "[dry-run] cargo fmt check"
cargo fmt --all -- --check

echo "[dry-run] cargo clippy"
cargo clippy --all-targets --all-features -- -D warnings

echo "[dry-run] cargo test"
cargo test --all --all-features

echo "[dry-run] simulation tests"
cargo test --test simulation_test

echo "[dry-run] done"
