use super::windows_audio_render_client::WindowsAudioRenderClientSnapshot;

#[test]
fn unavailable_snapshot_models_safe_state() {
    let snapshot = WindowsAudioRenderClientSnapshot::unavailable("not supported");

    assert!(!snapshot.render_client_available);
    assert_eq!(snapshot.buffer_frame_count, None);
    assert_eq!(snapshot.note.as_deref(), Some("not supported"));
    assert!(!snapshot.session.initialized);
    assert!(!snapshot.session.render_client_available);
    assert!(!snapshot.session.started);
    assert_eq!(snapshot.session.buffer_frame_count, None);
    assert_eq!(snapshot.session.queued_padding_frames, 0);
    assert_eq!(snapshot.session.available_frame_count, 0);
    assert_eq!(snapshot.session.written_frames, 0);
    assert_eq!(snapshot.session.note.as_deref(), Some("not supported"));
}

#[test]
fn ready_snapshot_models_buffer_padding_and_capacity() {
    let snapshot = WindowsAudioRenderClientSnapshot::ready(1024, 256);

    assert!(snapshot.render_client_available);
    assert_eq!(snapshot.buffer_frame_count, Some(1024));
    assert_eq!(snapshot.note, None);
    assert!(snapshot.session.initialized);
    assert!(snapshot.session.render_client_available);
    assert!(!snapshot.session.started);
    assert_eq!(snapshot.session.buffer_frame_count, Some(1024));
    assert_eq!(snapshot.session.queued_padding_frames, 256);
    assert_eq!(snapshot.session.available_frame_count, 768);
    assert_eq!(snapshot.session.written_frames, 0);
    assert_eq!(snapshot.session.note, None);
}
