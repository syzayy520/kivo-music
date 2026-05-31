use super::super::errors::PlaybackError;
use super::native_unsupported::{unsupported_operation, unsupported_operation_message};

#[test]
fn unsupported_operation_returns_typed_error() {
    let result = unsupported_operation::<()>("play");

    match result {
        Err(PlaybackError::UnsupportedOperation(message)) => {
            assert_eq!(message, "kivo core audio play is not implemented yet");
        }
        other => panic!("expected unsupported operation, got {other:?}"),
    }
}

#[test]
fn unsupported_operation_message_uses_native_core_label() {
    let message = unsupported_operation_message("seek");

    assert_eq!(message, "kivo core audio seek is not implemented yet");
}
