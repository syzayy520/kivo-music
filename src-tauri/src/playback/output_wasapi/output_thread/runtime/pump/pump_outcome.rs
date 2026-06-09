//! Pump outcome.
//!
//! Pure-data result of a single runtime pump tick.

use crate::playback::output_wasapi::output_thread::runtime::driver::driver_result::DriverResult;
use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::dispatch_outcome::DispatchOutcome;
use crate::playback::output_wasapi::output_thread::runtime::thread_loop::loop_result::LoopResult;
use crate::playback::output_wasapi::output_thread::runtime::thread_loop::loop_state::LoopState;

/// Result of a single runtime pump tick.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PumpOutcome {
    /// Tick completed, loop should continue.
    Continue {
        state: LoopState,
        driver_result: DriverResult,
        dispatch_outcome: Option<DispatchOutcome>,
    },
    /// Tick produced a stop signal.
    Stopped {
        state: LoopState,
        driver_result: DriverResult,
    },
    /// Tick encountered an error.
    Error { state: LoopState, message: String },
    /// Tick was a no-op (no command, idle within limits).
    Idle {
        state: LoopState,
        driver_result: DriverResult,
    },
}

impl PumpOutcome {
    /// Access the inner loop state regardless of variant.
    pub fn state(&self) -> &LoopState {
        match self {
            Self::Continue { state, .. }
            | Self::Stopped { state, .. }
            | Self::Error { state, .. }
            | Self::Idle { state, .. } => state,
        }
    }

    /// Build outcome from a loop result and optional dispatch.
    pub fn from_loop_result(
        loop_result: LoopResult,
        state: LoopState,
        driver_result: DriverResult,
        dispatch_outcome: Option<DispatchOutcome>,
    ) -> Self {
        match loop_result {
            LoopResult::Continue => Self::Continue {
                state,
                driver_result,
                dispatch_outcome,
            },
            LoopResult::Stop => Self::Stopped {
                state,
                driver_result,
            },
            LoopResult::Error(msg) => Self::Error {
                state,
                message: msg,
            },
        }
    }
}
