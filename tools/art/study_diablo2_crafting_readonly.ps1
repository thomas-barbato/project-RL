# Read vanilla Diablo II tables using the installed archive reader only.
# Run with Windows PowerShell 32-bit. No game process, mod data or saves.
$ErrorActionPreference = 'Stop'
if ([IntPtr]::Size -ne 4) { throw 'Use Windows PowerShell in SysWOW64.' }
$gameRoot = 'D:\JEUX\Diablo 2 old\Diablo II'
$projectRoot = [IO.Path]::GetFullPath((Join-Path $PSScriptRoot '..\..'))
$workRoot = Join-Path $projectRoot 'artifacts\itemisation-loot-01\diablo2-reference'
[IO.Directory]::CreateDirectory($workRoot) | Out-Null
Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
public static class CraftMpqRead {
 [DllImport(@"D:\JEUX\Diablo 2 old\Diablo II\ProjectD2\MpqFixer\SFMPQ.dll", CharSet=CharSet.Ansi)]
 public static extern bool SFileOpenArchive(string path, uint priority, uint flags, out IntPtr handle);
 [DllImport(@"D:\JEUX\Diablo 2 old\Diablo II\ProjectD2\MpqFixer\SFMPQ.dll", CharSet=CharSet.Ansi)]
 public static extern bool SFileOpenFileEx(IntPtr archive, string name, uint scope, out IntPtr handle);
 [DllImport(@"D:\JEUX\Diablo 2 old\Diablo II\ProjectD2\MpqFixer\SFMPQ.dll")]
 public static extern uint SFileGetFileSize(IntPtr file, IntPtr high);
 [DllImport(@"D:\JEUX\Diablo 2 old\Diablo II\ProjectD2\MpqFixer\SFMPQ.dll")]
 public static extern bool SFileReadFile(IntPtr file, byte[] bytes, uint count, out uint read, IntPtr overlapped);
 [DllImport(@"D:\JEUX\Diablo 2 old\Diablo II\ProjectD2\MpqFixer\SFMPQ.dll")]
 public static extern bool SFileCloseFile(IntPtr file);
 [DllImport(@"D:\JEUX\Diablo 2 old\Diablo II\ProjectD2\MpqFixer\SFMPQ.dll")]
 public static extern bool SFileCloseArchive(IntPtr archive);
}
'@
$tables = @('TreasureClassEx','CubeMain','Misc','MonStats','SuperUniques','ItemRatio','MagicPrefix','MagicSuffix','Weapons','Armor','ItemTypes','CharStats','Properties','ItemStatCost')
$found = @{}
$checks = @()
foreach ($archiveName in @('patch_d2.mpq','d2exp.mpq','d2data.mpq')) {
 $archivePath = Join-Path $gameRoot $archiveName
 $before = Get-Item -LiteralPath $archivePath
 $archive = [IntPtr]::Zero
 if (-not [CraftMpqRead]::SFileOpenArchive($archivePath,0,0,[ref]$archive)) { throw "Cannot open $archiveName" }
 try {
  foreach ($table in $tables) {
   if ($found.ContainsKey($table)) { continue }
   $file = [IntPtr]::Zero
   if (-not [CraftMpqRead]::SFileOpenFileEx($archive,"data\global\excel\$table.txt",0,[ref]$file)) { continue }
   try {
    $size = [CraftMpqRead]::SFileGetFileSize($file,[IntPtr]::Zero)
    if ($size -gt 5000000) { throw "Unexpected size for $table" }
    $bytes = New-Object byte[] $size
    [uint32]$read = 0
    if (-not [CraftMpqRead]::SFileReadFile($file,$bytes,$size,[ref]$read,[IntPtr]::Zero)) { throw "Read failed: $table" }
    if ($read -ne $size) { throw "Short read: $table" }
    [IO.File]::WriteAllBytes((Join-Path $workRoot "$table.txt"),$bytes)
    $hasher = [Security.Cryptography.SHA256]::Create()
    try { $hash = ([BitConverter]::ToString($hasher.ComputeHash($bytes))).Replace('-','').ToLowerInvariant() }
    finally { $hasher.Dispose() }
    $found[$table] = [ordered]@{archive=$archiveName;bytes=$size;sha256=$hash}
    Write-Output "$archiveName $table $size"
   } finally { [CraftMpqRead]::SFileCloseFile($file) | Out-Null }
  }
 } finally { [CraftMpqRead]::SFileCloseArchive($archive) | Out-Null }
 $after = Get-Item -LiteralPath $archivePath
 if ($before.Length -ne $after.Length -or $before.LastWriteTimeUtc -ne $after.LastWriteTimeUtc) { throw "Archive changed: $archiveName" }
 $checks += [ordered]@{archive=$archiveName;bytes=$after.Length;last_write_utc=$after.LastWriteTimeUtc.ToString('o');unchanged=$true}
}
$report = [ordered]@{source=$gameRoot;read_only=$true;vanilla_archives_only=$true;game_launched=$false;saves_accessed=$false;archive_precedence=@('patch_d2.mpq','d2exp.mpq','d2data.mpq');tables=$found;archive_checks=$checks}
[IO.File]::WriteAllText((Join-Path $workRoot 'inspection.json'),($report | ConvertTo-Json -Depth 7),(New-Object Text.UTF8Encoding($false)))
if ($found.Count -ne $tables.Count) { throw 'Some requested tables are absent; inspect inspection.json.' }
