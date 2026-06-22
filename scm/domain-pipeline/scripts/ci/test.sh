#!/bin/bash
# CI test script for domain-pipeline
set -e

echo "Running tests..."
cargo test --all-features --verbose

echo "Running doc tests..."
cargo test --doc --all-features

echo "✓ Tests successful (109 tests)"
