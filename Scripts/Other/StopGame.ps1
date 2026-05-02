$ErrorActionPreference = "Stop"

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..\..")
$TargetRoot = Join-Path $ProjectRoot "target"
$ProjectRootPath = $ProjectRoot.Path

$ProcessNames = @("game", "dx", "trunk", "cargo", "rustc", "rustdoc", "link")
$StoppedCount = 0
$StoppedProcessIds = @()

foreach ($ProcessName in $ProcessNames) {
    $Processes = Get-Process -Name $ProcessName -ErrorAction SilentlyContinue
    foreach ($Process in $Processes) {
        try {
            Write-Host "Stopping $($Process.ProcessName) process $($Process.Id)..."
            Stop-Process -Id $Process.Id -Force -ErrorAction Stop
            $StoppedProcessIds += $Process.Id
            $StoppedCount += 1
        } catch {
            Write-Host "Process $($Process.Id) exited before it could be stopped."
        }
    }
}

# Stop shells that are actively running the hot-reload scripts/commands so they do not respawn child processes.
$HostProcessNames = @("powershell.exe", "pwsh.exe")
$HostCommandLinePatterns = @(
    "RunGameDesktopWithHotReload.ps1",
    "RunGameWeb.ps1",
    "RunProjectWithHotReload.ps1",
    "dx serve"
)

foreach ($HostProcessName in $HostProcessNames) {
    $HostProcesses = Get-CimInstance Win32_Process -Filter "Name = '$HostProcessName'" -ErrorAction SilentlyContinue
    foreach ($HostProcess in $HostProcesses) {
        if ($HostProcess.ProcessId -eq $PID) {
            continue
        }

        $CommandLine = [string]$HostProcess.CommandLine
        if ([string]::IsNullOrWhiteSpace($CommandLine)) {
            continue
        }

        $ShouldStopHostProcess = $false
        foreach ($Pattern in $HostCommandLinePatterns) {
            if ($CommandLine -like "*$Pattern*") {
                $ShouldStopHostProcess = $true
                break
            }
        }

        if (-not $ShouldStopHostProcess) {
            continue
        }

        try {
            Write-Host "Stopping host shell process $($HostProcess.ProcessId) running hot reload..."
            Stop-Process -Id $HostProcess.ProcessId -Force -ErrorAction Stop
            $StoppedProcessIds += $HostProcess.ProcessId
            $StoppedCount += 1
        } catch {
            Write-Host "Host shell process $($HostProcess.ProcessId) exited before it could be stopped."
        }
    }
}

if ($StoppedCount -eq 0) {
    Write-Host "No running game processes found."
} else {
    foreach ($ProcessId in $StoppedProcessIds) {
        try {
            Wait-Process -Id $ProcessId -Timeout 10 -ErrorAction SilentlyContinue
        } catch {
            Write-Host "Process $ProcessId did not report a clean exit before timeout."
        }
    }

    Write-Host "Stopped $StoppedCount game process(es)."
}

if (Test-Path $TargetRoot) {
    $RemovedCargoLockCount = 0
    $FailedCargoLockCount = 0
    $CargoLockFiles = @()
    $KnownBuildRoots = @(
        (Join-Path $TargetRoot "debug"),
        (Join-Path $TargetRoot "desktop-dev"),
        (Join-Path $TargetRoot "wasm-dev"),
        (Join-Path $TargetRoot "x86_64-pc-windows-msvc\debug"),
        (Join-Path $TargetRoot "x86_64-pc-windows-msvc\desktop-dev"),
        (Join-Path $TargetRoot "wasm32-unknown-unknown\debug")
    )

    foreach ($BuildRoot in $KnownBuildRoots) {
        if (-not (Test-Path $BuildRoot)) {
            continue
        }

        $CargoLockFiles += Get-ChildItem -Path $BuildRoot -Recurse -Force -Filter ".cargo-lock" -ErrorAction SilentlyContinue
    }

    foreach ($CargoLockFile in $CargoLockFiles) {
        $RelativeLockPath = $CargoLockFile.FullName.Replace("$ProjectRootPath\", "")
        try {
            Remove-Item -LiteralPath $CargoLockFile.FullName -Force
            $RemovedCargoLockCount += 1
            Write-Host "[lock removed] $RelativeLockPath"
        } catch {
            $FailedCargoLockCount += 1
            Write-Host "[lock remove failed] $RelativeLockPath"
        }
    }

    if ($RemovedCargoLockCount -gt 0 -or $FailedCargoLockCount -gt 0) {
        Write-Host "Cargo lock cleanup summary: removed=$RemovedCargoLockCount, failed=$FailedCargoLockCount"
    }

    # Clean stale hot-reload game artifacts that can survive forced process stops and cause linker failures.
    $DesktopDevArtifactRoots = @(
        (Join-Path $TargetRoot "desktop-dev\deps"),
        (Join-Path $TargetRoot "x86_64-pc-windows-msvc\desktop-dev\deps")
    )

    $RemovedDesktopDevArtifactsCount = 0
    $FailedDesktopDevArtifactsCount = 0

    foreach ($ArtifactRoot in $DesktopDevArtifactRoots) {
        if (-not (Test-Path $ArtifactRoot)) {
            continue
        }

        $StaleGameArtifacts = Get-ChildItem -Path $ArtifactRoot -Force -ErrorAction SilentlyContinue |
            Where-Object { $_.Name -like "game.*" }

        foreach ($Artifact in $StaleGameArtifacts) {
            $RelativeArtifactPath = $Artifact.FullName.Replace("$ProjectRootPath\", "")
            try {
                Remove-Item -LiteralPath $Artifact.FullName -Force
                $RemovedDesktopDevArtifactsCount += 1
            } catch {
                $FailedDesktopDevArtifactsCount += 1
                Write-Host "[desktop-dev artifact remove failed] $RelativeArtifactPath"
            }
        }
    }

    if ($RemovedDesktopDevArtifactsCount -gt 0 -or $FailedDesktopDevArtifactsCount -gt 0) {
        Write-Host "Desktop-dev artifact cleanup summary: removed=$RemovedDesktopDevArtifactsCount, failed=$FailedDesktopDevArtifactsCount"
    }
}
