use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PlaybackCapabilities {
    pub can_seek: bool,
    pub can_select_output_device: bool,
    pub can_use_exclusive_output: bool,
    pub can_probe_metadata: bool,
    pub can_gapless: bool,
    pub can_replaygain: bool,
}
