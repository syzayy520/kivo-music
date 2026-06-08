//! Thread control type.
//!
//! Control interface types for the output thread. These represent
//! control requests that can be sent to the thread, distinct from
//! the data-only command types.

use crate::playback::output_wasapi::output_thread::command::ThreadCommand;
use crate::playback::output_wasapi::output_thread::config::ThreadConfig;

use super::thread_handle::ThreadHandle;

/// Control request sent to the output thread.
#[derive(Debug, Clone, PartialEq)]
pub enum ThreadControl {
    /// Initialize the thread with the given configuration.
    Initialize {
        /// The handle for this thread instance.
        handle: ThreadHandle,
        /// Configuration to apply.
        config: ThreadConfig,
    },
    /// Send a command to a running thread.
    Command {
        /// Target thread handle.
        handle: ThreadHandle,
        /// The command to execute.
        command: ThreadCommand,
    },
    /// Request a snapshot of the thread's current state.
    SnapshotRequest {
        /// Target thread handle.
        handle: ThreadHandle,
    },
}

impl Default for ThreadControl {
    fn default() -> Self {
        Self::Initialize {
            handle: ThreadHandle::default(),
            config: ThreadConfig::default(),
        }
    }
}
