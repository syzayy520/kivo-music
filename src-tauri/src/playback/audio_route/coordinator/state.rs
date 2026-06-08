use crate::playback::audio_route::AudioRouteReport;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct AudioRouteCoordinatorState {
    pub(super) feed_count: u64,
    pub(super) backpressure_count: u64,
    pub(super) partial_write_count: u64,
    pub(super) source_closed_seen: bool,
    pub(super) closed: bool,
}

impl AudioRouteCoordinatorState {
    pub(super) fn initialized() -> Self {
        Self {
            feed_count: 0,
            backpressure_count: 0,
            partial_write_count: 0,
            source_closed_seen: false,
            closed: false,
        }
    }

    pub(super) fn reset(&mut self) {
        *self = Self::initialized();
    }

    pub(super) fn mark_closed(&mut self) {
        self.closed = true;
    }

    pub(super) fn record_success(&mut self, report: &AudioRouteReport) {
        self.feed_count += 1;
        if report.last_partial_write {
            self.partial_write_count += 1;
            self.backpressure_count += 1;
        }
        if report.last_source_closed {
            self.source_closed_seen = true;
        }
    }

    pub(super) fn record_backpressure(&mut self) {
        self.backpressure_count += 1;
    }
}
