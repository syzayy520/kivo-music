use super::output_thread_state::OutputThreadState;

/// Runtime status of the output thread handle and its attachments.
///
/// Pure state — no thread queries, no device queries, no buffer reads.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeStatus {
    /// Current lifecycle state of the output thread.
    pub state: OutputThreadState,
    /// Whether a runtime handle descriptor is attached.
    pub has_handle: bool,
    /// Whether a command queue/channel is attached.
    pub has_queue: bool,
    /// Whether a report receiver is attached.
    pub has_report_receiver: bool,
    /// Whether a render target (ring buffer) is attached.
    pub has_render_target: bool,
}

impl OutputThreadRuntimeStatus {
    /// Create a new runtime status with explicit values.
    #[allow(dead_code)]
    pub(crate) fn new(
        state: OutputThreadState,
        has_handle: bool,
        has_queue: bool,
        has_report_receiver: bool,
        has_render_target: bool,
    ) -> Self {
        Self {
            state,
            has_handle,
            has_queue,
            has_report_receiver,
            has_render_target,
        }
    }

    /// Create an inactive status (all flags false, state Created).
    #[allow(dead_code)]
    pub(crate) fn inactive() -> Self {
        Self {
            state: OutputThreadState::Created,
            has_handle: false,
            has_queue: false,
            has_report_receiver: false,
            has_render_target: false,
        }
    }

    /// Create a scaffold-ready status (state Created, no attachments).
    #[allow(dead_code)]
    pub(crate) fn scaffold_ready() -> Self {
        Self {
            state: OutputThreadState::Created,
            has_handle: false,
            has_queue: false,
            has_report_receiver: false,
            has_render_target: false,
        }
    }

    /// Whether the thread is currently running.
    #[allow(dead_code)]
    pub(crate) fn is_running(self) -> bool {
        self.state == OutputThreadState::Running
    }

    /// Whether the runtime can accept audio frames right now.
    ///
    /// Requires: Running state + handle + queue + render target all attached.
    #[allow(dead_code)]
    pub(crate) fn can_accept_frames(self) -> bool {
        self.state == OutputThreadState::Running
            && self.has_handle
            && self.has_queue
            && self.has_render_target
    }

    /// Whether the runtime can be stopped.
    #[allow(dead_code)]
    pub(crate) fn can_stop(self) -> bool {
        self.state == OutputThreadState::Running
    }

    /// Whether the runtime can be closed.
    #[allow(dead_code)]
    pub(crate) fn can_close(self) -> bool {
        matches!(
            self.state,
            OutputThreadState::Created
                | OutputThreadState::Running
                | OutputThreadState::Stopping
                | OutputThreadState::Stopped
                | OutputThreadState::Failed
        )
    }

    /// Whether all runtime parts are attached.
    #[allow(dead_code)]
    pub(crate) fn is_fully_attached(self) -> bool {
        self.has_handle && self.has_queue && self.has_report_receiver && self.has_render_target
    }
}
