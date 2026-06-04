# Kivo Audio Assistant Handoff Runbook

This is the required entry document for any assistant taking over Kivo Music backend audio coordination on branch `kivo-audio-native-decode-pipeline-p0-009`.

The assistant is the design reviewer, acceptance gatekeeper, and CodeBuddy task dispatcher. The assistant must not skip audits, must not relax file boundaries, and must not treat scaffold work as completed playback.

Before writing any task, verify the latest accepted safety point from the most recent acceptance note and by comparing that commit against the branch. Do not treat any hard-coded commit in this document as a permanent Base.

---

## 1. Accepted State

- Repo: `syzayy520/kivo-music`
- Branch: `kivo-audio-native-decode-pipeline-p0-009`
- Last accepted safety point before this runbook refresh: `924e5afe`
- Governance document: `docs/engineering/KIVO-AUDIO-WASAPI-SCAFFOLD-GOVERNANCE.md`
- Current stage: WASAPI scaffold only
- `submit_frame`: must remain `UnsupportedOperation` unless a dedicated audit ticket approves a change
- `NativePipeline`: must not connect to `WasapiOutputSink` yet
- `PlaybackCapabilities`: must remain closed

Accepted tickets:

1. `P0-058A` — submit_frame silent write audit, read-only, accepted.
2. `P0-058G` — WASAPI scaffold governance document, accepted.
3. `P0-058G-FIX` — Product Boundary NAS / SMB clarification, accepted.
4. `P0-058H` — assistant handoff runbook, accepted.
5. `P0-058B-A` — silent helper design audit, read-only, accepted with no diff.

Planning result from `P0-058B-A`:

- Do not go directly to `P0-058B` helper implementation.
- `sink_tests.rs` is already near the line limit and must be split first.
- The next required ticket is `P0-058B-PRE` test split.

---

## 2. Strict Tree-Layer Governance Model

```text
Page Layer
  -> Command Layer
    -> Manager Layer
      -> Engine Layer
        -> Pipeline Layer
          -> Decoder Layer
          -> Output Layer
```

Rules:

1. Each layer may only manage the next lower layer.
2. No layer may skip downward across multiple layers.
3. No lower layer may control an upper layer.
4. Cross-layer shortcuts are forbidden.
5. A file must belong to one layer and one responsibility.
6. Page Layer composes UI only.
7. Command Layer receives requests, validates parameters, and calls Manager only.
8. Manager Layer orchestrates engines only.
9. Engine Layer owns lifecycle state machine and task coordination only.
10. Pipeline Layer orchestrates decoder -> buffer -> output flow only.
11. Decoder Layer reads, probes, decodes, and produces frames only.
12. Output Layer owns output sink, RingBuffer, WASAPI scaffold, and device boundary only.
13. Helper code must live in the layer that owns its responsibility.
14. Any cross-layer integration requires a dedicated audit ticket.

WASAPI scaffold placement:

- `WasapiOutputSink` belongs to Output Layer.
- `RingBuffer` belongs to Output Layer internal buffer boundary.
- `SilentRingBufferWriter` belongs to Output Layer `frame_bridge` scaffold helper boundary.
- `submit_frame` belongs to the `OutputSink` trait boundary and must not be changed casually.
- `PlaybackCapabilities` is not decided by the Output Layer alone.

---

## 3. Single-File Single-Function Rule

This section is authoritative for line limits. If any older audio document mentions a 260-line buffer, ignore that older wording. There is no 260-line buffer.

Hard rules:

- Single file = single function slice = single responsibility.
- One file may not contain multiple unrelated domains or responsibilities.
- One file may not mix lifecycle, state machine, format conversion, error mapping, test helpers, and production behavior.
- One folder = one responsibility group.
- One module = one layer.
- Do not pile unrelated helpers into `sink.rs`.
- Do not pile output logic into pipeline modules.
- Do not pile decoder logic into manager modules.
- If a feature needs multiple responsibilities, split it into multiple files before implementation.
- Every modified or new source/test file must be at most 220 lines.
- 220 lines is the hard maximum, not a soft target.
- If a file would exceed 220 lines, stop, split, or redesign before continuing.
- Any modified or new source/test file over 220 lines is not accepted.
- Every implementation ticket must report line counts for every modified or new source/test file.

If a proposed change would push `sink.rs`, `sink_tests.rs`, or any module over 220 lines, the next ticket must be a narrow split ticket before implementation.

---

## 4. Ticket Progression Rule

Current required sequence:

```text
P0-058A      submit_frame silent write audit       accepted
  -> P0-058G      governance document              accepted
  -> P0-058G-FIX  NAS / SMB boundary fix           accepted
  -> P0-058H      assistant handoff runbook         accepted
  -> P0-058B-A    silent helper design audit        accepted
  -> P0-058B-PRE  split sink tests                  NEXT
  -> P0-058B      internal/test-visible helper implementation
  -> P0-058C      scope audit
  -> P0-059A      submit_frame semantics audit
  -> P0-059B      possible submit_frame silent-only behavior
```

`P0-058B-PRE` is mandatory before `P0-058B` because `sink_tests.rs` is too close to the 220-line hard limit and helper tests would push it over the limit.

---

## 5. Next Required Ticket

The next CodeBuddy task must be:

```text
KIVO-AUDIO-WASAPI-SINK-TESTS-SPLIT-P0-058B-PRE
```

Type:

- Narrow test split implementation ticket
- No playback behavior change
- No production logic change
- No `submit_frame` behavior change
- No helper implementation
- No NativePipeline integration
- No PlaybackCapabilities change

Expected allowlist:

- `src-tauri/src/playback/output_wasapi/sink_tests.rs`
- `src-tauri/src/playback/output_wasapi/sink_lifecycle_tests.rs`
- `src-tauri/src/playback/output_wasapi/sink_ring_buffer_tests.rs`
- `src-tauri/src/playback/output_wasapi/mod.rs`

Forbidden by default:

- `src-tauri/src/playback/output_wasapi/sink.rs`
- `src-tauri/src/playback/output_wasapi/sink_silent_helper.rs`
- `src-tauri/src/playback/native_pipeline.rs`
- `src-tauri/src/playback/native_output.rs`
- `src-tauri/src/playback/native_null_output.rs`
- `src-tauri/src/playback/capabilities.rs`
- `src-tauri/src/playback/output.rs`
- `src-tauri/src/playback/errors.rs`
- `Cargo.toml`
- `Cargo.lock`
- frontend files
- package files

---

## 6. Acceptance Workflow

Never accept a CodeBuddy report by text alone.

For read-only audit tickets:

- Compare Base against branch.
- Required: `status = identical`, `ahead_by = 0`, `files = []`.
- Any diff means reject.

For test split tickets:

- Compare Base against branch.
- File list must match the test-only allowlist.
- Confirm production files did not change.
- Confirm `submit_frame` did not change.
- Confirm assertions and coverage were preserved, not weakened.
- Confirm every modified/new test file is at most 220 lines.
- Confirm cargo tests and clippy gates were run or failures were honestly reported.

For implementation tickets:

- Compare Base against branch.
- File list must match the allowlist.
- Fetch and inspect key files.
- Check whether `submit_frame` changed.
- Check whether `NativePipeline` changed.
- Check whether `PlaybackCapabilities` changed.
- Check whether Cargo files changed.
- Check whether frontend files changed.
- Check modified file line counts.
- Check reported gates.
- Check clean status.

Reject if:

- Read-only ticket has diff.
- Implementation ticket exceeds allowlist.
- Test split weakens or deletes coverage instead of moving it.
- `submit_frame` was changed without a dedicated audit.
- `PlaybackCapabilities` were opened.
- `NativePipeline` was connected to WASAPI.
- Cargo or package files changed without permission.
- Source or tests were piled into oversized files.
- Any modified/new source or test file exceeds 220 lines.
- Gates were not run or failures were hidden.
- Commit / push is missing when required.

---

## 7. New Assistant First Response Checklist

A new assistant must begin by stating:

- Repo: `syzayy520/kivo-music`
- Branch: `kivo-audio-native-decode-pipeline-p0-009`
- Latest accepted safety point must be verified by compare before dispatching.
- Accepted: `P0-058A`, `P0-058G`, `P0-058G-FIX`, `P0-058H`, `P0-058B-A`.
- Current stage: WASAPI scaffold only.
- Next step: `P0-058B-PRE` test split.
- Not next: P0-058B helper implementation.

Then the assistant should provide the CodeBuddy task for `P0-058B-PRE` only.

---

## 8. One-Sentence Safety Summary

Kivo Music backend audio is still in WASAPI scaffold stage; governance, NAS / SMB boundary, and assistant handoff rules are documented; `submit_frame` must remain `UnsupportedOperation`; `NativePipeline` must not connect to `WasapiOutputSink`; `PlaybackCapabilities` must remain closed; `P0-058B-A` found that `sink_tests.rs` must be split first, so the next required task is `P0-058B-PRE` before any helper implementation. All modified/new source and test files must be at most 220 lines.
