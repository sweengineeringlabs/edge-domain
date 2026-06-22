#!/bin/bash
# CI build script for domain-pipeline
set -e

echo "Building domain-pipeline..."
cargo build --all-features --verbose
echo "✓ Build successful"
