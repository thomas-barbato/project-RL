$ErrorActionPreference = 'Stop'
$workspace = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../../..')).Path
$destination = Join-Path $PSScriptRoot 'SM00-960'
if (Test-Path -LiteralPath $destination) { throw 'Capture already exists.' }
$process = Start-Process -FilePath (Join-Path $workspace 'target/debug/project-rl.exe') -ArgumentList @('--ui-cold-agent-skills-technomancien-960', $destination, '--capture-agent-skill=SM00') -WorkingDirectory $workspace -WindowStyle Hidden -RedirectStandardOutput (Join-Path $PSScriptRoot 'stdout.log') -RedirectStandardError (Join-Path $PSScriptRoot 'stderr.log') -PassThru
$null=$process.Handle
if (-not $process.WaitForExit(60000)) { $process.Kill(); throw 'Capture timed out.' }
$image=Join-Path $destination 'cold-start.png'
if ($null -eq $process.ExitCode -or $process.ExitCode -ne 0 -or -not (Test-Path -LiteralPath $image)) { throw 'Capture failed.' }
ConvertTo-Json -InputObject @([ordered]@{skill='SM00';file=$image;process_exit=$process.ExitCode;sha256=(Get-FileHash -LiteralPath $image -Algorithm SHA256).Hash.ToLower()}) | Set-Content -LiteralPath (Join-Path $PSScriptRoot 'capture-result.json') -Encoding utf8
Write-Output 'Captured mine branch: exit 0'
