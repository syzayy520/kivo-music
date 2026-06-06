use super::super::render_ring_buffer_boundary::{
    map_ring_buffer_boundary_error, RenderRingBufferBoundaryError,
};
use crate::playback::output_wasapi::ring_buffer_render_boundary::RingBufferRenderBoundaryError;
use crate::playback::output_wasapi::ring_buffer_source::RingBufferSourceError;

#[test]
fn commit_after_write_mapping_is_distinct() {
    let mapped =
        map_ring_buffer_boundary_error(RingBufferRenderBoundaryError::CommitAfterWriteFailed(
            RingBufferSourceError::InvalidMaxFrames,
        ));
    assert!(matches!(
        mapped,
        RenderRingBufferBoundaryError::CommitAfterWriteFailed(_)
    ));
}
