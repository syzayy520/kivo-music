//! Dispatch context type.
//!
//! Contextual metadata for a sink dispatch operation.
//! Pure data — no behavior, no IO, no WASAPI.

use crate::playback::output_wasapi::output_thread::runtime::driver::driver_result::DriverResult;

/// Context for a sink dispatch operation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DispatchContext {
    /// The driver result that triggered this dispatch.
    pub driver_result: DriverResult,
    /// Frame count for the request.
    pub frame_count: u64,
    /// Sample rate in Hz.
    pub sample_rate: u32,
    /// Number of audio channels.
    pub channel_count: u16,
}

impl Default for DispatchContext {
    fn default() -> Self {
        Self {
            driver_result: DriverResult::Idle,
            frame_count: 0,
            sample_rate: 44100,
            channel_count: 2,
        }
    }
}
