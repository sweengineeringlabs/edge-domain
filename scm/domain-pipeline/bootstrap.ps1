# Bootstrap script for edge-domain-pipeline on Windows
# Purpose: Set up development environment

param(
    [switch]$Test,
    [switch]$Lint,
    [switch]$Doc,
    [switch]$All
)

$ErrorActionPreference = "Stop"

function Test-Command {
    param([string]$Command)
    $null = Get-Command $Command -ErrorAction SilentlyContinue
    return $?
}

function Invoke-Build {
    Write-Host "Building domain-pipeline..." -ForegroundColor Cyan
    cargo build --all-features
    if ($LASTEXITCODE -ne 0) { exit 1 }
}

function Invoke-Test {
    Write-Host "Running tests..." -ForegroundColor Cyan
    cargo test --all-features
    if ($LASTEXITCODE -ne 0) { exit 1 }
}

function Invoke-Lint {
    Write-Host "Running clippy..." -ForegroundColor Cyan
    cargo clippy --all-features -- -D warnings
    if ($LASTEXITCODE -ne 0) { exit 1 }
}

function Invoke-Doc {
    Write-Host "Building documentation..." -ForegroundColor Cyan
    cargo doc --all-features --no-deps --open
    if ($LASTEXITCODE -ne 0) { exit 1 }
}

if (-not (Test-Command "cargo")) {
    Write-Error "Rust toolchain not found. Please install from https://rustup.rs"
    exit 1
}

Write-Host "edge-domain-pipeline bootstrap" -ForegroundColor Green

if ($All) {
    Invoke-Build
    Invoke-Lint
    Invoke-Test
    Invoke-Doc
} else {
    Invoke-Build
    if ($Test) { Invoke-Test }
    if ($Lint) { Invoke-Lint }
    if ($Doc) { Invoke-Doc }
}

Write-Host "✓ Bootstrap complete" -ForegroundColor Green
