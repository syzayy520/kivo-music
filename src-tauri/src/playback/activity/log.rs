use std::sync::Mutex;

use crate::playback::activity_entry::PlaybackActivityLogEntry;
use crate::playback::activity_snapshot::PlaybackActivityLogSnapshot;
use crate::playback::events::PlaybackEvent;

const DEFAULT_ACTIVITY_LOG_LIMIT: usize = 256;

#[derive(Clone, Debug)]
pub struct PlaybackActivityLog {
    items: Vec<PlaybackActivityLogEntry>,
    limit: usize,
}

impl PlaybackActivityLog {
    pub fn new(limit: usize) -> Self {
        Self {
            items: Vec::new(),
            limit,
        }
    }

    pub fn append(&mut self, item: PlaybackEvent) {
        if self.limit == 0 {
            return;
        }

        if self.items.len() >= self.limit {
            self.items.drain(0..1);
        }

        self.items.push(PlaybackActivityLogEntry::new(item));
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn snapshot(&self) -> PlaybackActivityLogSnapshot {
        PlaybackActivityLogSnapshot::new(self.items.clone(), self.limit)
    }
}

impl Default for PlaybackActivityLog {
    fn default() -> Self {
        Self::new(DEFAULT_ACTIVITY_LOG_LIMIT)
    }
}

#[derive(Debug, Default)]
pub struct PlaybackActivityLogState {
    log: Mutex<PlaybackActivityLog>,
}

impl PlaybackActivityLogState {
    pub fn append(&self, item: PlaybackEvent) {
        if let Ok(mut log) = self.log.lock() {
            log.append(item);
        }
    }

    pub fn clear(&self) {
        if let Ok(mut log) = self.log.lock() {
            log.clear();
        }
    }

    pub fn snapshot(&self) -> PlaybackActivityLogSnapshot {
        match self.log.lock() {
            Ok(log) => log.snapshot(),
            Err(_) => PlaybackActivityLogSnapshot::new(Vec::new(), 0),
        }
    }
}
