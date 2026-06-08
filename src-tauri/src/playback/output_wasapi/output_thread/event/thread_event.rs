//! Thread event type.
//!
//! Defines events representing lifecycle transitions of the output thread.

use crate::playback::output_wasapi::output_thread::state::OutputThreadLifecycle;

/// Events emitted on thread lifecycle transitions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ThreadEvent {
    /// Thread lifecycle state changed.
    StateChanged {
        /// Previous state.
        from: OutputThreadLifecycle,
        /// New state.
        to: OutputThreadLifecycle,
    },
    /// Thread spawned successfully.
    Spawned {
        /// Thread ID or index.
        thread_id: u64,
    },
    /// Thread join completed.
    Joined,
}

impl Default for ThreadEvent {
    fn default() -> Self {
        Self::StateChanged {
            from: OutputThreadLifecycle::default(),
            to: OutputThreadLifecycle::default(),
        }
    }
}
