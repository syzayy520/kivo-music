use crate::audio::errors::{AudioError, AudioResult};

use super::{
    lock_poison::lock_poison,
    service::AudioService,
    types::{AudioSessionState, AudioSessionStatus, QueueSnapshot},
};

impl AudioService {
    /// Periodic maintenance tick that can auto-advance the queue on natural EOF.
    pub fn tick(&self) -> AudioResult<(QueueSnapshot, AudioSessionStatus)> {
        let (snapshot, active_session_id) = {
            let q = lock_poison(&self.queue);
            (q.snapshot(), q.active_session_id())
        };

        let sid = active_session_id
            .ok_or_else(|| AudioError::InvalidInput("no active session".into()))?;

        let status = match self.status(sid) {
            Ok(status) => status,
            Err(AudioError::InvalidInput(_)) => {
                self.clear_active_session_if_matches(sid);
                return Err(AudioError::InvalidInput("no active session".into()));
            }
            Err(e) => return Err(e),
        };

        let should_autoplay = snapshot.autoplay
            && status.ended
            && status.state == AudioSessionState::Stopped
            && can_advance(&snapshot);

        if should_autoplay {
            let new_status = self.next()?;
            let new_snapshot = self.queue_get()?;
            return Ok((new_snapshot, new_status));
        }

        Ok((snapshot, status))
    }
}

fn can_advance(snapshot: &QueueSnapshot) -> bool {
    let len = snapshot.items.len();
    let Some(idx) = snapshot.current_index else {
        return false;
    };
    let idx = idx as usize;
    len > 0 && idx + 1 < len
}
