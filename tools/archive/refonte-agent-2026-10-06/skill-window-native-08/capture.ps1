$ErrorActionPreference = 'Stop'
$workspace = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../../..')).Path
$executable = Join-Path $workspace 'target/debug/project-rl.exe'
$candidates = @(
    @{ skill = 'ES03'; variant = 'ES03-C01'; number = 46 },
    @{ skill = 'ES05'; variant = 'ES05-C03'; number = 47 },
    @{ skill = 'ES06'; variant = 'ES06-C02'; number = 48 },
    @{ skill = 'T02'; variant = 'T02-C04'; number = 49 }
)
$results = @()
foreach ($candidate in $candidates) {
    $destination = Join-Path $PSScriptRoot ($candidate.skill + '-960')
    if (Test-Path -LiteralPath $destination) { throw "Capture destination already exists: $destination" }
    $stdout = Join-Path $PSScriptRoot ($candidate.skill + '-stdout.log')
    $stderr = Join-Path $PSScriptRoot ($candidate.skill + '-stderr.log')
    $timer = [System.Diagnostics.Stopwatch]::StartNew()
    $process = Start-Process -FilePath $executable -ArgumentList @('--ui-cold-agent-skills-technomancien-960', $destination, ('--capture-agent-skill=' + $candidate.skill)) -WorkingDirectory $workspace -WindowStyle Hidden -RedirectStandardOutput $stdout -RedirectStandardError $stderr -PassThru
    if (-not $process.WaitForExit(120000)) { $process.Kill(); throw "Capture timed out: $($candidate.skill)" }
    $process.Refresh()
    $image = Join-Path $destination 'cold-start.png'
    if ($process.ExitCode -ne 0 -or -not (Test-Path -LiteralPath $image)) { throw "Capture failed: $($candidate.skill), exit $($process.ExitCode)" }
    $results += [ordered]@{ skill = $candidate.skill; variant = $candidate.variant; number = $candidate.number; file = $image; sha256 = (Get-FileHash -LiteralPath $image -Algorithm SHA256).Hash.ToLower(); process_exit = $process.ExitCode; elapsed_seconds = [Math]::Round($timer.Elapsed.TotalSeconds, 2) }
    Write-Output "Captured $($candidate.skill): exit $($process.ExitCode)"
}
ConvertTo-Json -InputObject @($results) -Depth 4 | Set-Content -LiteralPath (Join-Path $PSScriptRoot 'capture-results.json') -Encoding utf8
