//! Render source adapter types.
//!
//! Pure data types for connecting the RenderSource trait into the
//! runtime pump/sink dispatch chain. No IO, no threads, no WASAPI.

pub mod adapter_context;
pub mod adapter_error;
pub mod adapter_outcome;
pub mod pump_integration;
pub mod source_invoker;
pub mod source_to_sink_dispatch;

pub use adapter_context::AdapterContext;
pub use adapter_error::AdapterError;
pub use adapter_outcome::AdapterOutcome;
pub use pump_integration::{execute_pump_tick_with_source, SourcePumpOutcome};
pub use source_invoker::{
    invoke_render_source, map_source_error_to_adapter_error, map_source_result_to_outcome,
};
pub use source_to_sink_dispatch::{dispatch_source_to_sink, SourceToSinkOutcome};
