//! Sink dispatch types.
//!
//! Pure memory dispatch types bridging the output consumer runtime
//! and the sink consumer boundary. No WASAPI, no IO, no actual rendering.

pub mod consumer_invoker;
pub mod dispatch_context;
pub mod dispatch_error;
pub mod dispatch_outcome;
pub mod driver_sink_dispatch;
pub mod event_routing;

pub use dispatch_context::DispatchContext;
pub use dispatch_error::DispatchError;
pub use dispatch_outcome::DispatchOutcome;
