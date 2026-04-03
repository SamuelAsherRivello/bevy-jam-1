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

Write-Host "Building shared crate..."
cargo build -p shared
