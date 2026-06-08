//! Command application rules.
//!
//! Pure function that maps (current lifecycle, command) to a transition result.

use crate::playback::output_wasapi::output_thread::command::drain_command::DrainCommand;
use crate::playback::output_wasapi::output_thread::command::shutdown_command::ShutdownCommand;
use crate::playback::output_wasapi::output_thread::command::thread_command::ThreadCommand;
use crate::playback::output_wasapi::output_thread::runtime::state_machine::{
    Transition, TransitionError, TransitionResult,
};
use crate::playback::output_wasapi::output_thread::state::OutputThreadLifecycle;

/// Apply a thread command to the current lifecycle state.
///
/// Returns the appropriate transition result based on the current state and command.
pub fn apply_command(current: OutputThreadLifecycle, command: ThreadCommand) -> TransitionResult {
    match command {
        ThreadCommand::Shutdown(shutdown) => apply_shutdown(current, shutdown),
        ThreadCommand::Drain(drain) => apply_drain(current, drain),
        ThreadCommand::Flush => apply_flush(current),
        ThreadCommand::SetVolume(_) => apply_set_volume(current),
        ThreadCommand::Pause => apply_pause(current),
        ThreadCommand::Resume => apply_resume(current),
    }
}

fn apply_shutdown(current: OutputThreadLifecycle, shutdown: ShutdownCommand) -> TransitionResult {
    match shutdown {
        ShutdownCommand::Graceful => match current {
            OutputThreadLifecycle::Running => {
                let transition = Transition::new(current, OutputThreadLifecycle::Draining);
                TransitionResult::Applied(transition)
            }
            OutputThreadLifecycle::Draining => {
                let transition = Transition::new(current, OutputThreadLifecycle::Stopping);
                TransitionResult::Applied(transition)
            }
            _ => invalid_transition(current, OutputThreadLifecycle::Stopping),
        },
        ShutdownCommand::Immediate => match current {
            OutputThreadLifecycle::Running | OutputThreadLifecycle::Draining => {
                let transition = Transition::new(current, OutputThreadLifecycle::Stopping);
                TransitionResult::Applied(transition)
            }
            _ => invalid_transition(current, OutputThreadLifecycle::Stopping),
        },
        ShutdownCommand::WithTimeout(_) => match current {
            OutputThreadLifecycle::Running => {
                let transition = Transition::new(current, OutputThreadLifecycle::Draining);
                TransitionResult::Applied(transition)
            }
            OutputThreadLifecycle::Draining => {
                let transition = Transition::new(current, OutputThreadLifecycle::Stopping);
                TransitionResult::Applied(transition)
            }
            _ => invalid_transition(current, OutputThreadLifecycle::Stopping),
        },
    }
}

fn apply_drain(current: OutputThreadLifecycle, _drain: DrainCommand) -> TransitionResult {
    match current {
        OutputThreadLifecycle::Running => {
            let transition = Transition::new(current, OutputThreadLifecycle::Draining);
            TransitionResult::Applied(transition)
        }
        OutputThreadLifecycle::Draining => TransitionResult::AlreadyAtTarget,
        _ => invalid_transition(current, OutputThreadLifecycle::Draining),
    }
}

fn apply_flush(current: OutputThreadLifecycle) -> TransitionResult {
    match current {
        OutputThreadLifecycle::Running => TransitionResult::AlreadyAtTarget,
        _ => invalid_transition(current, OutputThreadLifecycle::Running),
    }
}

fn apply_set_volume(current: OutputThreadLifecycle) -> TransitionResult {
    match current {
        OutputThreadLifecycle::Running => TransitionResult::AlreadyAtTarget,
        _ => invalid_transition(current, OutputThreadLifecycle::Running),
    }
}

fn apply_pause(current: OutputThreadLifecycle) -> TransitionResult {
    match current {
        OutputThreadLifecycle::Running => TransitionResult::AlreadyAtTarget,
        _ => invalid_transition(current, OutputThreadLifecycle::Running),
    }
}

fn apply_resume(current: OutputThreadLifecycle) -> TransitionResult {
    match current {
        OutputThreadLifecycle::Running => TransitionResult::AlreadyAtTarget,
        _ => invalid_transition(current, OutputThreadLifecycle::Running),
    }
}

fn invalid_transition(from: OutputThreadLifecycle, to: OutputThreadLifecycle) -> TransitionResult {
    let transition = Transition::new(from, to);
    TransitionResult::Rejected(transition, TransitionError::InvalidTransition)
}
