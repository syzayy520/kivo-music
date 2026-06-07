use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Error, Serialize)]
pub enum PlaybackError {
    #[error("backend error: {0}")]
    Backend(String),
    #[error("path error: {0}")]
    Path(String),
    #[error("queue error: {0}")]
    Queue(String),
    #[error("output error: {0}")]
    Output(String),
    #[error("unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("invalid control input: {0}")]
    InvalidControlInput(String),
    #[error("unsupported operation: {0}")]
    UnsupportedOperation(String),
    #[error("playback failed: {0}")]
    Playback(String),
}

pub type PlaybackResult<T> = Result<T, PlaybackError>;
