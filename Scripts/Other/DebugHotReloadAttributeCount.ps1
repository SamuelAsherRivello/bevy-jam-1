$ErrorActionPreference = "Stop"

$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Resolve-Path (Join-Path $ScriptRoot "..\..")
Push-Location $ProjectRoot

try {

# Ignore list: any file path containing one of these fragments will be skipped.
$ignorePathFragments = @(
    "Bevy\Crates\Shared\Runtime\3rdParty\"
)

# Match #[hot] and #[hot(...)] attributes.
$hotAttributePattern = '#\[\s*hot\s*(\]|\()'

$ripgrepCommand = Get-Command rg -ErrorAction SilentlyContinue
if ($ripgrepCommand) {
    $includeGlob = "Bevy/**/*.rs"
    $excludeGlob = "Bevy/Crates/Shared/Runtime/3rdParty/**"
    $rgArgs = @(
        "--line-number",
        "--no-heading",
        "--glob", $includeGlob,
        "--glob", "!$excludeGlob",
        $hotAttributePattern,
        "Bevy"
    )

    $rgMatches = & rg @rgArgs
    $rgExitCode = $LASTEXITCODE

    if ($rgExitCode -ne 0 -and $rgExitCode -ne 1) {
        throw "ripgrep failed with exit code $rgExitCode"
    }

    $matchesByFile = @{}
    foreach ($line in $rgMatches) {
        if ([string]::IsNullOrWhiteSpace($line)) {
            continue
        }

        $firstColonIndex = $line.IndexOf(":")
        if ($firstColonIndex -le 0) {
            continue
        }

        $relativePath = $line.Substring(0, $firstColonIndex)
        if ($matchesByFile.ContainsKey($relativePath)) {
            $matchesByFile[$relativePath] += 1
        } else {
            $matchesByFile[$relativePath] = 1
        }
    }

    $totalCount = 0
    foreach ($count in $matchesByFile.Values) {
        $totalCount += $count
    }

    Write-Host "Total hot reload attributes: $totalCount"
    Write-Host ""
    Write-Host "Files:"

    foreach ($path in ($matchesByFile.Keys | Sort-Object)) {
        Write-Host $path
    }

    return
}

$rustFiles = Get-ChildItem -Path "Bevy" -Recurse -File -Filter "*.rs"
$matchesByFile = @{}
$totalCount = 0

foreach ($file in $rustFiles) {
    $relativePath = $file.FullName.Replace($ProjectRoot.Path + [System.IO.Path]::DirectorySeparatorChar, "")

    $isIgnored = $false
    foreach ($fragment in $ignorePathFragments) {
        if ($relativePath.Contains($fragment)) {
            $isIgnored = $true
            break
        }
    }

    if ($isIgnored) {
        continue
    }

    $matches = Select-String -Path $file.FullName -Pattern $hotAttributePattern -AllMatches
    if (-not $matches) {
        continue
    }

    $fileCount = 0
    foreach ($lineMatch in $matches) {
        $fileCount += $lineMatch.Matches.Count
    }

    if ($fileCount -gt 0) {
        $matchesByFile[$relativePath] = $fileCount
        $totalCount += $fileCount
    }
}

Write-Host "Total hot reload attributes: $totalCount"
Write-Host ""
Write-Host "Files:"

foreach ($path in ($matchesByFile.Keys | Sort-Object)) {
    Write-Host $path
}
} finally {
    Pop-Location
}
