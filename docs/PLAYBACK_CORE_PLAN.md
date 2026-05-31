# Kivo Playback Core Plan

Kivo Playback Core is the final playback architecture for Kivo Music. It is not a temporary player, not a React component, and not a thin wrapper around a single backend. It must be designed from day one as the long-term playback heart of the product.

Execution authority note:
- This file defines architecture intent and stage targets.
- Strict execution precedence and ticket acceptance format are defined in `docs/AUDIO_CORE_EXECUTION_SPEC.md`.

## Final position

Kivo Playback Core owns playback control, state, timeline, queue, lyrics synchronization, metadata, output configuration, events, and backend abstraction.

The UI must only talk to Kivo Playback API. It must never directly talk to mpv, FFmpeg, audio devices, or Tauri commands scattered across components.

## Backend decision

- mpv is the first playback backend because it is a mature playback engine, not because it is a throwaway shortcut.
- FFmpeg remains important for media probing, metadata, format analysis, and future backend expansion.
- Kivo Playback Core must not expose mpv-specific details to the UI.
- Backend adapters must allow future expansion without rewriting UI or playback API.

## Non-negotiable bans

```text
Do not use an HTML audio tag as a temporary player.
Do not call playback commands directly inside React feature components.
Do not put playback logic in App.tsx.
Do not put playback logic in PlayerBar.tsx.
Do not create one large playback.rs file.
Do not expose mpv-specific types to the frontend.
Do not maintain fake UI playback state separate from real core state.
Do not add queue, lyrics, output devices, or metadata as afterthoughts.
Do not claim build or playback validation unless actually run.
```

## Rust backend boundary

Final P0 module shape:

```text
src-tauri/src/playback/
  mod.rs
  commands.rs
  types.rs
  errors.rs
  state.rs
  events.rs
  lifecycle.rs
  engine.rs
  queue.rs
  timeline.rs
  volume.rs
  output.rs
  capabilities.rs
  metadata.rs
  path.rs
  lyrics_clock.rs
  backends/
    mod.rs
    backend_types.rs
    mpv.rs
```

### File responsibilities

```text
mod.rs                  Module declarations and re-exports only.
commands.rs             Tauri command entry points only.
types.rs                Shared playback domain types only.
errors.rs               Typed playback errors only.
state.rs                Current playback state only.
events.rs               Event payloads and event dispatch boundary only.
lifecycle.rs            Init, shutdown, backend ownership, cleanup, recovery.
engine.rs               PlaybackEngine trait and core orchestration boundary.
queue.rs                Queue, current index, repeat, shuffle, reorder model.
timeline.rs             Duration, position, seek state, progress throttling.
volume.rs               Volume and mute model.
output.rs               Output device, exclusive mode, bit-perfect placeholders.
capabilities.rs         Backend capability model.
metadata.rs             Codec, container, sample rate, bit depth, channels, bitrate.
path.rs                 Local path normalization and validation.
lyrics_clock.rs         Timed lyric synchronization model.
backends/mod.rs         Backend module declarations only.
backends/backend_types.rs Backend-facing types only.
backends/mpv.rs         mpv backend implementation only.
```

## Frontend playback boundary

Final P0 frontend shape:

```text
src/shared/playback/
  playbackTypes.ts
  playbackApi.ts
  playbackEvents.ts
  playbackStore.ts
```

### Frontend responsibilities

```text
playbackTypes.ts        Frontend playback types only.
playbackApi.ts          Stable API wrapper only.
playbackEvents.ts       Event subscription boundary only.
playbackStore.ts        UI-facing playback state store only.
```

Feature components may consume this shared playback layer, but they must not directly call backend-specific APIs.

## Event rules

- Track change, error, stopped, loaded, and seek-complete events can be immediate.
- Progress events must be throttled.
- Default UI progress cadence should be 250ms or 500ms.
- Events must be typed and versionable.
- Frontend must be able to recover from missed progress events by requesting current state.

### Event ordering contract (must-test)

Event ordering must be explicit and stable. The following sequences are normative unless a ticket explicitly revises them.

1. `load(track)` success:
`TrackChanged` -> `StateChanged`
2. `play()` success:
`StateChanged` -> `Progress`
3. `seek(position)` success:
`StateChanged` -> `SeekComplete` -> `Progress`
4. track end with queue advance:
`TrackEnded` -> `TrackChanged` -> `StateChanged`
5. operation failure:
`Error` -> `StateChanged` (state contains typed error fact)

Rules:

1. No synthetic progress events are allowed before real runtime clock exists.
2. Event versions must remain backward-compatible for frontend playback boundary.
3. Event order changes require dedicated ticket and migration notes.

Implementation phase note:

1. `SeekComplete`, `TrackEnded`, and `BackendSwitched` are target events in this contract.
2. Until these event variants are implemented in code, tickets must mark them as `planned/not yet emitted`.
3. Do not claim these events are active without code + test evidence.

## Path safety rules

- Empty paths are invalid.
- Nonexistent paths are invalid.
- Unsupported file extensions must return typed errors.
- Paths must be normalized before reaching a backend.
- Future library-root allowlists must be possible without changing UI APIs.

## Metadata rules

Metadata belongs to Playback Core or a dedicated probe path, not UI mock state.

Must model:

```text
codec
container
sample_rate
bit_depth
channels
bitrate
duration
lossless / lossy
source path
```

P0 can define types without full probing implementation. P3 must connect real metadata.

## Output rules

Output handling must be modeled from P0 even if implementation arrives later.

Must reserve space for:

```text
output device list
selected output device
exclusive mode
bit-perfect mode
replaygain
gapless playback
platform-specific backends such as WASAPI, CoreAudio, ALSA, PulseAudio, PipeWire
```

## Error model

Errors must be typed. Do not use plain strings as the primary model.

Minimum categories:

```text
PlaybackError
BackendError
PathError
QueueError
OutputError
UnsupportedFormatError
```

Frontend must be able to distinguish:

```text
file missing
unsupported format
backend initialization failed
load failed
play failed
seek failed
output device unavailable
```

## State transition matrix (minimum contract)

Playback state transitions must be explicit and test-covered.

Allowed transitions:

1. `Idle -> Loading`
2. `Loading -> Paused` (loaded, ready to play)
3. `Paused -> Playing`
4. `Playing -> Paused`
5. `Playing -> Stopped`
6. `Paused -> Stopped`
7. `Stopped -> Loading`

Seek behavior:

1. `Playing -> Playing` with updated timeline position.
2. `Paused -> Paused` with updated timeline position.

Illegal transition handling:

1. Illegal transitions must return typed errors.
2. Illegal transitions must not mutate queue ownership.
3. Illegal transitions must not emit fake success events.

Scaffold-phase exemption:

1. Before runtime-closed playback loop lands, unsupported paths may return typed unsupported errors.
2. During scaffold phase, matrix compliance is evaluated on implemented transitions only.
3. Unsupported operations must remain explicit and must not mutate runtime into fake success states.

## Stage plan

### P0: Final architecture skeleton

Goal: establish the final core boundary without temporary shortcuts.

Must include:

```text
Rust playback module folder
Typed domain models
Typed errors
Playback state model
Event model
Lifecycle model
Engine trait
Backend adapter skeleton
mpv backend skeleton
Queue, timeline, volume, output, metadata, capabilities, path, lyrics clock placeholders
Frontend playback API boundary
```

P0 does not need full playback. P0 must prevent future rewrite.

### P1: Local file playback closure

Goal: play a real local audio file through the core.

Must support:

```text
load local path
play
pause
resume
stop
seek
volume
progress
duration
error events
```

### P2: Queue system

Goal: become a real player, not a single-file test harness.

Must support:

```text
queue
current index
next
previous
repeat off / one / all
shuffle
future reorder model
```

### P3: Audio quality and metadata

Goal: remove fake quality labels from UI state.

Must support:

```text
codec
container
sample rate
bit depth
channels
bitrate
lossless / lossy
duration
```

### P4: Lyrics synchronization

Goal: lyrics are driven by playback clock, not frontend fake timing.

Must support:

```text
current time event
active lyric line matching
seek-safe lyric sync
track-change reset
```

### P5: Output and professional audio

Goal: professional local player foundation.

Must support or reserve:

```text
output device selection
exclusive mode
bit-perfect mode
replaygain
gapless playback
platform-specific output models
```

## Gates for every Playback Core task

Every playback core handoff must report:

```text
Changed files
Each file's single responsibility
Whether any file should be split
Which stage requirement it satisfies
What was intentionally not implemented
Build / check commands run or explicitly not run
Latest commit SHA
```

Preferred validation:

```text
cargo fmt
cargo check
cargo clippy -- -D warnings
npm run build
```

If validation cannot be run, state that directly.

## Execution rule

Architecture is final from P0. Features land in stages. No temporary playback layer is allowed.
