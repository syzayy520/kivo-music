//! Idle step logic.
//!
//! Pure function that determines the driver result during idle periods.

use crate::playback::output_wasapi::output_thread::runtime::driver::driver_result::DriverResult;

/// Compute the driver result for an idle step.
///
/// Returns `DriverResult::Stop` if idle_count >= max_idle_steps,
/// otherwise `DriverResult::Idle`.
///
/// Pure function — no side effects.
pub fn execute_idle_step(idle_count: u64, max_idle_steps: u64) -> DriverResult {
    if idle_count >= max_idle_steps {
        DriverResult::Stop
    } else {
        DriverResult::Idle
    }
}
