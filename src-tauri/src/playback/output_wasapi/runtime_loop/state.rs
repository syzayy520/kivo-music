/// Mock-only runtime loop state.
///
/// Pure state — no real thread, no real output.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRuntimeLoopState {
    /// Loop has not started.
    Idle,
    /// Loop is actively running.
    Active,
    /// Loop is shutting down.
    Stopping,
    /// Loop has exited normally.
    Stopped,
    /// Loop has exited and cannot restart.
    Exited,
}

#[allow(dead_code)]
impl OutputThreadRuntimeLoopState {
    /// Whether the loop is actively running.
    pub(crate) fn is_active(self) -> bool {
        self == Self::Active
    }

    /// Whether the loop has exited and cannot restart.
    pub(crate) fn is_terminal(self) -> bool {
        self == Self::Exited
    }

    /// Whether the loop can accept a Start intent.
    pub(crate) fn can_accept_start(self) -> bool {
        matches!(self, Self::Idle | Self::Stopped)
    }

    /// Whether the loop can accept a Stop intent.
    pub(crate) fn can_accept_stop(self) -> bool {
        self == Self::Active
    }
}