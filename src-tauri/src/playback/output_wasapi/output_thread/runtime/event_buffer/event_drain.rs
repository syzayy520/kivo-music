//! Event drain logic.
//!
//! Pure function that drains all events from an EventBuffer.

use crate::playback::output_wasapi::output_thread::event::ThreadEvent;

use super::event_buffer::EventBuffer;

/// Drain all events from the buffer, returning them in insertion order.
///
/// The buffer is left empty after this call.
pub fn drain_buffer(buffer: &mut EventBuffer) -> Vec<ThreadEvent> {
    buffer.take_all()
}
