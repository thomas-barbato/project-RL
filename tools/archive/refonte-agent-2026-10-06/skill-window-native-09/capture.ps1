$ErrorActionPreference = 'Stop'
$workspace = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../../..')).Path
$executable = Join-Path $workspace 'target/debug/project-rl.exe'
$candidates = @(
    @{ skill = 'FE02'; variant = 'FE02-C01'; number = 50 },
    @{ skill = 'FE03'; variant = 'FE03-C01'; number = 51 },
    @{ skill = 'FE04'; variant = 'FE04-C01'; number = 52 },
    @{ skill = 'FE05'; variant = 'FE05-C03'; number = 17 },
    @{ skill = 'FE06'; variant = 'FE06-C01'; number = 53 }
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
