use crate::playback::errors::{PlaybackError, PlaybackResult};
use crate::playback::native_pipeline::NativePipeline;
use crate::playback::native_pipeline_drain::error::handle_drain_submit_error;
use crate::playback::native_pipeline_drain::submit::observe_submit_frame_result;
use crate::playback::native_pipeline_drain::submit_error_observation::observe_submit_frame_error;
use crate::playback::output::{AudioOutputFrame, OutputRuntimeStatus, OutputSettings, OutputSink};
use crate::playback::output_submit_error::operation::OutputSubmitOperation;
use crate::playback::output_submit_error::result::OutputSubmitErrorClassificationResult;

use super::helpers::test_frame;

struct SubmitErrorSink {
    error: Option<PlaybackError>,
    status: OutputRuntimeStatus,
}

impl SubmitErrorSink {
    fn new(error: PlaybackError) -> Self {
        Self {
            error: Some(error),
            status: OutputRuntimeStatus::default(),
        }
    }
}

impl OutputSink for SubmitErrorSink {
    fn open(&mut self, _settings: &OutputSettings) -> PlaybackResult<OutputRuntimeStatus> {
        Ok(self.status.clone())
    }

    fn submit_frame(&mut self, _frame: AudioOutputFrame) -> PlaybackResult<OutputRuntimeStatus> {
        Err(self.error.take().expect("submit error should be available"))
    }

    fn pause(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        Ok(self.status.clone())
    }

    fn resume(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        Ok(self.status.clone())
    }

    fn flush(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        Ok(self.status.clone())
    }

    fn stop(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        Ok(self.status.clone())
    }

    fn set_volume(&mut self, _level: f32) -> PlaybackResult<OutputRuntimeStatus> {
        Ok(self.status.clone())
    }

    fn set_muted(&mut self, _muted: bool) -> PlaybackResult<OutputRuntimeStatus> {
        Ok(self.status.clone())
    }

    fn status(&self) -> OutputRuntimeStatus {
        self.status.clone()
    }

    fn close(&mut self) -> PlaybackResult<()> {
        Ok(())
    }
}

fn assert_classified_submit_frame(
    result: &OutputSubmitErrorClassificationResult,
    expected_message: &str,
) {
    match result {
        OutputSubmitErrorClassificationResult::Classified(classification) => {
            assert_eq!(
                classification.operation(),
                OutputSubmitOperation::SubmitFrame
            );
            assert_eq!(classification.message(), expected_message);
        }
        OutputSubmitErrorClassificationResult::Unmapped(_) => {
            panic!("expected classified submit-frame error");
        }
    }
}

#[test]
fn drain_observation_classifies_output_submit_error() {
    let observed =
        observe_submit_frame_error(PlaybackError::Output("sink unavailable".to_string()));

    assert_classified_submit_frame(
        observed.observation().classification_result(),
        "sink unavailable",
    );
}

#[test]
fn drain_observation_preserves_unmapped_non_output_error() {
    let observed = observe_submit_frame_error(PlaybackError::Backend(
        "output text but backend".to_string(),
    ));

    match observed.observation().classification_result() {
        OutputSubmitErrorClassificationResult::Unmapped(PlaybackError::Backend(message)) => {
            assert_eq!(message, "output text but backend");
        }
        OutputSubmitErrorClassificationResult::Unmapped(other) => {
            panic!("expected backend error, got {other}");
        }
        OutputSubmitErrorClassificationResult::Classified(_) => {
            panic!("expected unmapped non-output error");
        }
    }
}

#[test]
fn submit_error_returns_original_output_error_after_observation() {
    let mut sink = SubmitErrorSink::new(PlaybackError::Output("sink unavailable".to_string()));
    let submit_result = sink.submit_frame(test_frame(0));
    let observed = observe_submit_frame_result(submit_result)
        .expect_err("submit should return observed error");

    assert_classified_submit_frame(
        observed.observation().classification_result(),
        "sink unavailable",
    );

    let mut pipeline = NativePipeline::new();
    match handle_drain_submit_error(&mut pipeline, observed) {
        Err(PlaybackError::Output(message)) => assert_eq!(message, "sink unavailable"),
        Err(other) => panic!("expected output error, got {other}"),
        Ok(()) => panic!("expected output error"),
    }
}

#[test]
fn submit_error_returns_original_non_output_error_after_observation() {
    let mut sink = SubmitErrorSink::new(PlaybackError::Backend(
        "output text but backend".to_string(),
    ));
    let submit_result = sink.submit_frame(test_frame(0));
    let observed = observe_submit_frame_result(submit_result)
        .expect_err("submit should return observed error");

    match observed.observation().classification_result() {
        OutputSubmitErrorClassificationResult::Unmapped(PlaybackError::Backend(message)) => {
            assert_eq!(message, "output text but backend");
        }
        OutputSubmitErrorClassificationResult::Unmapped(other) => {
            panic!("expected backend error, got {other}");
        }
        OutputSubmitErrorClassificationResult::Classified(_) => {
            panic!("expected unmapped non-output error");
        }
    }

    let mut pipeline = NativePipeline::new();
    match handle_drain_submit_error(&mut pipeline, observed) {
        Err(PlaybackError::Backend(message)) => assert_eq!(message, "output text but backend"),
        Err(other) => panic!("expected backend error, got {other}"),
        Ok(()) => panic!("expected backend error"),
    }
}
