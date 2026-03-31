use super::types::{PlayStartOptions, QueueSnapshot};
use crate::audio::errors::{AudioError, AudioResult};

#[derive(Debug, Default)]
pub(super) struct QueueState {
    pub(super) items: Vec<String>,
    pub(super) current_index: Option<u32>,
    pub(super) autoplay: bool,
    pub(super) active_session_id: Option<u64>,
    pub(super) last_play_opt: PlayStartOptions,
}

impl QueueState {
    pub(super) fn active_session_id(&self) -> Option<u64> {
        self.active_session_id
    }

    pub(super) fn snapshot(&self) -> QueueSnapshot {
        let current_path = self
            .current_index
            .and_then(|i| self.items.get(i as usize).cloned());
        QueueSnapshot {
            items: self.items.clone(),
            current_index: self.current_index,
            current_path,
            autoplay: self.autoplay,
        }
    }

    pub(super) fn note_play_started(
        &mut self,
        session_id: u64,
        input_path: &str,
        opt: &PlayStartOptions,
    ) {
        self.active_session_id = Some(session_id);
        self.last_play_opt = opt.clone();
        self.current_index = self
            .items
            .iter()
            .enumerate()
            .find(|(_, p)| p == &input_path)
            .map(|(idx, _)| idx as u32);
    }

    pub(super) fn validate_index(&self, idx: u32) -> AudioResult<()> {
        if idx as usize >= self.items.len() {
            return Err(AudioError::InvalidInput(format!(
                "queue index out of range: {idx} (len={})",
                self.items.len()
            )));
        }
        Ok(())
    }

    pub(super) fn normalized_items(items: Vec<String>) -> Vec<String> {
        items
            .into_iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    pub(super) fn next_play_opt(&self) -> PlayStartOptions {
        let mut opt = self.last_play_opt.clone();
        opt.start_seconds = None;
        opt.max_seconds = None;
        opt
    }
}
