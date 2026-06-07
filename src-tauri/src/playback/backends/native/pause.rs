use super::KivoNativeEngine;
use crate::playback::errors::PlaybackResult;
use crate::playback::state::PlaybackState;
use crate::playback::types::PlaybackStatus;

pub(super) fn pause_track(engine: &mut KivoNativeEngine) -> PlaybackResult<PlaybackState> {
    if !matches!(engine.state.status, PlaybackStatus::Playing) {
        return Ok(engine.state.clone());
    }

    engine.pipeline.pause_output()?;
    let status = engine.playback.mark_paused();
    engine.state.status = status;
    engine.state.error = None;

    Ok(engine.state.clone())
}
