use crate::playback::native_pipeline::NativePipeline;

impl NativePipeline {
    pub(in crate::playback) fn apply_drain_success_clock_position(
        &mut self,
        drained_position_ms: u64,
    ) {
        if self.clock.is_started() {
            self.clock.set_position(drained_position_ms);
        } else {
            self.clock.start_at(drained_position_ms);
        }
    }
}
