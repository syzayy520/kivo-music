use super::windows_audio_render_session::WindowsAudioRenderSessionState;
use super::windows_audio_render_write_cycle::WindowsAudioRenderWriteCycle;
use super::windows_audio_render_write_cycle_summary::WindowsAudioRenderWriteCycleSummary;

#[test]
fn cycle_summary_maps_fields_when_source_and_capacity_are_enough() {
    let session = WindowsAudioRenderSessionState::ready(1024).with_written_frames(100);
    let cycle = WindowsAudioRenderWriteCycle::from_session(&session, 256, 256);

    let summary = WindowsAudioRenderWriteCycleSummary::from_cycle(&cycle);

    assert_eq!(summary.requested_frame_count, 256);
    assert_eq!(summary.source_frame_count, 256);
    assert_eq!(summary.submitted_frame_count, 256);
    assert_eq!(summary.deferred_frame_count, 0);
    assert!(summary.should_request_buffer);
    assert!(!summary.should_defer);
    assert!(summary.can_continue);
}

#[test]
fn cycle_summary_keeps_deferred_fields_when_capacity_is_not_enough() {
    let session = WindowsAudioRenderSessionState::ready(128).with_written_frames(50);
    let cycle = WindowsAudioRenderWriteCycle::from_session(&session, 256, 256);

    let summary = WindowsAudioRenderWriteCycleSummary::from_cycle(&cycle);

    assert_eq!(summary.submitted_frame_count, 128);
    assert_eq!(summary.deferred_frame_count, 128);
    assert!(summary.should_request_buffer);
    assert!(summary.should_defer);
}

#[test]
fn cycle_summary_is_non_continuable_when_source_is_zero() {
    let session = WindowsAudioRenderSessionState::ready(512).with_written_frames(12);
    let cycle = WindowsAudioRenderWriteCycle::from_session(&session, 256, 0);

    let summary = WindowsAudioRenderWriteCycleSummary::from_cycle(&cycle);

    assert_eq!(summary.source_frame_count, 0);
    assert_eq!(summary.submitted_frame_count, 0);
    assert!(!summary.can_continue);
}

#[test]
fn cycle_summary_does_not_request_buffer_for_unavailable_session() {
    let session =
        WindowsAudioRenderSessionState::unavailable("render unavailable").with_written_frames(22);
    let cycle = WindowsAudioRenderWriteCycle::from_session(&session, 128, 128);

    let summary = WindowsAudioRenderWriteCycleSummary::from_cycle(&cycle);

    assert!(!summary.should_request_buffer);
    assert_eq!(summary.submitted_frame_count, 0);
}

#[test]
fn cycle_summary_passes_through_total_written_frames_from_outcome() {
    let session = WindowsAudioRenderSessionState::ready(1024).with_written_frames(500);
    let cycle = WindowsAudioRenderWriteCycle::from_session(&session, 200, 200);

    let summary = WindowsAudioRenderWriteCycleSummary::from_cycle(&cycle);

    assert_eq!(
        summary.total_written_frames,
        cycle.outcome.total_written_frames
    );
    assert_eq!(summary.total_written_frames, 700);
}
