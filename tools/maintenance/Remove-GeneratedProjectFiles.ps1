param(
    [Parameter(Mandatory = $true)]
    [string] $Manifest
)

$ErrorActionPreference = 'Stop'
$taskRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '../..')).Path
$manifestPath = (Resolve-Path -LiteralPath $Manifest).Path
$reportRoot = Join-Path $taskRoot 'docs/maintenance/nettoyage-2026-10-16'
if (-not $manifestPath.StartsWith($reportRoot + [IO.Path]::DirectorySeparatorChar, [StringComparison]::OrdinalIgnoreCase)) {
    throw 'Le manifeste doit appartenir au bilan de ce nettoyage.'
}
$plan = Get-Content -LiteralPath $manifestPath -Raw | ConvertFrom-Json
if ($plan.status -ne 'ready_after_validation' -or $plan.root -ne $taskRoot) {
    throw 'Le manifeste ne confirme pas la validation de ce projet.'
}
if (($plan.files | Where-Object { $_.path -like 'target/*' }) -and
    (Get-Process -Name cargo,rustc -ErrorAction SilentlyContinue)) {
    throw 'Une compilation est en cours ; aucun nettoyage effectué.'
}

$artifactRoot = Join-Path $taskRoot 'artifacts'
$targetRoot = Join-Path $taskRoot 'target'
$editorRoot = Join-Path $artifactRoot 'map-editor'
$kept = @{}
foreach ($relative in $plan.preserved_in_place) {
    $kept[[IO.Path]::GetFullPath((Join-Path $taskRoot $relative))] = $true
}
$epoch = [DateTime]::SpecifyKind([DateTime]'1970-01-01', [DateTimeKind]::Utc)
$checkedParents = @{}
$deletions = [Collections.Generic.List[string]]::new()
$totalBytes = 0L

# Validate every absolute path, ancestor and timestamp before deleting any file.
# Never recurse into a junction or pass paths to another shell.
foreach ($row in $plan.files) {
    $absolute = [IO.Path]::GetFullPath((Join-Path $taskRoot $row.path))
    $inArtifacts = $absolute.StartsWith($artifactRoot + '\', [StringComparison]::OrdinalIgnoreCase)
    $inTarget = $absolute.StartsWith($targetRoot + '\', [StringComparison]::OrdinalIgnoreCase)
    if ((-not $inArtifacts -and -not $inTarget) -or $kept.ContainsKey($absolute) -or
        $absolute.StartsWith($editorRoot + '\', [StringComparison]::OrdinalIgnoreCase)) {
        throw "Cible interdite : $absolute"
    }
    $parent = [IO.Path]::GetDirectoryName($absolute)
    while ($parent -ne $taskRoot) {
        if ($checkedParents.ContainsKey($parent)) { break }
        $directory = Get-Item -LiteralPath $parent -Force
        if (($directory.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) {
            throw "Parent lié interdit : $parent"
        }
        $checkedParents[$parent] = $true
        $parent = [IO.Path]::GetDirectoryName($parent)
    }
    $item = Get-Item -LiteralPath $absolute -Force
    if ($item.PSIsContainer -or (($item.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0)) {
        throw "La cible doit être un fichier ordinaire : $absolute"
    }
    $milliseconds = ($item.LastWriteTimeUtc - $epoch).TotalMilliseconds
    if ($item.Length -ne $row.bytes -or [Math]::Abs($milliseconds - $row.mtime_ms) -gt 2) {
        throw "Fichier modifié depuis le contrôle : $absolute"
    }
    $deletions.Add($absolute)
    $totalBytes += $item.Length
}

$removed = 0
foreach ($absolute in $deletions) {
    Remove-Item -LiteralPath $absolute -Force
    $removed++
}

# Remove only the inventoried junction objects themselves, without recursion.
foreach ($relative in $plan.junctions) {
    $absolute = [IO.Path]::GetFullPath((Join-Path $taskRoot $relative))
    if (-not $absolute.StartsWith($artifactRoot + '\', [StringComparison]::OrdinalIgnoreCase) -or
        $absolute.StartsWith($editorRoot + '\', [StringComparison]::OrdinalIgnoreCase)) {
        throw "Jonction hors périmètre : $absolute"
    }
    $item = Get-Item -LiteralPath $absolute -Force
    if (($item.Attributes -band [IO.FileAttributes]::ReparsePoint) -eq 0) {
        throw "Ce chemin n’est plus une jonction : $absolute"
    }
    Remove-Item -LiteralPath $absolute -Force
}

# Empty directories only: no recursive directory deletion is used.
$emptyCandidates = @($checkedParents.Keys)
foreach ($directory in (Get-ChildItem -LiteralPath $artifactRoot -Directory -Recurse -Force)) {
    $absolute = [IO.Path]::GetFullPath($directory.FullName)
    if (-not $absolute.StartsWith($artifactRoot + '\', [StringComparison]::OrdinalIgnoreCase) -or
        $absolute -eq $editorRoot -or
        $absolute.StartsWith($editorRoot + '\', [StringComparison]::OrdinalIgnoreCase) -or
        (($directory.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0)) { continue }
    $emptyCandidates += $absolute
}
foreach ($parent in ($emptyCandidates | Select-Object -Unique | Sort-Object Length -Descending)) {
    if ($parent -in @($artifactRoot, $targetRoot, $editorRoot)) { continue }
    if (Test-Path -LiteralPath $parent) {
        if ([IO.Directory]::GetFileSystemEntries($parent).Length -eq 0) {
            Remove-Item -LiteralPath $parent -Force
        }
    }
}

$result = [ordered]@{
    status = 'completed'
    files_removed = $removed
    logical_bytes_removed = $totalBytes
    junctions_removed_without_traversal = @($plan.junctions).Count
    executable_preserved = Test-Path -LiteralPath (Join-Path $targetRoot 'debug/project-rl.exe')
}
$resultName = if ($plan.result_name) { [string]$plan.result_name } else { 'cleanup-result.json' }
if ($resultName -notmatch '^[a-z0-9-]+\.json$') { throw 'Nom de bilan invalide.' }
$result | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $reportRoot $resultName) -Encoding utf8
$result | ConvertTo-Json -Compress
