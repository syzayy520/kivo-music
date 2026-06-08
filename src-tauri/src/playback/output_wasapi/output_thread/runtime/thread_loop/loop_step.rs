//! Loop step execution.
//!
//! Processes a single step of the thread loop using the
//! in-memory driver logic.

use crate::playback::output_wasapi::output_thread::command::ThreadCommand;
use crate::playback::output_wasapi::output_thread::event::ThreadEvent;
use crate::playback::output_wasapi::output_thread::runtime::driver::step_logic::{
    execute_command_step, execute_idle_step, CommandStepOutcome,
};
use crate::playback::output_wasapi::output_thread::runtime::thread_loop::loop_state::LoopState;
use crate::playback::output_wasapi::output_thread::runtime::thread_loop::LoopResult;

/// Outcome of a single loop step.
#[derive(Debug, Clone)]
pub struct LoopStepOutcome {
    /// The loop result indicating whether to continue or stop.
    pub result: LoopResult,
    /// Any event produced by this step.
    pub event: Option<ThreadEvent>,
    /// The state after this step.
    pub state: LoopState,
}

/// Execute a single loop step with an optional command.
///
/// If a command is provided, it is processed through the command step logic.
/// If no command, an idle step is executed.
pub fn execute_loop_step(
    state: &LoopState,
    command: Option<ThreadCommand>,
    max_idle_steps: u64,
) -> LoopStepOutcome {
    // If already terminal, stop immediately.
    if state.is_terminal() {
        return LoopStepOutcome {
            result: LoopResult::Stop,
            event: None,
            state: state.clone(),
        };
    }

    match command {
        Some(cmd) => {
            let CommandStepOutcome {
                transition: _,
                event,
                new_lifecycle,
                result,
            } = execute_command_step(cmd, state.lifecycle);

            let mut new_state = state.clone();
            new_state.transition_to(new_lifecycle);

            LoopStepOutcome {
                result: match result {
                    crate::playback::output_wasapi::output_thread::runtime::driver::DriverResult::Continue => LoopResult::Continue,
                    crate::playback::output_wasapi::output_thread::runtime::driver::DriverResult::Stop => LoopResult::Stop,
                    crate::playback::output_wasapi::output_thread::runtime::driver::DriverResult::Error => LoopResult::Error("Command step error".to_string()),
                    crate::playback::output_wasapi::output_thread::runtime::driver::DriverResult::Idle => LoopResult::Continue,
                },
                event,
                state: new_state,
            }
        }
        None => {
            let idle_result = execute_idle_step(state.idle_count + 1, max_idle_steps);
            let mut new_state = state.clone();
            new_state.record_idle();

            LoopStepOutcome {
                result: match idle_result {
                    crate::playback::output_wasapi::output_thread::runtime::driver::DriverResult::Continue => LoopResult::Continue,
                    crate::playback::output_wasapi::output_thread::runtime::driver::DriverResult::Stop => LoopResult::Stop,
                    crate::playback::output_wasapi::output_thread::runtime::driver::DriverResult::Error => LoopResult::Error("Idle step error".to_string()),
                    crate::playback::output_wasapi::output_thread::runtime::driver::DriverResult::Idle => LoopResult::Continue,
                },
                event: None,
                state: new_state,
            }
        }
    }
}
