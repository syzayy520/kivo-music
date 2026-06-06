use super::super::render_ring_buffer_boundary::{
    run_render_ring_buffer_boundary_with_callbacks, RenderRingBufferBoundaryError,
    MAX_RENDER_RING_BUFFER_BOUNDARY_FRAMES_PER_WRITE, MAX_RENDER_RING_BUFFER_BOUNDARY_ITERATIONS,
    MAX_SYNTHETIC_ZERO_SEED_FRAMES,
};
use super::fixtures::{assert_err, cfg, fmt, report, snap};

#[test]
fn invalid_config_rejected_when_enabled() {
    let cases = [
        (
            cfg(true, 0, 1, 0),
            RenderRingBufferBoundaryError::InvalidIterationCount,
        ),
        (
            cfg(true, MAX_RENDER_RING_BUFFER_BOUNDARY_ITERATIONS + 1, 1, 0),
            RenderRingBufferBoundaryError::IterationCountTooLarge,
        ),
        (
            cfg(true, 1, 0, 0),
            RenderRingBufferBoundaryError::InvalidFrameCount,
        ),
        (
            cfg(
                true,
                1,
                MAX_RENDER_RING_BUFFER_BOUNDARY_FRAMES_PER_WRITE + 1,
                0,
            ),
            RenderRingBufferBoundaryError::FrameCountTooLarge,
        ),
        (
            cfg(true, 1, 1, MAX_SYNTHETIC_ZERO_SEED_FRAMES + 1),
            RenderRingBufferBoundaryError::SyntheticSeedFrameCountTooLarge,
        ),
    ];

    for (config, expected) in cases {
        assert_err(
            run_render_ring_buffer_boundary_with_callbacks(
                fmt(),
                config,
                || Ok(snap(1)),
                |_, _| Ok(report(1, 8)),
            ),
            expected,
        );
    }
}
