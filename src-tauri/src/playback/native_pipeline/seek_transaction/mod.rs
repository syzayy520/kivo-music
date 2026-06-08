mod apply;
mod rollback;
mod snapshot;

use crate::playback::errors::PlaybackResult;
use crate::playback::native_pipeline::NativePipeline;

pub(in crate::playback) fn seek_decoder_transaction(
    pipeline: &mut NativePipeline,
    position_ms: u64,
) -> PlaybackResult<()> {
    let snapshot = snapshot::NativePipelineSeekSnapshot::capture(pipeline);

    match apply::apply_seek(pipeline, position_ms) {
        Ok(()) => Ok(()),
        Err(error) => {
            rollback::restore(pipeline, snapshot);
            Err(error)
        }
    }
}
