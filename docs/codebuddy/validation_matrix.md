# Validation Matrix

## Purpose

This matrix defines which validation gates are required for each ticket type. Gates not listed as "Required" for a given type are optional or N/A.

## Ticket Types

| Type | Description |
|------|-------------|
| `READ_ONLY` | Audit, analysis, investigation — no file changes |
| `DESIGN_ONLY` | Protocol/design document — no file changes |
| `DOC` | Documentation creation/modification — no code changes |
| `CONTRACT` | Contract types, error types, type definitions only |
| `TEST` | Test file creation/modification — no production code changes |
| `IMPL` | Production code implementation |
| `CLEANUP` | Flake fix, idempotent cleanup, formatting |
| `PUSH_AUTH` | Push authorization — verifies state and pushes |

## Gate Selection Matrix

| Gate | READ_ONLY | DESIGN_ONLY | DOC | CONTRACT | TEST | IMPL | CLEANUP | PUSH_AUTH |
|------|-----------|-------------|-----|----------|------|------|---------|-----------|
| `git status` (base gate) | Required | Required | Required | Required | Required | Required | Required | Required |
| `git rev-parse HEAD` | Required | Required | Required | Required | Required | Required | Required | Required |
| `git ls-remote` | Required | Required | Required | Required | Required | Required | Required | Required |
| `cargo fmt --check` | N/A | N/A | N/A | Required | Required | Required | Required | N/A |
| `cargo check` | N/A | N/A | N/A | Required | Required | Required | Required | N/A |
| `cargo clippy` | N/A | N/A | N/A | Required | Required | Required | Required | N/A |
| `cargo test` (focused) | N/A | N/A | N/A | Required | Required | Required | Required | N/A |
| `cargo test` (full) | N/A | N/A | N/A | Recommended | Recommended | Required | Recommended | N/A |
| `git diff --check` | N/A | N/A | Required | Required | Required | Required | Required | N/A |
| Line count audit | N/A | N/A | N/A | Required | Required | Required | Required | N/A |
| Genealogy check | N/A | N/A | N/A | Required | Required | Required | Required | N/A |
| Cross-layer guard | N/A | N/A | N/A | Required | Required | Required | N/A | N/A |
| Scope check | Required | Required | Required | Required | Required | Required | Required | Required |
| Read-only diff check | Required | Required | N/A | N/A | N/A | N/A | N/A | N/A |

## Default Rust Gates

### cargo fmt

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
```

- Exit 0 = PASS (no formatting changes needed)
- Exit 1 = FAIL (formatting changes needed)
- **Auto-fix**: Run `cargo fmt --manifest-path src-tauri/Cargo.toml` and re-check

### cargo check

```bash
cargo check --locked --manifest-path src-tauri/Cargo.toml
```

- Exit 0 = PASS (compiles successfully)
- Exit 1 = FAIL (compile errors)
- **Cannot auto-fix**: Requires code changes

### cargo clippy

```bash
cargo clippy --locked --manifest-path src-tauri/Cargo.toml --lib -- -D warnings
```

- Exit 0 = PASS (no warnings or errors)
- Exit 1 = FAIL (warnings or errors treated as errors)
- **Cannot auto-fix**: Requires code changes

### cargo test (focused)

```bash
cargo test --locked --manifest-path src-tauri/Cargo.toml --lib -- <test_filter>
```

- Exit 0 = PASS (all matching tests pass)
- Exit 1 = FAIL (test failure)
- **Must match > 0 tests**: 0 matches = STOP_ZERO_TEST_FILTER
- **Cannot auto-fix**: Requires code changes

### cargo test (full)

```bash
cargo test --locked --manifest-path src-tauri/Cargo.toml --lib
```

- Exit 0 = PASS (all lib tests pass)
- Exit 1 = FAIL (test failure)
- **Recommended for IMPL, optional for others**

### git diff --check

```bash
git diff --check
```

- Exit 0 = PASS (no conflict markers, no whitespace errors)
- Exit 1 = FAIL (conflict markers or whitespace errors found)
- **Cannot auto-fix**: Requires manual resolution

## Gate Execution Rules

### docs-only Tickets

Docs-only tickets do NOT require `cargo` gates unless the ticket explicitly states otherwise. Rationale: documentation changes do not affect Rust compilation.

### Focused Test Requirement

When running focused tests (`cargo test --lib -- <filter>`):

1. The filter MUST match at least 1 test
2. If 0 tests match: STOP_ZERO_TEST_FILTER
3. The 0-test result does NOT prove "all tests passed"

### Flake Re-run Policy

1. First failure: classify as FLAKE or HARD_FAIL
2. Re-run the exact same test command once
3. If passes on re-run: classify as FLAKE_PASSED_ON_RERUN, continue
4. If fails again: classify as HARD_FAIL, STOP_VALIDATION_FAILED

**Rule**: A re-run PASS cannot erase the first-run FAIL. Both results must be recorded in the Evidence Ledger.

### Only Flake-Audit Tickets Can Re-classify

A transient failure can only be re-classified from HARD_FAIL to FLAKE by a dedicated flake-audit ticket. The original ticket must stop at STOP_FLAKE_DETECTED.

### Advisory vs Required

| Status | Meaning |
|--------|---------|
| Required | Gate must pass for the ticket to proceed |
| Advisory | Gate is recommended but not blocking |
| N/A | Gate does not apply to this ticket type |

**Rule**: Advisory gate failures must be documented in the report but do not block the ticket. Advisory failures must NOT be described as "fixed" — they must be described as "advisory, documented, not addressed in this ticket".
