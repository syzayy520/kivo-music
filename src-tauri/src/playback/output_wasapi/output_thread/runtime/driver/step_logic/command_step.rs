//! Command step logic.
//!
//! Pure function that processes a single command against the current lifecycle state.

use crate::playback::output_wasapi::output_thread::command::ThreadCommand;
use crate::playback::output_wasapi::output_thread::event::ThreadEvent;
use crate::playback::output_wasapi::output_thread::runtime::driver::driver_result::DriverResult;
use crate::playback::output_wasapi::output_thread::runtime::driver::step_logic::result_builder::determine_driver_result;
use crate::playback::output_wasapi::output_thread::runtime::state_machine::rules::apply_command;
use crate::playback::output_wasapi::output_thread::runtime::state_machine::TransitionResult;
use crate::playback::output_wasapi::output_thread::state::OutputThreadLifecycle;

/// Outcome of processing a single command step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandStepOutcome {
    /// The transition result from applying the command.
    pub transition: TransitionResult,
    /// Event to emit if the state changed (None if no change).
    pub event: Option<ThreadEvent>,
    /// The lifecycle state after this step.
    pub new_lifecycle: OutputThreadLifecycle,
    /// The driver result indicating next action.
    pub result: DriverResult,
}

/// Execute a command step: apply command rules and produce outcome.
///
/// Pure function — no side effects, no I/O.
pub fn execute_command_step(
    command: ThreadCommand,
    current: OutputThreadLifecycle,
) -> CommandStepOutcome {
    let transition = apply_command(current, command);

    let (event, new_lifecycle) = match transition {
        TransitionResult::Applied(t) => {
            let evt = ThreadEvent::StateChanged {
                from: t.from,
                to: t.to,
            };
            (Some(evt), t.to)
        }
        _ => (None, current),
    };

    let result = determine_driver_result(&transition, new_lifecycle);

    CommandStepOutcome {
        transition,
        event,
        new_lifecycle,
        result,
    }
}
