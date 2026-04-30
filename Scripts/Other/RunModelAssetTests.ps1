param(
    [string[]]$Filter = @()
)

$ErrorActionPreference = "Stop"
if (Get-Variable -Name PSNativeCommandUseErrorActionPreference -ErrorAction SilentlyContinue) {
    $PSNativeCommandUseErrorActionPreference = $false
}

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..\..")
Set-Location $ProjectRoot

$env:CARGO_INCREMENTAL = "1"
if (-not $env:CARGO_BUILD_JOBS) {
    $env:CARGO_BUILD_JOBS = [Environment]::ProcessorCount
}

# Keep model validation builds away from the running app's normal target output.
$env:CARGO_TARGET_DIR = Join-Path $ProjectRoot "target\model-asset-tests"

if (Get-Command sccache -ErrorAction SilentlyContinue) {
    Write-Host "sccache detected but CARGO_INCREMENTAL is set: skipping compiler cache."
} else {
    Write-Host "No sccache detected."
}

Write-Host "Running model asset tests..."
Write-Host "Cargo target dir: $env:CARGO_TARGET_DIR"

function Invoke-CargoModelAssetTest {
    param(
        [string[]]$CargoArguments
    )

    $PreviousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    & cargo @CargoArguments
    $CargoExitCode = $LASTEXITCODE
    $ErrorActionPreference = $PreviousErrorActionPreference

    return $CargoExitCode
}

if ($Filter.Count -eq 0) {
    $ExitCode = Invoke-CargoModelAssetTest -CargoArguments @("test", "-p", "game", "--test", "ModelAssetTests", "--", "--nocapture")
    exit $ExitCode
}

foreach ($TestFilter in $Filter) {
    Write-Host "Running model asset test filter: $TestFilter"
    $ExitCode = Invoke-CargoModelAssetTest -CargoArguments @("test", "-p", "game", "--test", "ModelAssetTests", $TestFilter, "--", "--nocapture")
    if ($ExitCode -ne 0) {
        exit $ExitCode
    }
}
