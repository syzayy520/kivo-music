use crate::playback::errors::{PlaybackError, PlaybackResult};
use crate::playback::native_pipeline::NativePipeline;
use crate::playback::native_pipeline_drain::error::handle_drain_submit_error_with_exporter;
use crate::playback::native_pipeline_drain::submit::observe_submit_frame_result;
use crate::playback::native_pipeline_drain::submit_error_observation::{
    observe_submit_frame_error, NativePipelineDrainSubmitErrorObservation,
};
use crate::playback::output::{AudioOutputFrame, OutputRuntimeStatus, OutputSettings, OutputSink};
use crate::playback::output_submit_error::operation::OutputSubmitOperation;
use crate::playback::output_submit_error::result::OutputSubmitErrorClassificationResult;

use super::helpers::test_frame;

#[derive(Debug)]
enum SubmitErrorObservationSnapshot {
    Classified {
        operation: OutputSubmitOperation,
        message: String,
    },
    UnmappedBackend {
        message: String,
    },
}

struct SubmitErrorSink {
    error: Option<PlaybackError>,
}

impl SubmitErrorSink {
    fn new(error: PlaybackError) -> Self {
        Self { error: Some(error) }
    }
}

impl OutputSink for SubmitErrorSink {
    fn open(&mut self, _settings: &OutputSettings) -> PlaybackResult<OutputRuntimeStatus> {
        Ok(OutputRuntimeStatus::default())
    }

    fn submit_frame(&mut self, _frame: AudioOutputFrame) -> PlaybackResult<OutputRuntimeStatus> {
        Err(self.error.take().expect("submit error should be available"))
    }

    fn pause(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        Ok(OutputRuntimeStatus::default())
    }

    fn resume(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        Ok(OutputRuntimeStatus::default())
    }

    fn flush(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        Ok(OutputRuntimeStatus::default())
    }

    fn stop(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        Ok(OutputRuntimeStatus::default())
    }

    fn set_volume(&mut self, _level: f32) -> PlaybackResult<OutputRuntimeStatus> {
        Ok(OutputRuntimeStatus::default())
    }

    fn set_muted(&mut self, _muted: bool) -> PlaybackResult<OutputRuntimeStatus> {
        Ok(OutputRuntimeStatus::default())
    }

    fn status(&self) -> OutputRuntimeStatus {
        OutputRuntimeStatus::default()
    }

    fn close(&mut self) -> PlaybackResult<()> {
        Ok(())
    }
}

fn snapshot_observation(
    observation: &NativePipelineDrainSubmitErrorObservation,
) -> SubmitErrorObservationSnapshot {
    match observation.classification_result() {
        OutputSubmitErrorClassificationResult::Classified(classification) => {
            SubmitErrorObservationSnapshot::Classified {
                operation: classification.operation(),
                message: classification.message().to_owned(),
            }
        }
        OutputSubmitErrorClassificationResult::Unmapped(PlaybackError::Backend(message)) => {
            SubmitErrorObservationSnapshot::UnmappedBackend {
                message: message.to_owned(),
            }
        }
        OutputSubmitErrorClassificationResult::Unmapped(other) => {
            panic!("expected backend error, got {other}");
        }
    }
}

#[test]
fn submit_error_exports_output_observation_before_returning_original_error() {
    let mut sink = SubmitErrorSink::new(PlaybackError::Output("sink unavailable".to_string()));
    let submit_result = sink.submit_frame(test_frame(0));
    let observed = observe_submit_frame_result(submit_result)
        .expect_err("submit should return observed error");
    let mut snapshot = None;
    let mut pipeline = NativePipeline::new();

    let result = handle_drain_submit_error_with_exporter(&mut pipeline, observed, |observation| {
        snapshot = Some(snapshot_observation(observation));
    });

    match result {
        Err(PlaybackError::Output(message)) => assert_eq!(message, "sink unavailable"),
        Err(other) => panic!("expected output error, got {other}"),
        Ok(()) => panic!("expected output error"),
    }
    match snapshot.expect("collector should receive observation") {
        SubmitErrorObservationSnapshot::Classified { operation, message } => {
            assert_eq!(operation, OutputSubmitOperation::SubmitFrame);
            assert_eq!(message, "sink unavailable");
        }
        SubmitErrorObservationSnapshot::UnmappedBackend { .. } => {
            panic!("expected classified observation");
        }
    }
}

#[test]
fn submit_error_exports_unmapped_observation_before_returning_original_error() {
    let mut sink = SubmitErrorSink::new(PlaybackError::Backend(
        "output text but backend".to_string(),
    ));
    let submit_result = sink.submit_frame(test_frame(0));
    let observed = observe_submit_frame_result(submit_result)
        .expect_err("submit should return observed error");
    let mut snapshot = None;
    let mut pipeline = NativePipeline::new();

    let result = handle_drain_submit_error_with_exporter(&mut pipeline, observed, |observation| {
        snapshot = Some(snapshot_observation(observation));
    });

    match result {
        Err(PlaybackError::Backend(message)) => assert_eq!(message, "output text but backend"),
        Err(other) => panic!("expected backend error, got {other}"),
        Ok(()) => panic!("expected backend error"),
    }
    match snapshot.expect("collector should receive observation") {
        SubmitErrorObservationSnapshot::UnmappedBackend { message } => {
            assert_eq!(message, "output text but backend");
        }
        SubmitErrorObservationSnapshot::Classified { .. } => {
            panic!("expected unmapped backend observation");
        }
    }
}

#[test]
fn observation_exposes_classification_result_without_consuming_error() {
    let observed =
        observe_submit_frame_error(PlaybackError::Output("borrowed observation".to_string()));

    assert!(matches!(
        observed.observation().classification_result(),
        OutputSubmitErrorClassificationResult::Classified(_)
    ));
    assert!(matches!(
        observed.into_error(),
        PlaybackError::Output(message) if message == "borrowed observation"
    ));
}

#[test]
fn observation_into_error_still_returns_original_output_error() {
    let observed = observe_submit_frame_error(PlaybackError::Output("same output".to_string()));

    assert!(matches!(
        observed.into_error(),
        PlaybackError::Output(message) if message == "same output"
    ));
}

#[test]
fn observation_into_error_still_returns_original_unmapped_error() {
    let observed = observe_submit_frame_error(PlaybackError::Backend("same backend".to_string()));

    assert!(matches!(
        observed.into_error(),
        PlaybackError::Backend(message) if message == "same backend"
    ));
}

#[test]
fn observation_classification_result_is_reachable_from_playback_sibling_context_without_mod_rs_changes(
) {
    fn classification_result_from_sibling(
        observation: &NativePipelineDrainSubmitErrorObservation,
    ) -> &OutputSubmitErrorClassificationResult {
        observation.classification_result()
    }

    let observed = observe_submit_frame_error(PlaybackError::Output("reachable".to_string()));

    assert!(matches!(
        classification_result_from_sibling(observed.observation()),
        OutputSubmitErrorClassificationResult::Classified(_)
    ));
}
