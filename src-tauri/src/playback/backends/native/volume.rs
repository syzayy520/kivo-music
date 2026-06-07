use super::KivoNativeEngine;
use crate::playback::errors::{PlaybackError, PlaybackResult};
use crate::playback::state::PlaybackState;

pub(super) fn set_volume(
    engine: &mut KivoNativeEngine,
    level: f32,
) -> PlaybackResult<PlaybackState> {
    if !level.is_finite() {
        return Err(PlaybackError::InvalidControlInput(
            "volume level must be finite".to_string(),
        ));
    }

    engine.pipeline.set_output_volume(level)?;
    engine.state.volume.level = level.clamp(0.0, 1.0);
    engine.state.error = None;
    Ok(engine.state.clone())
}
