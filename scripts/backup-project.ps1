param(
  [string]$OutputDir = "backups"
)

$ErrorActionPreference = "Stop"

$root = Resolve-Path (Join-Path $PSScriptRoot "..")
$backupRoot = Join-Path $root $OutputDir
$stamp = Get-Date -Format "yyyyMMdd-HHmmss"
$tempDir = Join-Path $backupRoot "tmp-$stamp"
$zipPath = Join-Path $backupRoot "kivo-music-restart-$stamp.zip"

$excludeDirs = @(
  "node_modules",
  "dist",
  ".edge-shot",
  "backups",
  "src-tauri\\target",
  "src-tauri\\target-check"
)

$excludeFiles = @(
  "artifacts-*.png",
  "*.log",
  "pnpm-debug.log*",
  "*.dll"
)

function Test-ExcludedDir([string]$relativePath) {
  foreach ($dir in $excludeDirs) {
    if ($relativePath -eq $dir -or $relativePath.StartsWith("$dir\")) {
      return $true
    }
  }
  return $false
}

function Test-ExcludedFile([string]$name) {
  foreach ($pattern in $excludeFiles) {
    if ($name -like $pattern) {
      return $true
    }
  }
  return $false
}

New-Item -ItemType Directory -Force -Path $backupRoot | Out-Null
if (Test-Path $tempDir) { Remove-Item -Recurse -Force -LiteralPath $tempDir }
New-Item -ItemType Directory -Force -Path $tempDir | Out-Null

$items = Get-ChildItem -LiteralPath $root -Recurse -Force -File
foreach ($item in $items) {
  $relative = $item.FullName.Substring($root.Path.Length + 1)
  if (Test-ExcludedDir $relative) { continue }
  if (Test-ExcludedFile $item.Name) { continue }
  if ($relative.StartsWith("third_party\ffmpeg\bin\") -and $item.Extension -eq ".dll") { continue }
  $target = Join-Path $tempDir $relative
  $targetDir = Split-Path -Parent $target
  New-Item -ItemType Directory -Force -Path $targetDir | Out-Null
  Copy-Item -LiteralPath $item.FullName -Destination $target
}

if (Test-Path $zipPath) { Remove-Item -Force -LiteralPath $zipPath }
Compress-Archive -LiteralPath (Join-Path $tempDir "*") -DestinationPath $zipPath -CompressionLevel Optimal
Remove-Item -Recurse -Force -LiteralPath $tempDir

Write-Output $zipPath
