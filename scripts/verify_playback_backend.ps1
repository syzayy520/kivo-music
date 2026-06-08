<#
.SYNOPSIS
    Kivo Music playback backend verification script.
    Runs state gate, rust gate, diff gate, and temporary artifact gate.
    Does NOT modify the repository. Does NOT push, commit, reset, or clean.

.PARAMETER Mode
    quick | standard | full (default: standard)

.PARAMETER ExpectedBranch
    Expected git branch name (default: kivo-audio-native-decode-pipeline-p0-009)

.PARAMETER ExpectedHead
    Expected local HEAD hash. If empty, skip HEAD check.

.PARAMETER ExpectedRemote
    Expected remote HEAD hash. If empty, skip remote check.

.PARAMETER SkipCargo
    If true, skip all cargo commands.

.PARAMETER SkipFullLib
    If true, skip cargo test --lib (full run).

.PARAMETER AllowDirty
    If true, allow dirty working tree.

.PARAMETER Help
    Show help text.
#>

param(
    [ValidateSet("quick", "standard", "full")]
    [string]$Mode = "standard",
    [string]$ExpectedBranch = "kivo-audio-native-decode-pipeline-p0-009",
    [string]$ExpectedHead = "",
    [string]$ExpectedRemote = "",
    [switch]$SkipCargo,
    [switch]$SkipFullLib,
    [switch]$AllowDirty,
    [switch]$Help
)

$ErrorActionPreference = "Stop"
$script:failed = $false
$script:failedGate = ""
$script:failedCommand = ""
$script:evidence = @()

# --- Helpers ---

function Write-Help {
    Write-Host @"
Usage: pwsh -NoProfile -File scripts/verify_playback_backend.ps1 [options]

Parameters:
  -Mode quick|standard|full   Verification mode (default: standard)
  -ExpectedBranch <name>      Expected branch (default: kivo-audio-native-decode-pipeline-p0-009)
  -ExpectedHead <hash>        Expected local HEAD hash
  -ExpectedRemote <hash>      Expected remote HEAD hash
  -SkipCargo                  Skip all cargo commands
  -SkipFullLib                Skip full cargo test --lib
  -AllowDirty                 Allow dirty working tree
  -Help                       Show this help

Modes:
  quick    : State gate + cargo check + clippy (no tests)
  standard : State gate + cargo fmt/check/clippy + focused tests (shutdown, seek)
  full     : State gate + cargo fmt/check/clippy + focused tests + full lib test

This script does NOT modify the repository.
"@
    exit 0
}

function Add-Evidence {
    param(
        [string]$Gate,
        [string]$Command,
        [string]$ExitCode,
        [string]$Required,
        [string]$Summary,
        [string]$Proves
    )
    $script:evidence += [PSCustomObject]@{
        Gate = $Gate
        Command = $Command
        Cwd = (Get-Location).Path
        ExitCode = $ExitCode
        Required = $Required
        Summary = $Summary
        Proves = $Proves
    }
}

function Invoke-GateCommand {
    param(
        [string]$Gate,
        [string]$Command,
        [string]$Required = "required",
        [string]$Proves = "",
        [string]$FailClassification = "STOP_VALIDATION_FAILED"
    )
    Write-Host "  >> $Command"
    $exit = 0
    $output = @()
    # Use cmd /c for reliable exit code capture from external commands
    $output = cmd /c "$Command 2>&1" | ForEach-Object { $_ }
    $exit = $LASTEXITCODE
    if ($null -eq $exit) { $exit = 0 }
    $summary = ($output | Out-String).Trim()
    if ($summary.Length -gt 200) {
        $summary = $summary.Substring(0, 200) + "..."
    }
    Add-Evidence -Gate $Gate -Command $Command -ExitCode $exit -Required $Required -Summary $summary -Proves $Proves
    if ($exit -ne 0) {
        Write-Host "  FAIL (exit $exit)" -ForegroundColor Red
        $script:failed = $true
        $script:failedGate = $Gate
        $script:failedCommand = $Command
        return $false
    }
    Write-Host "  PASS (exit 0)" -ForegroundColor Green
    return $true
}

# --- Main ---

if ($Help) { Write-Help }

Write-Host "========================================" -ForegroundColor Cyan
Write-Host " Kivo Playback Backend Verification" -ForegroundColor Cyan
Write-Host " Mode: $Mode" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# === 1. STATE GATE ===
Write-Host "[1/5] State Gate" -ForegroundColor Yellow

# 1.1 git status
$status = git status --short --branch --untracked-files=all 2>&1
$statusStr = ($status | Out-String).Trim()
Add-Evidence -Gate "State Gate" -Command "git status --short --branch --untracked-files=all" -ExitCode "0" -Required "required" -Summary $statusStr -Proves "Working tree state"

# Check dirty
$dirty = $status | Where-Object { $_ -match '^\s*[MADR]' -or $_ -match '^\?\?' }
# Actually: untracked (??) is allowed; dirty means staged or modified tracked files
$stagedOrModified = $status | Where-Object { $_ -notmatch '^\?\?' -and $_ -notmatch '^##' -and $_.Trim() -ne "" }
if ($stagedOrModified -and -not $AllowDirty) {
    Add-Evidence -Gate "State Gate" -Command "dirty check" -ExitCode "1" -Required "required" -Summary "STOP_DIRTY_WORKTREE: Working tree has staged/modified tracked files" -Proves "Clean working tree required"
    Write-Host "  STOP_DIRTY_WORKTREE" -ForegroundColor Red
    $script:failed = $true
    $script:failedGate = "State Gate"
    $script:failedCommand = "dirty check"
}

# 1.2 branch check
$branch = (git rev-parse --abbrev-ref HEAD 2>&1).Trim()
Add-Evidence -Gate "State Gate" -Command "git rev-parse --abbrev-ref HEAD" -ExitCode "0" -Required "required" -Summary $branch -Proves "Current branch"
if ($branch -ne $ExpectedBranch) {
    Add-Evidence -Gate "State Gate" -Command "branch check" -ExitCode "1" -Required "required" -Summary "STOP_BASE_GATE_FAILED: expected=$ExpectedBranch actual=$branch" -Proves "Branch matches expectation"
    Write-Host "  STOP_BASE_GATE_FAILED: branch mismatch" -ForegroundColor Red
    $script:failed = $true
    $script:failedGate = "State Gate"
    $script:failedCommand = "branch check"
}

# 1.3 HEAD check
$head = (git rev-parse HEAD 2>&1).Trim()
Add-Evidence -Gate "State Gate" -Command "git rev-parse HEAD" -ExitCode "0" -Required "required" -Summary $head -Proves "Local HEAD"
if ($ExpectedHead -and $head -ne $ExpectedHead) {
    Add-Evidence -Gate "State Gate" -Command "HEAD check" -ExitCode "1" -Required "required" -Summary "STOP_BASE_GATE_FAILED: expected=$ExpectedHead actual=$head" -Proves "HEAD matches expectation"
    Write-Host "  STOP_BASE_GATE_FAILED: HEAD mismatch" -ForegroundColor Red
    $script:failed = $true
    $script:failedGate = "State Gate"
    $script:failedCommand = "HEAD check"
}

# 1.4 remote check
$remoteLine = (git ls-remote origin $ExpectedBranch 2>&1).Trim()
$remoteHead = ($remoteLine -split "`t")[0]
Add-Evidence -Gate "State Gate" -Command "git ls-remote origin $ExpectedBranch" -ExitCode "0" -Required "required" -Summary $remoteHead -Proves "Remote HEAD"
if ($ExpectedRemote -and $remoteHead -ne $ExpectedRemote) {
    Add-Evidence -Gate "State Gate" -Command "remote check" -ExitCode "1" -Required "required" -Summary "STOP_REMOTE_MISMATCH: expected=$ExpectedRemote actual=$remoteHead" -Proves "Remote HEAD matches expectation"
    Write-Host "  STOP_REMOTE_MISMATCH" -ForegroundColor Red
    $script:failed = $true
    $script:failedGate = "State Gate"
    $script:failedCommand = "remote check"
}

# 1.5 local-ahead
$localAhead = (git rev-list --count "origin/$ExpectedBranch..HEAD" 2>&1).Trim()
Add-Evidence -Gate "State Gate" -Command "git rev-list --count origin/$ExpectedBranch..HEAD" -ExitCode "0" -Required "advisory" -Summary "local-ahead=$localAhead" -Proves "Local commits ahead of remote"

Write-Host ""

# === 2. RUST GATE ===
if (-not $SkipCargo) {
    Write-Host "[2/5] Rust Gate" -ForegroundColor Yellow

    # 2.1 cargo fmt
    Invoke-GateCommand -Gate "Rust Gate" -Command "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check" -Proves "Code formatting clean" -FailClassification "STOP_VALIDATION_FAILED"

    # 2.2 cargo check
    Invoke-GateCommand -Gate "Rust Gate" -Command "cargo check --locked --manifest-path src-tauri/Cargo.toml" -Proves "Compilation clean" -FailClassification "STOP_VALIDATION_FAILED"

    # 2.3 cargo clippy
    Invoke-GateCommand -Gate "Rust Gate" -Command "cargo clippy --locked --manifest-path src-tauri/Cargo.toml --lib -- -D warnings" -Proves "Clippy warnings clean" -FailClassification "STOP_VALIDATION_FAILED"

    # 2.4 tests by mode
    if ($Mode -eq "quick") {
        Write-Host "  [quick mode: skipping tests]" -ForegroundColor DarkGray
        Add-Evidence -Gate "Rust Gate" -Command "cargo test (skipped)" -ExitCode "SKIP" -Required "advisory" -Summary "quick mode: tests skipped" -Proves "Quick mode does not run tests"
    }
    else {
        # focused tests
        Invoke-GateCommand -Gate "Rust Gate" -Command "cargo test --locked --manifest-path src-tauri/Cargo.toml --lib shutdown" -Required "required" -Proves "Shutdown tests pass" -FailClassification "STOP_VALIDATION_FAILED"

        Invoke-GateCommand -Gate "Rust Gate" -Command "cargo test --locked --manifest-path src-tauri/Cargo.toml --lib seek" -Required "required" -Proves "Seek tests pass" -FailClassification "STOP_VALIDATION_FAILED"

        if ($Mode -eq "full") {
            Invoke-GateCommand -Gate "Rust Gate" -Command "cargo test --locked --manifest-path src-tauri/Cargo.toml --lib output_wasapi" -Required "required" -Proves "WASAPI output tests pass" -FailClassification "STOP_VALIDATION_FAILED"

            Invoke-GateCommand -Gate "Rust Gate" -Command "cargo test --locked --manifest-path src-tauri/Cargo.toml --lib real_transport" -Required "required" -Proves "Real transport tests pass" -FailClassification "STOP_VALIDATION_FAILED"

            if (-not $SkipFullLib) {
                Invoke-GateCommand -Gate "Rust Gate" -Command "cargo test --locked --manifest-path src-tauri/Cargo.toml --lib" -Required "required" -Proves "Full lib tests pass" -FailClassification "STOP_VALIDATION_FAILED"
            }
            else {
                Write-Host "  [SkipFullLib: skipping full lib test]" -ForegroundColor DarkGray
                Add-Evidence -Gate "Rust Gate" -Command "cargo test --lib (skipped)" -ExitCode "SKIP" -Required "advisory" -Summary "SkipFullLib=true: full lib test skipped" -Proves "User requested skip"
            }
        }
    }
}
else {
    Write-Host "[2/5] Rust Gate (SKIPPED)" -ForegroundColor DarkGray
    Add-Evidence -Gate "Rust Gate" -Command "all cargo (skipped)" -ExitCode "SKIP" -Required "advisory" -Summary "SkipCargo=true: all cargo commands skipped" -Proves "User requested skip"
}

Write-Host ""

# === 3. DIFF GATE ===
Write-Host "[3/5] Diff Gate" -ForegroundColor Yellow

Invoke-GateCommand -Gate "Diff Gate" -Command "git diff --name-status --no-renames" -Required "required" -Proves "Tracked file changes" -FailClassification "STOP_VALIDATION_FAILED"
Invoke-GateCommand -Gate "Diff Gate" -Command "git diff --stat --no-renames" -Required "advisory" -Proves "Tracked change stats" -FailClassification "STOP_VALIDATION_FAILED"
Invoke-GateCommand -Gate "Diff Gate" -Command "git diff --check" -Required "required" -Proves "No conflict markers or whitespace errors" -FailClassification "STOP_VALIDATION_FAILED"
Invoke-GateCommand -Gate "Diff Gate" -Command "git diff --cached --name-status --no-renames" -Required "required" -Proves "Staged file changes" -FailClassification "STOP_VALIDATION_FAILED"
Invoke-GateCommand -Gate "Diff Gate" -Command "git diff --cached --stat --no-renames" -Required "advisory" -Proves "Staged change stats" -FailClassification "STOP_VALIDATION_FAILED"
Invoke-GateCommand -Gate "Diff Gate" -Command "git diff --cached --check" -Required "required" -Proves "No conflict markers in staged" -FailClassification "STOP_VALIDATION_FAILED"

Write-Host ""

# === 4. TEMPORARY ARTIFACT GATE ===
Write-Host "[4/5] Temporary Artifact Gate" -ForegroundColor Yellow

$tempPatterns = @("*.tmp", "*.log", "*.bak", "*.orig", "*.rej", "*.wav", "*.flac", "*.mp3")
$tempFound = @()
foreach ($pattern in $tempPatterns) {
    $files = Get-ChildItem -Path . -Recurse -Include $pattern -ErrorAction SilentlyContinue |
        Where-Object { $_.FullName -notmatch '\\target\\' -and $_.FullName -notmatch '\\node_modules\\' -and $_.FullName -notmatch '\\dist\\' -and $_.FullName -notmatch '\\build\\' }
    if ($files) {
        $tempFound += $files
    }
}

if ($tempFound.Count -gt 0) {
    $paths = ($tempFound | ForEach-Object { $_.FullName }) -join ", "
    Add-Evidence -Gate "Temp Artifact Gate" -Command "scan *.tmp/log/bak/orig/rej/wav/flac/mp3" -ExitCode "1" -Required "required" -Summary "STOP_SIDE_EFFECT: Found $($tempFound.Count) temp files: $paths" -Proves "No temporary artifacts"
    Write-Host "  STOP_SIDE_EFFECT: Found $($tempFound.Count) temporary files" -ForegroundColor Red
    $script:failed = $true
    $script:failedGate = "Temp Artifact Gate"
    $script:failedCommand = "temp file scan"
}
else {
    Add-Evidence -Gate "Temp Artifact Gate" -Command "scan *.tmp/log/bak/orig/rej/wav/flac/mp3" -ExitCode "0" -Required "required" -Summary "No temporary files found" -Proves "No temporary artifacts"
    Write-Host "  PASS: No temporary files found" -ForegroundColor Green
}

Write-Host ""

# === 5. FINAL SUMMARY ===
Write-Host "[5/5] Final Summary" -ForegroundColor Yellow
Write-Host ""

Write-Host "=== EVIDENCE LEDGER ===" -ForegroundColor Cyan
foreach ($e in $script:evidence) {
    $ts = Get-Date -Format "yyyy-MM-ddTHH:mm"
    Write-Host "[$ts] $($e.Gate) | $($e.Command) | exit=$($e.ExitCode) | $($e.Required) | $($e.Summary) | $($e.Proves)"
}
Write-Host "=== END LEDGER ===" -ForegroundColor Cyan
Write-Host ""

if ($script:failed) {
    Write-Host "RESULT: FAIL" -ForegroundColor Red
    Write-Host "Failed Gate: $script:failedGate" -ForegroundColor Red
    Write-Host "Failed Command: $script:failedCommand" -ForegroundColor Red
    Write-Host "Push Status = no push" -ForegroundColor Yellow
    exit 1
}
else {
    Write-Host "RESULT: PASS" -ForegroundColor Green
    Write-Host "Push Status = no push" -ForegroundColor Yellow
    exit 0
}
