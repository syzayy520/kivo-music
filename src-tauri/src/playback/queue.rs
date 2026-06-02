use serde::{Deserialize, Serialize};

use super::errors::{PlaybackError, PlaybackResult};
use super::types::{PlaybackTrack, RepeatMode};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PlaybackQueue {
    pub items: Vec<PlaybackTrack>,
    pub current_index: Option<usize>,
    pub repeat_mode: RepeatMode,
    pub shuffle: bool,
}

impl PlaybackQueue {
    pub fn append(&mut self, track: PlaybackTrack) {
        self.items.push(track);

        if self.current_index.is_none() {
            self.current_index = Some(0);
        }
    }

    pub fn remove(&mut self, index: usize) -> PlaybackResult<()> {
        if index >= self.items.len() {
            return Err(PlaybackError::Queue(
                "remove index out of range".to_string(),
            ));
        }

        self.items.remove(index);

        self.current_index = match self.current_index {
            None => None,
            Some(_) if self.items.is_empty() => None,
            Some(current) if index < current => Some(current - 1),
            Some(current) if index == current => Some(current.min(self.items.len() - 1)),
            Some(current) => Some(current),
        };

        Ok(())
    }

    pub fn current_track(&self) -> Option<PlaybackTrack> {
        let index = self.current_index?;
        self.items.get(index).cloned()
    }

    pub fn set_current_index(&mut self, index: usize) -> PlaybackResult<()> {
        if index >= self.items.len() {
            return Err(PlaybackError::Queue(
                "current index out of range".to_string(),
            ));
        }

        self.current_index = Some(index);
        Ok(())
    }
}
