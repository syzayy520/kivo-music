use crate::playback::errors::{PlaybackError, PlaybackResult};
use crate::playback::native_pipeline::NativePipeline;

use super::submit_error_observation::{
    NativePipelineDrainObservedSubmitError, NativePipelineDrainSubmitErrorObservation,
};

const EMPTY_BUFFER_ERROR_MESSAGE: &str = "pipeline buffer is empty, no frame to drain";

pub(in crate::playback) fn empty_buffer_error() -> PlaybackError {
    PlaybackError::Backend(EMPTY_BUFFER_ERROR_MESSAGE.to_string())
}

pub(in crate::playback) fn handle_drain_submit_error(
    pipeline: &mut NativePipeline,
    observed: NativePipelineDrainObservedSubmitError,
) -> PlaybackResult<()> {
    handle_drain_submit_error_with_exporter(pipeline, observed, |_| {})
}

pub(in crate::playback) fn handle_drain_submit_error_with_exporter<F>(
    pipeline: &mut NativePipeline,
    observed: NativePipelineDrainObservedSubmitError,
    export: F,
) -> PlaybackResult<()>
where
    F: FnOnce(&NativePipelineDrainSubmitErrorObservation),
{
    pipeline.refresh_drain_submit_error_status();
    export(observed.observation());
    let error = observed.into_error();
    Err(error)
}
