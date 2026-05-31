use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Error, Serialize)]
pub enum MediaProbeError {
    #[error("invalid probe path: {0}")]
    InvalidPath(String),
    #[error("probe backend unavailable: {0}")]
    BackendUnavailable(String),
    #[error("probe failed: {0}")]
    ProbeFailed(String),
    #[error("unsupported probe operation: {0}")]
    UnsupportedOperation(String),
}

pub type MediaProbeResultValue<T> = Result<T, MediaProbeError>;
