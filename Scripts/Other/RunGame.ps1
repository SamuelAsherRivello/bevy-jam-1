$ErrorActionPreference = "Stop"

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..\..")
& (Join-Path $ScriptRoot "StopGame.ps1")
Set-Location $ProjectRoot

$env:CARGO_INCREMENTAL = "1"
if (-not $env:CARGO_BUILD_JOBS) {
	$env:CARGO_BUILD_JOBS = [Environment]::ProcessorCount
}

if (Get-Command sccache -ErrorAction SilentlyContinue) {
	Write-Host "sccache detected but CARGO_INCREMENTAL is set: skipping compiler cache."
} else {
	Write-Host "No sccache detected."
}

$env:WGPU_BACKEND = "dx12"

Write-Host "Building game..."
cargo build -p game

Write-Host ""
Write-Host "Starting Windows game build."
Write-Host "Running without hot reload."
Write-Host "Press Ctrl+C to stop."
Write-Host ""

cargo run -p game
