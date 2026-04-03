$ErrorActionPreference = "Stop"

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..\..")
& (Join-Path $ScriptRoot "StopGame.ps1")
Set-Location $ProjectRoot

$env:CARGO_INCREMENTAL = "0"
if (-not $env:CARGO_BUILD_JOBS) {
	$env:CARGO_BUILD_JOBS = [Environment]::ProcessorCount
}

if (Get-Command sccache -ErrorAction SilentlyContinue) {
	Write-Host "sccache detected: using compiler cache."
} else {
	Write-Host "No sccache detected."
}

Write-Host "Running shared crate tests..."
cargo test -p shared
