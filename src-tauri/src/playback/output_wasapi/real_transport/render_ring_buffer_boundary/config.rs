use super::constants::{
    MAX_RENDER_RING_BUFFER_BOUNDARY_FRAMES_PER_WRITE, MAX_RENDER_RING_BUFFER_BOUNDARY_ITERATIONS,
    MAX_SYNTHETIC_ZERO_SEED_FRAMES,
};
use super::error::RenderRingBufferBoundaryError;
use crate::playback::output_wasapi::ring_buffer_render_boundary::RingBufferRenderBoundaryConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RenderRingBufferBoundaryConfig {
    pub enabled: bool,
    pub iterations: u32,
    pub max_frames_per_write: u32,
    pub synthetic_zero_seed_frames: u32,
}

pub(super) fn validate_config(
    config: RenderRingBufferBoundaryConfig,
) -> Result<(), RenderRingBufferBoundaryError> {
    if config.iterations == 0 {
        return Err(RenderRingBufferBoundaryError::InvalidIterationCount);
    }
    if config.max_frames_per_write == 0 {
        return Err(RenderRingBufferBoundaryError::InvalidFrameCount);
    }
    if config.iterations > MAX_RENDER_RING_BUFFER_BOUNDARY_ITERATIONS {
        return Err(RenderRingBufferBoundaryError::IterationCountTooLarge);
    }
    if config.max_frames_per_write > MAX_RENDER_RING_BUFFER_BOUNDARY_FRAMES_PER_WRITE {
        return Err(RenderRingBufferBoundaryError::FrameCountTooLarge);
    }
    if config.synthetic_zero_seed_frames > MAX_SYNTHETIC_ZERO_SEED_FRAMES {
        return Err(RenderRingBufferBoundaryError::SyntheticSeedFrameCountTooLarge);
    }
    Ok(())
}

pub(super) fn boundary_config(
    config: RenderRingBufferBoundaryConfig,
) -> RingBufferRenderBoundaryConfig {
    RingBufferRenderBoundaryConfig {
        iterations: config.iterations,
        max_frames_per_write: config.max_frames_per_write,
    }
}
