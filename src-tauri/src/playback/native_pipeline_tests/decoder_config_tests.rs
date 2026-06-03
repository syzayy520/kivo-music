use super::*;

#[test]
fn configure_decoder_open_creates_session_and_marks_open() {
    let mut pipeline = NativePipeline::new();

    pipeline.configure_decoder_open(request(), stream_info(), 2_000);
    pipeline.update_decoder_position(4_500);
    pipeline.count_decoded_frame();

    let state = pipeline.snapshot();
    let session = state
        .decoder_session
        .as_ref()
        .expect("decoder session should be created");

    assert_eq!(state.decoder_state.phase, DecoderRuntimePhase::Open);
    assert_eq!(session.track_id, "track-77");
    assert_eq!(session.source_path, "C:/Music/track-77.flac");
    assert_eq!(session.opened_at_ms, 2_000);
    assert_eq!(session.last_position_ms, 4_500);
    assert_eq!(session.decoded_frame_count, 1);
}
