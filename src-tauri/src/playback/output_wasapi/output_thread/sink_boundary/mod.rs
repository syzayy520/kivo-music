//! Sink boundary contract types.
//!
//! Device-agnostic types defining the interface between the output thread
//! runtime and a future sink/render backend. No WASAPI, no IO, no actual rendering.

pub mod consumer;
pub mod render_packet;
pub mod render_source;
pub mod sink_error;
pub mod sink_request;
pub mod sink_result;

pub use consumer::{ConsumerSnapshot, SinkConsumer};
pub use render_packet::{AudioPacket, PacketFormat, PacketTiming, SampleFormat};
pub use render_source::{RenderSourceError, RenderSourceRequest, RenderSourceResult};
pub use sink_error::SinkError;
pub use sink_request::SinkRequest;
pub use sink_result::SinkResult;
