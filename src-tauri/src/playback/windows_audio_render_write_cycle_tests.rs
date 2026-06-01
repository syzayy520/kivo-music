use super::windows_audio_render_session::WindowsAudioRenderSessionState;
use super::windows_audio_render_write_cycle::WindowsAudioRenderWriteCycle;

#[test]
fn write_cycle_requests_buffer_without_defer_when_source_and_capacity_are_enough() {
    let session = WindowsAudioRenderSessionState::ready(1024).with_written_frames(100);

    let cycle = WindowsAudioRenderWriteCycle::from_session(&session, 256, 256);

    assert!(cycle.should_request_buffer);
    assert!(!cycle.should_defer);
    assert_eq!(cycle.outcome.submitted_frame_count, 256);
}

#[test]
fn write_cycle_requests_buffer_and_defers_when_capacity_is_not_enough() {
    let session = WindowsAudioRenderSessionState::ready(128);

    let cycle = WindowsAudioRenderWriteCycle::from_session(&session, 256, 256);

    assert!(cycle.should_request_buffer);
    assert!(cycle.should_defer);
    assert_eq!(cycle.outcome.submitted_frame_count, 128);
    assert_eq!(cycle.outcome.deferred_frame_count, 128);
}

#[test]
fn write_cycle_does_not_request_buffer_when_source_is_zero() {
    let session = WindowsAudioRenderSessionState::ready(512).with_written_frames(12);

    let cycle = WindowsAudioRenderWriteCycle::from_session(&session, 256, 0);

    assert!(!cycle.should_request_buffer);
    assert!(!cycle.should_defer);
    assert_eq!(cycle.outcome.submitted_frame_count, 0);
    assert_eq!(cycle.outcome.total_written_frames, 12);
}

#[test]
fn write_cycle_does_not_request_buffer_for_unavailable_session() {
    let session =
        WindowsAudioRenderSessionState::unavailable("render unavailable").with_written_frames(22);

    let cycle = WindowsAudioRenderWriteCycle::from_session(&session, 128, 128);

    assert!(!cycle.should_request_buffer);
    assert!(cycle.should_defer);
    assert_eq!(cycle.outcome.submitted_frame_count, 0);
    assert_eq!(cycle.outcome.deferred_frame_count, 128);
    assert_eq!(cycle.outcome.total_written_frames, 22);
}

#[test]
fn write_cycle_accumulates_total_written_frames_from_session_state() {
    let session = WindowsAudioRenderSessionState::ready(1024).with_written_frames(500);

    let cycle = WindowsAudioRenderWriteCycle::from_session(&session, 200, 200);

    assert_eq!(cycle.outcome.submitted_frame_count, 200);
    assert_eq!(cycle.outcome.total_written_frames, 700);
}

#[test]
fn write_cycle_total_written_frames_saturates_near_u64_max() {
    let session = WindowsAudioRenderSessionState::ready(300).with_written_frames(u64::MAX - 50);

    let cycle = WindowsAudioRenderWriteCycle::from_session(&session, 200, 200);

    assert_eq!(cycle.outcome.submitted_frame_count, 200);
    assert_eq!(cycle.outcome.total_written_frames, u64::MAX);
}
