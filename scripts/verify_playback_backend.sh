#!/usr/bin/env bash
# Kivo Music playback backend verification script.
# Runs state gate, rust gate, diff gate, and temporary artifact gate.
# Does NOT modify the repository. Does NOT push, commit, reset, or clean.

set -euo pipefail

# --- Defaults ---
MODE="standard"
EXPECTED_BRANCH="kivo-audio-native-decode-pipeline-p0-009"
EXPECTED_HEAD=""
EXPECTED_REMOTE=""
SKIP_CARGO=false
SKIP_FULL_LIB=false
ALLOW_DIRTY=false

# --- Parse args ---
while [[ $# -gt 0 ]]; do
    case "$1" in
        --mode)        MODE="$2"; shift 2 ;;
        --expected-branch) EXPECTED_BRANCH="$2"; shift 2 ;;
        --expected-head)   EXPECTED_HEAD="$2"; shift 2 ;;
        --expected-remote) EXPECTED_REMOTE="$2"; shift 2 ;;
        --skip-cargo)  SKIP_CARGO=true; shift ;;
        --skip-full-lib) SKIP_FULL_LIB=true; shift ;;
        --allow-dirty) ALLOW_DIRTY=true; shift ;;
        --help|-h)
            echo "Usage: bash scripts/verify_playback_backend.sh [options]"
            echo ""
            echo "Options:"
            echo "  --mode quick|standard|full   Verification mode (default: standard)"
            echo "  --expected-branch <name>     Expected branch"
            echo "  --expected-head <hash>       Expected local HEAD hash"
            echo "  --expected-remote <hash>     Expected remote HEAD hash"
            echo "  --skip-cargo                 Skip all cargo commands"
            echo "  --skip-full-lib              Skip full cargo test --lib"
            echo "  --allow-dirty                Allow dirty working tree"
            echo "  --help                       Show this help"
            echo ""
            echo "Modes:"
            echo "  quick    : State gate + cargo check + clippy (no tests)"
            echo "  standard : State gate + cargo fmt/check/clippy + focused tests"
            echo "  full     : State gate + cargo fmt/check/clippy + focused + full lib"
            exit 0
            ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

FAILED=false
FAILED_GATE=""
FAILED_CMD=""
EVIDENCE=""

# --- Helpers ---
add_evidence() {
    local gate="$1" cmd="$2" exit_code="$3" required="$4" summary="$5" proves="$6"
    local ts
    ts=$(date -u +"%Y-%m-%dT%H:%M:%S" 2>/dev/null || date +"%Y-%m-%dT%H:%M:%S")
    EVIDENCE+="[$ts] $gate | $cmd | exit=$exit_code | $required | $summary | $proves"$'\n'
}

run_gate() {
    local gate="$1" cmd="$2" required="${3:-required}" proves="${4:-}" fail_class="${5:-STOP_VALIDATION_FAILED}"
    echo "  >> $cmd"
    local output exit_code=0
    output=$(eval "$cmd" 2>&1) || exit_code=$?
    local summary
    summary=$(echo "$output" | head -c 200)
    add_evidence "$gate" "$cmd" "$exit_code" "$required" "$summary" "$proves"
    if [[ $exit_code -ne 0 ]]; then
        echo "  FAIL (exit $exit_code)" >&2
        FAILED=true
        FAILED_GATE="$gate"
        FAILED_CMD="$cmd"
        return 1
    fi
    echo "  PASS (exit 0)"
    return 0
}

# --- Main ---
echo "========================================"
echo " Kivo Playback Backend Verification"
echo " Mode: $MODE"
echo "========================================"
echo ""

# === 1. STATE GATE ===
echo "[1/5] State Gate"

# 1.1 git status
STATUS=$(git status --short --branch --untracked-files=all 2>&1)
add_evidence "State Gate" "git status --short --branch --untracked-files=all" "0" "required" "$STATUS" "Working tree state"

# Check dirty
DIRTY_LINES=$(echo "$STATUS" | grep -v '^##' | grep -v '^$' | grep -v '^??' || true)
if [[ -n "$DIRTY_LINES" ]] && [[ "$ALLOW_DIRTY" != "true" ]]; then
    add_evidence "State Gate" "dirty check" "1" "required" "STOP_DIRTY_WORKTREE: Working tree has staged/modified tracked files" "Clean working tree required"
    echo "  STOP_DIRTY_WORKTREE" >&2
    FAILED=true
    FAILED_GATE="State Gate"
    FAILED_CMD="dirty check"
fi

# 1.2 branch check
BRANCH=$(git rev-parse --abbrev-ref HEAD 2>&1)
add_evidence "State Gate" "git rev-parse --abbrev-ref HEAD" "0" "required" "$BRANCH" "Current branch"
if [[ "$BRANCH" != "$EXPECTED_BRANCH" ]]; then
    add_evidence "State Gate" "branch check" "1" "required" "STOP_BASE_GATE_FAILED: expected=$EXPECTED_BRANCH actual=$BRANCH" "Branch matches expectation"
    echo "  STOP_BASE_GATE_FAILED: branch mismatch" >&2
    FAILED=true
    FAILED_GATE="State Gate"
    FAILED_CMD="branch check"
fi

# 1.3 HEAD check
HEAD=$(git rev-parse HEAD 2>&1)
add_evidence "State Gate" "git rev-parse HEAD" "0" "required" "$HEAD" "Local HEAD"
if [[ -n "$EXPECTED_HEAD" ]] && [[ "$HEAD" != "$EXPECTED_HEAD" ]]; then
    add_evidence "State Gate" "HEAD check" "1" "required" "STOP_BASE_GATE_FAILED: expected=$EXPECTED_HEAD actual=$HEAD" "HEAD matches expectation"
    echo "  STOP_BASE_GATE_FAILED: HEAD mismatch" >&2
    FAILED=true
    FAILED_GATE="State Gate"
    FAILED_CMD="HEAD check"
fi

# 1.4 remote check
REMOTE_LINE=$(git ls-remote origin "$EXPECTED_BRANCH" 2>&1)
REMOTE_HEAD=$(echo "$REMOTE_LINE" | cut -f1)
add_evidence "State Gate" "git ls-remote origin $EXPECTED_BRANCH" "0" "required" "$REMOTE_HEAD" "Remote HEAD"
if [[ -n "$EXPECTED_REMOTE" ]] && [[ "$REMOTE_HEAD" != "$EXPECTED_REMOTE" ]]; then
    add_evidence "State Gate" "remote check" "1" "required" "STOP_REMOTE_MISMATCH: expected=$EXPECTED_REMOTE actual=$REMOTE_HEAD" "Remote HEAD matches expectation"
    echo "  STOP_REMOTE_MISMATCH" >&2
    FAILED=true
    FAILED_GATE="State Gate"
    FAILED_CMD="remote check"
fi

# 1.5 local-ahead
LOCAL_AHEAD=$(git rev-list --count "origin/$EXPECTED_BRANCH..HEAD" 2>&1)
add_evidence "State Gate" "git rev-list --count origin/$EXPECTED_BRANCH..HEAD" "0" "advisory" "local-ahead=$LOCAL_AHEAD" "Local commits ahead of remote"

echo ""

# === 2. RUST GATE ===
if [[ "$SKIP_CARGO" != "true" ]]; then
    echo "[2/5] Rust Gate"

    # 2.1 cargo fmt
    run_gate "Rust Gate" "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check" "required" "Code formatting clean" || true

    # 2.2 cargo check
    run_gate "Rust Gate" "cargo check --locked --manifest-path src-tauri/Cargo.toml" "required" "Compilation clean" || true

    # 2.3 cargo clippy
    run_gate "Rust Gate" "cargo clippy --locked --manifest-path src-tauri/Cargo.toml --lib -- -D warnings" "required" "Clippy warnings clean" || true

    # 2.4 tests by mode
    if [[ "$MODE" == "quick" ]]; then
        echo "  [quick mode: skipping tests]"
        add_evidence "Rust Gate" "cargo test (skipped)" "SKIP" "advisory" "quick mode: tests skipped" "Quick mode does not run tests"
    else
        run_gate "Rust Gate" "cargo test --locked --manifest-path src-tauri/Cargo.toml --lib shutdown" "required" "Shutdown tests pass" || true
        run_gate "Rust Gate" "cargo test --locked --manifest-path src-tauri/Cargo.toml --lib seek" "required" "Seek tests pass" || true

        if [[ "$MODE" == "full" ]]; then
            run_gate "Rust Gate" "cargo test --locked --manifest-path src-tauri/Cargo.toml --lib output_wasapi" "required" "WASAPI output tests pass" || true
            run_gate "Rust Gate" "cargo test --locked --manifest-path src-tauri/Cargo.toml --lib real_transport" "required" "Real transport tests pass" || true

            if [[ "$SKIP_FULL_LIB" != "true" ]]; then
                run_gate "Rust Gate" "cargo test --locked --manifest-path src-tauri/Cargo.toml --lib" "required" "Full lib tests pass" || true
            else
                echo "  [SkipFullLib: skipping full lib test]"
                add_evidence "Rust Gate" "cargo test --lib (skipped)" "SKIP" "advisory" "SkipFullLib=true: full lib test skipped" "User requested skip"
            fi
        fi
    fi
else
    echo "[2/5] Rust Gate (SKIPPED)"
    add_evidence "Rust Gate" "all cargo (skipped)" "SKIP" "advisory" "SkipCargo=true: all cargo commands skipped" "User requested skip"
fi

echo ""

# === 3. DIFF GATE ===
echo "[3/5] Diff Gate"

run_gate "Diff Gate" "git diff --name-status --no-renames" "required" "Tracked file changes" || true
run_gate "Diff Gate" "git diff --stat --no-renames" "advisory" "Tracked change stats" || true
run_gate "Diff Gate" "git diff --check" "required" "No conflict markers or whitespace errors" || true
run_gate "Diff Gate" "git diff --cached --name-status --no-renames" "required" "Staged file changes" || true
run_gate "Diff Gate" "git diff --cached --stat --no-renames" "advisory" "Staged change stats" || true
run_gate "Diff Gate" "git diff --cached --check" "required" "No conflict markers in staged" || true

echo ""

# === 4. TEMPORARY ARTIFACT GATE ===
echo "[4/5] Temporary Artifact Gate"

TEMP_FOUND=0
TEMP_PATHS=""
for pattern in "*.tmp" "*.log" "*.bak" "*.orig" "*.rej" "*.wav" "*.flac" "*.mp3"; do
    # Find files, exclude target/node_modules/dist/build
    while IFS= read -r -d '' file; do
        TEMP_FOUND=$((TEMP_FOUND + 1))
        TEMP_PATHS+="$file "
    done < <(find . -name "$pattern" -not -path "*/target/*" -not -path "*/node_modules/*" -not -path "*/dist/*" -not -path "*/build/*" -print0 2>/dev/null || true)
done

if [[ $TEMP_FOUND -gt 0 ]]; then
    add_evidence "Temp Artifact Gate" "scan *.tmp/log/bak/orig/rej/wav/flac/mp3" "1" "required" "STOP_SIDE_EFFECT: Found $TEMP_FOUND temp files: $TEMP_PATHS" "No temporary artifacts"
    echo "  STOP_SIDE_EFFECT: Found $TEMP_FOUND temporary files" >&2
    FAILED=true
    FAILED_GATE="Temp Artifact Gate"
    FAILED_CMD="temp file scan"
else
    add_evidence "Temp Artifact Gate" "scan *.tmp/log/bak/orig/rej/wav/flac/mp3" "0" "required" "No temporary files found" "No temporary artifacts"
    echo "  PASS: No temporary files found"
fi

echo ""

# === 5. FINAL SUMMARY ===
echo "[5/5] Final Summary"
echo ""
echo "=== EVIDENCE LEDGER ==="
echo "$EVIDENCE"
echo "=== END LEDGER ==="
echo ""

if [[ "$FAILED" == "true" ]]; then
    echo "RESULT: FAIL"
    echo "Failed Gate: $FAILED_GATE"
    echo "Failed Command: $FAILED_CMD"
    echo "Push Status = no push"
    exit 1
else
    echo "RESULT: PASS"
    echo "Push Status = no push"
    exit 0
fi
