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
echo "[1/6] State Gate"

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
    echo "[2/6] Rust Gate"

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
    echo "[2/6] Rust Gate (SKIPPED)"
    add_evidence "Rust Gate" "all cargo (skipped)" "SKIP" "advisory" "SkipCargo=true: all cargo commands skipped" "User requested skip"
fi

echo ""

# === 3. DIFF GATE ===
echo "[3/6] Diff Gate"

run_gate "Diff Gate" "git diff --name-status --no-renames" "required" "Tracked file changes" || true
run_gate "Diff Gate" "git diff --stat --no-renames" "advisory" "Tracked change stats" || true
run_gate "Diff Gate" "git diff --check" "required" "No conflict markers or whitespace errors" || true
run_gate "Diff Gate" "git diff --cached --name-status --no-renames" "required" "Staged file changes" || true
run_gate "Diff Gate" "git diff --cached --stat --no-renames" "advisory" "Staged change stats" || true
run_gate "Diff Gate" "git diff --cached --check" "required" "No conflict markers in staged" || true

echo ""

# === 4. FOLDER FAN-OUT GATE ===
echo "[4/6] Folder Fan-out Gate"

FANOUT_BLOCKED=false
FANOUT_WARNINGS=""
FANOUT_LEGACY_VIOLATIONS=""

# --- Detect touched/new folders from this commit ---
declare -A TOUCHED_DIRS
while IFS= read -r f; do
    [[ -z "$f" ]] && continue
    parent_dir=$(dirname "$f")
    if [[ -d "$parent_dir" ]]; then
        TOUCHED_DIRS["$parent_dir"]=1
    fi
done < <(git diff --name-only HEAD^ HEAD 2>/dev/null || true)

TOUCHED_COUNT=${#TOUCHED_DIRS[@]}
echo "  Touched folders detected: $TOUCHED_COUNT"
add_evidence "Fan-out Gate" "git diff HEAD^ HEAD" "0" "advisory" "Touched folders: $TOUCHED_COUNT" "Touched folder detection"

# --- Directories to scan ---
CHECK_DIRS=(
    "src-tauri/src/playback/output_wasapi/output_thread/runtime"
    "src-tauri/src/playback/output_wasapi/output_thread/runtime/device_buffer_writer"
    "src-tauri/src/playback/output_wasapi/output_thread/sink_boundary"
    "src-tauri/src/playback/output_wasapi/output_thread/tests"
)

FORBIDDEN_NAMES="helper.rs utils.rs glue.rs facade.rs bridge.rs common.rs misc.rs"

# Function: scan a single directory
scan_fanout_dir() {
    local dir="$1"
    local is_touched="$2"

    [[ ! -d "$dir" ]] && return

    local ALL_FILES=$(find "$dir" -maxdepth 1 -name "*.rs" -type f 2>/dev/null)
    [[ -z "$ALL_FILES" ]] && return

    local TOTAL_COUNT=$(echo "$ALL_FILES" | wc -l)
    local MOD_RS_EXISTS=$(echo "$ALL_FILES" | grep -c "/mod.rs$" || true)
    local BIZ_COUNT=$((TOTAL_COUNT - MOD_RS_EXISTS))
    local FILE_LIST=$(echo "$ALL_FILES" | xargs -I{} basename {} | tr '\n' ', ' | sed 's/, $//')
    local IS_TESTS_DIR=$(echo "$dir" | grep -c '/tests$' || true)
    local ENFORCE_LABEL="legacy (report-only)"
    [[ "$is_touched" == "true" ]] && ENFORCE_LABEL="TOUCHED (enforced)"

    if [[ $IS_TESTS_DIR -gt 0 ]]; then
        # Test directory: target <= 10
        if [[ $BIZ_COUNT -gt 10 ]]; then
            if [[ "$is_touched" == "true" ]]; then
                add_evidence "Fan-out Gate" "folder scan: $dir" "1" "required" "BLOCKED_FOLDER_FANOUT_GATE: touched tests dir has $BIZ_COUNT test files (target=10). Files: $FILE_LIST" "Test folder fan-out (TOUCHED, HARD BLOCK)"
                echo "  BLOCKED: $dir has $BIZ_COUNT test files (target=10) — TOUCHED folder" >&2
                FANOUT_BLOCKED=true
            else
                add_evidence "Fan-out Gate" "folder scan: $dir" "0" "advisory" "LEGACY_VIOLATION: tests dir has $BIZ_COUNT test files (target=10). Files: $FILE_LIST" "Test folder fan-out (legacy, report-only)"
                echo "  LEGACY VIOLATION: $dir has $BIZ_COUNT test files (target=10) — report-only"
                FANOUT_LEGACY_VIOLATIONS+="$dir ($BIZ_COUNT files, target=10); "
            fi
        else
            add_evidence "Fan-out Gate" "folder scan: $dir" "0" "required" "PASS: tests dir has $BIZ_COUNT files (target=10). $ENFORCE_LABEL. Files: $FILE_LIST" "Test folder fan-out"
            echo "  PASS: $dir has $BIZ_COUNT test files ($ENFORCE_LABEL)"
        fi
    else
        # Production directory: target <= 7, hard limit <= 9
        if [[ $BIZ_COUNT -gt 9 ]]; then
            if [[ "$is_touched" == "true" ]]; then
                add_evidence "Fan-out Gate" "folder scan: $dir" "1" "required" "BLOCKED_FOLDER_FANOUT_GATE: touched dir has $BIZ_COUNT business files (hard limit=9). Files: $FILE_LIST" "Production folder fan-out (TOUCHED, HARD BLOCK)"
                echo "  BLOCKED: $dir has $BIZ_COUNT business files (hard limit=9) — TOUCHED folder" >&2
                FANOUT_BLOCKED=true
            else
                add_evidence "Fan-out Gate" "folder scan: $dir" "0" "advisory" "LEGACY_VIOLATION: $dir has $BIZ_COUNT business files (hard limit=9). Files: $FILE_LIST" "Production folder fan-out (legacy, report-only)"
                echo "  LEGACY VIOLATION: $dir has $BIZ_COUNT business files (hard limit=9) — report-only"
                FANOUT_LEGACY_VIOLATIONS+="$dir ($BIZ_COUNT files, hard limit=9); "
            fi
        elif [[ $BIZ_COUNT -gt 7 ]]; then
            if [[ "$is_touched" == "true" ]]; then
                add_evidence "Fan-out Gate" "folder scan: $dir" "0" "advisory" "WARNING: touched dir has $BIZ_COUNT business files (target=7). Files: $FILE_LIST" "Production folder fan-out (TOUCHED, warning)"
                echo "  WARNING: $dir has $BIZ_COUNT business files (target=7) — TOUCHED folder"
            else
                add_evidence "Fan-out Gate" "folder scan: $dir" "0" "advisory" "WARNING: $dir has $BIZ_COUNT business files (target=7). Files: $FILE_LIST" "Production folder fan-out"
                echo "  WARNING: $dir has $BIZ_COUNT business files (target=7)"
            fi
            FANOUT_WARNINGS+="$dir ($BIZ_COUNT files); "
        else
            add_evidence "Fan-out Gate" "folder scan: $dir" "0" "required" "PASS: $dir has $BIZ_COUNT business files (target=7). $ENFORCE_LABEL. Files: $FILE_LIST" "Production folder fan-out"
            echo "  PASS: $dir has $BIZ_COUNT business files ($ENFORCE_LABEL)"
        fi
    fi

    # Forbidden bucket file check (always HARD BLOCK regardless of touched/legacy)
    for f in $ALL_FILES; do
        BASENAME=$(basename "$f")
        if echo "$FORBIDDEN_NAMES" | grep -qw "$BASENAME"; then
            add_evidence "Fan-out Gate" "forbidden name: $f" "1" "required" "STOP_GENEALOGY_VIOLATION: forbidden bucket file $BASENAME in $dir" "No forbidden bucket files"
            echo "  STOP_GENEALOGY_VIOLATION: forbidden file $BASENAME in $dir" >&2
            FANOUT_BLOCKED=true
        fi
    done

    # mod.rs size check (warn if > 50 lines)
    if [[ $MOD_RS_EXISTS -gt 0 ]]; then
        MOD_PATH="$dir/mod.rs"
        MOD_LINES=$(wc -l < "$MOD_PATH" 2>/dev/null || echo "0")
        if [[ $MOD_LINES -gt 50 ]]; then
            add_evidence "Fan-out Gate" "mod.rs size: $MOD_PATH" "0" "advisory" "WARNING: $MOD_PATH has $MOD_LINES lines (expected < 50). May contain logic." "mod.rs declaration only"
            echo "  WARNING: $MOD_PATH has $MOD_LINES lines (may contain logic)"
            FANOUT_WARNINGS+="$MOD_PATH ($MOD_LINES lines); "
        fi
    fi
}

for dir in "${CHECK_DIRS[@]}"; do
    is_touched="false"
    [[ -n "${TOUCHED_DIRS[$dir]+x}" ]] && is_touched="true"
    scan_fanout_dir "$dir" "$is_touched"
done

# --- Overall result ---
if [[ "$FANOUT_BLOCKED" == "true" ]]; then
    add_evidence "Fan-out Gate" "overall" "1" "required" "BLOCKED_FOLDER_FANOUT_GATE: touched folder(s) violate fan-out limits" "Folder fan-out compliance"
    echo "  RESULT: BLOCKED_FOLDER_FANOUT_GATE" >&2
    FAILED=true
    FAILED_GATE="Fan-out Gate"
    FAILED_CMD="touched folder fan-out violation"
elif [[ -n "$FANOUT_LEGACY_VIOLATIONS" ]]; then
    add_evidence "Fan-out Gate" "overall" "0" "advisory" "LEGACY_VIOLATIONS_DETECTED (report-only): $FANOUT_LEGACY_VIOLATIONS" "Folder fan-out compliance"
    echo "  RESULT: LEGACY VIOLATIONS DETECTED (report-only, legacy violation(s))"
    echo "  NOTE: Legacy violations require dedicated restructuring tickets. Not blocked."
elif [[ -n "$FANOUT_WARNINGS" ]]; then
    add_evidence "Fan-out Gate" "overall" "0" "advisory" "PASS with warnings: $FANOUT_WARNINGS" "Folder fan-out compliance"
    echo "  RESULT: PASS (with warnings)"
else
    add_evidence "Fan-out Gate" "overall" "0" "required" "PASS: all folders within fan-out limits" "Folder fan-out compliance"
    echo "  RESULT: PASS"
fi

echo ""

# === 5. TEMPORARY ARTIFACT GATE ===
echo "[5/6] Temporary Artifact Gate"

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
echo "[6/6] Final Summary"
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
