//! Consumer invoker.
//!
//! Pure memory invocation of a SinkConsumer trait object.
//! No threads, no IO, no WASAPI.

use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::{
    DispatchContext, DispatchError, DispatchOutcome,
};
use crate::playback::output_wasapi::output_thread::sink_boundary::{
    consumer::consumer_contract::SinkConsumer, SinkError, SinkRequest, SinkResult,
};

/// Invoke a sink consumer with a request and return a DispatchOutcome.
///
/// Returns `DispatchOutcome::Skipped` if consumer is not ready.
/// Pure memory — no threads, no IO, no WASAPI.
pub fn invoke_sink_consumer<C: SinkConsumer>(
    consumer: &mut C,
    request: &SinkRequest,
    _context: &DispatchContext,
) -> Result<DispatchOutcome, DispatchError> {
    if !consumer.is_ready() {
        return Ok(DispatchOutcome::Skipped);
    }

    match consumer.process_request(request) {
        Ok(result) => Ok(map_sink_result_to_outcome(&result)),
        Err(err) => Err(map_sink_error_to_dispatch_error(err)),
    }
}

/// Map a SinkResult to a DispatchOutcome.
pub fn map_sink_result_to_outcome(result: &SinkResult) -> DispatchOutcome {
    match result {
        SinkResult::Success {
            frames_processed,
            bytes_written,
        } => DispatchOutcome::Success {
            frames_processed: *frames_processed,
            bytes_written: *bytes_written,
        },
        SinkResult::SilenceFilled { frames_written } => DispatchOutcome::SilenceFilled {
            frames_written: *frames_written,
        },
        SinkResult::Skipped => DispatchOutcome::Skipped,
        SinkResult::Noop => DispatchOutcome::Noop,
    }
}

/// Map a SinkError to a DispatchError.
pub fn map_sink_error_to_dispatch_error(err: SinkError) -> DispatchError {
    match err {
        SinkError::BufferUnderrun { frames_missing } => {
            DispatchError::BufferUnderrun { frames_missing }
        }
        SinkError::DeviceLost => DispatchError::DeviceLost,
        SinkError::InvalidRequest { reason } => DispatchError::InvalidRequest { reason },
        SinkError::Internal { description } => DispatchError::Internal { description },
    }
}
