use super::windows_audio_render_session::WindowsAudioRenderSessionState;
use super::windows_audio_render_write_intent::WindowsAudioRenderWriteIntent;
use super::windows_audio_render_write_outcome::WindowsAudioRenderWriteOutcome;

#[test]
fn write_outcome_submitted_frame_count_matches_intent_plan() {
    let session = WindowsAudioRenderSessionState::ready(1024);
    let intent = WindowsAudioRenderWriteIntent::from_session(&session, 256, 256);

    let outcome = WindowsAudioRenderWriteOutcome::from_intent(&intent, 0);

    assert_eq!(outcome.requested_frame_count, 256);
    assert_eq!(outcome.submitted_frame_count, 256);
    assert!(outcome.can_continue);
}

#[test]
fn write_outcome_deferred_frame_count_matches_intent_plan() {
    let session = WindowsAudioRenderSessionState::ready(128);
    let intent = WindowsAudioRenderWriteIntent::from_session(&session, 256, 256);

    let outcome = WindowsAudioRenderWriteOutcome::from_intent(&intent, 0);

    assert_eq!(outcome.submitted_frame_count, 128);
    assert_eq!(outcome.deferred_frame_count, 128);
    assert!(outcome.can_continue);
}

#[test]
fn write_outcome_accumulates_written_frames() {
    let session = WindowsAudioRenderSessionState::ready(512);
    let intent = WindowsAudioRenderWriteIntent::from_session(&session, 120, 120);

    let outcome = WindowsAudioRenderWriteOutcome::from_intent(&intent, 400);

    assert_eq!(outcome.submitted_frame_count, 120);
    assert_eq!(outcome.total_written_frames, 520);
}

#[test]
fn write_outcome_saturates_total_written_frames_near_u64_max() {
    let session = WindowsAudioRenderSessionState::ready(256);
    let intent = WindowsAudioRenderWriteIntent::from_session(&session, 128, 128);

    let outcome = WindowsAudioRenderWriteOutcome::from_intent(&intent, u64::MAX - 64);

    assert_eq!(outcome.submitted_frame_count, 128);
    assert_eq!(outcome.total_written_frames, u64::MAX);
}

#[test]
fn write_outcome_stops_when_source_or_capacity_is_zero() {
    let ready_session = WindowsAudioRenderSessionState::ready(512);
    let zero_source_intent = WindowsAudioRenderWriteIntent::from_session(&ready_session, 256, 0);
    let zero_source_outcome = WindowsAudioRenderWriteOutcome::from_intent(&zero_source_intent, 12);

    assert_eq!(zero_source_outcome.submitted_frame_count, 0);
    assert!(!zero_source_outcome.can_continue);

    let unavailable_session = WindowsAudioRenderSessionState::unavailable("render unavailable");
    let zero_capacity_intent =
        WindowsAudioRenderWriteIntent::from_session(&unavailable_session, 128, 128);
    let zero_capacity_outcome =
        WindowsAudioRenderWriteOutcome::from_intent(&zero_capacity_intent, 34);

    assert_eq!(zero_capacity_outcome.submitted_frame_count, 0);
    assert!(!zero_capacity_outcome.can_continue);
}
