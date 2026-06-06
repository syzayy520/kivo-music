use super::{tap_diagnostic, KivoNativeEngine};
use crate::playback::errors::{PlaybackError, PlaybackResult};
use crate::playback::state::PlaybackState;
use crate::playback::types::PlaybackStatus;

pub(super) fn unsupported(
    engine: &mut KivoNativeEngine,
    operation: &str,
) -> PlaybackResult<PlaybackState> {
    let message = super::super::native_unsupported::unsupported_operation_message(operation);
    engine.state.error = Some(message.clone());
    Err(PlaybackError::UnsupportedOperation(message))
}

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

pub(super) fn set_output_volume(engine: &mut KivoNativeEngine, level: f32) {
    engine.state.volume.level = level.clamp(0.0, 1.0);
    let _ = engine.pipeline.set_output_volume(level);
}

pub(super) fn set_output_muted(engine: &mut KivoNativeEngine, muted: bool) {
    engine.state.volume.muted = muted;
    let _ = engine.pipeline.set_output_muted(muted);
}
