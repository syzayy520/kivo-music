use std::sync::Mutex;

use super::activity_snapshot::MediaProbeActivityLogSnapshot;
use super::event::MediaProbeEvent;

const DEFAULT_ACTIVITY_LOG_LIMIT: usize = 128;

#[derive(Debug)]
pub struct MediaProbeActivityLogState {
    entries: Mutex<Vec<MediaProbeEvent>>,
    limit: usize,
}

impl Default for MediaProbeActivityLogState {
    fn default() -> Self {
        Self::new(DEFAULT_ACTIVITY_LOG_LIMIT)
    }
}

impl MediaProbeActivityLogState {
    pub fn new(limit: usize) -> Self {
        Self {
            entries: Mutex::new(Vec::new()),
            limit: limit.max(1),
        }
    }

    pub fn record(&self, event: MediaProbeEvent) {
        let mut entries = self
            .entries
            .lock()
            .expect("media probe activity log lock failed");

        entries.push(event);
        let overflow_count = entries.len().saturating_sub(self.limit);

        if overflow_count > 0 {
            entries.drain(0..overflow_count);
        }
    }

    pub fn entries(&self) -> Vec<MediaProbeEvent> {
        let entries = self
            .entries
            .lock()
            .expect("media probe activity log lock failed");

        entries.clone()
    }

    pub fn snapshot(&self) -> MediaProbeActivityLogSnapshot {
        MediaProbeActivityLogSnapshot::new(self.entries(), self.limit)
    }

    pub fn clear(&self) {
        let mut entries = self
            .entries
            .lock()
            .expect("media probe activity log lock failed");

        entries.clear();
    }
}
