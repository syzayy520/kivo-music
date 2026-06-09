//! Render source contract trait.
//!
//! Device-agnostic trait defining the interface for a render source.
//! No WASAPI, no IO, no actual decoding.

use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::{
    RenderSourceError, RenderSourceRequest, RenderSourceResult, SourceCursor, SourceSnapshot,
};

/// Trait for a render source that provides audio packets.
///
/// This is a contract shell — implementations will be added later.
/// No WASAPI, no IO, no actual decoding.
pub trait RenderSource {
    /// Process a source request and return a result.
    fn process_request(
        &mut self,
        request: &RenderSourceRequest,
    ) -> Result<RenderSourceResult, RenderSourceError>;

    /// Get a snapshot of the source's current state.
    fn snapshot(&self) -> SourceSnapshot;

    /// Get the current read cursor position.
    fn cursor(&self) -> SourceCursor;

    /// Check if the source is ready to serve requests.
    fn is_ready(&self) -> bool;

    /// Check if the source is fully exhausted.
    fn is_exhausted(&self) -> bool;

    /// Reset the source to its initial state.
    fn reset(&mut self);
}
