#!/usr/bin/env bash
set -euo pipefail

# edge-domain-agent bootstrap script
# Sets up the development environment for the agent domain primitive crate

echo "=== edge-domain-agent bootstrap ==="
echo "Installing Rust toolchain..."
rustup update

echo "Checking dependencies..."
cargo check

echo "Running tests..."
cargo test

echo "=== Bootstrap complete ==="
