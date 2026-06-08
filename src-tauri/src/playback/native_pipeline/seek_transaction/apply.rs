use crate::playback::errors::PlaybackResult;
use crate::playback::native_pipeline::NativePipeline;

/// Apply seek: call decoder.seek in a narrow scope, then commit wrapper mutations.
///
/// On decoder-not-open: returns error, no wrapper mutation.
/// On decoder.seek failure: returns error, no wrapper mutation (rollback handled by caller).
/// On success: commits infallible wrapper-state changes.
pub(super) fn apply_seek(pipeline: &mut NativePipeline, position_ms: u64) -> PlaybackResult<()> {
    let seek_result = {
        let decoder = pipeline.decoder.as_mut().ok_or_else(|| {
            crate::playback::errors::PlaybackError::Backend(
                "native pipeline decoder is not open".to_string(),
            )
        })?;
        decoder.seek(position_ms)
    };

    match seek_result {
        Ok(()) => {
            commit_seek(pipeline, position_ms);
            Ok(())
        }
        Err(error) => {
            pipeline.state.decoder_state.mark_failed(error.to_string());
            Err(error)
        }
    }
}

/// Infallible commit of wrapper-state mutations after successful decoder.seek.
fn commit_seek(pipeline: &mut NativePipeline, position_ms: u64) {
    pipeline.update_decoder_position(position_ms);
    pipeline.state.last_decoded_frame = None;
    pipeline.clear_buffer();
    pipeline.clock.set_position(position_ms);
}
