use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AudioOutputMode {
    Shared,
    Dedicated,
}

impl Default for AudioOutputMode {
    fn default() -> Self {
        Self::Shared
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AudioOutputFallbackPolicy {
    SystemDefault,
    KeepCurrent,
    Halt,
}

impl Default for AudioOutputFallbackPolicy {
    fn default() -> Self {
        Self::SystemDefault
    }
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
