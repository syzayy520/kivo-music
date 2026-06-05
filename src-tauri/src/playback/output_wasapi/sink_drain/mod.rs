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
mod output_thread_adapter_slot;
mod output_thread_adapter_slot_error;
mod output_thread_adapter_slot_report;
#[cfg(test)]
mod output_thread_adapter_slot_tests;
mod output_thread_owned_state;
mod output_thread_owned_state_error;
mod output_thread_owned_state_report;
#[cfg(test)]
mod output_thread_owned_state_tests;
mod render_loop_error;
mod render_loop_report;
mod render_loop_step;
#[cfg(test)]
mod render_loop_tests;
mod render_step_adapter;
mod render_step_adapter_error;
mod render_step_adapter_report;
#[cfg(test)]
mod render_step_adapter_tests;
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

#[allow(unused_imports)] // temporary until P0-074E5 real output-thread wiring
pub use drain_once::{drain_ring_buffer_once_with_writer, drain_wasapi_output_sink_once};
#[allow(unused_imports)] // temporary until P0-074E5 real output-thread wiring
pub use error::WasapiRingBufferDrainError;
#[allow(unused_imports)] // temporary until P0-074E5 real output-thread wiring
pub(crate) use output_thread_adapter_slot::run_wasapi_output_thread_adapter_slot;
#[allow(unused_imports)] // temporary until P0-074E5 real output-thread wiring
pub(crate) use output_thread_adapter_slot_error::WasapiOutputThreadAdapterSlotError;
#[allow(unused_imports)] // temporary until P0-074E5 real output-thread wiring
pub(crate) use output_thread_adapter_slot_report::{
    WasapiOutputThreadAdapterSlotConfig, WasapiOutputThreadAdapterSlotReport,
};
#[allow(unused_imports)] // temporary until P0-074E5B command contract wiring
pub(crate) use output_thread_owned_state::{
    default_wasapi_output_thread_owned_state_contract,
    validate_wasapi_output_thread_owned_state_contract,
};
#[allow(unused_imports)] // temporary until P0-074E5B command contract wiring
pub(crate) use output_thread_owned_state_error::WasapiOutputThreadOwnedStateError;
#[allow(unused_imports)] // temporary until P0-074E5B command contract wiring
pub(crate) use output_thread_owned_state_report::{
    WasapiOutputThreadOwnedStateContract, WasapiOutputThreadOwnedStateOwner,
    WasapiOutputThreadOwnedStateReport, WasapiOutputThreadOwnedStateStage,
};
#[allow(unused_imports)] // temporary until P0-074E5 real output-thread wiring
pub(crate) use render_loop_error::WasapiRenderLoopStepError;
#[allow(unused_imports)] // temporary until P0-074E5 real output-thread wiring
pub(crate) use render_loop_report::{
    WasapiRenderLoopStepPlan, WasapiRenderLoopStepReport, WasapiRenderLoopStepSkipReason,
};
#[allow(unused_imports)] // temporary until P0-074E5 real output-thread wiring
pub(crate) use render_loop_step::run_manual_drain_render_loop_step;
#[allow(unused_imports)] // temporary until P0-074E5 real output-thread wiring
pub(crate) use render_step_adapter::run_wasapi_render_step_adapter;
#[allow(unused_imports)] // temporary until P0-074E5 real output-thread wiring
pub(crate) use render_step_adapter_error::WasapiRenderStepAdapterError;
#[allow(unused_imports)] // temporary until P0-074E5 real output-thread wiring
pub(crate) use render_step_adapter_report::{
    WasapiRenderStepAdapterConfig, WasapiRenderStepAdapterReport,
};
#[allow(unused_imports)] // temporary until P0-074E5 real output-thread wiring
pub use report::WasapiRingBufferDrainReport;
#[allow(unused_imports)] // temporary until P0-074E5 real output-thread wiring
pub(crate) use tick::manual_drain_tick;
#[allow(unused_imports)] // temporary until P0-074E5 real output-thread wiring
pub(crate) use tick_error::WasapiDrainTickError;
#[allow(unused_imports)] // temporary until P0-074E5 real output-thread wiring
pub(crate) use tick_report::{WasapiDrainTickReport, WasapiDrainTickSkipReason};
