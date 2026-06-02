# Kivo Audio Core Design Checklist

> Ticket: KIVO-AUDIO-DESIGN-CHECKLIST-P0  
> Scope: executable design checklist for future backend audio-core work  
> Status: documentation-only  
> Related audit: `docs/audio-core-design-audit.md`

## 0. Mandatory Iron Law: No Code Piling

This rule is mandatory and must be checked before every audio-core code change.

Kivo audio code must not be piled into large mixed files. A file can only have one clear responsibility. When a file becomes large or starts taking on a second responsibility, the correct action is to split it by layer, function, or sub-responsibility. Do not keep adding unrelated logic just because the file already exists.

Hard requirements:

- [ ] One source file = one responsibility.
- [ ] One folder = one responsibility class.
- [ ] Large files must be split when they become hard to review or mix responsibilities.
- [ ] Do not mix manager logic, decoder logic, output logic, event logic, queue logic, and command logic in the same file.
- [ ] Do not add helper functions to a random existing file just because it is convenient.
- [ ] Do not create vague dumping files such as `utils.rs`, `helpers.rs`, `misc.rs`, or `common.rs` unless the responsibility is narrow, named, and justified.
- [ ] If a change needs a new responsibility, create a focused new file/module for that responsibility.
- [ ] If a file grows because tests or types are being added, move tests/types to focused sibling files when appropriate.
- [ ] If the developer cannot explain the file's single job in one sentence, the file is already wrong.

Review threshold:

- Around 200-250 lines is a warning zone, not an automatic failure.
- Exceeding that range requires a written justification in the delivery report.
- If a file exceeds that range because multiple responsibilities are mixed, it must be split before delivery.
- A smaller file can still be rejected if it mixes responsibilities.

Bad example:

```text
manager.rs does all of this:
- queue selection
- decoder opening
- output frame submission
- WASAPI device policy
- event mapping
- metadata probing
```

Correct example:

```text
manager.rs                 high-level coordination only
native_pipeline.rs         decode/output orchestration only
decoders/wav_decoder.rs    WAV decoding only
native_output.rs           native output sink boundary only
playback_event_bus.rs      event dispatch/bus only
commands.rs                Tauri command boundary only
```

Any ticket that violates this rule must be rejected even if it compiles.

## 1. Purpose

This checklist turns the Kivo audio-core design rules into concrete review items. Every future backend audio ticket must use this document before implementation and again before delivery.

The goal is not only to make the audio core work. The goal is to make it maintainable for years.

Kivo must not become a pile of cross-calls, giant files, temporary shortcuts, or fake playback claims.

## 2. Tree-Layer Design Checklist

The audio core must follow a strict tree-shaped control model.

```text
Tauri Command Layer
└── PlaybackManagerState
    └── PlaybackManager
        └── PlaybackEngine / KivoNativeEngine
            └── NativePipeline
                ├── Decoder Runtime
                │   └── Concrete Decoder
                ├── Output Runtime
                │   └── Concrete Output Sink
                └── Worker / Event Runtime
```

Before any code change, verify:

- [ ] The changed layer is clearly identified.
- [ ] The changed layer only calls its direct child layer.
- [ ] No high-level layer skips into low-level internals.
- [ ] No low-level layer imports UI/Tauri/page concepts.
- [ ] No decoder code mutates manager state.
- [ ] No output code mutates queue state.
- [ ] No command code opens decoders or submits audio frames directly.
- [ ] No manager code contains concrete WASAPI, FFmpeg, mpv, or WAV parsing details.

If any item fails, stop and redesign before coding.

## 3. Folder Responsibility Checklist

Each folder must own one responsibility class.

Expected ownership:

```text
playback/commands.rs                  command boundary only
playback/manager.rs                   high-level coordination only
playback/engine.rs                    engine trait only
playback/backends/                    backend descriptors / backend adapters
playback/native_pipeline.rs           internal pipeline orchestration
playback/decoder*.rs                  decoder contracts and runtime state
playback/decoders/                    concrete decoder implementations
playback/output*.rs                   output contracts and runtime state
playback/native_output.rs             native output sink boundary
playback/windows_audio.rs             Windows audio plan/status/capability boundary
playback/playback_worker_*            worker command/state/transition rules
playback/events.rs                    event data types only
playback/playback_event_*             event dispatch/bus/bridge only
playback/activity_*                   activity log storage/snapshots only
playback/lifecycle_*                  lifecycle activity/commands only
```

Review items:

- [ ] The folder still has one responsibility after the change.
- [ ] No folder becomes a mixed dumping ground.
- [ ] New code is placed under the most specific existing folder.
- [ ] New folder is created only when it has a stable responsibility boundary.
- [ ] No new generic `utils`, `helpers`, or `misc` dumping folder is introduced.

## 4. File Responsibility Checklist

Each file must have one job.

Review items:

- [ ] The file name describes exactly one responsibility.
- [ ] The file does not mix policy + runtime + IO + tests + formatting.
- [ ] The file does not grow into an oversized coordinator.
- [ ] New types are split if they belong to a different responsibility.
- [ ] Tests are either in focused test files or tightly scoped module tests.
- [ ] No drive-by refactor is mixed into the ticket.

Hard split requirements:

- [ ] If a file starts doing more than one job, split it before delivery.
- [ ] If a feature needs a new responsibility, add a focused file instead of piling into an existing file.
- [ ] If a touched file becomes large because unrelated logic was added, split by responsibility.
- [ ] If the only reason code is in a file is convenience, move it to the correct layer/file.

Review threshold:

- [ ] Around 200-250 lines requires extra attention.
- [ ] Over the threshold requires written justification in delivery.
- [ ] Over the threshold with mixed responsibilities is not allowed.
- [ ] A file under the threshold can still fail if it mixes responsibilities.

## 5. Truthfulness Checklist

Kivo must never claim unsupported features are implemented.

Review items:

- [ ] Backend status honestly reports whether real output is available.
- [ ] Native playback is not described as implemented until audio actually plays.
- [ ] WASAPI is not described as supported until real shared/exclusive output exists.
- [ ] Bit-perfect is not described as supported until format negotiation proves it.
- [ ] mpv is not described as active unless a real runtime adapter exists.
- [ ] FFmpeg is not described as the playback core.
- [ ] Unsupported operations return typed unsupported errors.
- [ ] User-facing wording distinguishes planned vs available.

Allowed wording:

```text
scaffolded
planned
unsupported
not wired yet
internal pipeline boundary
native-first architecture
compatibility backend descriptor
```

Forbidden wording unless true:

```text
real playback implemented
production-ready audio engine
WASAPI output supported
bit-perfect supported
mpv backend active
FFmpeg playback core
```

## 6. Current Reality Checklist

As of this checklist, the backend audio core should be treated as:

- [x] Native-first architecture direction exists.
- [x] Playback manager boundary exists.
- [x] Engine trait exists.
- [x] Native engine scaffold exists.
- [x] Native pipeline scaffold exists.
- [x] WAV decoder direction exists.
- [x] Output sink boundary exists.
- [x] Event/activity-log boundary exists.
- [x] Queue boundary exists.
- [ ] Real native playback exists.
- [ ] Real output submission exists.
- [ ] WASAPI shared mode exists.
- [ ] WASAPI exclusive mode exists.
- [ ] Bit-perfect output exists.
- [ ] Real mpv backend runtime exists.
- [ ] Multi-format decode beyond staged WAV work exists.
- [ ] Production playback worker loop exists.

Any implementation plan must start from this reality, not from a future assumption.

## 7. Command Layer Checklist

For `playback/commands.rs`:

- [ ] Commands only call managed state / service boundary.
- [ ] Commands do not open files directly.
- [ ] Commands do not parse audio formats.
- [ ] Commands do not submit frames.
- [ ] Commands do not know concrete output device behavior.
- [ ] Commands return typed serializable results.
- [ ] Commands preserve activity/event recording through approved helpers.

## 8. Manager Layer Checklist

For `PlaybackManagerState` and `PlaybackManager`:

- [ ] State wrapper only locks and delegates.
- [ ] Poisoned lock handling remains consistent.
- [ ] Manager coordinates queue and primary engine only.
- [ ] Manager does not perform decoder work.
- [ ] Manager does not perform output work.
- [ ] Manager does not contain WASAPI/mpv/FFmpeg details.
- [ ] Queue policy stays in queue-related modules.
- [ ] Engine runtime stays under engine/pipeline modules.

Special risk:

- [ ] Repeated lock/delegate boilerplate has not grown into unreadable code.
- [ ] If many new manager methods are added, introduce focused helper extraction.

## 9. Engine Layer Checklist

For `PlaybackEngine` and `KivoNativeEngine`:

- [ ] Engine maps high-level playback commands to pipeline operations.
- [ ] Engine owns engine-level state only.
- [ ] Engine descriptor/capability data remains honest.
- [ ] Engine does not implement concrete decoder internals.
- [ ] Engine does not implement concrete output internals.
- [ ] Engine does not know Tauri command details.
- [ ] Engine command semantics are atomic or explicitly documented.

Critical check:

- [ ] Failed load/decode/play operations must not silently corrupt committed playback state.

## 10. Pipeline Layer Checklist

For `NativePipeline` and worker transition modules:

- [ ] Pipeline owns decode/output orchestration.
- [ ] Pipeline command routing is explicit.
- [ ] Pipeline state transitions are test-covered.
- [ ] Runtime errors are captured in pipeline/output status.
- [ ] Pipeline does not know UI or Tauri details.
- [ ] Pipeline does not own queue policy.
- [ ] Pipeline does not scrape metadata.
- [ ] Pipeline does not claim output is real while output sink is unsupported.

Before WASAPI work:

- [ ] Pipeline can open decoder session.
- [ ] Pipeline can perform at least one controlled decode step.
- [ ] Pipeline can route decoded frames to an internal/null sink for tests.
- [ ] Pipeline can record EOF/error states.

## 11. Decoder Layer Checklist

For `decoder.rs`, `decoder_request.rs`, `decoder_session.rs`, `decoders/*`:

- [ ] Decoder interface only decodes audio.
- [ ] Concrete decoder does not mutate playback manager state.
- [ ] Concrete decoder does not emit UI events.
- [ ] Concrete decoder does not choose output devices.
- [ ] Concrete decoder errors are typed.
- [ ] Unsupported format returns `UnsupportedFormat`.
- [ ] Seek behavior is test-covered.
- [ ] Frame position calculation is test-covered.
- [ ] End-of-stream behavior is explicit.

Future expansion:

- [ ] Multi-format support must go behind the decoder factory/adapter boundary.
- [ ] FFmpeg/Symphonia must not leak into manager/command/UI layers.

## 12. Output Layer Checklist

For `output.rs`, `native_output.rs`, future output sinks:

- [ ] Output sink only opens/closes devices and submits frames.
- [ ] Output sink does not select queue tracks.
- [ ] Output sink does not decode audio.
- [ ] Output sink status reports open/active/device/latency/gaps/errors.
- [ ] Unsupported output operations return typed unsupported errors.
- [ ] Shared mode is implemented before exclusive mode.
- [ ] Exclusive mode fallback behavior is documented before implementation.
- [ ] Bit-perfect is not claimed without format negotiation tests.

## 13. Windows/WASAPI Checklist

Before implementing WASAPI:

- [ ] Native pipeline decode path works internally.
- [ ] Null output/internal output test path exists.
- [ ] Device capability structures are stable.
- [ ] Shared-mode plan is documented.
- [ ] Exclusive-mode plan is explicitly deferred.
- [ ] Device lost behavior is planned.
- [ ] Format mismatch behavior is planned.
- [ ] Latency/gap reporting behavior is planned.

During WASAPI shared-mode implementation:

- [ ] No UI changes.
- [ ] No frontend settings claims unless backend capability is real.
- [ ] No bit-perfect claim.
- [ ] No exclusive-mode implementation in same ticket.
- [ ] Tests cover unsupported/fallback status where possible.

## 14. Event and Activity Checklist

For event/activity modules:

- [ ] Command result events map correctly to state/track/error events.
- [ ] Progress events are throttled.
- [ ] Runtime events are separated from command result events where needed.
- [ ] Activity log remains append/snapshot/clear only.
- [ ] Event modules do not perform playback decisions.
- [ ] Event modules do not decode or output audio.

Before real playback:

- [ ] Track ended event is planned.
- [ ] Runtime decode error event is planned.
- [ ] Output device error event is planned.
- [ ] Progress event source comes from runtime state, not only command result.

## 15. Queue Checklist

For queue-related work:

- [ ] Queue policy stays separate from audio output.
- [ ] Queue next/previous selection is test-covered.
- [ ] Failed load from queue does not destroy current valid state unless explicitly designed.
- [ ] Repeat/shuffle behavior stays in queue policy modules.
- [ ] Queue does not know decoder/output internals.

## 16. Dependency Checklist

Before adding any dependency:

- [ ] The ticket explicitly states why the dependency is needed.
- [ ] The dependency belongs to the correct layer.
- [ ] License/distribution risk is checked.
- [ ] The dependency does not force UI or unrelated architecture changes.
- [ ] The dependency is not added as a shortcut around existing boundaries.

Special dependencies:

- [ ] FFmpeg must remain behind probe/decoder adapter boundaries.
- [ ] mpv must remain a compatibility backend, not primary architecture.
- [ ] WASAPI/windows crates must remain under output/device boundary.

## 17. Ticket Scope Checklist

Every backend audio ticket must state:

```text
Ticket ID:
Base commit:
Target layer:
Direct child layer touched:
Allowed files:
Forbidden files:
Goal:
Non-goals:
Acceptance:
Rollback plan:
```

Mandatory non-goals unless explicitly approved:

- [ ] No frontend UI.
- [ ] No broad refactor.
- [ ] No mpv runtime.
- [ ] No FFmpeg integration.
- [ ] No WASAPI exclusive mode.
- [ ] No fake playback claims.
- [ ] No unrelated settings/library/search changes.

## 18. Required Gates

Every backend audio delivery must include:

```text
cargo fmt --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
git diff --stat
git status --short
```

Delivery must report:

- [ ] Commit hash.
- [ ] Changed files only.
- [ ] Gate output key lines.
- [ ] Whether working tree is clean.
- [ ] Layer ownership check result.
- [ ] Truthfulness check result.
- [ ] Single-responsibility check result.
- [ ] No-code-piling check result.
- [ ] Any file over the warning threshold and the split/justification decision.

## 19. Recommended Next Ticket

```text
KIVO-AUDIO-NATIVE-DECODE-PIPELINE-P0-009
```

Goal:

```text
Connect the WAV decoder path into NativePipeline enough to open a decoder session and execute one controlled decode step, without real audio output.
```

Allowed direction:

- [ ] Use existing decoder factory.
- [ ] Open WAV decoder from pipeline request.
- [ ] Store decoder/session state in pipeline-owned structures.
- [ ] Execute one decode step.
- [ ] Convert decoded frame to pipeline/output boundary structure if needed.
- [ ] Keep real output unsupported.
- [ ] Add focused tests.

Forbidden direction:

- [ ] Do not implement WASAPI.
- [ ] Do not implement mpv runtime.
- [ ] Do not introduce FFmpeg.
- [ ] Do not modify frontend.
- [ ] Do not claim playback is working.
- [ ] Do not merge metadata scraping into decoder.
- [ ] Do not pile multiple new responsibilities into one existing file.
- [ ] Do not exceed the file warning threshold without split/justification.

## 20. Final Rule

If a future implementation makes the code easier to demo but harder to maintain, reject it.

Kivo audio core must be:

```text
Layered.
Honest.
Native-first.
Testable.
Single-responsibility.
Tree-governed.
No fake playback claims.
No giant mixed files.
No code piling.
Split large/mixed files before delivery.
```
