//! Render loop configuration type.
//!
//! Defines the runtime configuration for the render loop.

/// Render loop timing configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RenderLoopConfig {
    /// Target frame count per render cycle.
    pub frames_per_cycle: u32,
    /// Maximum number of retry attempts on transient errors.
    pub max_retries: u32,
    /// Sleep duration in microseconds when idle.
    pub idle_sleep_us: u64,
}

impl Default for RenderLoopConfig {
    fn default() -> Self {
        Self {
            frames_per_cycle: 1024,
            max_retries: 3,
            idle_sleep_us: 1000,
        }
    }
}