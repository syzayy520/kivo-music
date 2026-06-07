use super::super::backends::native::KivoNativeEngine;
use super::super::capabilities::PlaybackCapabilities;
use super::super::engine::PlaybackEngine;
use super::super::errors::PlaybackError;

#[test]
fn public_native_engine_keeps_native_control_contract() {
    let mut engine = KivoNativeEngine::new();

    let result = engine.play();
    assert!(matches!(
        result,
        Err(PlaybackError::UnsupportedOperation(_))
    ));

    let pause_state = engine
        .pause()
        .expect("native pause without track should succeed after P0-147");
    assert!(matches!(
        pause_state.status,
        crate::playback::types::PlaybackStatus::Idle
    ));

    let resume_state = engine
        .resume()
        .expect("native resume without track should succeed after P0-147");
    assert!(matches!(
        resume_state.status,
        crate::playback::types::PlaybackStatus::Idle
    ));

    let result = engine.stop();
    assert!(
        result.is_ok(),
        "stop should succeed even without a loaded track"
    );

    let result = engine.seek(0);
    assert!(matches!(
        result,
        Err(PlaybackError::NoTrack(_))
    ));

    let volume_state = engine
        .set_volume(1.0)
        .expect("native set_volume should succeed after P0-145");
    assert_eq!(volume_state.volume.level, 1.0);

    let muted_state = engine
        .set_muted(false)
        .expect("native set_muted should succeed after P0-145");
    assert!(!muted_state.volume.muted);
}

#[test]
fn capabilities_remain_default() {
    let caps = PlaybackCapabilities::default();
    assert!(!caps.can_seek);
    assert!(!caps.can_select_output_device);
    assert!(!caps.can_use_exclusive_output);
    assert!(!caps.can_probe_metadata);
    assert!(!caps.can_gapless);
    assert!(!caps.can_replaygain);
}
