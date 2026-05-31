use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PlaybackLifecycleEventKind {
    Initialized,
    ShutdownStarted,
    ShutdownSucceeded,
    ShutdownFailed,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlaybackLifecycleEvent {
    pub kind: PlaybackLifecycleEventKind,
    pub backend_name: String,
    pub message: Option<String>,
}

impl PlaybackLifecycleEvent {
    pub fn initialized(backend_name: impl Into<String>) -> Self {
        Self {
            kind: PlaybackLifecycleEventKind::Initialized,
            backend_name: backend_name.into(),
            message: None,
        }
    }

    pub fn shutdown_started(backend_name: impl Into<String>) -> Self {
        Self {
            kind: PlaybackLifecycleEventKind::ShutdownStarted,
            backend_name: backend_name.into(),
            message: None,
        }
    }

    pub fn shutdown_succeeded(backend_name: impl Into<String>) -> Self {
        Self {
            kind: PlaybackLifecycleEventKind::ShutdownSucceeded,
            backend_name: backend_name.into(),
            message: None,
        }
    }

    pub fn shutdown_failed(backend_name: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            kind: PlaybackLifecycleEventKind::ShutdownFailed,
            backend_name: backend_name.into(),
            message: Some(message.into()),
        }
    }
}
