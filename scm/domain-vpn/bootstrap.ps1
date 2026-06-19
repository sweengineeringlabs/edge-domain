# edge-domain-vpn bootstrap — install toolchain and verify build (Windows)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$ProjectRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $ProjectRoot

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Installing Rust toolchain..."
    Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "$env:TEMP\rustup-init.exe"
    & "$env:TEMP\rustup-init.exe" -y --no-modify-path
    $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
}

cargo build
cargo test
cargo clippy -- -D warnings
Write-Host "edge-domain-vpn bootstrap complete"
