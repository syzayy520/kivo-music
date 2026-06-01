# Kivo Music Startup Checklist v2

> Status: active engineering checklist.  
> Scope: Kivo Music Windows desktop local music player.  
> Purpose: every new Kivo Music task window must read this document before editing.  
> Rule: this document is a planning and execution guardrail, not proof that any playback feature is complete.

---

## 1. Project Positioning

Kivo Music is a Windows desktop local music player.

The product target is a high-end, local-first, restrained, elegant, professional desktop music player. The product direction is not a web resource tool, not a cloud-drive player, not a downloader, not a magnet/torrent client, not a piracy-resource aggregator, and not an online resource search product.

Current mainline:

- Native-first self-controlled playback core.
- Local music library and local playback experience.
- High-quality Windows desktop playback foundation.
- Strict engineering discipline: small tickets, explicit file boundaries, gates, and completion reports.

Forbidden product semantics:

- resource search
- cloud-drive search
- automatic cloud transfer
- download entry
- magnet
- torrent
- piracy resource sites
- external platform aggregation
- any wording that makes Kivo look like a piracy/resource player

These words may appear only in negative guardrails like this section.

---

## 2. Definition of the Self-Controlled Playback Core

`KivoNativeEngine` is the entry point and control surface for Kivo Music's self-controlled native playback core.

Kivo should self-control the playback chain at these layers:

- playback command semantics: `load`, `play`, `pause`, `resume`, `stop`, `seek`
- playback state machine
- queue and playback plan: current, next, previous, repeat, shuffle, play next
- decoder scheduling
- PCM frame buffering
- pipeline orchestration
- output sink abstraction
- Windows WASAPI output strategy
- event and progress propagation
- error state and recovery behavior
- later: gapless, bit-perfect, exclusive mode, device selection, sample-rate strategy

Important definition:

- WASAPI is a Windows system audio API. Kivo does not reinvent Windows audio APIs or drivers.
- Kivo can still self-control its WASAPI wrapper, output strategy, buffer policy, device policy, and playback scheduling.
- Codec support may use mature libraries. Using mature codec libraries does not mean the playback core is not self-controlled.
- `mpv` is only a future compatibility or fallback backend. It is not the primary Kivo playback core.
- FFmpeg / ffprobe may be used for probing, metadata, and format support boundaries. They should not be described as the primary Kivo playback core.

Correct product framing:

```text
Kivo uses a Native-first self-controlled playback core.
Kivo controls playback state, queue, decoder scheduling, buffering,
output abstraction, WASAPI strategy, and event propagation.
Lower-level OS audio APIs and mature codec libraries may be used where appropriate.
```

Incorrect framing:

```text
Kivo already has a complete native playback core.
Kivo rewrites all codecs from scratch.
Kivo replaces Windows audio APIs.
mpv is Kivo's main playback core.
FFmpeg is Kivo's playback engine.
```

---

## 3. Current Fact Baseline

This section summarizes the current engineering understanding from recent audits. Treat it as a planning baseline, not as a guarantee that the repository state is unchanged. Every task must still inspect the current HEAD before editing.

Known frontend facts:

- The frontend still contains significant mock data.
- Frontend playback state and backend playback state are not yet fully bridged.
- PlayerBar, QueuePreview, Now Playing, Lyrics, and related playback UI must not be prematurely wired to an unstable backend.
- Frontend i18n still has hardcoded UI strings that can be cleaned independently.
- Frontend playback bridge work must wait until the backend WAV playback loop is actually stable.

Known backend facts:

- The backend already has useful skeleton pieces: commands, PlaybackManager, queue, state machine, event system, WAV decoder, and WASAPI infrastructure.
- The true playback chain is still under construction.
- A feature is not complete until the chain has been tested end to end.
- Do not claim real playback is done unless Windows hardware testing confirms actual audible output.
- Current backend priority is to make the WAV backend playback loop real and stable.

Critical distinction:

```text
Infrastructure exists != playback is complete.
Returning Ok != real playback is complete.
A state transition != audible playback is complete.
A WAV decoder existing != output is complete.
WASAPI helpers existing != OutputSink is complete.
```

---

## 4. Current Division of Work

### 4.1 Current Web GPT Window

The current web GPT coding window owns backend audio core work.

Responsible scope:

- `src-tauri/src/playback/**`
- `KivoNativeEngine`
- NativePlayback
- NativeBackend
- NativePipeline
- NativeOutputSink
- WAV decoder integration
- WASAPI OutputSink integration
- `load`, `play`, `pause`, `resume`, `stop`
- backend playback state machine progress
- cargo tests
- Windows real-device playback validation

Forbidden unless explicitly assigned:

- frontend UI changes
- i18n cleanup
- library/database work
- SQLite work
- cover cache work
- settings page work
- search work
- FLAC / MP3 / AAC / ALAC before WAV closure
- drive-by refactors outside the backend audio ticket

### 4.2 Xiaomi MiMo Window

Xiaomi MiMo is a side-lane helper. It should not touch backend audio core while another window is working there.

Allowed work:

- i18n hardcoded string cleanup
- frontend mock dependency audit
- task brief drafting
- low-risk structure audit
- locale key completion
- read-only review

Forbidden by default:

- `src-tauri/**`
- backend audio core
- Native Engine
- WASAPI implementation
- decoder implementation
- OutputSink implementation
- playback bridge
- mock replacement
- dependency changes
- visual UI changes

### 4.3 Design / Validation Window

The design and validation window owns task selection and acceptance.

Responsibilities:

- filter audit reports
- prevent scope creep
- compress large tasks into small tickets
- validate backend completion reports
- decide when the project can move to the next phase
- prevent frontend, backend, i18n, and product-positioning work from colliding

---

## 5. Phase Roadmap

### Phase 0: Freeze Boundaries

Status: active.

Goals:

- Backend audio core proceeds through one owner only.
- Xiaomi MiMo does not touch backend audio core.
- Frontend playback bridge does not start early.
- i18n cleanup can proceed independently.

Acceptance:

- No two windows modify `src-tauri/**` at the same time.
- No two windows modify playback bridge files at the same time.
- No package or lock files are touched unless the ticket explicitly allows it.
- No visual UI changes occur in non-UI tickets.
- Every active task has a Ticket ID and allowed/forbidden file list.

### Phase 1: WAV Backend Real Playback Loop

This is the current backend mainline.

Goal:

```text
local WAV file
  -> playback_load succeeds
  -> playback_play produces audible output
  -> pause pauses
  -> resume resumes
  -> stop stops and releases resources
  -> no Tauri main-thread blocking
  -> cargo tests pass
  -> Windows real-device validation passes
```

Recommended ticket split:

1. `P0-WAV-LOAD-DECODER`
   - only make load succeed for WAV
   - no output
   - no frontend

2. `P0-WASAPI-OUTPUT-SINK`
   - only make OutputSink open WASAPI and accept frames
   - no queue auto-advance
   - no format expansion

3. `P0-WAV-DECODE-LOOP`
   - only connect decoder frame production to output submission
   - no frontend
   - no metadata/library work

4. `P0-WAV-PLAYBACK-CONTROL`
   - only connect play/pause/resume/stop semantics and state
   - no seek unless explicitly included

5. `P0-WAV-PROGRESS-BASIC`
   - only basic position/duration progress
   - no complex event bridge

Forbidden during Phase 1:

- FLAC
- MP3
- AAC
- ALAC
- frontend playback bridge
- queue auto-advance
- gapless
- exclusive mode
- device selection UI
- broad audio architecture rewrite

### Phase 2: Backend Playback Stabilization

Start only after WAV real output exists.

Goals:

- playback thread safety
- no leftover playback thread after stop
- stable pause/resume
- correct end-of-track state
- error propagation
- repeated load does not leak resources
- repeated play/stop does not panic
- output device resources are released correctly

Focus areas:

- COM threading model
- WASAPI Start / Stop / Reset lifecycle
- output buffer lifetime
- decoder lifetime
- pipeline shutdown
- error state handling
- PlaybackState synchronization

Required validation questions:

```text
Was sound actually audible?
Was it tested on Windows hardware?
Did play work?
Did pause work?
Did resume work?
Did stop release resources?
Can the same file be loaded and played again?
Is the Tauri UI responsive during playback?
Are there any panics, deadlocks, or resource leaks?
```

### Phase 3: Frontend Playback Bridge

Do not start now. Start only after backend WAV playback is stable.

Ticket split:

1. `FE-PLAYBACK-STATE-BRIDGE`
   - only connect `playback_get_state`
   - no UI redesign

2. `FE-PLAYBACK-CONTROLS-BRIDGE`
   - only connect play/pause/resume/stop buttons
   - no queue bridge

3. `FE-PLAYBACK-EVENTS-BRIDGE`
   - only connect state events or polling update
   - no full player refactor

4. `FE-QUEUE-BRIDGE-BASIC`
   - only connect queue get/append/remove
   - no drag reorder unless explicitly assigned

5. `FE-MOCK-RETIRE-PLAYER`
   - gradually retire playerState/playerData/queueData mock dependencies
   - no new mock state source

Forbidden during Phase 3 unless explicitly assigned:

- visual redesign
- PlayerBar rewrite
- search implementation
- library/database implementation
- cover cache implementation
- settings implementation

### Phase 4: Format Expansion

Start only after WAV is stable.

Recommended order:

1. FLAC
2. MP3
3. AAC / M4A
4. ALAC
5. OGG / OPUS

Rules:

- One format per ticket.
- Each format must include load validation.
- Each format must include decode validation.
- Each format must include real playback validation.
- Unsupported/corrupt format handling must be explicit.
- Do not add a full codec stack casually if one smaller integration is enough for the current ticket.

### Phase 5: Advanced Audio Selling Points

Start only after foundational playback is stable.

Possible features:

- WASAPI exclusive mode
- bit-perfect output
- device selection
- sample-rate matching
- gapless playback
- optional crossfade
- replay gain
- audio device hot reload
- ASIO as a later optional path
- output latency tuning

Rules:

- One selling point per ticket.
- Every selling point must include measurable validation.
- Never claim audiophile-grade behavior without proof.

### Phase 6: Library and Large-Scale Local Collection

Start after playback foundation is stable enough.

Goals:

- SQLite library
- incremental scan
- 300k track scale target
- Tracks / Albums / Artists / Folders
- search
- sort
- filter
- virtual list
- cover cache
- metadata backup/restore

Rules:

- Frontend displays data; it must not compute huge library aggregations on the UI thread.
- Pagination/query/aggregation should be backend-owned where practical.
- Frontend large lists must use virtualization.
- Data model and i18n must remain clean from the beginning.

---

## 6. Current Ticket Board

### Active

- Web GPT backend window: current `P0-REAL-AUDIO` backend audio core task.

### Xiaomi MiMo Allowed

- `KIVO-MUSIC-I18N-HARDCODED-STRINGS-P1-001`
  - Clean frontend hardcoded UI strings.
  - Complete all locale keys.
  - Do not touch backend.

### Paused / Waiting

- `FE-PLAYBACK-BRIDGE`
  - wait until backend WAV playback is stable

- `MOCK-RETIRE`
  - wait until bridge has real state

- `QUEUE-FRONTEND-BRIDGE`
  - wait until backend playback state is stable

### Backend Candidate Tickets

These must be regenerated from the current backend completion report. Do not execute old audit tickets blindly.

- `P0-WAV-LOAD-DECODER`
- `P0-WASAPI-OUTPUT-SINK`
- `P0-WAV-DECODE-LOOP`
- `P0-WAV-PLAYBACK-CONTROL`
- `P0-WAV-PROGRESS-BASIC`

---

## 7. Kivo Music Execution Rules v2

1. Every task must have a Ticket ID.
2. Every task must state Base HEAD.
3. Every task must state allowed files.
4. Every task must state forbidden files.
5. No whole-file replacement unless the ticket explicitly says so.
6. No broad unrelated formatting.
7. No drive-by refactor.
8. No visual UI changes unless the ticket explicitly asks for UI work.
9. No package or lock file changes unless explicitly allowed.
10. No two windows may modify the same module at the same time.
11. Backend audio work must have only one active owner.
12. Xiaomi MiMo does not touch `src-tauri/**` by default.
13. Frontend playback bridge waits for proven backend playback.
14. Do not create new mock sources.
15. Do not create duplicate playback state sources.
16. Every new UI string must use i18n.
17. All locale files must keep key parity.
18. Do not add resource/download/cloud/magnet/torrent/piracy/online aggregation semantics.
19. Do not claim completion without gates and, where relevant, real-device validation.
20. Every completion report must include changed files, what changed, what did not change, gates, and commit.
21. Always inspect current code before editing.
22. If the current code differs from the task assumptions, stop and report instead of guessing.
23. Keep files small and single-responsibility.
24. Pages should compose/call/render; business logic should be moved into appropriate modules over time.
25. Do not pile display data, playback state, queue state, and backend state into one file.

---

## 8. Backend Audio Completion Report Template

Every backend audio ticket must report the following.

### State Header

- Repo:
- Branch:
- Base HEAD:
- New HEAD:
- Ticket ID:
- Working tree:

### Modified Files

List each modified file.

### Playback Chain Covered By This Ticket

Examples:

- load
- decoder
- output sink
- decode loop
- play/pause/stop
- progress
- events

### Explicitly Not Covered

Examples:

- FLAC not done
- MP3 not done
- frontend not done
- queue auto-advance not done
- exclusive mode not done
- device selection not done

### Gates

- `cargo fmt`
- `cargo test`
- `cargo check`
- `npm build` if frontend was touched
- `git diff --stat`
- `git status --short`

### Windows Real-Device Validation

Must answer:

- Was audio actually audible:
- Test file format:
- `play` worked:
- `pause` worked:
- `resume` worked:
- `stop` worked:
- UI remained responsive:
- repeated load/play worked:
- panic/error observed:

### Commit

- hash:
- message:

---

## 9. Xiaomi MiMo Completion Report Template

Every Xiaomi MiMo ticket must report the following.

### State Header

- Repo:
- Branch:
- Base HEAD:
- New HEAD:
- Ticket ID:
- Working tree:

### Modified Files

List each modified file.

### What Changed

Explain by file or area.

### What Did Not Change

Must confirm:

- backend not changed
- `src-tauri` not changed
- package files not changed
- visual UI not changed
- playback bridge not done
- mock replacement not done
- unrelated features not done

### Gates

- `npm.cmd run build`
- `npm.cmd run typecheck` if available
- `git diff --stat`
- `git status --short`

### Commit

- hash:
- message:

---

## 10. Current Best Next Step

The current best next step is parallel but non-overlapping:

1. Web GPT continues backend audio core work.
2. Xiaomi MiMo performs i18n hardcoded string cleanup.
3. Backend completion report is validated before frontend bridge starts.
4. Xiaomi i18n completion report is validated for file boundary safety.
5. Do not start frontend playback bridge before WAV real playback exists.
6. Do not expand formats before WAV real playback exists.

---

## 11. Startup Checklist For Every New Task Window

Before editing, every task window must answer:

1. What is the Ticket ID?
2. What is the Base HEAD?
3. What branch am I on?
4. Is the working tree clean?
5. What files am I allowed to edit?
6. What files are forbidden?
7. Could another window be editing the same module?
8. Does this task touch backend audio core?
9. Does this task touch frontend playback bridge?
10. Does this task touch i18n?
11. Does this task touch package/lock files?
12. Does this task change UI visuals?
13. What gates must pass?
14. What does completion mean for this ticket?
15. What must explicitly remain undone?

If any answer is unclear, stop and report before editing.

---

## 12. Non-Negotiable Completion Standard

A task is not complete because code was written.

A task is complete only when:

- scope stayed within allowed files
- forbidden files were not touched
- gates ran and passed or failures were honestly reported
- completion report is specific
- commit exists when the task asks for a commit
- working tree is clean after commit
- real-device validation is included when the task claims real playback

For backend playback specifically:

```text
No audible Windows test = do not claim real playback complete.
```
