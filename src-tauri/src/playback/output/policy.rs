use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum AudioOutputMode {
    #[default]
    Shared,
    Dedicated,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum AudioOutputFallbackPolicy {
    #[default]
    SystemDefault,
    KeepCurrent,
    Halt,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AudioOutputPolicy {
    pub selected_device_id: Option<String>,
    pub mode: AudioOutputMode,
    pub bit_perfect_intent: bool,
    pub fallback_policy: AudioOutputFallbackPolicy,
}

impl Default for AudioOutputPolicy {
    fn default() -> Self {
        Self {
            selected_device_id: None,
            mode: AudioOutputMode::Shared,
            bit_perfect_intent: false,
            fallback_policy: AudioOutputFallbackPolicy::SystemDefault,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AudioOutputHealth {
    pub active_device_id: Option<String>,
    pub last_error: Option<String>,
    pub device_disconnected: bool,
}
