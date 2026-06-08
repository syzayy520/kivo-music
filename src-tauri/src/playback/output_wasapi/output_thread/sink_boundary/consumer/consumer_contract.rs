//! Consumer contract trait.
//!
//! Device-agnostic trait defining the interface for a sink consumer.
//! No WASAPI, no IO, no actual rendering.

use crate::playback::output_wasapi::output_thread::sink_boundary::{
    SinkError, SinkRequest, SinkResult,
};

use super::consumer_snapshot::ConsumerSnapshot;

/// Trait for a sink consumer that processes render requests.
///
/// This is a contract shell — implementations will be added later.
/// No WASAPI, no IO, no actual rendering.
pub trait SinkConsumer {
    /// Process a sink request and return a result.
    fn process_request(&mut self, request: &SinkRequest) -> Result<SinkResult, SinkError>;

    /// Get a snapshot of the consumer's current state.
    fn snapshot(&self) -> ConsumerSnapshot;

    /// Check if the consumer is ready to accept requests.
    fn is_ready(&self) -> bool;

    /// Reset the consumer to its initial state.
    fn reset(&mut self);
}
