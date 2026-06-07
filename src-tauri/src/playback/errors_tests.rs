use super::errors::{PlaybackError, PlaybackResult};

#[test]
fn invalid_control_input_formats_with_control_context() {
    let error = PlaybackError::InvalidControlInput("volume level must be finite".to_string());

    assert_eq!(
        error.to_string(),
        "invalid control input: volume level must be finite"
    );
}

#[test]
fn playback_result_alias_accepts_invalid_control_input() {
    fn invalid_control_input_result() -> PlaybackResult<()> {
        Err(PlaybackError::InvalidControlInput(
            "volume level must be finite".to_string(),
        ))
    }

    match invalid_control_input_result() {
        Err(PlaybackError::InvalidControlInput(message)) => {
            assert_eq!(message, "volume level must be finite");
        }
        other => panic!("expected invalid control input, got {other:?}"),
    }
}

#[test]
fn existing_error_formatting_remains_unchanged() {
    let cases = [
        (PlaybackError::Backend("x".to_string()), "backend error: x"),
        (PlaybackError::Path("x".to_string()), "path error: x"),
        (PlaybackError::Queue("x".to_string()), "queue error: x"),
        (PlaybackError::Output("x".to_string()), "output error: x"),
        (
            PlaybackError::UnsupportedFormat("x".to_string()),
            "unsupported format: x",
        ),
        (
            PlaybackError::UnsupportedOperation("x".to_string()),
            "unsupported operation: x",
        ),
        (
            PlaybackError::Playback("x".to_string()),
            "playback failed: x",
        ),
        (PlaybackError::NoTrack("x".to_string()), "no track: x"),
        (
            PlaybackError::InvalidControlState("x".to_string()),
            "invalid control state: x",
        ),
        (
            PlaybackError::SeekOutOfRange {
                position_ms: 9999,
                duration_ms: 5000,
            },
            "seek out of range: position 9999ms exceeds duration 5000ms",
        ),
        (
            PlaybackError::SeekTransactionFailed("x".to_string()),
            "seek transaction failed: x",
        ),
    ];

    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn no_track_error_formats_with_message() {
    let error = PlaybackError::NoTrack("playlist is empty".to_string());
    assert_eq!(error.to_string(), "no track: playlist is empty");
}

#[test]
fn invalid_control_state_error_formats_with_message() {
    let error = PlaybackError::InvalidControlState("engine not initialized".to_string());
    assert_eq!(
        error.to_string(),
        "invalid control state: engine not initialized"
    );
}

#[test]
fn seek_out_of_range_error_formats_with_position_and_duration() {
    let error = PlaybackError::SeekOutOfRange {
        position_ms: 300_000,
        duration_ms: 180_000,
    };
    assert_eq!(
        error.to_string(),
        "seek out of range: position 300000ms exceeds duration 180000ms"
    );
}

#[test]
fn seek_transaction_failed_error_formats_with_message() {
    let error = PlaybackError::SeekTransactionFailed("decoder crashed".to_string());
    assert_eq!(
        error.to_string(),
        "seek transaction failed: decoder crashed"
    );
}
