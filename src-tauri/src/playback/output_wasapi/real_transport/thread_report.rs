//! Report type for real output thread skeleton.
//!
//! Captures thread lifecycle and worker loop results
//! without referencing COM, WASAPI, or audio primitives.

use super::super::worker_loop::runner::OutputThreadWorkerLoopRunResult;

/// Report from a completed real output thread.
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct RealOutputThreadReport {
    /// Thread entry started executing.
    pub thread_started: bool,
    /// Owned-state validation passed inside thread.
    pub owned_state_validated: bool,
    /// Worker loop received CloseTransport command.
    pub shutdown_received: bool,
    /// Worker loop exited cleanly (no panic).
    pub exited_cleanly: bool,
    /// Number of commands processed by worker loop.
    pub commands_processed: usize,
    /// Raw worker loop run result for extensibility.
    pub loop_result: Option<OutputThreadWorkerLoopRunResult>,
    /// Thread panicked (set in error path, false in normal report).
    pub panicked: bool,

    // --- COM / WasapiContext lifecycle flags (P0-076) ---
    /// COM was initialized inside the thread (via WasapiContext::open).
    pub com_initialized: bool,
    /// COM was uninitialized inside the thread (context dropped).
    pub com_uninitialized: bool,
    /// WasapiContext open was requested by config.
    pub wasapi_context_open_requested: bool,
    /// WasapiContext::open() succeeded.
    pub wasapi_context_opened: bool,
    /// WasapiContext was closed (dropped) inside the thread.
    pub wasapi_context_closed: bool,

    // --- IAudioClient Start/Stop lifecycle flags (P0-077A) ---
    /// IAudioClient::Start was requested by config.
    pub audio_client_start_requested: bool,
    /// IAudioClient::Start succeeded.
    pub audio_client_started: bool,
    /// Explicit StartedClientGuard::stop() was requested.
    pub audio_client_stop_requested: bool,
    /// Explicit StartedClientGuard::stop() succeeded.
    pub audio_client_stopped: bool,

    // --- One-shot silent render write flags (P0-077B) ---
    /// One-shot silent render write was requested by config.
    pub render_silence_once_requested: bool,
    /// One-shot silent render write succeeded.
    pub render_silence_once_written: bool,
    /// Frames requested for one-shot silent render write.
    pub render_silence_once_frames_requested: u32,
    /// Frames written by one-shot silent render write.
    pub render_silence_once_frames_written: u32,
    /// Whether the one-shot render used AUDCLNT_BUFFERFLAGS_SILENT.
    pub render_silence_once_used_silent_flag: bool,
}
