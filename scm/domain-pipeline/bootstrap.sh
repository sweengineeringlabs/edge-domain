#!/bin/bash
# Bootstrap script for edge-domain-pipeline on Unix
# Purpose: Set up development environment

set -e

print_header() {
    echo -e "\033[32m$1\033[0m"
}

print_step() {
    echo -e "\033[36m$1\033[0m"
}

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo "Rust toolchain not found. Please install from https://rustup.rs"
    exit 1
fi

print_header "edge-domain-pipeline bootstrap"

# Parse arguments
run_test=false
run_lint=false
run_doc=false
run_all=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --test) run_test=true; shift ;;
        --lint) run_lint=true; shift ;;
        --doc) run_doc=true; shift ;;
        --all) run_all=true; shift ;;
        *) shift ;;
    esac
done

# Build
print_step "Building domain-pipeline..."
cargo build --all-features

if [ "$run_all" = true ]; then
    run_test=true
    run_lint=true
    run_doc=true
fi

if [ "$run_test" = true ]; then
    print_step "Running tests..."
    cargo test --all-features
fi

if [ "$run_lint" = true ]; then
    print_step "Running clippy..."
    cargo clippy --all-features -- -D warnings
fi

if [ "$run_doc" = true ]; then
    print_step "Building documentation..."
    cargo doc --all-features --no-deps --open
fi

print_header "✓ Bootstrap complete"
