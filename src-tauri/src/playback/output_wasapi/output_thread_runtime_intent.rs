use super::output_thread_control::OutputThreadCommand;
use super::output_thread_runtime_status::OutputThreadRuntimeStatus;
use super::output_thread_state::OutputThreadState;

/// Intent to perform an operation on the output thread runtime.
///
/// Pure intent mapping — no command sending.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRuntimeIntent {
    /// Start the consumer loop.
    Start,
    /// Pause playback.
    Pause,
    /// Resume playback after pause.
    Resume,
    /// Flush the buffer.
    Flush,
    /// Stop the consumer loop.
    Stop,
    /// Close the runtime and release resources.
    Close,
    /// Reset the audio device.
    ResetDevice,
}

#[allow(dead_code)]
impl OutputThreadRuntimeIntent {
    /// Convert this intent to a command.
    pub(crate) fn to_command(self) -> OutputThreadCommand {
        match self {
            Self::Start => OutputThreadCommand::Start,
            Self::Pause => OutputThreadCommand::Pause,
            Self::Resume => OutputThreadCommand::Resume,
            Self::Flush => OutputThreadCommand::Flush,
            Self::Stop => OutputThreadCommand::Stop,
            Self::Close => OutputThreadCommand::Close,
            Self::ResetDevice => OutputThreadCommand::ResetDevice,
        }
    }

    /// Whether this intent requires the thread to be running.
    pub(crate) fn requires_running(self) -> bool {
        matches!(self, Self::Pause | Self::Resume | Self::Flush | Self::Stop)
    }

    /// Whether this intent requests shutdown.
    pub(crate) fn requests_shutdown(self) -> bool {
        matches!(self, Self::Stop | Self::Close)
    }

    /// Whether this intent clears the buffer.
    pub(crate) fn clears_buffer(self) -> bool {
        matches!(self, Self::Flush | Self::ResetDevice)
    }

    /// Whether this intent can be applied to the given status.
    pub(crate) fn can_apply_to(self, status: OutputThreadRuntimeStatus) -> bool {
        match self {
            Self::Start => matches!(
                status.state,
                OutputThreadState::Created | OutputThreadState::Stopped | OutputThreadState::Joined
            ),
            Self::Pause | Self::Resume | Self::Flush | Self::Stop => {
                status.state == OutputThreadState::Running
            }
            Self::Close => matches!(
                status.state,
                OutputThreadState::Created
                    | OutputThreadState::Running
                    | OutputThreadState::Stopping
                    | OutputThreadState::Stopped
                    | OutputThreadState::Failed
            ),
            Self::ResetDevice => matches!(
                status.state,
                OutputThreadState::Running | OutputThreadState::Failed | OutputThreadState::Stopped
            ),
        }
    }
}
