# Kivo Audio Core Execution Spec (Strict)

This document is the strict execution contract for audio-core tickets.

## 1. Source-of-truth precedence

If documents conflict, apply this order:

1. `docs/AUDIO_CORE_EXECUTION_SPEC.md` (this file, execution authority)
2. `docs/AUDIO_CORE_LAYERED_EXECUTION_RULES.md` (layering and process rules)
3. `docs/audio-core-implementation-checklist.md` (roadmap and ticket ordering)
4. `docs/PLAYBACK_CORE_PLAN.md` (architecture and product intent)

Conflict resolver:

1. If two rules conflict, the stricter rule wins.
2. If a local ticket instruction conflicts with this spec, stop and escalate before coding.

## 1A. Iron laws (non-negotiable)

These laws are mandatory for every assistant and every ticket.

1. Single responsibility per module:
Each module/file must have exactly one primary responsibility.
2. Small files by default:
Prefer small files and split early before mixed concerns accumulate.
3. Composition-only pages:
UI pages/components at page level may compose modules only; they must not own playback business logic, backend calls, decoder logic, or output logic.
4. No cross-layer shortcuts:
If a task appears faster with a shortcut, it is still forbidden.
5. If a change violates any iron law, the ticket is rejected.

## 2. Delegation chain and allowed calls

Allowed parent -> child chain:

1. `commands` -> `manager` only
2. `manager` -> `engine` and `manager_*` facades only
3. `engine` -> `pipeline` only
4. `pipeline` -> domain layers only

Hard ban:

1. No parent may call grandchild internals directly.
2. No UI/shared frontend layer may import backend-specific types.

## 3. Ticket required header (must include)

1. Ticket ID
2. Ticket type (`ARCH`, `RUNTIME`, `OUTPUT`, `PROBE`, `QUEUE`, `EVENTS`, `DOC`)
3. Allowed files
4. Forbidden files
5. Intended state transitions
6. Intended event order changes (or "none")
7. Fallback impact (or "none")
8. Responsibility statement per touched file (one sentence each)
9. Split plan statement (`not needed` or explicit split target files)
10. Page composition statement (`no page-level business logic added` or explicit exception ticket)

DOC ticket note:

For `DOC` tickets, items 5/6/7 may be `N/A` when no runtime behavior is touched.

## 4. Runtime closure checklist (for non-DOC tickets)

All items are mandatory for `RUNTIME`, `OUTPUT`, `QUEUE`, and `EVENTS` ticket types.

1. No fake success paths.
2. Unsupported operations remain explicit until implemented.
3. State transitions match matrix contract.
4. Event order matches event ordering contract.
5. Typed errors include operation context and backend context.

## 5. Backend fallback decision table (must follow)

1. `BackendInitFailed` -> fallback allowed -> next compatibility backend -> emit `BackendSwitched`
2. `BackendProcessUnavailable` -> fallback allowed -> next compatibility backend -> emit `BackendSwitched`
3. `OutputDeviceUnavailable` -> fallback allowed if policy permits -> next compatibility backend -> emit `BackendSwitched`
4. `InvalidPath` -> fallback denied -> fail fast typed error
5. `UnsupportedFormatAllBackends` -> fallback denied -> fail fast typed error
6. `QueuePolicyViolation` -> fallback denied -> fail fast typed error

## 6. Machine-checkable guard commands (must run and report)

These are minimum static checks to catch cross-layer leakage.

1. `rg -n "crate::playback::decoder|crate::playback::output" src-tauri/src/playback/commands.rs`
Expected: no matches
2. `rg -n "crate::playback::decoder|crate::playback::output" src-tauri/src/playback/manager.rs`
Expected: no matches
3. `rg -n "native_pipeline::|decoder_|output_" src/shared src/features`
Expected: no backend internals leaked to frontend
4. `rg -n "invoke\\(|@tauri-apps/api/core" src/features src/app`
Expected: page-level files do not directly call backend commands; shared playback boundary owns calls

Directory existence rule:

If a target directory does not exist in this repository, report it as `N/A (directory missing)` instead of failing the ticket.

## 6A. File split guard policy

1. If a touched file exceeds 160 lines and mixes concerns, split in the same ticket.
2. If a touched file exceeds 220 lines, split is mandatory unless ticket type is `DOC`.
3. New code must prefer helper/facade extraction over branch growth in existing large files.
4. "Will split later" is invalid unless a linked follow-up split ticket ID is provided.

Scope note:

This split policy is mandatory for code files under `src-tauri/src/**/*.rs`. It does not apply to docs-only tickets.

## 7. Ticket acceptance form (copy/paste)

```text
Ticket:
Type:
Allowed files:
Forbidden files:

State transitions touched:
Event order touched:
Fallback behavior touched:

Cross-layer guard checks:
1)
2)
3)

Gates run:
- cargo fmt:
- cargo check:
- cargo test:
- npm run build:

What changed:
What did NOT change:
Risks remaining:
Commit:
```
