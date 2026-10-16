$ErrorActionPreference = 'Stop'
$workspace = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../../..')).Path
$out = Join-Path $PSScriptRoot 'after-correction'
New-Item -ItemType Directory -Path $out -Force | Out-Null
$destination = Join-Path $out 'A02-960-confirmed'
if (Test-Path -LiteralPath $destination) { throw 'Correction capture already exists' }
$process = Start-Process -FilePath (Join-Path $workspace 'target/debug/project-rl.exe') -ArgumentList @('--ui-cold-agent-skills-technomancien-960', $destination, '--capture-agent-skill=A02') -WorkingDirectory $workspace -WindowStyle Hidden -RedirectStandardOutput (Join-Path $out 'A02-stdout.log') -RedirectStandardError (Join-Path $out 'A02-stderr.log') -PassThru
$null = $process.Handle
if (-not $process.WaitForExit(60000)) { $process.Kill(); throw 'Correction capture timed out' }
$exitCode = $process.ExitCode
$image = Join-Path $destination 'cold-start.png'
if ($null -eq $exitCode -or $exitCode -ne 0 -or -not (Test-Path -LiteralPath $image)) { throw "Correction capture failed: $exitCode" }
ConvertTo-Json -InputObject ([ordered]@{skill='A02'; purpose='Neutral placeholder after withdrawn approval'; process_exit=$exitCode; file=$image; sha256=(Get-FileHash -LiteralPath $image -Algorithm SHA256).Hash.ToLower()}) | Set-Content -LiteralPath (Join-Path $out 'capture-result.json') -Encoding utf8
Write-Output 'Captured A02 after correction: exit 0'
