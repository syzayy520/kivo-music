use crossbeam_channel::bounded;

use crate::audio::errors::AudioError;

use super::{
    service::AudioService,
    test_support::{insert_session, make_status},
    types::ServiceCmd,
};

#[test]
fn stop_dispatches_stop_command() {
    let svc = AudioService::default();
    let (cmd_tx, cmd_rx) = bounded::<ServiceCmd>(1);
    insert_session(&svc, 71, cmd_tx, make_status(71, "a.mp3"));

    svc.stop(71).expect("stop should succeed");

    assert!(matches!(cmd_rx.recv().unwrap(), ServiceCmd::Stop));
}

#[test]
fn pause_dispatches_pause_command() {
    let svc = AudioService::default();
    let (cmd_tx, cmd_rx) = bounded::<ServiceCmd>(1);
    insert_session(&svc, 72, cmd_tx, make_status(72, "a.mp3"));

    svc.pause(72, true).expect("pause should succeed");

    assert!(matches!(cmd_rx.recv().unwrap(), ServiceCmd::Pause(true)));
}

#[test]
fn seek_rejects_negative_seconds() {
    let svc = AudioService::default();
    let err = svc
        .seek(99, -1.0)
        .expect_err("negative seek should fail before session lookup");

    assert!(matches!(err, AudioError::InvalidInput(_)));
}

#[test]
fn seek_on_full_queue_returns_ok_without_blocking() {
    let svc = AudioService::default();
    let (cmd_tx, _cmd_rx) = bounded::<ServiceCmd>(0);
    insert_session(&svc, 73, cmd_tx, make_status(73, "a.mp3"));

    svc.seek(73, 12.5)
        .expect("full queue should drop seek instead of failing");
}

#[test]
fn seek_dispatches_command_when_queue_has_capacity() {
    let svc = AudioService::default();
    let (cmd_tx, cmd_rx) = bounded::<ServiceCmd>(1);
    insert_session(&svc, 74, cmd_tx, make_status(74, "a.mp3"));

    svc.seek(74, 8.25).expect("seek should succeed");

    assert!(matches!(
        cmd_rx.recv().unwrap(),
        ServiceCmd::Seek { seconds } if (seconds - 8.25).abs() < f64::EPSILON
    ));
}
