use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OutputDevice {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OutputSettings {
    pub selected_device_id: Option<String>,
    pub exclusive_mode: bool,
    pub bit_perfect_mode: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OutputLatency {
    pub requested_ms: Option<u32>,
    pub measured_ms: Option<u32>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OutputRuntimeStatus {
    pub is_open: bool,
    pub is_active: bool,
    pub active_device_id: Option<String>,
    pub pending_frames: usize,
    pub latency: OutputLatency,
    pub gap_count: u64,
    pub last_error: Option<String>,
}
