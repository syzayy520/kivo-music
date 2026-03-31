use std::collections::HashMap;

use super::service::{AudioService, SessionHandle};

impl AudioService {
    pub(super) fn cleanup_finished_locked(map: &mut HashMap<u64, SessionHandle>) {
        let mut finished_ids = Vec::new();

        for (id, h) in map.iter_mut() {
            if let Some(j) = h.join.as_ref() {
                if j.is_finished() {
                    finished_ids.push(*id);
                }
            }
        }

        for id in finished_ids {
            if let Some(mut h) = map.remove(&id) {
                if let Some(j) = h.join.take() {
                    let _ = j.join();
                }
            }
        }
    }
}
