use super::super::render_ring_buffer_boundary::{
    MAX_RENDER_RING_BUFFER_BOUNDARY_FRAMES_PER_WRITE, MAX_RENDER_RING_BUFFER_BOUNDARY_ITERATIONS,
    MAX_SYNTHETIC_ZERO_SEED_FRAMES,
};
use super::super::thread_error::RealOutputThreadSkeletonError;
use super::super::thread_stages::{run_post_start_stages, validate_thread_stage_configs};
use super::fixtures::stage_configs;
use crate::playback::output_wasapi::wasapi_context::WasapiContext;

#[test]
fn disabled_boundary_allows_zero_values_without_open_or_start() {
    assert_eq!(
        validate_thread_stage_configs(stage_configs(false, 0, 0, 0), false, false),
        Ok(())
    );
}

#[test]
fn invalid_boundary_numbers_are_rejected_before_open_requirements() {
    let cases = [
        (
            stage_configs(true, 0, 1, 0),
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryInvalidIterationCount,
        ),
        (
            stage_configs(true, 1, 0, 0),
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryInvalidFrameCount,
        ),
        (
            stage_configs(true, MAX_RENDER_RING_BUFFER_BOUNDARY_ITERATIONS + 1, 1, 0),
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryIterationCountTooLarge,
        ),
        (
            stage_configs(
                true,
                1,
                MAX_RENDER_RING_BUFFER_BOUNDARY_FRAMES_PER_WRITE + 1,
                0,
            ),
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryFrameCountTooLarge,
        ),
        (
            stage_configs(true, 1, 1, MAX_SYNTHETIC_ZERO_SEED_FRAMES + 1),
            RealOutputThreadSkeletonError::RenderRingBufferBoundarySyntheticSeedFrameCountTooLarge,
        ),
    ];

    for (configs, expected) in cases {
        assert_eq!(
            validate_thread_stage_configs(configs, false, false),
            Err(expected)
        );
    }
}

#[test]
fn enabled_boundary_requires_open_then_started_client() {
    let configs = stage_configs(true, 1, 1, 0);
    assert_eq!(
        validate_thread_stage_configs(configs, false, false),
        Err(RealOutputThreadSkeletonError::RenderRingBufferBoundaryRequiresOpenContext)
    );
    assert_eq!(
        validate_thread_stage_configs(configs, true, false),
        Err(RealOutputThreadSkeletonError::RenderRingBufferBoundaryRequiresStartedClient)
    );
}

#[test]
fn runtime_missing_format_cache_maps_to_thread_error() {
    let context = WasapiContext::new();
    let results = run_post_start_stages(Some(&context), true, stage_configs(true, 1, 1, 0));

    assert_eq!(
        results
            .render_ring_buffer_boundary_result
            .as_ref()
            .unwrap_err(),
        &RealOutputThreadSkeletonError::RenderRingBufferBoundaryMissingFormatCache
    );
    assert!(!results.can_run_worker_loop());
}
