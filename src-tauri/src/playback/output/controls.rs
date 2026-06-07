use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OutputControlState {
    pub volume_level: f32,
    pub muted: bool,
}

impl Default for OutputControlState {
    fn default() -> Self {
        Self {
            volume_level: 1.0,
            muted: false,
        }
    }
}
