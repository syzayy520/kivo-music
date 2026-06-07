use serde::{Deserialize, Serialize};

use super::controls::OutputControlState;
use super::latency::OutputLatency;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OutputRuntimeStatus {
    pub is_open: bool,
    pub is_active: bool,
    pub active_device_id: Option<String>,
    pub pending_frames: usize,
    pub latency: OutputLatency,
    pub controls: OutputControlState,
    pub gap_count: u64,
    pub last_error: Option<String>,
}
