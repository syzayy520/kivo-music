use super::super::engine::PlaybackEngine;
use super::super::errors::PlaybackError;
use super::native::KivoNativeEngine;

#[test]
fn set_volume_updates_state_and_returns_typed_unsupported() {
    let mut engine = KivoNativeEngine::new();

    let result = engine.set_volume(1.5);

    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(message, "kivo core audio set volume is not implemented yet");
        }
        other => panic!("expected unsupported operation, got {other:?}"),
    }

    let state = engine.current_state();
    assert_eq!(state.volume.level, 1.0);
    assert_eq!(
        state.error.as_deref(),
        Some("kivo core audio set volume is not implemented yet")
    );
}

#[test]
fn set_muted_updates_state_and_returns_typed_unsupported() {
    let mut engine = KivoNativeEngine::new();

    let result = engine.set_muted(true);

    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(message, "kivo core audio set muted is not implemented yet");
        }
        other => panic!("expected unsupported operation, got {other:?}"),
    }

    let state = engine.current_state();
    assert!(state.volume.muted);
    assert_eq!(
        state.error.as_deref(),
        Some("kivo core audio set muted is not implemented yet")
    );
}
