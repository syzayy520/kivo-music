use super::windows_audio_render_session::WindowsAudioRenderSessionState;
use super::windows_audio_render_write_intent::WindowsAudioRenderWriteIntent;

#[test]
fn write_intent_submits_when_source_and_capacity_are_enough() {
    let session = WindowsAudioRenderSessionState::ready(1024).with_padding(128);

    let intent = WindowsAudioRenderWriteIntent::from_session(&session, 256, 512);

    assert_eq!(intent.requested_frame_count, 256);
    assert_eq!(intent.source_frame_count, 512);
    assert_eq!(intent.plan.requested_frame_count, 256);
    assert_eq!(intent.plan.available_frame_count, 896);
    assert_eq!(intent.plan.writable_frame_count, 256);
    assert_eq!(intent.plan.deferred_frame_count, 0);
    assert!(intent.can_submit);
}

#[test]
fn write_intent_uses_source_when_source_is_smaller_than_requested() {
    let session = WindowsAudioRenderSessionState::ready(1024);

    let intent = WindowsAudioRenderWriteIntent::from_session(&session, 400, 120);

    assert_eq!(intent.requested_frame_count, 400);
    assert_eq!(intent.source_frame_count, 120);
    assert_eq!(intent.plan.requested_frame_count, 120);
    assert_eq!(intent.plan.writable_frame_count, 120);
    assert_eq!(intent.plan.deferred_frame_count, 0);
    assert!(intent.can_submit);
}

#[test]
fn write_intent_defers_when_capacity_is_smaller_than_effective_request() {
    let session = WindowsAudioRenderSessionState::ready(256).with_padding(196);

    let intent = WindowsAudioRenderWriteIntent::from_session(&session, 200, 180);

    assert_eq!(intent.plan.requested_frame_count, 180);
    assert_eq!(intent.plan.available_frame_count, 60);
    assert_eq!(intent.plan.writable_frame_count, 60);
    assert_eq!(intent.plan.deferred_frame_count, 120);
    assert!(intent.can_submit);
}

#[test]
fn write_intent_blocks_when_source_is_zero() {
    let session = WindowsAudioRenderSessionState::ready(512);

    let intent = WindowsAudioRenderWriteIntent::from_session(&session, 256, 0);

    assert_eq!(intent.plan.requested_frame_count, 0);
    assert_eq!(intent.plan.writable_frame_count, 0);
    assert_eq!(intent.plan.deferred_frame_count, 0);
    assert!(!intent.can_submit);
}

#[test]
fn write_intent_blocks_for_unavailable_session() {
    let session = WindowsAudioRenderSessionState::unavailable("render unavailable");

    let intent = WindowsAudioRenderWriteIntent::from_session(&session, 128, 128);

    assert_eq!(intent.plan.requested_frame_count, 128);
    assert_eq!(intent.plan.available_frame_count, 0);
    assert_eq!(intent.plan.writable_frame_count, 0);
    assert_eq!(intent.plan.deferred_frame_count, 128);
    assert!(!intent.can_submit);
}
