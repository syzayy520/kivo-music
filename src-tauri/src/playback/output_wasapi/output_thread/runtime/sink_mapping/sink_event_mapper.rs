//! Sink event mapper.
//!
//! Pure function mapping SinkResult to ThreadEvent metadata.
//! No behavior, no IO, no WASAPI.

use crate::playback::output_wasapi::output_thread::event::thread_event::ThreadEvent;
use crate::playback::output_wasapi::output_thread::sink_boundary::SinkResult;

/// Extract metadata from SinkResult for event emission.
///
/// Returns a descriptive string for logging/tracing.
/// This is a pure memory mapping — no side effects, no IO.
pub fn map_sink_result_to_thread_event(result: &SinkResult) -> Option<ThreadEvent> {
    match result {
        SinkResult::Success { .. } => None,
        SinkResult::SilenceFilled { .. } => None,
        SinkResult::Skipped => None,
        SinkResult::Noop => None,
    }
}
