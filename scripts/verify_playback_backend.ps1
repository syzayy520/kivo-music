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
Write-Host "[1/6] State Gate" -ForegroundColor Yellow

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
    Write-Host "[2/6] Rust Gate" -ForegroundColor Yellow

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
    Write-Host "[2/6] Rust Gate (SKIPPED)" -ForegroundColor DarkGray
    Add-Evidence -Gate "Rust Gate" -Command "all cargo (skipped)" -ExitCode "SKIP" -Required "advisory" -Summary "SkipCargo=true: all cargo commands skipped" -Proves "User requested skip"
}

Write-Host ""

# === 3. DIFF GATE ===
Write-Host "[3/6] Diff Gate" -ForegroundColor Yellow

Invoke-GateCommand -Gate "Diff Gate" -Command "git diff --name-status --no-renames" -Required "required" -Proves "Tracked file changes" -FailClassification "STOP_VALIDATION_FAILED"
Invoke-GateCommand -Gate "Diff Gate" -Command "git diff --stat --no-renames" -Required "advisory" -Proves "Tracked change stats" -FailClassification "STOP_VALIDATION_FAILED"
Invoke-GateCommand -Gate "Diff Gate" -Command "git diff --check" -Required "required" -Proves "No conflict markers or whitespace errors" -FailClassification "STOP_VALIDATION_FAILED"
Invoke-GateCommand -Gate "Diff Gate" -Command "git diff --cached --name-status --no-renames" -Required "required" -Proves "Staged file changes" -FailClassification "STOP_VALIDATION_FAILED"
Invoke-GateCommand -Gate "Diff Gate" -Command "git diff --cached --stat --no-renames" -Required "advisory" -Proves "Staged change stats" -FailClassification "STOP_VALIDATION_FAILED"
Invoke-GateCommand -Gate "Diff Gate" -Command "git diff --cached --check" -Required "required" -Proves "No conflict markers in staged" -FailClassification "STOP_VALIDATION_FAILED"

Write-Host ""

# === 4. FOLDER FAN-OUT GATE ===
Write-Host "[4/6] Folder Fan-out Gate" -ForegroundColor Yellow

$fanoutBlocked = $false
$fanoutWarnings = @()
$fanoutLegacyViolations = @()

# --- Detect touched/new folders from this commit ---
$touchedDirs = @{}
try {
    $touchedFiles = git diff --name-only HEAD^ HEAD 2>&1 | Where-Object { $_.Trim() -ne "" }
    foreach ($f in $touchedFiles) {
        $parentDir = Split-Path $f -Parent
        if ($parentDir -and (Test-Path $parentDir)) {
            $touchedDirs[$parentDir] = $true
        }
    }
} catch {
    # If HEAD^ doesn't exist (initial commit), treat all as legacy
    Add-Evidence -Gate "Fan-out Gate" -Command "git diff HEAD^ HEAD" -ExitCode "SKIP" -Required "advisory" -Summary "Cannot diff HEAD^ (initial commit?), treating all as legacy" -Proves "Touched folder detection"
}

$touchedCount = $touchedDirs.Count
Write-Host "  Touched folders detected: $touchedCount" -ForegroundColor DarkGray
Add-Evidence -Gate "Fan-out Gate" -Command "git diff HEAD^ HEAD" -ExitCode "0" -Required "advisory" -Summary "Touched folders: $touchedCount" -Proves "Touched folder detection"

# --- Directories to scan ---
$checkDirs = @(
    "src-tauri/src/playback/output_wasapi/output_thread/runtime",
    "src-tauri/src/playback/output_wasapi/output_thread/runtime/device_buffer_writer",
    "src-tauri/src/playback/output_wasapi/output_thread/sink_boundary",
    "src-tauri/src/playback/output_wasapi/output_thread/tests"
)

function Test-FanoutDir {
    param(
        [string]$Dir,
        [bool]$IsTouched
    )

    if (-not (Test-Path $Dir)) { return }

    $allFiles = Get-ChildItem -Path $Dir -File -Filter "*.rs" -ErrorAction SilentlyContinue
    if (-not $allFiles) { return }

    $businessFiles = $allFiles | Where-Object { $_.Name -ne "mod.rs" }
    $hasModRs = ($allFiles | Where-Object { $_.Name -eq "mod.rs" }).Count -gt 0
    $isTestsDir = $Dir -match '[/\\]tests$'

    $fileList = ($allFiles | ForEach-Object { $_.Name }) -join ", "
    $bizCount = $businessFiles.Count
    $totalCount = $allFiles.Count

    $enforceLabel = if ($IsTouched) { "TOUCHED (enforced)" } else { "legacy (report-only)" }

    if ($isTestsDir) {
        # Test directory: target <= 10
        if ($bizCount -gt 10) {
            if ($IsTouched) {
                Add-Evidence -Gate "Fan-out Gate" -Command "folder scan: $Dir" -ExitCode "1" -Required "required" -Summary "BLOCKED_FOLDER_FANOUT_GATE: touched tests dir has $bizCount test files (target=10). Files: $fileList" -Proves "Test folder fan-out (TOUCHED, HARD BLOCK)"
                Write-Host "  BLOCKED: $Dir has $bizCount test files (target=10) — TOUCHED folder" -ForegroundColor Red
                $script:fanoutBlocked = $true
            } else {
                Add-Evidence -Gate "Fan-out Gate" -Command "folder scan: $Dir" -ExitCode "0" -Required "advisory" -Summary "LEGACY_VIOLATION: tests dir has $bizCount test files (target=10). Files: $fileList" -Proves "Test folder fan-out (legacy, report-only)"
                Write-Host "  LEGACY VIOLATION: $Dir has $bizCount test files (target=10) — report-only" -ForegroundColor Yellow
                $script:fanoutLegacyViolations += "$Dir ($bizCount files, target=10)"
            }
        } else {
            Add-Evidence -Gate "Fan-out Gate" -Command "folder scan: $Dir" -ExitCode "0" -Required "required" -Summary "PASS: tests dir has $bizCount files (target=10). $enforceLabel. Files: $fileList" -Proves "Test folder fan-out"
            Write-Host "  PASS: $Dir has $bizCount test files ($enforceLabel)" -ForegroundColor Green
        }
    } else {
        # Production directory: target <= 7, hard limit <= 9
        if ($bizCount -gt 9) {
            if ($IsTouched) {
                Add-Evidence -Gate "Fan-out Gate" -Command "folder scan: $Dir" -ExitCode "1" -Required "required" -Summary "BLOCKED_FOLDER_FANOUT_GATE: touched dir has $bizCount business files (hard limit=9). Files: $fileList" -Proves "Production folder fan-out (TOUCHED, HARD BLOCK)"
                Write-Host "  BLOCKED: $Dir has $bizCount business files (hard limit=9) — TOUCHED folder" -ForegroundColor Red
                $script:fanoutBlocked = $true
            } else {
                Add-Evidence -Gate "Fan-out Gate" -Command "folder scan: $Dir" -ExitCode "0" -Required "advisory" -Summary "LEGACY_VIOLATION: $Dir has $bizCount business files (hard limit=9). Files: $fileList" -Proves "Production folder fan-out (legacy, report-only)"
                Write-Host "  LEGACY VIOLATION: $Dir has $bizCount business files (hard limit=9) — report-only" -ForegroundColor Yellow
                $script:fanoutLegacyViolations += "$Dir ($bizCount files, hard limit=9)"
            }
        } elseif ($bizCount -gt 7) {
            if ($IsTouched) {
                Add-Evidence -Gate "Fan-out Gate" -Command "folder scan: $Dir" -ExitCode "0" -Required "advisory" -Summary "WARNING: touched dir has $bizCount business files (target=7). Files: $fileList" -Proves "Production folder fan-out (TOUCHED, warning)"
                Write-Host "  WARNING: $Dir has $bizCount business files (target=7) — TOUCHED folder" -ForegroundColor Yellow
            } else {
                Add-Evidence -Gate "Fan-out Gate" -Command "folder scan: $Dir" -ExitCode "0" -Required "advisory" -Summary "WARNING: $Dir has $bizCount business files (target=7). Files: $fileList" -Proves "Production folder fan-out"
                Write-Host "  WARNING: $Dir has $bizCount business files (target=7)" -ForegroundColor Yellow
            }
            $script:fanoutWarnings += "$Dir ($bizCount files)"
        } else {
            Add-Evidence -Gate "Fan-out Gate" -Command "folder scan: $Dir" -ExitCode "0" -Required "required" -Summary "PASS: $Dir has $bizCount business files (target=7). $enforceLabel. Files: $fileList" -Proves "Production folder fan-out"
            Write-Host "  PASS: $Dir has $bizCount business files ($enforceLabel)" -ForegroundColor Green
        }
    }

    # Forbidden bucket file check (always HARD BLOCK regardless of touched/legacy)
    $forbiddenNames = @("helper.rs", "utils.rs", "glue.rs", "facade.rs", "bridge.rs", "common.rs", "misc.rs")
    foreach ($f in $businessFiles) {
        if ($f.Name -in $forbiddenNames) {
            Add-Evidence -Gate "Fan-out Gate" -Command "forbidden name: $($f.FullName)" -ExitCode "1" -Required "required" -Summary "STOP_GENEALOGY_VIOLATION: forbidden bucket file $($f.Name) in $Dir" -Proves "No forbidden bucket files"
            Write-Host "  STOP_GENEALOGY_VIOLATION: forbidden file $($f.Name) in $Dir" -ForegroundColor Red
            $script:fanoutBlocked = $true
        }
    }

    # mod.rs size check (warn if > 50 lines)
    if ($hasModRs) {
        $modPath = Join-Path $Dir "mod.rs"
        $modLines = (cmd /c "find /c /v `"`" `"$modPath`"" 2>&1 | Select-String '(\d+)$' | ForEach-Object { $_.Matches[0].Groups[1].Value })
        if ($modLines -gt 50) {
            Add-Evidence -Gate "Fan-out Gate" -Command "mod.rs size: $modPath" -ExitCode "0" -Required "advisory" -Summary "WARNING: $modPath has $modLines lines (expected < 50). May contain logic." -Proves "mod.rs declaration only"
            Write-Host "  WARNING: $modPath has $modLines lines (may contain logic)" -ForegroundColor Yellow
            $script:fanoutWarnings += "$modPath ($modLines lines)"
        }
    }
}

foreach ($dir in $checkDirs) {
    $isTouched = $touchedDirs.ContainsKey($dir)
    Test-FanoutDir -Dir $dir -IsTouched $isTouched
}

# --- Overall result ---
if ($fanoutBlocked) {
    Add-Evidence -Gate "Fan-out Gate" -Command "overall" -ExitCode "1" -Required "required" -Summary "BLOCKED_FOLDER_FANOUT_GATE: touched folder(s) violate fan-out limits" -Proves "Folder fan-out compliance"
    Write-Host "  RESULT: BLOCKED_FOLDER_FANOUT_GATE" -ForegroundColor Red
    $script:failed = $true
    $script:failedGate = "Fan-out Gate"
    $script:failedCommand = "touched folder fan-out violation"
} elseif ($fanoutLegacyViolations.Count -gt 0) {
    $violationList = $fanoutLegacyViolations -join "; "
    Add-Evidence -Gate "Fan-out Gate" -Command "overall" -ExitCode "0" -Required "advisory" -Summary "LEGACY_VIOLATIONS_DETECTED (report-only): $violationList" -Proves "Folder fan-out compliance"
    Write-Host "  RESULT: LEGACY VIOLATIONS DETECTED (report-only, $($fanoutLegacyViolations.Count) legacy violation(s))" -ForegroundColor Yellow
    Write-Host "  NOTE: Legacy violations require dedicated restructuring tickets. Not blocked." -ForegroundColor DarkYellow
} elseif ($fanoutWarnings.Count -gt 0) {
    $warnList = $fanoutWarnings -join "; "
    Add-Evidence -Gate "Fan-out Gate" -Command "overall" -ExitCode "0" -Required "advisory" -Summary "PASS with warnings: $warnList" -Proves "Folder fan-out compliance"
    Write-Host "  RESULT: PASS (with $($fanoutWarnings.Count) warning(s))" -ForegroundColor Yellow
} else {
    Add-Evidence -Gate "Fan-out Gate" -Command "overall" -ExitCode "0" -Required "required" -Summary "PASS: all folders within fan-out limits" -Proves "Folder fan-out compliance"
    Write-Host "  RESULT: PASS" -ForegroundColor Green
}

Write-Host ""

# === 5. TEMPORARY ARTIFACT GATE ===
Write-Host "[5/6] Temporary Artifact Gate" -ForegroundColor Yellow

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
Write-Host "[6/6] Final Summary" -ForegroundColor Yellow
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
