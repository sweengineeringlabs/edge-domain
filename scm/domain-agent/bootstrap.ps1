# edge-domain-agent bootstrap script
# Sets up the development environment for the agent domain primitive crate

Write-Host "=== edge-domain-agent bootstrap ===" -ForegroundColor Green

Write-Host "Installing Rust toolchain..." -ForegroundColor Cyan
rustup update

Write-Host "Checking dependencies..." -ForegroundColor Cyan
cargo check

Write-Host "Running tests..." -ForegroundColor Cyan
cargo test

Write-Host "=== Bootstrap complete ===" -ForegroundColor Green
