//! Safe drain helper for RingBuffer to WASAPI render buffer.
//!
//! Provides a one-shot drain operation that:
//! 1. Peeks frames from RingBuffer (non-destructive)
//! 2. Writes to WASAPI render buffer via `write_render_buffer_bytes`
//! 3. Only consumes from RingBuffer after successful write
//! 4. Only decrements `pending_frames` after successful consume
//!
//! **This module does NOT:**
//! - Modify `submit_frame` logic
//! - Automatically drain during playback
//! - Call `IAudioClient::Start`
//! - Create output threads
//! - Produce audible output

mod drain_once;
mod error;
mod report;
#[cfg(test)]
mod tests_core;
#[cfg(test)]
mod tests_error;
mod tick;
mod tick_error;
mod tick_report;
#[cfg(test)]
mod tick_tests;

#[allow(unused_imports)] // temporary until P0-074E2 render loop wiring
pub use drain_once::{drain_ring_buffer_once_with_writer, drain_wasapi_output_sink_once};
#[allow(unused_imports)] // temporary until P0-074E2 render loop wiring
pub use error::WasapiRingBufferDrainError;
#[allow(unused_imports)] // temporary until P0-074E2 render loop wiring
pub use report::WasapiRingBufferDrainReport;
#[allow(unused_imports)] // temporary until P0-074E2 render loop wiring
pub(crate) use tick::manual_drain_tick;
#[allow(unused_imports)] // temporary until P0-074E2 render loop wiring
pub(crate) use tick_error::WasapiDrainTickError;
#[allow(unused_imports)] // temporary until P0-074E2 render loop wiring
pub(crate) use tick_report::{WasapiDrainTickReport, WasapiDrainTickSkipReason};
