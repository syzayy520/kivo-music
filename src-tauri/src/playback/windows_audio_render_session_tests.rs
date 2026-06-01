use super::windows_audio_render_session::WindowsAudioRenderSessionState;

#[test]
fn unavailable_session_is_safe_and_not_started() {
    let session = WindowsAudioRenderSessionState::unavailable("not available");

    assert!(!session.initialized);
    assert!(!session.render_client_available);
    assert!(!session.started);
    assert_eq!(session.buffer_frame_count, None);
    assert_eq!(session.queued_padding_frames, 0);
    assert_eq!(session.available_frame_count, 0);
    assert_eq!(session.written_frames, 0);
    assert_eq!(session.note.as_deref(), Some("not available"));
}

#[test]
fn ready_session_models_full_buffer_without_starting() {
    let session = WindowsAudioRenderSessionState::ready(480);

    assert!(session.initialized);
    assert!(session.render_client_available);
    assert!(!session.started);
    assert_eq!(session.buffer_frame_count, Some(480));
    assert_eq!(session.queued_padding_frames, 0);
    assert_eq!(session.available_frame_count, 480);
    assert_eq!(session.written_frames, 0);
    assert_eq!(session.note, None);
}

#[test]
fn padding_and_written_frames_are_explicit_model_updates() {
    let session = WindowsAudioRenderSessionState::ready(960)
        .with_padding(120)
        .with_written_frames(240);

    assert!(session.initialized);
    assert!(session.render_client_available);
    assert!(!session.started);
    assert_eq!(session.buffer_frame_count, Some(960));
    assert_eq!(session.queued_padding_frames, 120);
    assert_eq!(session.available_frame_count, 840);
    assert_eq!(session.written_frames, 240);
}

#[test]
fn available_frame_count_saturates_when_padding_exceeds_buffer() {
    let session = WindowsAudioRenderSessionState::ready(128).with_padding(256);

    assert_eq!(session.buffer_frame_count, Some(128));
    assert_eq!(session.queued_padding_frames, 256);
    assert_eq!(session.available_frame_count, 0);
}
