use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MediaProbeEventKind {
    Started,
    Succeeded,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MediaProbeEvent {
    pub kind: MediaProbeEventKind,
    pub path: String,
    pub backend_name: String,
    pub message: Option<String>,
}

impl MediaProbeEvent {
    pub fn started(path: impl Into<String>, backend_name: impl Into<String>) -> Self {
        Self {
            kind: MediaProbeEventKind::Started,
            path: path.into(),
            backend_name: backend_name.into(),
            message: None,
        }
    }

    pub fn succeeded(path: impl Into<String>, backend_name: impl Into<String>) -> Self {
        Self {
            kind: MediaProbeEventKind::Succeeded,
            path: path.into(),
            backend_name: backend_name.into(),
            message: None,
        }
    }

    pub fn failed(
        path: impl Into<String>,
        backend_name: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind: MediaProbeEventKind::Failed,
            path: path.into(),
            backend_name: backend_name.into(),
            message: Some(message.into()),
        }
    }
}
