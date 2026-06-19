#!/usr/bin/env bash
set -euo pipefail

# edge-domain-vpn bootstrap — install toolchain and verify build
cd "$(dirname "$0")"

if ! command -v cargo &>/dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
    source "$HOME/.cargo/env"
fi

cargo build
cargo test
cargo clippy -- -D warnings
echo "edge-domain-vpn bootstrap complete"
