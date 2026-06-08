# STOP Rules

## Overview

A STOP is an immediate halt to ticket execution. When a STOP triggers, CodeBuddy must:

1. **Stop execution immediately** — do not continue to the next phase
2. **Do NOT fix** — unless the STOP type explicitly allows auto-fix
3. **Do NOT reset** — do not revert any changes
4. **Do NOT push** — never push after a STOP
5. **Do NOT expand scope** — do not change additional files to resolve the STOP
6. **Do NOT auto-proceed to next ticket** — STOP blocks follow-up execution
7. **Report** — include the STOP in the Final Report with full evidence

## Standard STOP Table (16 classifications)

### 1. STOP_BASE_GATE_FAILED

- **Trigger**: Base gate check fails (branch mismatch, HEAD mismatch, or state inconsistency)
- **Required action**: Report the exact mismatch between expected and actual state
- **Forbidden action**: Do not reset, do not checkout, do not force-advance
- **Report requirement**: Show expected vs actual for branch, local HEAD, remote HEAD
- **Suggested follow-up**: `BLOCKED_HUMAN_DECISION` — requires human to reconcile state

### 2. STOP_DIRTY_WORKTREE

- **Trigger**: `git status --porcelain` shows uncommitted changes at base gate
- **Required action**: Report the exact list of dirty files
- **Forbidden action**: Do not stash, do not reset, do not commit the dirty files
- **Report requirement**: Show `git status --short` output
- **Suggested follow-up**: `BLOCKED_HUMAN_DECISION` — requires human to commit or discard changes

### 3. STOP_REMOTE_MISMATCH

- **Trigger**: Remote HEAD does not match expected safety point
- **Required action**: Report expected vs actual remote HEAD
- **Forbidden action**: Do not force-push, do not reset remote
- **Report requirement**: Show `git ls-remote` output
- **Suggested follow-up**: `BLOCKED_HUMAN_DECISION` — requires human to reconcile remote state

### 4. STOP_SCOPE_PLAN_REJECTED

- **Trigger**: Scope plan contradicts ticket constraints (e.g., allowed files overlap forbidden files)
- **Required action**: Report the specific contradiction
- **Forbidden action**: Do not proceed with implementation
- **Report requirement**: Show allowed list, forbidden list, and the overlap
- **Suggested follow-up**: `BLOCKED_HUMAN_DECISION` — requires ticket clarification

### 5. STOP_SCOPE_EXPANSION

- **Trigger**: Implementation touches files beyond the allowed list
- **Required action**: Report the exact paths of out-of-scope files
- **Forbidden action**: Do not reset, do not delete the out-of-scope changes
- **Report requirement**: Show `git diff --name-status` with out-of-scope files highlighted
- **Suggested follow-up**: `BLOCKED_HUMAN_DECISION` — requires decision on out-of-scope changes

### 6. STOP_FORBIDDEN_FILE_TOUCHED

- **Trigger**: Implementation modifies a file explicitly listed as forbidden in the ticket
- **Required action**: Report which forbidden file was touched and what changed
- **Forbidden action**: Do not reset, do not revert the forbidden change
- **Report requirement**: Show the specific forbidden file and the diff
- **Suggested follow-up**: `BLOCKED_HUMAN_DECISION` — requires ticket scope adjustment

### 7. STOP_GENEALOGY_VIOLATION

- **Trigger**: File placement violates genealogy rules (wrong family, wrong depth, forbidden name)
- **Required action**: Report the specific genealogy rule violated
- **Forbidden action**: Do not move or rename the file
- **Report requirement**: Show file path, expected location, and violated rule
- **Suggested follow-up**: `BLOCKED_HUMAN_DECISION` — requires genealogy-compliant restructure

### 8. STOP_VALIDATION_FAILED

- **Trigger**: A required validation gate fails (cargo fmt, check, clippy, test)
- **Required action**: Report which gate failed with full output
- **Forbidden action**: Do not skip the gate, do not relax the threshold
- **Report requirement**: Show the exact command, exit code, and output
- **Suggested follow-up**: Ticket-specific — may be auto-fixable (fmt) or requires code change (test)

### 9. STOP_FLAKE_DETECTED

- **Trigger**: Test failure classified as transient (first run fails, re-run passes)
- **Required action**: Report the flake with first-run and re-run evidence
- **Forbidden action**: Do not suppress the flake, do not mark as "passed"
- **Report requirement**: Show both test runs with timestamps and pass/fail status
- **Suggested follow-up**: `FOLLOW_UP_SCHEDULED` — flake fix ticket recommended

### 10. STOP_OUT_OF_SCOPE_FLAKE_DETECTED

- **Trigger**: Test failure in a test outside the ticket's scope
- **Required action**: Report the out-of-scope test failure
- **Forbidden action**: Do not fix the out-of-scope test, do not suppress it
- **Report requirement**: Show the failing test name and the ticket's scope
- **Suggested follow-up**: `FOLLOW_UP_IMMEDIATE` — out-of-scope flake needs separate ticket

### 11. STOP_REMOTE_MOVED

- **Trigger**: Remote branch advanced between base gate and commit/push
- **Required action**: Report the remote state at base gate vs current
- **Forbidden action**: Do not force-push, do not rebase over remote
- **Report requirement**: Show remote HEAD at base gate and at detection time
- **Suggested follow-up**: `BLOCKED_HUMAN_DECISION` — requires pull or rebase decision

### 12. STOP_PUSH_NOT_AUTHORIZED

- **Trigger**: Ticket does not authorize push, but push was attempted or implied
- **Required action**: Report that push is not authorized
- **Forbidden action**: Do not push
- **Report requirement**: Show ticket's push policy (or lack thereof)
- **Suggested follow-up**: Follow ticket's push policy (usually `no push`)

### 13. STOP_HUMAN_DECISION_REQUIRED

- **Trigger**: A decision point that requires human input (ambiguity, conflicting requirements, architecture choice)
- **Required action**: Report the decision point and the options
- **Forbidden action**: Do not make the decision autonomously
- **Report requirement**: Show the ambiguity and possible resolutions
- **Suggested follow-up**: `BLOCKED_HUMAN_DECISION`

### 14. STOP_SIDE_EFFECT

- **Trigger**: Unexpected file creation or side effect detected after implementation
- **Required action**: Report the exact paths of unexpected files
- **Forbidden action**: Do not delete the unexpected files, do not push
- **Report requirement**: Show `git status --short` with unexpected files highlighted
- **Suggested follow-up**: `BLOCKED_HUMAN_DECISION` — requires decision on unexpected files

### 15. STOP_ZERO_TEST_FILTER

- **Trigger**: Test filter matches 0 tests (e.g., `cargo test --lib -- nonexistent_test`)
- **Required action**: Report the filter used and the 0-test result
- **Forbidden action**: Do not claim "all tests passed" — 0 tests is not proof
- **Report requirement**: Show the exact test command and output
- **Suggested follow-up**: `BLOCKED_HUMAN_DECISION` — requires correct test filter

### 16. STOP_NEEDS_CLARIFICATION

- **Trigger**: Ticket instructions are ambiguous, contradictory, or incomplete
- **Required action**: Report the specific ambiguity
- **Forbidden action**: Do not assume intent, do not proceed with best-guess
- **Report requirement**: Show the ambiguous instruction and possible interpretations
- **Suggested follow-up**: `BLOCKED_HUMAN_DECISION`

## STOP Execution Flow

```
Phase N executing
    ↓
STOP triggered
    ↓
Halt execution immediately
    ↓
Record STOP in Evidence Ledger
    ↓
Report STOP in Final Report
    ↓
Do NOT proceed to Phase N+1
    ↓
Do NOT commit
    ↓
Do NOT push
    ↓
Primary Final Classification = <ticket-id> BLOCKED_<reason>
```
