use super::KivoNativeEngine;
use crate::playback::errors::{PlaybackError, PlaybackResult};
use crate::playback::state::PlaybackState;
use crate::playback::types::PlaybackStatus;

pub(super) fn record_error(
    engine: &mut KivoNativeEngine,
    error: PlaybackError,
) -> PlaybackResult<PlaybackState> {
    engine.state.error = Some(error.to_string());
    Err(error)
}

pub(super) fn apply_status_result(
    engine: &mut KivoNativeEngine,
    result: PlaybackResult<PlaybackStatus>,
) -> PlaybackResult<PlaybackState> {
    match result {
        Ok(status) => {
            engine.state.status = status;
            Ok(engine.state.clone())
        }
        Err(error) => record_error(engine, error),
    }
}
