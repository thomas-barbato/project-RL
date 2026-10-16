$ErrorActionPreference = 'Stop'
$workspace = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../../..')).Path
$destination = Join-Path $PSScriptRoot 'M03-960'
if (Test-Path -LiteralPath $destination) { throw 'Capture destination already exists' }
$process = Start-Process -FilePath (Join-Path $workspace 'target/debug/project-rl.exe') -ArgumentList @('--ui-cold-agent-skills-technomancien-960', $destination, '--capture-agent-skill=M03') -WorkingDirectory $workspace -WindowStyle Hidden -RedirectStandardOutput (Join-Path $PSScriptRoot 'capture-stdout.log') -RedirectStandardError (Join-Path $PSScriptRoot 'capture-stderr.log') -PassThru
$null = $process.Handle
if (-not $process.WaitForExit(60000)) { $process.Kill(); throw 'Capture timed out' }
$exitCode = $process.ExitCode
$image = Join-Path $destination 'cold-start.png'
if ($null -eq $exitCode -or $exitCode -ne 0 -or -not (Test-Path -LiteralPath $image)) { throw "Capture failed: $exitCode" }
ConvertTo-Json -InputObject ([ordered]@{skill='M03'; variant='M03-C05'; number=8; file=$image; process_exit=$exitCode; sha256=(Get-FileHash -LiteralPath $image -Algorithm SHA256).Hash.ToLower()}) | Set-Content -LiteralPath (Join-Path $PSScriptRoot 'capture-result.json') -Encoding utf8
Write-Output 'Captured M03-C05: exit 0'
