use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KivoCoreAudioProfile {
    pub brand_name: String,
    pub engine_name: String,
    pub primary_backend_name: String,
    pub compatibility_backend_names: Vec<String>,
    pub native_first: bool,
}

impl Default for KivoCoreAudioProfile {
    fn default() -> Self {
        Self {
            brand_name: "Kivo Core Audio".to_string(),
            engine_name: "KivoNativeEngine".to_string(),
            primary_backend_name: "kivo-core-audio".to_string(),
            compatibility_backend_names: vec!["mpv".to_string()],
            native_first: true,
        }
    }
}
