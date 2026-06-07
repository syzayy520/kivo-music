use super::super::backends::native::KivoNativeEngine;
use super::super::capabilities::PlaybackCapabilities;
use super::super::engine::PlaybackEngine;
use super::super::errors::PlaybackError;

#[test]
fn public_native_engine_remains_typed_unsupported() {
    let mut engine = KivoNativeEngine::new();

    let result = engine.play();
    assert!(matches!(
        result,
        Err(PlaybackError::UnsupportedOperation(_))
    ));

    let result = engine.pause();
    assert!(matches!(
        result,
        Err(PlaybackError::UnsupportedOperation(_))
    ));

    let result = engine.resume();
    assert!(matches!(
        result,
        Err(PlaybackError::UnsupportedOperation(_))
    ));

    let result = engine.stop();
    assert!(result.is_ok(), "stop should succeed even without a loaded track");

    let result = engine.seek(0);
    assert!(matches!(
        result,
        Err(PlaybackError::UnsupportedOperation(_))
    ));

    let result = engine.set_volume(1.0);
    assert!(matches!(
        result,
        Err(PlaybackError::UnsupportedOperation(_))
    ));

    let result = engine.set_muted(false);
    assert!(matches!(
        result,
        Err(PlaybackError::UnsupportedOperation(_))
    ));
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
