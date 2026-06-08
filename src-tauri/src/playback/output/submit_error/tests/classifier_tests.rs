use crate::playback::errors::PlaybackError;

use super::super::classifier::OutputSubmitErrorClassifier;
use super::super::operation::OutputSubmitOperation;
use super::super::result::OutputSubmitErrorClassificationResult;

fn classify_submit_frame_error(error: PlaybackError) -> OutputSubmitErrorClassificationResult {
    let classifier = OutputSubmitErrorClassifier;
    classifier.classify_submit_frame_error(error)
}

fn assert_unmapped(
    result: OutputSubmitErrorClassificationResult,
    assert_error: impl FnOnce(PlaybackError),
) {
    match result {
        OutputSubmitErrorClassificationResult::Unmapped(error) => assert_error(error),
        OutputSubmitErrorClassificationResult::Classified(_) => {
            panic!("expected unmapped playback error")
        }
    }
}

#[test]
fn submit_frame_output_error_is_classified() {
    let result = classify_submit_frame_error(PlaybackError::Output("sink unavailable".into()));

    match result {
        OutputSubmitErrorClassificationResult::Classified(classification) => {
            assert_eq!(
                classification.operation(),
                OutputSubmitOperation::SubmitFrame
            );
            assert_eq!(classification.operation().as_str(), "submit_frame");
            assert_eq!(classification.message(), "sink unavailable");
        }
        OutputSubmitErrorClassificationResult::Unmapped(_) => {
            panic!("expected classified output submit error")
        }
    }
}

#[test]
fn output_message_uses_inner_payload_not_display_text() {
    let message = "distinctive sink payload";
    let result = classify_submit_frame_error(PlaybackError::Output(message.into()));

    match result {
        OutputSubmitErrorClassificationResult::Classified(classification) => {
            assert_eq!(classification.message(), message);
        }
        OutputSubmitErrorClassificationResult::Unmapped(_) => {
            panic!("expected classified output submit error")
        }
    }
}

#[test]
fn non_output_error_with_output_text_remains_unmapped() {
    let message = "backend output path rejected";
    assert_unmapped(
        classify_submit_frame_error(PlaybackError::Backend(message.into())),
        |error| match error {
            PlaybackError::Backend(actual) => assert_eq!(actual, message),
            _ => panic!("expected original backend error"),
        },
    );
}

#[test]
fn queue_error_is_unmapped() {
    assert_unmapped(
        classify_submit_frame_error(PlaybackError::Queue("queue rejected".into())),
        |error| match error {
            PlaybackError::Queue(message) => assert_eq!(message, "queue rejected"),
            _ => panic!("expected original queue error"),
        },
    );
}

#[test]
fn path_error_is_unmapped() {
    assert_unmapped(
        classify_submit_frame_error(PlaybackError::Path("missing path".into())),
        |error| match error {
            PlaybackError::Path(message) => assert_eq!(message, "missing path"),
            _ => panic!("expected original path error"),
        },
    );
}

#[test]
fn unsupported_format_is_unmapped() {
    assert_unmapped(
        classify_submit_frame_error(PlaybackError::UnsupportedFormat("codec".into())),
        |error| match error {
            PlaybackError::UnsupportedFormat(message) => assert_eq!(message, "codec"),
            _ => panic!("expected original unsupported format error"),
        },
    );
}

#[test]
fn unsupported_operation_is_unmapped() {
    assert_unmapped(
        classify_submit_frame_error(PlaybackError::UnsupportedOperation("operation".into())),
        |error| match error {
            PlaybackError::UnsupportedOperation(message) => {
                assert_eq!(message, "operation")
            }
            _ => panic!("expected original unsupported operation error"),
        },
    );
}

#[test]
fn playback_error_is_unmapped() {
    assert_unmapped(
        classify_submit_frame_error(PlaybackError::Playback("playback stopped".into())),
        |error| match error {
            PlaybackError::Playback(message) => assert_eq!(message, "playback stopped"),
            _ => panic!("expected original playback error"),
        },
    );
}

#[test]
fn result_supports_exhaustive_match() {
    let description = match classify_submit_frame_error(PlaybackError::Output("mapped".into())) {
        OutputSubmitErrorClassificationResult::Classified(_) => "classified",
        OutputSubmitErrorClassificationResult::Unmapped(_) => "unmapped",
    };

    assert_eq!(description, "classified");
}

#[test]
fn classifier_is_stateless_zst() {
    assert_eq!(std::mem::size_of::<OutputSubmitErrorClassifier>(), 0);
}
