#!/bin/bash
# CI lint script for domain-pipeline
set -e

echo "Checking format..."
cargo fmt --all -- --check

echo "Running clippy..."
cargo clippy --all-features -- -D warnings

echo "Running arch audit..."
arch audit --rs

echo "✓ Lint successful"
