//! Thread driver type.
//!
//! Pure data struct representing the driver configuration and state
//! for the output thread. No actual driving logic.

use crate::playback::output_wasapi::output_thread::config::ThreadConfig;
use crate::playback::output_wasapi::output_thread::runtime::thread_handle::ThreadHandle;

/// Configuration and identity for the output thread driver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreadDriver {
    /// Handle identifying this driver instance.
    pub handle: ThreadHandle,
    /// Thread configuration.
    pub config: ThreadConfig,
    /// Maximum consecutive idle steps before yielding.
    pub max_idle_steps: u64,
}

impl ThreadDriver {
    /// Create a new thread driver with the given handle and config.
    pub fn new(handle: ThreadHandle, config: ThreadConfig) -> Self {
        Self {
            handle,
            config,
            max_idle_steps: 100,
        }
    }

    /// Set the maximum idle steps.
    pub fn with_max_idle_steps(mut self, max_idle_steps: u64) -> Self {
        self.max_idle_steps = max_idle_steps;
        self
    }
}
