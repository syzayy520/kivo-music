//! Worker loop state pure type.
//!
//! Describes the state of a non-blocking worker loop skeleton
//! without referencing real thread, sync, or audio primitives.

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadWorkerLoopState {
    /// Loop has not started polling.
    NotStarted,
    /// Loop is actively polling for commands.
    Polling,
    /// Last poll found no command available.
    NoCommand,
    /// Last poll handled a runtime intent command.
    CommandHandled,
    /// A stop-type intent was received.
    StopRequested,
    /// Transport channel is closed or disconnected.
    TransportClosed,
}

impl OutputThreadWorkerLoopState {
    /// Returns true if the loop cannot continue from this state.
    #[allow(dead_code)]
    pub(crate) fn is_terminal(self) -> bool {
        matches!(self, Self::StopRequested | Self::TransportClosed)
    }

    /// Returns true if the loop may poll from this state.
    #[allow(dead_code)]
    pub(crate) fn can_poll(self) -> bool {
        !self.is_terminal()
    }

    /// Returns true if a real worker thread exists.
    /// Always false — this is a skeleton without real threads.
    #[allow(dead_code)]
    pub(crate) fn has_worker_thread(self) -> bool {
        false
    }
}
