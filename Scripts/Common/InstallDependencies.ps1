$ErrorActionPreference = "Stop"

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..\..")
& (Join-Path $ScriptRoot "..\Other\StopGame.ps1")
Set-Location $ProjectRoot

function Add-CargoToPathIfPresent {
    $CargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
    if ((Test-Path $CargoBin) -and (-not (($env:Path -split ";") -contains $CargoBin))) {
        $env:Path = "$CargoBin;$env:Path"
    }
}

function Ensure-RustToolchain {
    Add-CargoToPathIfPresent

    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        Write-Host "Rust is already installed."
        return
    }

    if (-not (Get-Command winget -ErrorAction SilentlyContinue)) {
        throw "Rust is not installed and winget is unavailable. Install rustup from https://rustup.rs/ and rerun this script."
    }

    Write-Host "Installing Rust via rustup..."
    winget install --id Rustlang.Rustup -e --accept-package-agreements --accept-source-agreements

    Add-CargoToPathIfPresent

    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        throw "Rust installation finished, but cargo is not available in this PowerShell session yet. Open a new terminal and rerun this script."
    }
}

function Ensure-DioxusCli {
    $RequiredDxVersion = "0.7.6"
    $DxCommand = Get-Command dx -ErrorAction SilentlyContinue

    if ($DxCommand) {
        $DxVersionOutput = (& dx --version | Out-String).Trim()
        if ($DxVersionOutput -match "^dioxus\s+0\.7(\.|-|$)") {
            Write-Host "Dioxus CLI is already installed and compatible ($DxVersionOutput)."
            return
        }

        Write-Host "Dioxus CLI is installed but not on a known-compatible 0.7 version: $DxVersionOutput"
        Write-Host "Reinstalling Dioxus CLI $RequiredDxVersion for hot reload compatibility..."
        cargo install dioxus-cli@$RequiredDxVersion --force
        return
    }

    Write-Host "Installing Dioxus CLI $RequiredDxVersion for hot reload..."
    cargo install dioxus-cli@$RequiredDxVersion
}

function Ensure-Sccache {
    if (Get-Command sccache -ErrorAction SilentlyContinue) {
        Write-Host "sccache is already installed."
        return
    }

    Write-Host "Installing sccache for faster repeated Rust builds..."
    cargo install --locked sccache

    if (Get-Command sccache -ErrorAction SilentlyContinue) {
        Write-Host "sccache installation complete."
        return
    }

    Write-Warning "sccache latest failed to install; retrying with 0.9.1 for toolchain compatibility..."
    cargo install --locked sccache@0.9.1

    if (Get-Command sccache -ErrorAction SilentlyContinue) {
        Write-Host "sccache installation complete (0.9.1)."
        return
    }

    Write-Warning "sccache installation failed; builds will continue without compiler caching."
}

Ensure-RustToolchain
Ensure-DioxusCli
Ensure-Sccache

Write-Host ""
Write-Host "Dependency install complete."
