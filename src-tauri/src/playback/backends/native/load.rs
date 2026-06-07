use super::{tap_diagnostic, KivoNativeEngine};
use crate::playback::decoder_request::AudioDecoderOpenRequest;
use crate::playback::errors::{PlaybackError, PlaybackResult};
use crate::playback::state::PlaybackState;
use crate::playback::types::PlaybackTrack;

pub(super) fn load_track(
    engine: &mut KivoNativeEngine,
    track: PlaybackTrack,
) -> PlaybackResult<PlaybackState> {
    if let Err(error) = load_pipeline_until_output_boundary(engine, &track) {
        engine.state.error = Some(error.to_string());
        return Err(PlaybackError::UnsupportedOperation(
            super::super::native_unsupported::unsupported_operation_message("load"),
        ));
    }

    engine.playback.load_track(track);
    engine.state.current_track = engine.playback.current_track();
    engine.state.status = engine.playback.current_status();
    engine.state.error = None;

    Ok(engine.state.clone())
}

fn load_pipeline_until_output_boundary(
    engine: &mut KivoNativeEngine,
    track: &PlaybackTrack,
) -> PlaybackResult<()> {
    let request = AudioDecoderOpenRequest::from_track(track);
    let session = engine.pipeline.open_decoder(request, 0)?;
    tap_diagnostic::open_after_decoder_open(
        &mut engine.tap_diagnostic,
        &mut engine.pipeline,
        session.stream_info,
    );
    engine.pipeline.schedule_decode_step()?;

    match engine.pipeline.schedule_output_submit_step() {
        Ok(()) => Ok(()),
        Err(PlaybackError::UnsupportedOperation(_)) => Ok(()),
        Err(error) => Err(error),
    }
}
