use super::KivoNativeEngine;
use crate::playback::errors::PlaybackResult;
use crate::playback::state::PlaybackState;

pub(super) fn set_muted(
    engine: &mut KivoNativeEngine,
    muted: bool,
) -> PlaybackResult<PlaybackState> {
    engine.pipeline.set_output_muted(muted)?;
    engine.state.volume.muted = muted;
    engine.state.error = None;
    Ok(engine.state.clone())
}
