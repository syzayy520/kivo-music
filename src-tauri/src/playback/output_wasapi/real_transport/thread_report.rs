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
}
