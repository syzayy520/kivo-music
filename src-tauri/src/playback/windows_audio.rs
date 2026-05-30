use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WindowsAudioPlan {
    pub device_id: Option<String>,
    pub dedicated_mode: bool,
    pub exact_format_requested: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WindowsAudioStatus {
    pub available: bool,
    pub active_device_id: Option<String>,
    pub note: Option<String>,
}
