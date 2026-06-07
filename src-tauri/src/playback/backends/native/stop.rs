use super::{tap_diagnostic, KivoNativeEngine};
use crate::playback::errors::PlaybackResult;
use crate::playback::state::PlaybackState;

pub(super) fn stop_track(engine: &mut KivoNativeEngine) -> PlaybackResult<PlaybackState> {
    tap_diagnostic::close_on_stop(&mut engine.tap_diagnostic, &mut engine.pipeline);

    engine.pipeline.stop_output()?;

    let status = engine.playback.stop()?;
    engine.state.status = status;
    engine.state.current_track = engine.playback.current_track();
    engine.state.error = None;

    Ok(engine.state.clone())
}
