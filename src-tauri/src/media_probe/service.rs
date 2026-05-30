use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::backends::ffprobe::FfprobeBackend;
use super::backends::ffprobe_status::ffprobe_status;
use super::status::MediaProbeBackendStatus;
use super::types::MediaProbeResult;

#[derive(Clone, Debug, Deserialize, Error, Serialize)]
pub enum MediaProbeError {
    #[error("probe backend unavailable: {0}")]
    BackendUnavailable(String),
    #[error("probe failed: {0}")]
    ProbeFailed(String),
    #[error("unsupported probe operation: {0}")]
    UnsupportedOperation(String),
}

pub type MediaProbeResultValue<T> = Result<T, MediaProbeError>;

pub trait ProbeBackend {
    fn name(&self) -> &'static str;
    fn probe(&self, path: &str) -> MediaProbeResultValue<MediaProbeResult>;
}

#[derive(Clone, Debug, Default)]
pub struct MediaProbeService {
    backend: FfprobeBackend,
}

impl MediaProbeService {
    pub fn new() -> Self {
        Self {
            backend: FfprobeBackend::default(),
        }
    }

    pub fn backend_name(&self) -> &'static str {
        self.backend.name()
    }

    pub fn backend_status(&self) -> MediaProbeBackendStatus {
        ffprobe_status()
    }

    pub fn probe(&self, path: &str) -> MediaProbeResultValue<MediaProbeResult> {
        self.backend.probe(path)
    }
}

#[derive(Debug, Default)]
pub struct MediaProbeServiceState {
    service: Mutex<MediaProbeService>,
}

impl MediaProbeServiceState {
    pub fn backend_name(&self) -> &'static str {
        let service = self
            .service
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        service.backend_name()
    }

    pub fn backend_status(&self) -> MediaProbeBackendStatus {
        let service = self
            .service
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        service.backend_status()
    }

    pub fn probe(&self, path: &str) -> MediaProbeResultValue<MediaProbeResult> {
        let service = self
            .service
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        service.probe(path)
    }
}
