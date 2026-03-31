use crate::audio::errors::{AudioError, AudioResult};
use std::time::Duration;

use super::{lock_poison::lock_poison, service::AudioService, types::ServiceCmd};

impl AudioService {
    /// Set linear output volume for an existing session.
    ///
    /// - clamps to [0.0, 1.0]
    /// - rejects NaN/INF
    pub fn set_volume(&self, session_id: u64, volume_0_1: f32) -> AudioResult<()> {
        let volume = validate_volume(volume_0_1)?;

        let status = {
            let mut map = lock_poison(&self.sessions);
            Self::cleanup_finished_locked(&mut map);
            let h = map.get(&session_id).ok_or_else(|| {
                AudioError::InvalidInput(format!("unknown session_id={session_id}"))
            })?;
            h.status.clone()
        };
        self.dispatch_cmd(
            session_id,
            "set_volume",
            ServiceCmd::SetVolume(volume),
            Duration::from_millis(100),
        )?;
        lock_poison(&status).volume = volume;

        Ok(())
    }
}

fn validate_volume(volume_0_1: f32) -> AudioResult<f32> {
    if !volume_0_1.is_finite() {
        return Err(AudioError::InvalidInput(
            "volume must be a finite number".to_string(),
        ));
    }
    Ok(volume_0_1.clamp(0.0, 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossbeam_channel::bounded;

    use crate::audio::state::service::test_support::{insert_session, make_status};

    #[test]
    fn set_volume_updates_status_after_successful_dispatch() {
        let svc = AudioService::default();
        let (cmd_tx, cmd_rx) = bounded::<ServiceCmd>(1);
        let status = insert_session(&svc, 1, cmd_tx, make_status(1, "test.mp3"));

        svc.set_volume(1, 0.25).expect("set_volume should succeed");

        assert!(matches!(
            cmd_rx.recv().unwrap(),
            ServiceCmd::SetVolume(0.25)
        ));
        assert_eq!(lock_poison(&status).volume, 0.25);
    }

    #[test]
    fn set_volume_does_not_mutate_status_when_dispatch_fails() {
        let svc = AudioService::default();
        let (cmd_tx, _cmd_rx) = bounded::<ServiceCmd>(0);
        let status = insert_session(&svc, 2, cmd_tx, make_status(2, "test.mp3"));

        let err = svc
            .set_volume(2, 0.4)
            .expect_err("zero-capacity queue should fail");

        assert!(matches!(err, AudioError::Unsupported(_)));
        assert_eq!(lock_poison(&status).volume, 1.0);
    }
}
