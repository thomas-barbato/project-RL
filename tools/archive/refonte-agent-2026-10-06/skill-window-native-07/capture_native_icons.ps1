$ErrorActionPreference = 'Stop'
$captureRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../../..')).Path
$reviewPath = Join-Path $captureRoot 'artifacts/refonte-agent-2026-10-06/skill-icon-review-23/review.json'
$captureReview = Get-Content -LiteralPath $reviewPath -Raw | ConvertFrom-Json
$captureResults = @()
foreach ($candidate in $captureReview.candidates) {
    $captureDir = Join-Path $PSScriptRoot ($candidate.id + '-960')
    if (Test-Path -LiteralPath $captureDir) {
        throw "Capture directory already exists: $captureDir"
    }
    $captureLogs = Join-Path $PSScriptRoot ($candidate.id + '-logs')
    New-Item -ItemType Directory -Path $captureLogs | Out-Null
    $captureArguments = @('--ui-cold-agent-skills-technomancien-960', $captureDir, ('--capture-agent-skill=' + $candidate.id))
    $captureTimer = [Diagnostics.Stopwatch]::StartNew()
    $captureProcess = Start-Process -FilePath (Join-Path $captureRoot 'target/debug/project-rl.exe') -ArgumentList $captureArguments -WorkingDirectory $captureRoot -WindowStyle Hidden -PassThru -RedirectStandardOutput (Join-Path $captureLogs 'stdout.log') -RedirectStandardError (Join-Path $captureLogs 'stderr.log')
    $null = $captureProcess.Handle
    if (-not $captureProcess.WaitForExit(30000)) {
        $captureProcess.Kill()
        throw "Capture timed out for $($candidate.id)"
    }
    $captureProcess.WaitForExit()
    $captureTimer.Stop()
    $capturePng = Join-Path $captureDir 'cold-start.png'
    if ($captureProcess.ExitCode -ne 0 -or -not (Test-Path -LiteralPath $capturePng)) {
        throw "Capture failed for $($candidate.id); see $captureLogs"
    }
    $captureResults += [pscustomobject]@{
        skill = $candidate.id
        variant = $candidate.variant
        number = $candidate.number
        file = $capturePng
        sha256 = (Get-FileHash -LiteralPath $capturePng -Algorithm SHA256).Hash.ToLowerInvariant()
        process_exit = $captureProcess.ExitCode
        elapsed_seconds = [Math]::Round($captureTimer.Elapsed.TotalSeconds, 2)
    }
    ConvertTo-Json -InputObject @($captureResults) -Depth 6 | Set-Content -LiteralPath (Join-Path $PSScriptRoot 'capture-results.json') -Encoding utf8
    Write-Output ("Captured {0}/5: {1} in {2}s" -f $captureResults.Count, $candidate.id, [Math]::Round($captureTimer.Elapsed.TotalSeconds, 2))
}
