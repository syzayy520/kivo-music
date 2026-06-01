use std::sync::Mutex;

use super::lifecycle_activity_snapshot::PlaybackLifecycleActivityLogSnapshot;
use super::lifecycle_event::PlaybackLifecycleEvent;

const DEFAULT_LIFECYCLE_ACTIVITY_LOG_LIMIT: usize = 64;

#[derive(Clone, Debug)]
pub struct PlaybackLifecycleActivityLog {
    items: Vec<PlaybackLifecycleEvent>,
    limit: usize,
}

impl PlaybackLifecycleActivityLog {
    pub fn new(limit: usize) -> Self {
        Self {
            items: Vec::new(),
            limit: limit.max(1),
        }
    }

    pub fn append(&mut self, item: PlaybackLifecycleEvent) {
        self.items.push(item);
        let overflow_count = self.items.len().saturating_sub(self.limit);

        if overflow_count > 0 {
            self.items.drain(0..overflow_count);
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn snapshot(&self) -> PlaybackLifecycleActivityLogSnapshot {
        PlaybackLifecycleActivityLogSnapshot::new(self.items.clone(), self.limit)
    }
}

impl Default for PlaybackLifecycleActivityLog {
    fn default() -> Self {
        Self::new(DEFAULT_LIFECYCLE_ACTIVITY_LOG_LIMIT)
    }
}

#[derive(Debug, Default)]
pub struct PlaybackLifecycleActivityLogState {
    log: Mutex<PlaybackLifecycleActivityLog>,
}

impl PlaybackLifecycleActivityLogState {
    pub fn append(&self, item: PlaybackLifecycleEvent) {
        if let Ok(mut log) = self.log.lock() {
            log.append(item);
        }
    }

    pub fn clear(&self) {
        if let Ok(mut log) = self.log.lock() {
            log.clear();
        }
    }

    pub fn snapshot(&self) -> PlaybackLifecycleActivityLogSnapshot {
        match self.log.lock() {
            Ok(log) => log.snapshot(),
            Err(_) => PlaybackLifecycleActivityLogSnapshot::new(Vec::new(), 0),
        }
    }

    #[cfg(test)]
    pub fn poison_for_test(&self) {
        let _guard = self.log.lock().unwrap();
        panic!("intentional poison");
    }
}
