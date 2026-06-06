use super::super::render_ring_buffer_boundary::RenderRingBufferBoundaryError;
use super::super::thread_error::RealOutputThreadSkeletonError;

pub(crate) fn map_render_ring_buffer_boundary_error(
    error: RenderRingBufferBoundaryError,
) -> RealOutputThreadSkeletonError {
    match error {
        RenderRingBufferBoundaryError::RequiresContext => {
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryRequiresOpenContext
        }
        RenderRingBufferBoundaryError::RequiresStartedClient => {
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryRequiresStartedClient
        }
        RenderRingBufferBoundaryError::InvalidIterationCount => {
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryInvalidIterationCount
        }
        RenderRingBufferBoundaryError::InvalidFrameCount => {
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryInvalidFrameCount
        }
        RenderRingBufferBoundaryError::IterationCountTooLarge => {
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryIterationCountTooLarge
        }
        RenderRingBufferBoundaryError::FrameCountTooLarge => {
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryFrameCountTooLarge
        }
        RenderRingBufferBoundaryError::SyntheticSeedFrameCountTooLarge => {
            RealOutputThreadSkeletonError::RenderRingBufferBoundarySyntheticSeedFrameCountTooLarge
        }
        RenderRingBufferBoundaryError::MissingFormatCache => {
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryMissingFormatCache
        }
        RenderRingBufferBoundaryError::UnsupportedFormat => {
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryUnsupportedFormat
        }
        RenderRingBufferBoundaryError::CommitAfterWriteFailed(error) => {
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryCommitAfterWriteFailed(error)
        }
        RenderRingBufferBoundaryError::PcmAdapter(error) => {
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryFailed(format!(
                "PcmAdapter({error:?})"
            ))
        }
        RenderRingBufferBoundaryError::SyntheticSourceSeedFailed(error) => {
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryFailed(format!(
                "SyntheticSourceSeedFailed({error})"
            ))
        }
        RenderRingBufferBoundaryError::BoundaryFailed(error) => {
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryFailed(error)
        }
        RenderRingBufferBoundaryError::ByteLengthOverflow => {
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryFailed(
                "ByteLengthOverflow".to_string(),
            )
        }
    }
}
