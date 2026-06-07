use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct OutputLatency {
    pub requested_ms: Option<u32>,
    pub measured_ms: Option<u32>,
}
