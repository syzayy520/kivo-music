use super::native_pipeline::NativePipeline;

/// Read-only progress snapshot of pipeline clock and buffer state.
///
/// This snapshot does NOT:
/// - produce side effects
/// - know about UI
/// - know about manager
/// - send events
/// - do real time advancement
/// - connect to real audio devices
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(in crate::playback) struct NativePipelineProgress {
    pub(in crate::playback) position_ms: u64,
    pub(in crate::playback) is_started: bool,
    pub(in crate::playback) is_paused: bool,
    pub(in crate::playback) buffered_frames: usize,
    pub(in crate::playback) output_pending_frames: usize,
}

impl NativePipeline {
    /// Capture a read-only progress snapshot of the pipeline clock and buffer state.
    ///
    /// This method is pure read-only and produces no side effects.
    #[allow(dead_code)]
    pub(in crate::playback) fn progress_snapshot(&self) -> NativePipelineProgress {
        NativePipelineProgress {
            position_ms: self.clock.position_ms(),
            is_started: self.clock.is_started(),
            is_paused: self.clock.is_paused(),
            buffered_frames: self.buffer.len(),
            output_pending_frames: self.state.output_status.pending_frames,
        }
    }
}
