use super::lock_poison::lock_poison;
use super::queue_state::QueueState;
use super::service::AudioService;
use super::types::{AudioSessionStatus, QueueSnapshot};
use crate::audio::errors::{AudioError, AudioResult};

impl AudioService {
    pub fn queue_set(
        &self,
        items: Vec<String>,
        start_index: Option<u32>,
        autoplay: bool,
    ) -> AudioResult<QueueSnapshot> {
        let items = QueueState::normalized_items(items);

        let mut q = lock_poison(&self.queue);
        q.items = items;
        q.autoplay = autoplay;
        q.active_session_id = None;
        q.last_play_opt = Default::default();

        q.current_index = if q.items.is_empty() {
            None
        } else {
            let idx = start_index.unwrap_or(0);
            q.validate_index(idx)?;
            Some(idx)
        };

        Ok(q.snapshot())
    }

    pub fn queue_add(&self, items: Vec<String>) -> AudioResult<QueueSnapshot> {
        let to_add = QueueState::normalized_items(items);

        let mut q = lock_poison(&self.queue);
        q.items.extend(to_add);

        if q.current_index.is_none() && !q.items.is_empty() {
            q.current_index = Some(0);
        }

        Ok(q.snapshot())
    }

    pub fn queue_get(&self) -> AudioResult<QueueSnapshot> {
        let q = lock_poison(&self.queue);
        Ok(q.snapshot())
    }

    pub fn queue_clear(&self) -> AudioResult<QueueSnapshot> {
        let mut q = lock_poison(&self.queue);
        q.items.clear();
        q.current_index = None;
        q.active_session_id = None;
        q.last_play_opt = Default::default();
        Ok(q.snapshot())
    }

    pub fn next(&self) -> AudioResult<AudioSessionStatus> {
        self.move_by(1)
    }

    pub fn prev(&self) -> AudioResult<AudioSessionStatus> {
        self.move_by(-1)
    }

    fn move_by(&self, delta: i32) -> AudioResult<AudioSessionStatus> {
        let (from_idx, to_idx, to_path, autoplay, active_session_id, opt) = {
            let mut q = lock_poison(&self.queue);
            if q.items.is_empty() {
                return Err(AudioError::InvalidInput("queue is empty".into()));
            }
            let from = q
                .current_index
                .ok_or_else(|| AudioError::InvalidInput("queue has no current_index".into()))?;
            let to_i64 = from as i64 + delta as i64;
            if to_i64 < 0 {
                return Err(AudioError::InvalidInput("queue prev out of range".into()));
            }
            let to = u32::try_from(to_i64).unwrap_or(u32::MAX);
            q.validate_index(to)?;
            let to_path = q.items.get(to as usize).cloned().unwrap_or_default();
            let autoplay = q.autoplay;
            let active = q.active_session_id;
            let opt = q.next_play_opt();

            if !autoplay {
                q.current_index = Some(to);
            }
            (from, to, to_path, autoplay, active, opt)
        };

        if !autoplay {
            let Some(sid) = active_session_id else {
                return Err(AudioError::InvalidInput(
                    "autoplay is false and no active session".into(),
                ));
            };
            return match self.status(sid) {
                Ok(status) => Ok(status),
                Err(AudioError::InvalidInput(_)) => {
                    self.clear_active_session_if_matches(sid);
                    Err(AudioError::InvalidInput(
                        "autoplay is false and no active session".into(),
                    ))
                }
                Err(e) => Err(e),
            };
        }

        let new_id = match self.play_start(to_path, opt) {
            Ok(id) => id,
            Err(e) => {
                if let Some(sid) = active_session_id {
                    self.write_last_error(sid, &e);
                }
                return Err(e);
            }
        };

        if let Some(old_id) = active_session_id {
            let _ = self.stop(old_id);
        }

        {
            let mut q = lock_poison(&self.queue);
            if q.current_index == Some(from_idx) {
                q.current_index = Some(to_idx);
            }
            q.active_session_id = Some(new_id);
        }

        self.status(new_id)
    }

    fn write_last_error(&self, session_id: u64, err: &AudioError) {
        let status = {
            let mut map = lock_poison(&self.sessions);
            Self::cleanup_finished_locked(&mut map);
            map.get(&session_id).map(|h| h.status.clone())
        };

        if let Some(status) = status {
            let mut s = lock_poison(&status);
            s.set_last_error_from_audio_error(err);
        }
    }
}
