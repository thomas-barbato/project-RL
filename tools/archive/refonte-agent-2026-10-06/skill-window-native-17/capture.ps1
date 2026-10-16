$ErrorActionPreference = 'Stop'
$workspace = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../../..')).Path
$results = @()
foreach ($skill in @('AF06', 'B01', 'BD04', 'BM01')) {
    $destination = Join-Path $PSScriptRoot ($skill + '-960')
    if (Test-Path -LiteralPath $destination) { throw "Capture destination already exists: $destination" }
    $process = Start-Process -FilePath (Join-Path $workspace 'target/debug/project-rl.exe') -ArgumentList @('--ui-cold-agent-skills-technomancien-960', $destination, ('--capture-agent-skill=' + $skill)) -WorkingDirectory $workspace -WindowStyle Hidden -RedirectStandardOutput (Join-Path $PSScriptRoot ($skill + '-stdout.log')) -RedirectStandardError (Join-Path $PSScriptRoot ($skill + '-stderr.log')) -PassThru
    $null = $process.Handle
    if (-not $process.WaitForExit(60000)) { $process.Kill(); throw "Capture timed out: $skill" }
    $exitCode = $process.ExitCode
    $image = Join-Path $destination 'cold-start.png'
    if ($null -eq $exitCode -or $exitCode -ne 0 -or -not (Test-Path -LiteralPath $image)) { throw "Capture failed for $skill : $exitCode" }
    $results += [ordered]@{skill=$skill;file=$image;process_exit=$exitCode;sha256=(Get-FileHash -LiteralPath $image -Algorithm SHA256).Hash.ToLower()}
    Write-Output "Captured $skill : exit 0"
}
ConvertTo-Json -InputObject $results | Set-Content -LiteralPath (Join-Path $PSScriptRoot 'capture-result.json') -Encoding utf8
