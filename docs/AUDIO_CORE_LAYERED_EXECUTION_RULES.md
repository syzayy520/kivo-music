# Kivo Audio Core Layered Execution Rules

This document is the mandatory execution contract for Kivo Native-first playback core work.
It formalizes a strict layered delegation model so code stays small, maintainable, and honest.

## 1. Core doctrine

1. Native-first self-owned playback core is the main line.
2. No fake playback success before real decode + output loop exists.
3. Every layer manages only its direct children.
4. No cross-layer shortcuts.
5. One ticket = one responsibility block.

## 2. Family-tree architecture (delegation chain)

Use this fixed hierarchy:

1. `commands` layer (entry layer)
2. `manager` layer (orchestration layer)
3. `engine` layer (`KivoNativeEngine`, compatibility engines)
4. `pipeline` layer (`NativePipeline` and pipeline facades)
5. domain implementation layers (decoder, session/runtime state, buffer, output, events)

### Layer responsibilities

1. `commands`
- Accept Tauri command input.
- Call manager APIs.
- Record command activity through helper modules.
- Must not hold queue/decoder/output logic.

2. `manager`
- Own orchestration strategy and policy-level decisions.
- Call engine and manager facades (`manager_queue`, future manager facades).
- Must not implement decoder/output internals.

3. `engine`
- Implement `PlaybackEngine` contract.
- Synchronize engine-facing state and typed errors.
- Call pipeline as a child dependency.
- Must not absorb queue policy or command concerns.

4. `pipeline`
- Coordinate decode/buffer/output boundaries.
- Expose explicit state snapshots for observability.
- Keep runtime operations honest (typed unsupported until implemented).
- Must not become a monolith; split by concern.

5. domain layers
- `decoder_*`, `output_*`, `audio_buffer`, `playback_worker_*`, `events_*`.
- Single-purpose, test-focused, no UI coupling.

## 3. Cross-layer bans

Hard bans:

1. `commands` calling backend internals directly.
2. `manager` touching decoder/output concrete internals.
3. `engine` bypassing pipeline to mutate low-level structures.
4. UI-facing layers depending on backend-specific types.
5. Cross-level ownership leaks (grandparent mutating grandchild internals directly).

## 4. File size and split rules

Use these thresholds:

1. Soft threshold: 160 lines.
2. Hard threshold: 220 lines.
3. If a file grows by more than 80 lines in one ticket, justify split before merge.

Mandatory split triggers:

1. More than one domain concern in one file.
2. Repeated operation templates in the same file.
3. Branching logic for unrelated concerns.

## 5. Ticket execution protocol

Every ticket must follow:

1. State Header (repo, branch, base, ticket, allowed/forbidden files, goals).
2. Read-only analysis first.
3. Minimal in-scope edit only.
4. Full gates run:
- `cargo fmt --manifest-path src-tauri/Cargo.toml`
- `cargo check --manifest-path src-tauri/Cargo.toml`
- `cargo test --manifest-path src-tauri/Cargo.toml`
- `npm run build`
- `git status --short`
5. Exact-file staging only (never `git add .`).
6. One ticket, one commit, ticket ID in commit message.

## 6. Refactor safety protocol

For refactor/thinning tickets:

1. Behavior must remain unchanged.
2. Existing tests must pass without weakening assertions.
3. Add focused tests only for extracted helpers/facades.
4. Report explicit "What did NOT change".

## 7. Honesty gate for playback capability

Before real runtime loop tickets are approved:

1. `start/play/submit` paths must keep returning typed unsupported.
2. No optimistic state transitions that imply real output exists.
3. No fake progress clocks.

## 8. Clean worktree policy

Before opening any new ticket:

1. `git status --short` must be clean.
2. No backup/temp/log artifacts are allowed in worktree.
3. EOL/index noise must be cleaned before feature work.

## 9. Scope control policy

1. If required files are outside `Allowed files`, stop and escalate.
2. Do not pull future-ticket work into current ticket.
3. Do not add dependencies unless the ticket explicitly allows it.

## 10. Required delivery format

Each ticket handoff must include:

1. `Ticket`
2. `Base`
3. `HEAD`
4. `Modified files`
5. `Added files`
6. `Deleted files`
7. `What changed`
8. `What did NOT change`
9. `Gates` results
10. `git diff --stat`
11. `git status --short`
12. `Commit hash`

## 11. Rule precedence

If rules conflict, apply this order:

1. Safety and honesty gates
2. Allowed/forbidden file scope
3. Layered delegation rules
4. File split/size rules
5. Ticket convenience

## 12. Historical work handling policy

When new rules are introduced:

1. New and ongoing tickets must follow the latest rules immediately.
2. Already completed historical tickets are not rewritten in-place by default.
3. Historical cleanups must be deferred into a dedicated stabilization pass.
4. Stabilization pass items must be tracked as explicit tickets, not mixed into feature tickets.
5. No retroactive broad refactor is allowed during active feature progression unless explicitly approved.
