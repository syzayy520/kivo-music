use crate::playback::output_wasapi::pcm_adapter::PcmAdapterError;
use crate::playback::output_wasapi::ring_buffer::RingBufferError;
use crate::playback::output_wasapi::ring_buffer_render_boundary::RingBufferRenderBoundaryError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RenderRingBufferBoundaryError {
    InvalidIterationCount,
    InvalidFrameCount,
    IterationCountTooLarge,
    FrameCountTooLarge,
    SyntheticSeedFrameCountTooLarge,
    RequiresContext,
    RequiresStartedClient,
    MissingFormatCache,
    UnsupportedFormat,
    PcmAdapter(PcmAdapterError),
    SyntheticSourceSeedFailed(String),
    BoundaryFailed(String),
    CommitAfterWriteFailed(String),
    ByteLengthOverflow,
}

pub(crate) fn map_ring_buffer_boundary_error(
    error: RingBufferRenderBoundaryError,
) -> RenderRingBufferBoundaryError {
    match error {
        RingBufferRenderBoundaryError::CommitAfterWriteFailed(error) => {
            RenderRingBufferBoundaryError::CommitAfterWriteFailed(format!("{error:?}"))
        }
        other => RenderRingBufferBoundaryError::BoundaryFailed(format!("{other:?}")),
    }
}

pub(super) fn map_seed_error(error: RingBufferError) -> RenderRingBufferBoundaryError {
    RenderRingBufferBoundaryError::SyntheticSourceSeedFailed(format!("{error:?}"))
}
