$ErrorActionPreference = "Stop"
Write-Host "This script serves the game in browser/wasm."

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

$env:BEVY_ASSET_ROOT = "$ProjectRoot"

if (-not (Get-Command dx -ErrorAction SilentlyContinue)) {
    throw "Dioxus CLI is required for browser serving. Install it with: cargo install dioxus-cli@0.7.6"
}

Write-Host ""
Write-Host "Starting game browser/wasm serve."
Write-Host "Running without hot reload."
Write-Host "Press Ctrl+C to stop."
Write-Host ""

$WebAddress = "127.0.0.1"
$WebPort = "8080"

# Ensure the interactive dashboard has enough columns to render the full address.
try {
    $MinimumColumns = 140
    $RawUi = $Host.UI.RawUI
    if ($RawUi.BufferSize.Width -lt $MinimumColumns) {
        $BufferSize = $RawUi.BufferSize
        $BufferSize.Width = $MinimumColumns
        $RawUi.BufferSize = $BufferSize
    }
    if ($RawUi.WindowSize.Width -lt $MinimumColumns) {
        $WindowSize = $RawUi.WindowSize
        $WindowSize.Width = $MinimumColumns
        $RawUi.WindowSize = $WindowSize
    }
} catch {
    # Some hosts do not allow resizing; continue without failing.
}

# Use the web platform so the app is built for wasm and served in the browser.
dx serve --platform web --addr $WebAddress --port $WebPort --open true --package game --bin game
