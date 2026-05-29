use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlaybackCapabilities {
    pub can_seek: bool,
    pub can_select_output_device: bool,
    pub can_use_exclusive_output: bool,
    pub can_probe_metadata: bool,
    pub can_gapless: bool,
    pub can_replaygain: bool,
}

impl Default for PlaybackCapabilities {
    fn default() -> Self {
        Self {
            can_seek: false,
            can_select_output_device: false,
            can_use_exclusive_output: false,
            can_probe_metadata: false,
            can_gapless: false,
            can_replaygain: false,
        }
    }
}
