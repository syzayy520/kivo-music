# KIVO-AUDIO-WASAPI-REAL-TRANSPORT-PREFLIGHT-P0-065

## 1. State

- Repo: syzayy520/kivo-music
- Branch: kivo-audio-native-decode-pipeline-p0-009
- Base: 81d818a
- Current phase: WASAPI scaffold
- This document is preflight only
- No real transport is implemented

## 2. Current completed foundation

The following pure models have been built on this branch:

- Runtime handle pure model (P0-058B through P0-060A)
- Runtime queue pure model (P0-061D, P0-061E)
- Queue bridge pure model (P0-061E)
- Mock runtime loop skeleton (P0-062)
- Mock runtime loop scenarios (P0-063)
- Bridge split / declaration split (P0-064)

All models are Copy/Clone/Debug/PartialEq/Eq.
All models are #[allow(dead_code)] scaffolding.
No model produces audible output.

## 3. What P0-065 allows

This ticket introduces:

- A real transport preflight document (this file)
- A pure Rust contract descriptor for future real transport
- Stage enum describing transport lifecycle phases
- Ownership role enum for future sink/worker ownership
- Unsupported reason enum documenting why transport cannot start
- Checklist and guardrails for future transport work
- Tests verifying contract defaults

## 4. What P0-065 forbids

This ticket does NOT introduce:

- std::sync (Mutex, Arc, Condvar, RwLock)
- std::thread (spawn, JoinHandle)
- std::time (Duration, Instant, sleep)
- std::sync::mpsc (Sender, Receiver, channel)
- Sender / Receiver types
- JoinHandle types
- Arc / Mutex / Condvar / AtomicBool
- RingBuffer ownership or bridging
- WASAPI API calls (IAudioClient, IAudioRenderClient, GetBuffer, ReleaseBuffer)
- windows:: crate references
- HRESULT handling
- submit_frame changes
- NativePipeline integration
- PlaybackCapabilities unlock
- Real audio output
- Real command transport
- Worker thread spawning

## 5. Future phase order

The following phases are planned after P0-065:

| Phase | Description | Key additions |
|-------|-------------|---------------|
| P0-066 | Real command transport scaffold | May consider mpsc/Sender/Receiver, still no spawn |
| P0-067+ | Output thread handle / spawn preflight | JoinHandle contract, thread ownership model |
| P0-068+ | Worker loop skeleton | Actual loop structure, state machine integration |
| Later | RingBuffer ownership bridge | Buffer lifecycle, producer/consumer roles |
| Later | WASAPI client/render boundary | Device management, format negotiation |
| Later | submit_frame integration | Frame delivery path, error propagation |
| Later | NativePipeline integration | Pipeline connection, lifecycle management |
| Finally | PlaybackCapabilities unlock | Feature flag, user-facing playback enable |

Each phase requires its own preflight document and contract.

## 6. Fake playback prevention

The following naming conventions are enforced:

- No "ready" naming in transport types
- No "playback active" naming
- No "output active" naming
- No claim of audio output capability
- No claim of device opened status
- No claim of worker running status

Contract types use:
- `NotCreated` / `ContractOnly` / `TransportNotStarted` (not Ready/Active)
- `SinkOwned` / `WorkerOwned` / `ReportOnly` (not Playing/Streaming)

## 7. Acceptance criteria

P0-065 is accepted when:

- [ ] No real transport created
- [ ] No sender/receiver created
- [ ] No thread created or spawned
- [ ] No WASAPI API referenced
- [ ] No RingBuffer referenced
- [ ] No submit_frame behavior changed
- [ ] Contract defaults verify transport is not started
- [ ] All new Rust files are ≤220 lines
- [ ] cargo fmt passes
- [ ] cargo clippy passes
- [ ] All existing tests still pass
- [ ] New contract tests pass

## 8. P0-066 transport scaffold note

P0-066 creates a typed mpsc command channel for real transport scaffold:

- Creates `OutputThreadRealTransportCommand` enum (RuntimeIntent / CloseTransport)
- Creates `OutputThreadRealTransportStatus` enum and `StatusReport` struct
- Creates `OutputThreadRealTransportChannel` with mpsc Sender/Receiver pair
- Provides `send_command` and non-blocking `try_recv_command`
- Does NOT spawn a worker thread
- Does NOT create JoinHandle
- Does NOT touch RingBuffer
- Does NOT touch WASAPI
- Does NOT change submit_frame
- Does NOT integrate NativePipeline
- Does NOT unlock PlaybackCapabilities
- All transport types remain scaffolding with `#[allow(dead_code)]`

## 9. P0-067 worker lifecycle preflight note

P0-067 creates worker lifecycle preflight contracts:

- Creates `OutputThreadWorkerLifecycleStage` enum (NotCreated, ContractOnly, HandleNotStarted, StopRequested, Stopped, Failed)
- Creates `OutputThreadWorkerLifecycleReason` enum (NotStarted, StopRequested, ClosedTransport, FailedPreflight, WorkerUnavailable)
- Creates `OutputThreadWorkerHandleOwnership` enum (NotOwned, TransportOwned, FutureWorkerOwned)
- Creates `OutputThreadWorkerHandleContract` struct with lifecycle, ownership, handle, join, stop, worker loop flags
- Creates `OutputThreadWorkerShutdownRequest` enum (None, RequestStop, CloseTransport)
- Creates `OutputThreadWorkerShutdownOutcome` enum (NoWorker, StopMarked, AlreadyStopped, TransportClosed, Unsupported)
- Creates `OutputThreadWorkerLifecycleInput` and `OutputThreadWorkerLifecycleDecision` structs
- Creates `plan_worker_lifecycle` pure function with decision rules
- Creates scenario matrix with 5 fixed scenarios
- Does NOT spawn a worker thread
- Does NOT create JoinHandle
- Does NOT touch RingBuffer
- Does NOT touch WASAPI
- Does NOT change submit_frame
- Does NOT integrate NativePipeline
- Does NOT unlock PlaybackCapabilities
- `std::sync::mpsc` remains only in channel file
- All worker types remain scaffolding with `#[allow(dead_code)]`
- `std::sync::mpsc` is imported only in the channel file
- All transport types remain scaffolding with `#[allow(dead_code)]`
