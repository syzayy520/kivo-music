use super::super::output_thread_control::OutputThreadCommand;
use super::handle::OutputThreadRuntimeHandle;
use super::super::output_thread_state::OutputThreadState;

/// Errors from runtime lifecycle transitions.
#[allow(dead_code, clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadRuntimeLifecycleError {
    /// Cannot start from current state.
    CannotStart,
    /// Cannot stop from current state.
    CannotStop,
    /// Cannot close from current state.
    CannotClose,
    /// Cannot pause from current state.
    CannotPause,
    /// Cannot resume from current state.
    CannotResume,
    /// Cannot flush from current state.
    CannotFlush,
    /// Cannot reset from current state.
    CannotReset,
}

/// Result type for runtime lifecycle transitions.
#[allow(dead_code)]
pub(crate) type OutputThreadRuntimeLifecycleResult =
    Result<OutputThreadRuntimeHandle, OutputThreadRuntimeLifecycleError>;

/// Apply a command to a runtime handle, producing a new handle state.
///
/// Pure reducer — no thread spawning, no command sending, no state mutation.
#[allow(dead_code)]
pub(crate) fn apply_runtime_command(
    handle: OutputThreadRuntimeHandle,
    command: OutputThreadCommand,
) -> OutputThreadRuntimeLifecycleResult {
    match command {
        OutputThreadCommand::Start => apply_start(handle),
        OutputThreadCommand::Pause => apply_pause(handle),
        OutputThreadCommand::Resume => apply_resume(handle),
        OutputThreadCommand::Flush => apply_flush(handle),
        OutputThreadCommand::Stop => apply_stop(handle),
        OutputThreadCommand::Close => apply_close(handle),
        OutputThreadCommand::ResetDevice => apply_reset(handle),
    }
}

fn apply_start(handle: OutputThreadRuntimeHandle) -> OutputThreadRuntimeLifecycleResult {
    match handle.status.state {
        OutputThreadState::Created | OutputThreadState::Stopped | OutputThreadState::Joined => {
            Ok(handle.with_state(OutputThreadState::Running))
        }
        _ => Err(OutputThreadRuntimeLifecycleError::CannotStart),
    }
}

fn apply_pause(handle: OutputThreadRuntimeHandle) -> OutputThreadRuntimeLifecycleResult {
    if handle.status.state == OutputThreadState::Running {
        Ok(handle) // state remains Running
    } else {
        Err(OutputThreadRuntimeLifecycleError::CannotPause)
    }
}

fn apply_resume(handle: OutputThreadRuntimeHandle) -> OutputThreadRuntimeLifecycleResult {
    if handle.status.state == OutputThreadState::Running {
        Ok(handle) // state remains Running
    } else {
        Err(OutputThreadRuntimeLifecycleError::CannotResume)
    }
}

fn apply_flush(handle: OutputThreadRuntimeHandle) -> OutputThreadRuntimeLifecycleResult {
    if handle.status.state == OutputThreadState::Running {
        Ok(handle) // state remains Running
    } else {
        Err(OutputThreadRuntimeLifecycleError::CannotFlush)
    }
}

fn apply_stop(handle: OutputThreadRuntimeHandle) -> OutputThreadRuntimeLifecycleResult {
    if handle.status.state == OutputThreadState::Running {
        Ok(handle.with_state(OutputThreadState::Stopping))
    } else {
        Err(OutputThreadRuntimeLifecycleError::CannotStop)
    }
}

fn apply_close(handle: OutputThreadRuntimeHandle) -> OutputThreadRuntimeLifecycleResult {
    match handle.status.state {
        OutputThreadState::Created
        | OutputThreadState::Running
        | OutputThreadState::Stopping
        | OutputThreadState::Stopped
        | OutputThreadState::Failed => Ok(handle.with_state(OutputThreadState::Closed)),
        _ => Err(OutputThreadRuntimeLifecycleError::CannotClose),
    }
}

fn apply_reset(handle: OutputThreadRuntimeHandle) -> OutputThreadRuntimeLifecycleResult {
    match handle.status.state {
        OutputThreadState::Running | OutputThreadState::Failed | OutputThreadState::Stopped => {
            Ok(handle.with_state(OutputThreadState::Created))
        }
        _ => Err(OutputThreadRuntimeLifecycleError::CannotReset),
    }
}
