use super::state::OutputThreadState;

/// Control commands for the WASAPI output thread.
///
/// These are pure intent declarations — they do NOT execute
/// any thread operation, no concurrency primitives.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadCommand {
    /// Start the consumer loop (first run after creation).
    Start,
    /// Pause playback (consumer fills silence).
    Pause,
    /// Resume playback after pause.
    Resume,
    /// Clear buffer and reset pending_frames.
    Flush,
    /// Request clean shutdown; drain then exit.
    Stop,
    /// Shutdown + join thread + release device.
    Close,
    /// Reset audio device (re-initialize).
    ResetDevice,
}

impl OutputThreadCommand {
    /// Whether this command requires the thread to already be running.
    #[allow(dead_code)]
    pub(crate) fn requires_running_thread(self) -> bool {
        match self {
            Self::Start | Self::Close | Self::ResetDevice => false,
            Self::Pause | Self::Resume | Self::Flush | Self::Stop => true,
        }
    }

    /// Whether this command requests the thread to shut down.
    #[allow(dead_code)]
    pub(crate) fn requests_shutdown(self) -> bool {
        matches!(self, Self::Stop | Self::Close | Self::ResetDevice)
    }

    /// Whether this command clears the buffer.
    #[allow(dead_code)]
    pub(crate) fn clears_buffer(self) -> bool {
        matches!(self, Self::Flush | Self::Close | Self::ResetDevice)
    }
}

/// Snapshot of the output thread's current control state.
///
/// Pure data — no thread queries, no device queries.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadControlSnapshot {
    pub state: OutputThreadState,
    pub paused: bool,
    pub shutdown_requested: bool,
    pub flush_requested: bool,
}

impl Default for OutputThreadControlSnapshot {
    fn default() -> Self {
        Self {
            state: OutputThreadState::Created,
            paused: false,
            shutdown_requested: false,
            flush_requested: false,
        }
    }
}

impl OutputThreadControlSnapshot {
    /// Whether the thread is currently paused.
    #[allow(dead_code)]
    pub(crate) fn is_paused(self) -> bool {
        self.paused
    }

    /// Whether shutdown has been requested.
    #[allow(dead_code)]
    pub(crate) fn is_shutdown_requested(self) -> bool {
        self.shutdown_requested
    }

    /// Whether the thread can accept new frames right now.
    ///
    /// Determined solely from snapshot fields — no external state queries.
    #[allow(dead_code)]
    pub(crate) fn can_accept_frames(self) -> bool {
        self.state == OutputThreadState::Running
            && !self.paused
            && !self.shutdown_requested
            && !self.flush_requested
    }
}
