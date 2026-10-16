$ErrorActionPreference = 'Stop'
$rlRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../../..')).Path
$rlOutput = Join-Path $rlRoot 'artifacts/refonte-agent-2026-10-06/skill-window-native-03'
$rlExecutable = Join-Path $rlRoot 'target/debug/project-rl.exe'
New-Item -ItemType Directory -Path $rlOutput -Force | Out-Null
$rlCaptures = @()
foreach ($rlSkill in @('D56', 'D57', 'D58', 'D66', 'DP01', 'DR01', 'DP02', 'HY07')) {
    $rlDirectory = Join-Path $rlOutput ($rlSkill + '-960')
    if (Test-Path -LiteralPath $rlDirectory) {
        $rlDirectory = Join-Path $rlOutput ($rlSkill + '-960-verified')
        if (Test-Path -LiteralPath $rlDirectory) { throw "Verified capture output already exists: $rlDirectory" }
    }
    New-Item -ItemType Directory -Path $rlDirectory | Out-Null
    $rlProcess = Start-Process -FilePath $rlExecutable -ArgumentList @(
        '--ui-cold-agent-skills-technomancien-960', $rlDirectory, ('--capture-agent-skill=' + $rlSkill)
    ) -WorkingDirectory $rlRoot -WindowStyle Hidden -PassThru `
        -RedirectStandardOutput (Join-Path $rlDirectory 'stdout.log') `
        -RedirectStandardError (Join-Path $rlDirectory 'stderr.log')
    if (-not $rlProcess.WaitForExit(50000)) { throw "Capture timed out for $rlSkill (PID $($rlProcess.Id))" }
    $rlProcess.Refresh()
    if ($rlProcess.ExitCode -ne 0) { throw "Capture failed for $rlSkill (exit $($rlProcess.ExitCode))" }
    $rlImage = Join-Path $rlDirectory 'cold-start.png'
    if (-not (Test-Path -LiteralPath $rlImage)) { throw "Capture missing for $rlSkill" }
    $rlCaptures += [pscustomobject]@{
        skill = $rlSkill
        file = $rlImage
        sha256 = (Get-FileHash -LiteralPath $rlImage -Algorithm SHA256).Hash.ToLowerInvariant()
        process_exit = $rlProcess.ExitCode
    }
    Write-Output "Captured $rlSkill"
}
$rlCaptures | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath (Join-Path $rlOutput 'capture-results.json') -Encoding UTF8
