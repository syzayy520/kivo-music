use super::{tap_diagnostic, KivoNativeEngine};
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

pub(super) fn seek_decoder(engine: &mut KivoNativeEngine, position_ms: u64) {
    if engine.pipeline.seek_decoder(position_ms).is_ok() {
        tap_diagnostic::reset_after_seek_success(&mut engine.tap_diagnostic, &mut engine.pipeline);
    }
}
