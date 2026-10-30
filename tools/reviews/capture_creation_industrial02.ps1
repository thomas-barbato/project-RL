param(
    [ValidatePattern('^r[0-9]{2}$')][string] $Revision = 'r03',
    [switch] $Quick,
    [switch] $Integrated
)
$ErrorActionPreference = 'Stop'
$reviewWorkspace = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../..')).Path
Push-Location -LiteralPath $reviewWorkspace
try {
    $reviewOut = Join-Path $reviewWorkspace "artifacts/character-creation-review-02/current/industrial-02D/$Revision"
    if ($Integrated) {
        $reviewOut = Join-Path $reviewWorkspace 'artifacts/character-creation-review-02/integration-02D-r03'
    }
    $reviewCases = @(
        @('metamorphe', '--ui-cold-character-creation-industrial-skill', 'metamorphe'),
        @('arcaniste-small', '--ui-cold-character-creation-industrial-skill-small', 'arcaniste'),
        @('audit-1280', '--ui-cold-character-creation-industrial-audit', 'metamorphe'),
        @('audit-960', '--ui-cold-character-creation-industrial-audit-small', 'metamorphe'),
        @('terminal', '--ui-cold-character-creation-industrial-skill-terminal', 'metamorphe'),
        @('audit-terminal', '--ui-cold-character-creation-industrial-audit-terminal', 'metamorphe'),
        @('reduced', '--ui-cold-character-creation-industrial-reduced-skill', 'metamorphe'),
        @('normal-control', '--ui-cold-character-creation-skill', 'arcaniste')
    )
    if ($Integrated) {
        foreach ($reviewCase in $reviewCases) { $reviewCase[1] = $reviewCase[1].Replace('-industrial', '') }
    }
    if ($Quick) {
        $reviewCases = @($reviewCases | Where-Object { $_[0] -in @('metamorphe', 'arcaniste-small', 'terminal', 'normal-control') })
    }
    foreach ($reviewCase in $reviewCases) {
        if (Test-Path -LiteralPath (Join-Path $reviewOut ($reviewCase[0] + '/cold-start.png'))) {
            throw 'Preserve the current capture before regenerating it.'
        }
    }
    # A running capture holds the executable open on Windows. Finish the build
    # successfully before starting any of the isolated native capture processes.
    cargo build --locked --bin project-rl
    if ($LASTEXITCODE -ne 0) { throw 'Build failed; no captures started.' }
    New-Item -ItemType Directory -Path $reviewOut -Force | Out-Null
    $reviewSourceHashes = [ordered]@{}
    foreach ($reviewSource in @('src/ascii_app.rs', 'src/character_creation_industrial.rs', 'src/character_creation_polish.rs', 'src/character_creation_details.rs', 'src/asset_preparation.rs', 'src/main.rs', 'target/debug/project-rl.exe')) {
        $reviewSourceHashes[$reviewSource] = (Get-FileHash -LiteralPath (Join-Path $reviewWorkspace $reviewSource) -Algorithm SHA256).Hash.ToLowerInvariant()
    }
    $reviewSourceHashes | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $reviewOut 'capture-source.json') -Encoding utf8
    foreach ($reviewCase in $reviewCases) {
        $reviewTarget = Join-Path $reviewOut $reviewCase[0]
        New-Item -ItemType Directory -Path $reviewTarget -Force | Out-Null
        $reviewProcess = Start-Process -FilePath (Join-Path $reviewWorkspace 'target/debug/project-rl.exe') -ArgumentList @($reviewCase[1], $reviewTarget, "--capture-creation-class=$($reviewCase[2])") -WorkingDirectory $reviewWorkspace -WindowStyle Hidden -RedirectStandardOutput (Join-Path $reviewTarget 'stdout.log') -RedirectStandardError (Join-Path $reviewTarget 'stderr.log') -PassThru -Wait
        if ($reviewProcess.ExitCode -ne 0) {
            Get-Content -LiteralPath (Join-Path $reviewTarget 'stderr.log')
            throw "Native capture failed: $($reviewCase[0])"
        }
        Write-Output "Capture OK: $($reviewCase[0])"
    }
} finally {
    Pop-Location
}
