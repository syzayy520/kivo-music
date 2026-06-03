/// Pipeline clock boundary for runtime position tracking.
///
/// Records minimal playback position state for progress/drain/playback loop.
/// This clock does NOT:
/// - use real wall-clock timing
/// - use WASAPI clock
/// - use system device time
/// - know about decoder/output/engine/UI
#[derive(Clone, Debug, Default)]
pub(in crate::playback) struct NativePipelineClock {
    position_ms: u64,
    is_started: bool,
    is_paused: bool,
}

impl NativePipelineClock {
    pub(in crate::playback) fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub(in crate::playback) fn start_at(&mut self, position_ms: u64) {
        self.position_ms = position_ms;
        self.is_started = true;
        self.is_paused = false;
    }

    #[allow(dead_code)]
    pub(in crate::playback) fn pause(&mut self) {
        self.is_paused = true;
    }

    #[allow(dead_code)]
    pub(in crate::playback) fn resume(&mut self) {
        self.is_paused = false;
    }

    pub(in crate::playback) fn reset(&mut self) {
        self.position_ms = 0;
        self.is_started = false;
        self.is_paused = false;
    }

    #[allow(dead_code)]
    pub(in crate::playback) fn set_position(&mut self, position_ms: u64) {
        self.position_ms = position_ms;
    }

    pub(in crate::playback) fn position_ms(&self) -> u64 {
        self.position_ms
    }

    #[allow(dead_code)]
    pub(in crate::playback) fn is_started(&self) -> bool {
        self.is_started
    }

    #[allow(dead_code)]
    pub(in crate::playback) fn is_paused(&self) -> bool {
        self.is_paused
    }
}
