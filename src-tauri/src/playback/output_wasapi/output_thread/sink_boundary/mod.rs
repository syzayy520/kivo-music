//! Sink boundary contract types.
//!
//! Device-agnostic types defining the interface between the output thread
//! runtime and a future sink/render backend. No WASAPI, no IO, no actual rendering.

pub mod sink_error;
pub mod sink_request;
pub mod sink_result;

pub use sink_error::SinkError;
pub use sink_request::SinkRequest;
pub use sink_result::SinkResult;
