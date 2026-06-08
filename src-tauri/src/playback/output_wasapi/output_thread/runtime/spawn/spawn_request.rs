//! Spawn request type.
//!
//! Pure data struct representing a request to spawn an output thread.
//! No spawning logic — only the data needed to configure a spawn.

use crate::playback::output_wasapi::output_thread::config::ThreadConfig;
use crate::playback::output_wasapi::output_thread::runtime::thread_handle::ThreadHandle;

/// Request to spawn a new output thread.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpawnRequest {
    /// Handle identifying the thread to spawn.
    pub handle: ThreadHandle,
    /// Thread configuration for the new thread.
    pub config: ThreadConfig,
    /// Whether to start the thread immediately after spawn.
    pub auto_start: bool,
}

impl SpawnRequest {
    /// Create a new spawn request with auto-start enabled.
    pub fn new(handle: ThreadHandle, config: ThreadConfig) -> Self {
        Self {
            handle,
            config,
            auto_start: true,
        }
    }

    /// Set whether to auto-start the thread.
    pub fn with_auto_start(mut self, auto_start: bool) -> Self {
        self.auto_start = auto_start;
        self
    }
}
