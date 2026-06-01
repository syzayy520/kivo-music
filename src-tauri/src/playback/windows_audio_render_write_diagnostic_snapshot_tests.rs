use super::windows_audio_render_session::WindowsAudioRenderSessionState;
use super::windows_audio_render_write_cycle::WindowsAudioRenderWriteCycle;
use super::windows_audio_render_write_diagnostic_snapshot::WindowsAudioRenderWriteDiagnosticSnapshot;

#[test]
fn diagnostic_snapshot_reports_requestable_without_backpressure_when_capacity_is_enough() {
    let session = WindowsAudioRenderSessionState::ready(1024).with_written_frames(10);
    let cycle = WindowsAudioRenderWriteCycle::from_session(&session, 256, 256);

    let snapshot =
        WindowsAudioRenderWriteDiagnosticSnapshot::from_session_and_cycle(&session, &cycle);

    assert!(snapshot.can_request_buffer);
    assert!(!snapshot.backpressure_active);
    assert_eq!(snapshot.cycle_summary.submitted_frame_count, 256);
}

#[test]
fn diagnostic_snapshot_marks_backpressure_when_capacity_is_not_enough() {
    let session = WindowsAudioRenderSessionState::ready(128);
    let cycle = WindowsAudioRenderWriteCycle::from_session(&session, 256, 256);

    let snapshot =
        WindowsAudioRenderWriteDiagnosticSnapshot::from_session_and_cycle(&session, &cycle);

    assert!(snapshot.can_request_buffer);
    assert!(snapshot.backpressure_active);
    assert_eq!(snapshot.cycle_summary.deferred_frame_count, 128);
}

#[test]
fn diagnostic_snapshot_cannot_request_buffer_when_source_is_zero() {
    let session = WindowsAudioRenderSessionState::ready(512).with_written_frames(77);
    let cycle = WindowsAudioRenderWriteCycle::from_session(&session, 128, 0);

    let snapshot =
        WindowsAudioRenderWriteDiagnosticSnapshot::from_session_and_cycle(&session, &cycle);

    assert!(!snapshot.can_request_buffer);
    assert_eq!(snapshot.cycle_summary.submitted_frame_count, 0);
    assert!(!snapshot.cycle_summary.can_continue);
}

#[test]
fn diagnostic_snapshot_keeps_unavailable_session_note_and_availability() {
    let session =
        WindowsAudioRenderSessionState::unavailable("render client missing").with_written_frames(5);
    let cycle = WindowsAudioRenderWriteCycle::from_session(&session, 128, 128);

    let snapshot =
        WindowsAudioRenderWriteDiagnosticSnapshot::from_session_and_cycle(&session, &cycle);

    assert!(!snapshot.render_client_available);
    assert_eq!(snapshot.note.as_deref(), Some("render client missing"));
    assert!(!snapshot.can_request_buffer);
}

#[test]
fn diagnostic_snapshot_passes_through_session_buffer_padding_available_and_written() {
    let session = WindowsAudioRenderSessionState::ready(1024)
        .with_padding(320)
        .with_written_frames(500);
    let cycle = WindowsAudioRenderWriteCycle::from_session(&session, 200, 200);

    let snapshot =
        WindowsAudioRenderWriteDiagnosticSnapshot::from_session_and_cycle(&session, &cycle);

    assert_eq!(snapshot.buffer_frame_count, Some(1024));
    assert_eq!(snapshot.queued_padding_frames, 320);
    assert_eq!(snapshot.available_frame_count, 704);
    assert_eq!(snapshot.written_frames, 500);
}

#[test]
fn diagnostic_snapshot_uses_cycle_summary_projection() {
    let session = WindowsAudioRenderSessionState::ready(200).with_written_frames(50);
    let cycle = WindowsAudioRenderWriteCycle::from_session(&session, 180, 160);

    let snapshot =
        WindowsAudioRenderWriteDiagnosticSnapshot::from_session_and_cycle(&session, &cycle);

    assert_eq!(snapshot.cycle_summary.requested_frame_count, 180);
    assert_eq!(snapshot.cycle_summary.source_frame_count, 160);
    assert_eq!(
        snapshot.cycle_summary.submitted_frame_count,
        cycle.outcome.submitted_frame_count
    );
    assert_eq!(
        snapshot.cycle_summary.total_written_frames,
        cycle.outcome.total_written_frames
    );
}
