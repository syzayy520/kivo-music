use crate::audio::state::{AudioSessionStatus, QueueSnapshot};
use serde::Serialize;

#[cfg(not(windows))]
#[derive(Debug, Clone, Serialize)]
pub struct RenderDeviceInfo {
    pub id: String,
    pub name: String,
    pub is_default: bool,
}

#[derive(Clone, Serialize)]
pub struct AudioServiceTickResult {
    pub queue: QueueSnapshot,
    pub status: AudioSessionStatus,
}
