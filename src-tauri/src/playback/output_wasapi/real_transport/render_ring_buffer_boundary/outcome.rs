use super::config::RenderRingBufferBoundaryConfig;
use crate::playback::output_wasapi::ring_buffer_render_boundary::RingBufferRenderBoundaryOutcome;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct RenderRingBufferBoundaryOutcome {
    pub requested: bool,
    pub started: bool,
    pub completed: bool,
    pub skipped_after_prior_failure: bool,
    pub iterations_requested: u32,
    pub iterations_completed: u32,
    pub iterations_skipped_no_available: u32,
    pub iterations_source_empty_open: u32,
    pub source_closed: bool,
    pub max_frames_per_write: u32,
    pub synthetic_zero_seed_frames: u32,
    pub write_attempts: u32,
    pub frames_peeked_total: u32,
    pub frames_written_total: u32,
    pub frames_committed_total: u32,
    pub source_remaining_frames: u32,
    pub last_capacity: u32,
    pub last_padding: u32,
    pub last_available: u32,
    pub used_silent_flag: bool,
}

impl RenderRingBufferBoundaryOutcome {
    pub(crate) fn disabled(config: RenderRingBufferBoundaryConfig) -> Self {
        Self {
            synthetic_zero_seed_frames: config.synthetic_zero_seed_frames,
            ..Self::default()
        }
    }

    pub(crate) fn skipped_after_prior_failure(config: RenderRingBufferBoundaryConfig) -> Self {
        Self {
            requested: config.enabled,
            skipped_after_prior_failure: true,
            iterations_requested: config.iterations,
            max_frames_per_write: config.max_frames_per_write,
            synthetic_zero_seed_frames: config.synthetic_zero_seed_frames,
            ..Self::default()
        }
    }
}

pub(super) fn outcome_from_boundary(
    config: RenderRingBufferBoundaryConfig,
    boundary: RingBufferRenderBoundaryOutcome,
    source_remaining_frames: u32,
) -> RenderRingBufferBoundaryOutcome {
    RenderRingBufferBoundaryOutcome {
        requested: config.enabled,
        started: boundary.started,
        completed: boundary.completed,
        skipped_after_prior_failure: false,
        iterations_requested: boundary.iterations_requested,
        iterations_completed: boundary.iterations_completed,
        iterations_skipped_no_available: boundary.iterations_skipped_no_available,
        iterations_source_empty_open: boundary.iterations_source_empty_open,
        source_closed: boundary.source_closed,
        max_frames_per_write: boundary.max_frames_per_write,
        synthetic_zero_seed_frames: config.synthetic_zero_seed_frames,
        write_attempts: boundary.write_attempts,
        frames_peeked_total: boundary.frames_peeked_total,
        frames_written_total: boundary.frames_written_total,
        frames_committed_total: boundary.frames_committed_total,
        source_remaining_frames,
        last_capacity: boundary.last_capacity,
        last_padding: boundary.last_padding,
        last_available: boundary.last_available,
        used_silent_flag: boundary.used_silent_flag,
    }
}
