/// Configuration for the output thread runtime.
///
/// Pure configuration — no device queries, no WASAPI calls.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeConfig {
    /// Target audio latency in milliseconds.
    pub target_latency_ms: u32,
    /// Maximum time to wait for thread start in milliseconds.
    pub max_start_wait_ms: u32,
    /// Maximum time to wait for thread stop in milliseconds.
    pub max_stop_wait_ms: u32,
    /// Maximum time to wait for thread join in milliseconds.
    pub max_join_wait_ms: u32,
    /// Whether device reset is allowed during error recovery.
    pub allow_device_reset: bool,
}

impl Default for OutputThreadRuntimeConfig {
    fn default() -> Self {
        Self {
            target_latency_ms: 50,
            max_start_wait_ms: 500,
            max_stop_wait_ms: 1000,
            max_join_wait_ms: 1500,
            allow_device_reset: false,
        }
    }
}

#[allow(dead_code)]
impl OutputThreadRuntimeConfig {
    /// Create a low-latency configuration.
    ///
    /// Uses a lower target latency while keeping reasonable timeouts.
    pub(crate) fn low_latency() -> Self {
        Self {
            target_latency_ms: 20,
            max_start_wait_ms: 300,
            max_stop_wait_ms: 800,
            max_join_wait_ms: 1200,
            allow_device_reset: false,
        }
    }

    /// Enable device reset on error recovery.
    pub(crate) fn with_device_reset_enabled(mut self) -> Self {
        self.allow_device_reset = true;
        self
    }

    /// Whether device reset is allowed.
    pub(crate) fn is_reset_allowed(self) -> bool {
        self.allow_device_reset
    }

    /// Whether all timeout values are valid (non-zero).
    pub(crate) fn has_valid_timeouts(self) -> bool {
        self.target_latency_ms > 0
            && self.max_start_wait_ms > 0
            && self.max_stop_wait_ms > 0
            && self.max_join_wait_ms > 0
    }
}
