//! Result builder logic.
//!
//! Pure function that maps a transition result and lifecycle state to a DriverResult.

use crate::playback::output_wasapi::output_thread::runtime::driver::driver_result::DriverResult;
use crate::playback::output_wasapi::output_thread::runtime::state_machine::TransitionResult;
use crate::playback::output_wasapi::output_thread::state::OutputThreadLifecycle;

/// Determine the driver result based on the transition outcome and new lifecycle.
///
/// - Applied + terminal lifecycle → Stop
/// - Applied + active lifecycle → Continue
/// - Rejected → Error
/// - AlreadyAtTarget → Continue
///
/// Pure function — no side effects.
pub fn determine_driver_result(
    transition: &TransitionResult,
    lifecycle: OutputThreadLifecycle,
) -> DriverResult {
    match transition {
        TransitionResult::Applied(_) => {
            if lifecycle.is_terminal() {
                DriverResult::Stop
            } else {
                DriverResult::Continue
            }
        }
        TransitionResult::Rejected(_, _) => DriverResult::Error,
        TransitionResult::AlreadyAtTarget => DriverResult::Continue,
    }
}
