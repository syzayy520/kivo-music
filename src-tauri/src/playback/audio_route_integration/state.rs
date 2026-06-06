#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct AudioRouteIntegrationState {
    pub(super) input_count: u64,
    pub(super) closed: bool,
}

impl AudioRouteIntegrationState {
    pub(super) fn initialized() -> Self {
        Self {
            input_count: 0,
            closed: false,
        }
    }

    pub(super) fn reset(&mut self) {
        *self = Self::initialized();
    }

    pub(super) fn mark_closed(&mut self) {
        self.closed = true;
    }

    pub(super) fn record_successful_input(&mut self) {
        self.input_count += 1;
    }
}
