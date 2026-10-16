param([switch]$Before)
$ErrorActionPreference = 'Stop'
$workspace = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../../..')).Path
$skills = if ($Before) { @('FE01') } else { @('FE01', 'HL04') }
$results = @()
foreach ($skill in $skills) {
    $name = if ($Before) { $skill + '-before-960' } else { $skill + '-960' }
    $destination = Join-Path $PSScriptRoot $name
    if (Test-Path -LiteralPath $destination) { throw "Capture destination already exists: $destination" }
    $process = Start-Process -FilePath (Join-Path $workspace 'target/debug/project-rl.exe') -ArgumentList @('--ui-cold-agent-skills-technomancien-960', $destination, ('--capture-agent-skill=' + $skill)) -WorkingDirectory $workspace -WindowStyle Hidden -RedirectStandardOutput (Join-Path $PSScriptRoot ($name + '-stdout.log')) -RedirectStandardError (Join-Path $PSScriptRoot ($name + '-stderr.log')) -PassThru
    $null = $process.Handle
    if (-not $process.WaitForExit(60000)) { $process.Kill(); throw "Capture timed out: $skill" }
    $image = Join-Path $destination 'cold-start.png'
    if ($null -eq $process.ExitCode -or $process.ExitCode -ne 0 -or -not (Test-Path -LiteralPath $image)) { throw "Capture failed: $skill" }
    $results += [ordered]@{skill=$skill;file=$image;process_exit=$process.ExitCode;sha256=(Get-FileHash -LiteralPath $image -Algorithm SHA256).Hash.ToLower()}
    Write-Output "Captured $name : exit 0"
}
$receipt = if ($Before) { 'capture-before-result.json' } else { 'capture-result.json' }
ConvertTo-Json -InputObject $results | Set-Content -LiteralPath (Join-Path $PSScriptRoot $receipt) -Encoding utf8
