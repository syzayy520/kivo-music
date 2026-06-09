# CodeBuddy Auto-Gated Task Protocol

## Purpose

This protocol defines the mandatory execution flow for every CodeBuddy task on Kivo Music backend audio. It ensures repeatable, auditable, and safe ticket execution without relying on assistant memory or ad-hoc procedures.

## Non-Agent Boundary

CodeBuddy is an AI coding assistant. This protocol governs how it executes tickets. It does not replace human judgment for:

- Architecture decisions beyond documented governance
- Push authorization (always requires explicit ticket instruction)
- CI/CD pipeline creation (requires human approval)
- Production deployment decisions

## One-Ticket Execution Rule

Each ticket execution is atomic. CodeBuddy must:

1. Execute exactly one ticket per invocation
2. Complete all phases before reporting
3. Not start a second ticket without explicit user instruction
4. Not infer follow-up ticket execution from recommendations

**Violation**: If CodeBuddy attempts to execute a follow-up ticket without user instruction, STOP.

## Push Default Rule

```
DEFAULT: NO PUSH
```

Push requires explicit ticket authorization with the exact command format:

```
git push origin HEAD:<branch-name>
```

If the ticket does not contain an explicit push authorization instruction, Push Status = `no push`. Validation PASS does not imply push-ready.

## Instruction Precedence Rule

When documents conflict, apply this order:

1. `docs/AUDIO_CORE_EXECUTION_SPEC.md` (execution authority)
2. `docs/AUDIO_CORE_LAYERED_EXECUTION_RULES.md` (layering and process)
3. `docs/audio-core-implementation-checklist.md` (roadmap and ordering)
4. `docs/PLAYBACK_CORE_PLAN.md` (architecture and product intent)

Resolution: The stricter rule wins. If a local ticket instruction conflicts with the spec, STOP and escalate before coding.

## Local-Ahead / Remote-Moved Rule

| Condition | Action |
|-----------|--------|
| Local-ahead = 0, Remote at expected | Proceed |
| Local-ahead > 0, ticket does not authorize push | STOP, report unpushed commits |
| Remote HEAD ≠ expected | STOP_REMOTE_MISMATCH |
| Remote advanced since base gate | STOP_REMOTE_MOVED |

## Zero-Test-Proof Rule

If a ticket claims "all tests pass" but:

- The validation gate was skipped, OR
- Test output was not captured, OR
- Test filter matched 0 tests

Then the claim is invalid. Every test assertion requires captured evidence. `cargo test --lib -- <filter>` returning 0 tests is STOP_ZERO_TEST_FILTER.

## Final Classification Rule

Every ticket must end with a Primary Final Classification in the format:

```
<ticket-id> <classification>
```

The classification describes THIS ticket's result only. Follow-up recommendations are separate.

## 10-Phase Protocol

### Phase 0: Intake / Parse Ticket

**Purpose**: Parse ticket, extract constraints, validate format.

**Required ticket header fields**:
1. Ticket ID
2. Ticket type
3. Allowed files (or "design-only" / "docs-only")
4. Forbidden files
5. Intended behavior changes (or "none")
6. Commit policy
7. Push policy

**STOP conditions**:
- STOP_MISSING_TICKET_ID
- STOP_NEEDS_CLARIFICATION

### Phase 1: Base Gate

**Purpose**: Establish and verify the starting point.

**Commands**:
```bash
git status --short --branch --untracked-files=all
git rev-parse --abbrev-ref HEAD
git rev-parse HEAD
git log -5 --oneline
git ls-remote origin <branch>
```

**Required checks**:
- Branch matches ticket expectation
- Local HEAD matches expected safety point
- Remote HEAD matches expected safety point
- Working tree clean

**STOP conditions**:
- STOP_BASE_GATE_FAILED
- STOP_DIRTY_WORKTREE
- STOP_REMOTE_MISMATCH

### Phase 2: Scope Plan

**Purpose**: Map ticket intent to concrete file operations.

**Required output**:
- Files to create (with path and purpose)
- Files to modify (with path and change description)
- Files explicitly NOT touched (cross-reference with forbidden list)

**STOP conditions**:
- STOP_SCOPE_PLAN_REJECTED
- STOP_SCOPE_EXPANSION
- STOP_FORBIDDEN_FILE_TOUCHED

### Phase 3: Implementation

**Purpose**: Execute the planned changes.

**Rules**:
- One file = one responsibility
- Max 220 lines per file (hard limit)
- No cross-layer shortcuts
- Follow genealogy rules (see `kivo_backend_genealogy_checklist.md`)

**STOP conditions**:
- STOP_GENEALOGY_VIOLATION

### Phase 4: Validation

**Purpose**: Run required gates and capture evidence.

See `validation_matrix.md` for gate selection by ticket type.

**STOP conditions**:
- STOP_VALIDATION_FAILED
- STOP_FLAKE_DETECTED
- STOP_ZERO_TEST_FILTER

### Phase 5: Validation Failure / Flake Policy

**Failure classification**:

| Type | Classification | Action |
|------|---------------|--------|
| Compile error | HARD_FAIL | STOP, fix immediately |
| Test assertion failure | HARD_FAIL | STOP, fix logic |
| Transient test timeout | FLAKE | Re-run once; if passes, FLAKE_PASSED_ON_RERUN |
| Transient panic in cleanup | FLAKE | Apply idempotent cleanup, re-run |
| Known WASAPI race | KNOWN_FLAKE | Document, do not block |

**Re-run policy**: First failure → classify → re-run once → if passes, continue → if fails again, STOP.

### Phase 6: Self Audit

**Purpose**: Internal consistency check before commit decision.

**Checklist** (see `kivo_backend_genealogy_checklist.md` for full 28 items):
- Genealogy compliance
- Dependency direction
- Single responsibility
- Side effects documented
- Evidence captured
- Line counts verified
- **Folder Fan-out Gate** (see `kivo_backend_genealogy_checklist.md` — Folder Fan-out Gate section)

**STOP conditions**:
- STOP_AUDIT_FAILURE
- BLOCKED_FOLDER_FANOUT_GATE
- STOP_GENEALOGY_VIOLATION

### Phase 7: Commit / No Commit Decision

**Decision matrix**:

| Condition | Decision |
|-----------|----------|
| DESIGN_ONLY ticket | NO COMMIT |
| DOC ticket with no code changes | NO COMMIT |
| Implementation ticket, all gates pass | COMMIT |
| Implementation ticket, HARD_FAIL | NO COMMIT |
| Push not authorized | NO PUSH |

**Commit format**:
```bash
git add <allowed-files>
git commit -m "<ticket-id> <concise-description>"
```

**Forbidden**: `git commit --amend`, `git push` (unless explicitly authorized)

### Phase 8: Final Report

**Purpose**: Produce the comprehensive ticket completion report.

See `task_report_template.md` for required sections.

### Phase 9: Follow-up Planner

**Purpose**: Identify next required tickets.

**Follow-up classifications**:
- `FOLLOW_UP_NONE` — no further action needed
- `FOLLOW_UP_IMMEDIATE` — next ticket required before other work
- `FOLLOW_UP_SCHEDULED` — next ticket can be deferred
- `FOLLOW_UP_DOCS_THEN_SCRIPTED_VALIDATION` — docs first, then scripts, then CI
- `BLOCKED_HUMAN_DECISION` — human input required

**Rule**: CodeBuddy may recommend follow-up but MUST NOT execute it without user instruction.

## Failure Repair Boundary Rule

When a gate fails, CodeBuddy may:

- Fix linting/formatting issues (SOFT_FAIL only)
- Apply idempotent cleanup patterns for flake fixes
- Re-run failed gates once

CodeBuddy must NOT:

- Reset or revert commits
- Expand scope to fix adjacent issues
- Skip or relax gate requirements
- Change production behavior to make tests pass

## Evidence Ledger Rule

Every gate result must be recorded with:

| Field | Required |
|-------|----------|
| Gate name | YES |
| Command executed | YES |
| Working directory | YES |
| Exit code | YES |
| stdout/stderr summary | YES |
| Required / Advisory | YES |
| File changes produced | YES |
| What it proves | YES |
| PASS / FAIL | YES |

## Ticket Contradiction Gate

Before executing, verify:

| Check | STOP if violated |
|-------|------------------|
| Task type vs requested actions | STOP_NEEDS_CLARIFICATION |
| Read-only/design-only vs file creation | STOP_SCOPE_EXPANSION |
| Allowed files vs forbidden files overlap | STOP_SCOPE_PLAN_REJECTED |
| Push policy vs task type | STOP_PUSH_NOT_AUTHORIZED |
| Validation requires forbidden files | STOP_FORBIDDEN_FILE_TOUCHED |
| Expected branch/HEAD vs current state | STOP_BASE_GATE_FAILED |
| Final classifications missing/invalid | STOP_NEEDS_CLARIFICATION |
| Commit policy vs implementation request | STOP_PUSH_NOT_AUTHORIZED |

## Side Effect / Temporary Artifact Gate

After implementation, verify no unexpected files:

```bash
git status --short --untracked-files=all
```

Scan for: `*.tmp`, `*.log`, `*.bak`, `*.orig`, `*.rej`, `*.wav`, `*.flac`, `*.mp3`

Exclude: `target/`, `node_modules/`, `dist/`, `build/`

If new temporary files found: STOP_SIDE_EFFECT.
