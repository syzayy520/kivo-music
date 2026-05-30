use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::backends::ffprobe::FfprobeBackend;
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

    pub fn probe(&self, path: &str) -> MediaProbeResultValue<MediaProbeResult> {
        self.backend.probe(path)
    }
}
