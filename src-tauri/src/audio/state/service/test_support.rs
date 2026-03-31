use std::sync::{Arc, Mutex};

use crossbeam_channel::Sender;

use super::{
    lock_poison::lock_poison,
    service::{AudioService, SessionHandle},
    types::{AudioSessionState, AudioSessionStatus, ServiceCmd},
};

pub(super) fn make_status(session_id: u64, input_path: &str) -> AudioSessionStatus {
    AudioSessionStatus {
        session_id,
        state: AudioSessionState::Playing,
        pos_seconds: 0.0,
        duration_seconds: None,
        sample_rate: 0,
        channels: 0,
        device_name: None,
        input_path: input_path.into(),
        last_error: None,
        ended: false,
        volume: 1.0,
        output_rebuilds: 0,
        last_output_rebuild_reason: None,
        buffer_fill_frames: 0,
        buffer_capacity_frames: 0,
        underrun_events: 0,
        underrun_frames: 0,
    }
}

pub(super) fn insert_session(
    svc: &AudioService,
    session_id: u64,
    cmd_tx: Sender<ServiceCmd>,
    status: AudioSessionStatus,
) -> Arc<Mutex<AudioSessionStatus>> {
    let status = Arc::new(Mutex::new(status));
    let mut map = lock_poison(&svc.sessions);
    map.insert(
        session_id,
        SessionHandle {
            cmd_tx,
            status: status.clone(),
            join: None,
        },
    );
    status
}
