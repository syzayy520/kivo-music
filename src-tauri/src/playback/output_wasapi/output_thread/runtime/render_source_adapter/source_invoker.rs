//! Source invoker.
//!
//! Pure memory invocation of a RenderSource trait object.
//! No threads, no IO, no WASAPI.

use crate::playback::output_wasapi::output_thread::runtime::render_source_adapter::{
    AdapterContext, AdapterError, AdapterOutcome,
};
use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::{
    RenderSource, RenderSourceError, RenderSourceRequest, RenderSourceResult,
};

/// Invoke a render source with a request and return an AdapterOutcome.
///
/// Returns `AdapterOutcome::Skipped` if source is not ready.
/// Pure memory — no threads, no IO, no WASAPI.
pub fn invoke_render_source<S: RenderSource>(
    source: &mut S,
    request: &RenderSourceRequest,
    _context: &AdapterContext,
) -> Result<AdapterOutcome, AdapterError> {
    if !source.is_ready() {
        return Ok(AdapterOutcome::Skipped);
    }

    match source.process_request(request) {
        Ok(result) => Ok(map_source_result_to_outcome(&result)),
        Err(err) => Err(map_source_error_to_adapter_error(err)),
    }
}

/// Map a RenderSourceResult to an AdapterOutcome.
pub fn map_source_result_to_outcome(result: &RenderSourceResult) -> AdapterOutcome {
    match result {
        RenderSourceResult::Packet {
            frames_provided,
            bytes_read,
        } => AdapterOutcome::Packet {
            frames_provided: *frames_provided,
            bytes_read: *bytes_read,
        },
        RenderSourceResult::Exhausted => AdapterOutcome::Exhausted,
        RenderSourceResult::Skipped => AdapterOutcome::Skipped,
        RenderSourceResult::Noop => AdapterOutcome::Noop,
    }
}

/// Map a RenderSourceError to an AdapterError.
pub fn map_source_error_to_adapter_error(err: RenderSourceError) -> AdapterError {
    match err {
        RenderSourceError::SourceExhausted => AdapterError::SourceExhausted,
        RenderSourceError::FormatMismatch { expected, actual } => {
            AdapterError::FormatMismatch { expected, actual }
        }
        RenderSourceError::SourceClosed => AdapterError::SourceClosed,
        RenderSourceError::Internal { description } => AdapterError::Internal { description },
    }
}
