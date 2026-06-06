use super::super::render_loop::validate_render_silence_loop_config;
use super::super::render_once::validate_render_silence_once_config;
use super::super::render_padding_loop::validate_render_padding_loop_config;
use super::super::render_ring_buffer_boundary::{
    RenderRingBufferBoundaryConfig, MAX_RENDER_RING_BUFFER_BOUNDARY_FRAMES_PER_WRITE,
    MAX_RENDER_RING_BUFFER_BOUNDARY_ITERATIONS, MAX_SYNTHETIC_ZERO_SEED_FRAMES,
};
use super::super::thread_error::RealOutputThreadSkeletonError;
use super::config::RealOutputThreadStageConfigs;

pub(crate) fn validate_thread_stage_configs(
    configs: RealOutputThreadStageConfigs,
    open_wasapi_context_on_start: bool,
    start_audio_client_on_start: bool,
) -> Result<(), RealOutputThreadSkeletonError> {
    validate_render_silence_once_config(configs.render_once)?;
    validate_render_silence_loop_config(
        configs.render_loop,
        open_wasapi_context_on_start,
        start_audio_client_on_start,
    )?;
    validate_render_padding_loop_config(
        configs.render_padding_loop,
        open_wasapi_context_on_start,
        start_audio_client_on_start,
    )?;
    validate_render_ring_buffer_boundary_config(
        configs.render_ring_buffer_boundary,
        open_wasapi_context_on_start,
        start_audio_client_on_start,
    )
}

fn validate_render_ring_buffer_boundary_config(
    config: RenderRingBufferBoundaryConfig,
    open_wasapi_context_on_start: bool,
    start_audio_client_on_start: bool,
) -> Result<(), RealOutputThreadSkeletonError> {
    if !config.enabled {
        return Ok(());
    }
    if config.iterations == 0 {
        return Err(RealOutputThreadSkeletonError::RenderRingBufferBoundaryInvalidIterationCount);
    }
    if config.max_frames_per_write == 0 {
        return Err(RealOutputThreadSkeletonError::RenderRingBufferBoundaryInvalidFrameCount);
    }
    if config.iterations > MAX_RENDER_RING_BUFFER_BOUNDARY_ITERATIONS {
        return Err(RealOutputThreadSkeletonError::RenderRingBufferBoundaryIterationCountTooLarge);
    }
    if config.max_frames_per_write > MAX_RENDER_RING_BUFFER_BOUNDARY_FRAMES_PER_WRITE {
        return Err(RealOutputThreadSkeletonError::RenderRingBufferBoundaryFrameCountTooLarge);
    }
    if config.synthetic_zero_seed_frames > MAX_SYNTHETIC_ZERO_SEED_FRAMES {
        return Err(
            RealOutputThreadSkeletonError::RenderRingBufferBoundarySyntheticSeedFrameCountTooLarge,
        );
    }
    if !open_wasapi_context_on_start {
        return Err(RealOutputThreadSkeletonError::RenderRingBufferBoundaryRequiresOpenContext);
    }
    if !start_audio_client_on_start {
        return Err(RealOutputThreadSkeletonError::RenderRingBufferBoundaryRequiresStartedClient);
    }
    Ok(())
}
