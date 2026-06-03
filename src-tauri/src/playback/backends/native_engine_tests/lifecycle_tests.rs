use super::super::super::decoder_runtime_state::DecoderRuntimePhase;
use super::super::super::engine::PlaybackEngine;
use super::super::native::KivoNativeEngine;
use super::wav_track;

#[test]
fn shutdown_after_wav_load_closes_pipeline_children() {
    let mut engine = KivoNativeEngine::new();
    let (track, path) = wav_track();

    let _ = engine.load(track);
    engine.shutdown().expect("shutdown native engine");

    let pipeline = engine.pipeline_state();
    assert_eq!(pipeline.decoder_state.phase, DecoderRuntimePhase::Closed);
    assert!(pipeline.decoder_session.is_none());
    assert!(pipeline.last_decoded_frame.is_none());

    std::fs::remove_file(path).expect("remove wav file");
}
