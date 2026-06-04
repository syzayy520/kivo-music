# Kivo Audio Assistant Handoff Runbook

**Scope**: This runbook is the required entry document for any new assistant taking over Kivo Music backend audio coordination on branch `kivo-audio-native-decode-pipeline-p0-009`.

**Role**: The assistant is the design reviewer, acceptance gatekeeper, and CodeBuddy task dispatcher. The assistant must not skip audits, must not relax file boundaries, and must not treat scaffold work as real playback.

---

## 1. Current Safe State

- **Repo**: `syzayy520/kivo-music`
- **Branch**: `kivo-audio-native-decode-pipeline-p0-009`
- **Current accepted safety point**: `a6d6d4c`
- **Governance document**: `docs/engineering/KIVO-AUDIO-WASAPI-SCAFFOLD-GOVERNANCE.md`
- **Current stage**: WASAPI scaffold only
- **Real playback**: not implemented
- **Audible Windows output**: not validated
- **NativePipeline integration**: not allowed yet
- **PlaybackCapabilities enablement**: not allowed yet
- **submit_frame**: must remain `UnsupportedOperation` unless a dedicated audit ticket approves a change

Accepted tickets:

1. `P0-058A` — submit_frame silent write audit, read-only, accepted.
2. `P0-058G` — WASAPI scaffold governance document, accepted.
3. `P0-058G-FIX` — Product Boundary NAS / SMB clarification, accepted.

Do not update this safety point unless remote diff inspection and acceptance pass.

---

## 2. Current Code Truth

The assistant must repeat these facts before planning any next task:

- `WasapiOutputSink` is still a lifecycle-only scaffold.
- `WasapiOutputSink::submit_frame` still returns `UnsupportedOperation`.
- `WasapiOutputSink` owns `Option<RingBuffer>` only as a scaffold boundary.
- `prepare_ring_buffer_for_stream` creates or replaces a `RingBuffer`; it does not write samples.
- `SilentRingBufferWriter` exists, but it is not connected to `submit_frame`.
- `RingBuffer::write_frames` must not be introduced into a production path unless explicitly allowed.
- `NativePipeline` is not connected to `WasapiOutputSink`.
- `KivoNativeOutputSink` must not be switched from `KivoNullOutputSink` to WASAPI without a dedicated audit.
- `PlaybackCapabilities` must remain closed.
- No non-silent data path may be introduced.
- No output thread may be started.
- No Windows audio API may be used from scaffold tickets.

---

## 3. Strict Tree-Layer Governance Model

Kivo backend audio must follow this strict tree-layer model:

```text
Page Layer
  -> Command Layer
    -> Manager Layer
      -> Engine Layer
        -> Pipeline Layer
          -> Decoder Layer
          -> Output Layer
```

Mandatory rules:

1. Each layer may only manage the next lower layer.
2. No layer may skip downward across multiple layers.
3. No lower layer may control an upper layer.
4. Cross-layer shortcuts are forbidden.
5. A file must belong to one layer and one responsibility.
6. A folder must belong to one responsibility group.
7. A module must belong to one layer.
8. Page Layer composes UI only.
9. Command Layer receives requests, validates parameters, and calls Manager only.
10. Manager Layer orchestrates engines only.
11. Engine Layer owns lifecycle state machine and task coordination only.
12. Pipeline Layer orchestrates decoder -> buffer -> output flow only.
13. Decoder Layer reads, probes, decodes, and produces frames only.
14. Output Layer owns output sink, RingBuffer, WASAPI scaffold, and device boundary only.
15. Output Layer must not manage Pipeline Layer.
16. Decoder Layer must not manage output devices.
17. Pipeline Layer must not modify UI, commands, manager state, or product capabilities.
18. Helper code must live in the layer that owns its responsibility.
19. If a change crosses more than one layer, create a read-only design audit first.
20. Cross-layer integration, such as `NativePipeline -> WasapiOutputSink`, requires a dedicated audit ticket.

WASAPI scaffold placement:

- `WasapiOutputSink` belongs to Output Layer.
- `RingBuffer` belongs to Output Layer internal buffer boundary.
- `SilentRingBufferWriter` belongs to Output Layer `frame_bridge` scaffold helper boundary.
- `submit_frame` belongs to the `OutputSink` trait boundary and must not be changed casually.
- `PlaybackCapabilities` is not decided by the Output Layer alone.

---

## 4. File Responsibility And Size Rules

Hard rules:

- One file, one responsibility.
- One folder, one responsibility group.
- One module, one layer.
- Do not pile unrelated helpers into `sink.rs`.
- Do not pile state machine logic into output modules.
- Do not pile output logic into pipeline modules.
- Do not pile decoder logic into manager modules.
- Do not mix production logic, test helper logic, error mapping, state management, and format conversion in one file.
- Do not expand a file for convenience when a small dedicated file would keep the boundary cleaner.
- A modified source file over 220 lines requires split or explicit justification before implementation.
- A modified source file over 260 lines fails implementation acceptance unless the ticket is read-only.
- Future implementation tickets must report line counts for every modified source/test file.

If a proposed change would push `sink.rs`, `sink_tests.rs`, or any module over the limit, the next ticket must be a split/refactor audit or a narrow test split ticket before implementation.

---

## 5. Product Boundary

Kivo Music is a local-first desktop music player for user-controlled music files.

Forbidden positive semantics:

- Resource search
- Download entry
- Cloud-drive resource search
- Automatic cloud transfer
- Magnet links
- Torrents
- Piracy resource sites
- External platform aggregation
- Unauthorized third-party online resource discovery

Allowed local ownership semantics:

- User-owned local files
- User-selected local folders
- User-owned local NAS
- SMB / local network shares
- Local network storage indexing when explicitly assigned
- Local-first library management for user-controlled storage

Do not confuse user-owned NAS / SMB storage with cloud-drive resource search or piracy resource aggregation.

---

## 6. Ticket Progression Rule

The assistant must classify every next step before writing a task:

1. Read-only audit
2. Governance document
3. Design audit
4. Implementation
5. Scope audit
6. Fix ticket
7. Gate-only verification

Never jump from governance directly to implementation when the implementation introduces a new helper, new file boundary, line-count risk, trait semantics risk, or cross-layer risk.

Current required sequence:

```text
P0-058A  submit_frame silent write audit       accepted
  -> P0-058G      governance document          accepted
  -> P0-058G-FIX  NAS / SMB boundary fix       accepted
  -> P0-058B-A    silent helper design audit   NEXT
  -> P0-058B      internal/test-visible helper implementation
  -> P0-058C      scope audit
  -> P0-059A      submit_frame semantics audit
  -> P0-059B      possible submit_frame silent-only behavior
  -> later        output thread consumption audit
  -> later        NativePipeline integration audit
  -> later        PlaybackCapabilities enablement audit
```

`P0-058B-A` is mandatory. Do not send an implementation ticket for `P0-058B` until `P0-058B-A` is accepted.

---

## 7. Next Required Ticket

The next CodeBuddy task must be:

```text
KIVO-AUDIO-WASAPI-SILENT-HELPER-DESIGN-AUDIT-P0-058B-A
```

Type:

- Read-only design audit
- No code changes
- No Markdown changes
- No commit
- No push

Purpose:

- Decide where the helper belongs.
- Decide whether a new file is required.
- Decide whether `sink.rs` or `sink_tests.rs` would exceed 220 / 260 lines.
- Decide whether test splitting must happen before implementation.
- Confirm `submit_frame` must stay unchanged.
- Confirm the helper remains inside the Output Layer.
- Confirm P0-058B allowlist and forbidden files.

Required decision points:

- Current `submit_frame` state.
- Current `sink.rs` line count.
- Current `sink_tests.rs` line count.
- Whether helper in `sink.rs` violates one-file-one-responsibility.
- Whether helper should live in a new file.
- Whether tests should be split first.
- Helper name.
- Helper visibility: `pub(crate)` vs `#[cfg(test)]`.
- Whether helper may call `SilentRingBufferWriter`.
- Whether helper may indirectly call `RingBuffer::write_frames`.
- Whether helper may update `pending_frames`.
- Whether helper may update `last_error`.
- Error behavior for `ring_buffer == None`.
- Error behavior for non-silent frame.
- Implementation allowlist for P0-058B.
- Forbidden files for P0-058B.
- Gates for P0-058B.

---

## 8. CodeBuddy Task Writing Rules

Every CodeBuddy task must be a single copy-paste block and must include:

1. Ticket ID
2. Task type
3. Repo / Branch / Base
4. Current accepted state
5. Purpose
6. Highest architecture rules
7. Safety state confirmation commands
8. Read-only scope or allowlist
9. Forbidden files
10. Audit questions or implementation steps
11. 220 / 260 line-count rules
12. Gates
13. `cargo fmt` non-allowlist rule
14. Commit rule
15. Push rule
16. Required delivery report format
17. Final safety statement

Mandatory text in implementation tickets:

- Do not `git add .`.
- Do not force push.
- Do not modify non-allowlist files.
- If HEAD is not the stated Base, stop.
- If `git status --short` is not clean at start, stop.
- If current code differs from assumptions, stop and report.
- If `cargo fmt` changes non-allowlist files, stop and do not stage them.
- If any file exceeds 220 lines, split or justify before continuing.
- If any file exceeds 260 lines, stop unless the ticket explicitly allows it.

---

## 9. Acceptance Workflow

Never accept a CodeBuddy report by text alone.

For read-only audit tickets:

- Compare Base against branch.
- Required: `status = identical`, `ahead_by = 0`, `files = []`.
- Any diff means reject.

For Markdown-only tickets:

- Compare Base against branch.
- Only the allowlisted Markdown file may change.
- No source, config, Cargo, package, README, or UI changes.
- Fetch and inspect the changed section.

For implementation tickets:

- Compare Base against branch.
- File list must match the allowlist.
- Fetch and inspect key files.
- Check whether `submit_frame` changed.
- Check whether `NativePipeline` changed.
- Check whether `PlaybackCapabilities` changed.
- Check whether non-silent path was introduced.
- Check whether Cargo files changed.
- Check whether UI or frontend files changed.
- Check modified file line counts.
- Check reported gates.
- Check clean status.

Reject if:

- Read-only ticket has diff.
- Implementation ticket exceeds allowlist.
- `submit_frame` was changed without a dedicated audit.
- `PlaybackCapabilities` were opened.
- `NativePipeline` was connected to WASAPI.
- Non-silent path was introduced.
- Cargo or package files changed without permission.
- Source or tests were piled into oversized files.
- A modified implementation file exceeds 260 lines.
- Gates were not run or failures were hidden.
- Commit / push is missing when required.

---

## 10. Non-Negotiable Wording Rules

Allowed wording:

- WASAPI scaffold
- output boundary scaffold
- silent-only helper
- internal/test-visible helper
- RingBuffer ownership scaffold
- no real device opened
- no audible output
- not connected to NativePipeline

Forbidden wording unless real Windows audible validation exists:

- WASAPI ready
- native playback ready
- real output done
- playback complete
- audio output implemented
- native engine completed
- device output working

---

## 11. New Assistant First Response Checklist

A new assistant must begin by stating:

- Repo: `syzayy520/kivo-music`
- Branch: `kivo-audio-native-decode-pipeline-p0-009`
- Safety point: `a6d6d4c`
- Accepted: `P0-058A`, `P0-058G`, `P0-058G-FIX`
- Current stage: WASAPI scaffold only
- Next step: `P0-058B-A` read-only design audit
- Not next: P0-058B implementation

Then the assistant should provide the CodeBuddy task for `P0-058B-A` only.

---

## 12. One-Sentence Safety Summary

As of `a6d6d4c`, Kivo Music backend audio is still in WASAPI scaffold stage; governance and NAS / SMB boundary are documented; `submit_frame` must remain `UnsupportedOperation`; `NativePipeline` must not connect to `WasapiOutputSink`; `PlaybackCapabilities` must remain closed; the next required step is `P0-058B-A` read-only design audit before any helper implementation.
