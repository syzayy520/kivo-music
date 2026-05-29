use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlaybackVolume {
    pub level: f32,
    pub muted: bool,
}

impl Default for PlaybackVolume {
    fn default() -> Self {
        Self {
            level: 1.0,
            muted: false,
        }
    }
}
