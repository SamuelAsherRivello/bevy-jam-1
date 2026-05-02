$ErrorActionPreference = "Stop"

$FastHotReloadFeature = "fast-hot-reload"
$EnableFastHotReloadFeature = $true
$IsWindowsHost = $env:OS -eq "Windows_NT"

# On Windows/MSVC, Bevy dynamic linking can launch successfully but Dioxus'
# hot-patch linker can fail to resolve dllimport symbols from Bevy, std, log,
# and tracing. Keep the hot-patch path compatible.
if ($IsWindowsHost) {
    $EnableFastHotReloadFeature = $false
}

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..\..")
$HotReloadSourcePath = Join-Path $ProjectRoot "Bevy\Crates\Shared\Runtime\3rdParty\bevy_simple_subsecond_system"
Write-Host "This script uses local bevy_simple_subsecond_system hot reload from: $HotReloadSourcePath"
& (Join-Path $ScriptRoot "..\Other\StopGame.ps1")
Set-Location $ProjectRoot

$env:CARGO_INCREMENTAL = "1"
$env:CARGO_TARGET_DIR = (Join-Path $ProjectRoot "target")
if (-not $env:CARGO_BUILD_JOBS) {
    $env:CARGO_BUILD_JOBS = [Environment]::ProcessorCount
}

if (-not $EnableFastHotReloadFeature) {
    $DesktopDevProfileRoots = @(
        (Join-Path $ProjectRoot "target\desktop-dev"),
        (Join-Path $ProjectRoot "target\x86_64-pc-windows-msvc\desktop-dev")
    )

    $HasDynamicLinkingArtifacts = $false
    foreach ($ProfileRoot in $DesktopDevProfileRoots) {
        if (-not (Test-Path $ProfileRoot)) {
            continue
        }

        if (Get-ChildItem -Path $ProfileRoot -Filter "bevy_dylib*.dll" -Recurse -ErrorAction SilentlyContinue) {
            $HasDynamicLinkingArtifacts = $true
            break
        }
    }

    if ($HasDynamicLinkingArtifacts) {
        foreach ($ProfileRoot in $DesktopDevProfileRoots) {
            if (-not (Test-Path $ProfileRoot)) {
                continue
            }

            $ResolvedProfileRoot = Resolve-Path -LiteralPath $ProfileRoot
            if (-not $ResolvedProfileRoot.Path.StartsWith($ProjectRoot.Path, [StringComparison]::OrdinalIgnoreCase)) {
                throw "Refusing to clean desktop-dev artifacts outside project root: $ResolvedProfileRoot"
            }

            Remove-Item -LiteralPath $ResolvedProfileRoot.Path -Recurse -Force
            Write-Host "[desktop-dev profile cleaned] $($ResolvedProfileRoot.Path)"
        }
    }
}

if (Get-Command sccache -ErrorAction SilentlyContinue) {
    # Skip sccache when CARGO_INCREMENTAL is enabled to avoid compatibility issues
    Write-Host "sccache detected but CARGO_INCREMENTAL is set: skipping compiler cache."
} else {
    Write-Host "No sccache detected."
}

$env:WGPU_BACKEND = "dx12"
$env:BEVY_ASSET_ROOT = "$ProjectRoot"

# Remove stale desktop-dev artifacts that can make dx fat-binary linking fail.
$DesktopDevDepsRoots = @(
    (Join-Path $ProjectRoot "target\desktop-dev\deps"),
    (Join-Path $ProjectRoot "target\x86_64-pc-windows-msvc\desktop-dev\deps")
)

foreach ($DepsRoot in $DesktopDevDepsRoots) {
    if (-not (Test-Path $DepsRoot)) {
        continue
    }

    Get-ChildItem -Path $DepsRoot -Force -ErrorAction SilentlyContinue |
        Where-Object {
            $_.Name -like "rustc*" -or
            $_.Name -like "game.*"
        } |
        ForEach-Object {
            try {
                Remove-Item -LiteralPath $_.FullName -Recurse -Force -ErrorAction Stop
            } catch {
                Write-Host "[cleanup skipped] $($_.FullName)"
            }
        }
}

if (-not (Get-Command dx -ErrorAction SilentlyContinue)) {
    throw "Dioxus CLI is required for subsecond hot reload. Install it with: cargo install dioxus-cli@0.7.6"
}

$dxVersionOutput = (& dx --version | Out-String).Trim()
# Accept all 0.7.x releases (including pre-releases) which are known to support --hot-patch.
$supportsHotPatch = $dxVersionOutput -match "0\.7(\.|-|$)"

Write-Host ""
if ($supportsHotPatch) {
    Write-Host "Starting game subsecond hot reload."
} else {
    Write-Warning "Detected '$dxVersionOutput'."
    Write-Warning "This dx major/minor version is not known to work with local subsecond hot patching in this template."
    Write-Host "Install a compatible 0.7 release for hot patching: cargo install dioxus-cli@0.7.6 --force"
    throw "Hot reload requires Dioxus CLI 0.7.x with --hot-patch support."
}
Write-Host "Edit hot-annotated systems under Bevy\Crates\Game\Runtime and save."
if ($EnableFastHotReloadFeature) {
    Write-Host "Using game feature: $FastHotReloadFeature"
} else {
    Write-Host "Hot-patch compatibility mode: running without '$FastHotReloadFeature'."
}
Write-Host "Press Ctrl+C to stop."
Write-Host ""

$dxArgs = @("serve", "--hot-patch", "--windows", "--package", "game", "--bin", "game")
if ($EnableFastHotReloadFeature) {
    $dxArgs += @("--features", $FastHotReloadFeature)
}

& dx @dxArgs
$dxExitCode = $LASTEXITCODE
if ($dxExitCode -eq 0) {
    return
}

# dx may report non-zero when interrupted (Ctrl+C, terminal stop/timeout).
# In those cases, do not start a fallback cargo run.
$InterruptedExitCodes = @(-1, 130, 3221225786)
if ($InterruptedExitCodes -contains $dxExitCode) {
    Write-Host "dx serve stopped (exit code $dxExitCode)."
    return
}

throw "dx serve hot reload failed with exit code $dxExitCode."
