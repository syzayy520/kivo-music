use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OutputSettings {
    pub selected_device_id: Option<String>,
    pub exclusive_mode: bool,
    pub bit_perfect_mode: bool,
}
