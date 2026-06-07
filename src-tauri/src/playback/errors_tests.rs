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
    ];

    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }
}
