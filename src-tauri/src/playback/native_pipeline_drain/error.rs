use crate::playback::errors::{PlaybackError, PlaybackResult};
use crate::playback::native_pipeline::NativePipeline;

const EMPTY_BUFFER_ERROR_MESSAGE: &str = "pipeline buffer is empty, no frame to drain";

pub(in crate::playback) fn empty_buffer_error() -> PlaybackError {
    PlaybackError::Backend(EMPTY_BUFFER_ERROR_MESSAGE.to_string())
}

pub(in crate::playback) fn handle_drain_submit_error(
    pipeline: &mut NativePipeline,
    error: PlaybackError,
) -> PlaybackResult<()> {
    pipeline.refresh_drain_submit_error_status();
    Err(error)
}
