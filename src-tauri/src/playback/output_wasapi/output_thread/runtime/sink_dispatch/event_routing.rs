//! Event routing.
//!
//! Maps DispatchOutcome and SinkResult to ThreadEvent or event buffer input.
//! No WASAPI, no IO.

use crate::playback::output_wasapi::output_thread::event::thread_event::ThreadEvent;
use crate::playback::output_wasapi::output_thread::runtime::sink_dispatch::DispatchOutcome;

/// Convert a DispatchOutcome into an optional ThreadEvent.
///
/// Returns None for all variants — ThreadEvent only supports lifecycle events
/// (StateChanged/Spawned/Joined), and dispatch outcomes are runtime-internal
/// data that cannot create new cross-family events.
/// STOP_SCOPE_EXPANSION: do not add new ThreadEvent variants here.
pub fn map_dispatch_outcome_to_event(_outcome: &DispatchOutcome) -> Option<ThreadEvent> {
    None
}
