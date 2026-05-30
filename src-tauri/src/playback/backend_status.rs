use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BackendStatus {
    pub backend_name: String,
    pub available: bool,
    pub note: Option<String>,
}
