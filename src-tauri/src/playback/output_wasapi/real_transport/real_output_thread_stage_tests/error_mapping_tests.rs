use super::super::render_ring_buffer_boundary::RenderRingBufferBoundaryError;
use super::super::thread_error::RealOutputThreadSkeletonError;
use super::super::thread_stages::map_render_ring_buffer_boundary_error;

#[test]
fn preflight_errors_map_to_stable_thread_errors() {
    let cases = [
        (
            RenderRingBufferBoundaryError::RequiresContext,
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryRequiresOpenContext,
        ),
        (
            RenderRingBufferBoundaryError::RequiresStartedClient,
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryRequiresStartedClient,
        ),
        (
            RenderRingBufferBoundaryError::MissingFormatCache,
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryMissingFormatCache,
        ),
        (
            RenderRingBufferBoundaryError::UnsupportedFormat,
            RealOutputThreadSkeletonError::RenderRingBufferBoundaryUnsupportedFormat,
        ),
    ];

    for (error, expected) in cases {
        assert_eq!(map_render_ring_buffer_boundary_error(error), expected);
    }
}

#[test]
fn commit_after_write_failure_remains_a_distinct_fatal_error() {
    assert_eq!(
        map_render_ring_buffer_boundary_error(
            RenderRingBufferBoundaryError::CommitAfterWriteFailed("commit".to_string())
        ),
        RealOutputThreadSkeletonError::RenderRingBufferBoundaryCommitAfterWriteFailed(
            "commit".to_string()
        )
    );
}

#[test]
fn general_boundary_failure_stays_out_of_commit_error_variant() {
    assert_eq!(
        map_render_ring_buffer_boundary_error(RenderRingBufferBoundaryError::BoundaryFailed(
            "write".to_string()
        )),
        RealOutputThreadSkeletonError::RenderRingBufferBoundaryFailed("write".to_string())
    );
}
