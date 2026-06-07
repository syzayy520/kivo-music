use crate::playback::errors::PlaybackError;
use crate::playback::output_submit_error::classification::OutputSubmitErrorClassification;
use crate::playback::output_submit_error::classifier::OutputSubmitErrorClassifier;
use crate::playback::output_submit_error::operation::OutputSubmitOperation;
use crate::playback::output_submit_error::result::OutputSubmitErrorClassificationResult;

use super::super::sink_failure_mapping::{
    mapper::ProductionOutputRouteSinkFailureMapper,
    result::ProductionOutputRouteSinkFailureMappingResult,
};

fn mapper() -> ProductionOutputRouteSinkFailureMapper {
    ProductionOutputRouteSinkFailureMapper
}

#[test]
fn route_adapter_maps_neutral_submit_frame_output_classification() {
    let classification = OutputSubmitErrorClassification::new(
        OutputSubmitOperation::SubmitFrame,
        "sink unavailable".into(),
    );

    match mapper().map_output_submit_classification(classification) {
        ProductionOutputRouteSinkFailureMappingResult::Mapped(failure) => {
            assert_eq!(
                failure.operation(),
                OutputSubmitOperation::SubmitFrame.as_str()
            );
            assert_eq!(failure.message(), "sink unavailable");
        }
        ProductionOutputRouteSinkFailureMappingResult::Unmapped(_) => {
            panic!("expected mapped route sink failure")
        }
    }
}

#[test]
fn route_adapter_maps_neutral_classification_result() {
    let classification = OutputSubmitErrorClassification::new(
        OutputSubmitOperation::SubmitFrame,
        "writer failed".into(),
    );
    let neutral_result = OutputSubmitErrorClassificationResult::Classified(classification);

    match mapper().map_output_submit_classification_result(neutral_result) {
        ProductionOutputRouteSinkFailureMappingResult::Mapped(failure) => {
            assert_eq!(
                failure.operation(),
                OutputSubmitOperation::SubmitFrame.as_str()
            );
            assert_eq!(failure.message(), "writer failed");
        }
        ProductionOutputRouteSinkFailureMappingResult::Unmapped(_) => {
            panic!("expected mapped route sink failure")
        }
    }
}

#[test]
fn route_adapter_preserves_unmapped_playback_error_by_value() {
    let neutral_result =
        OutputSubmitErrorClassificationResult::Unmapped(PlaybackError::Backend("not sink".into()));

    match mapper().map_output_submit_classification_result(neutral_result) {
        ProductionOutputRouteSinkFailureMappingResult::Unmapped(PlaybackError::Backend(
            message,
        )) => assert_eq!(message, "not sink"),
        ProductionOutputRouteSinkFailureMappingResult::Unmapped(_) => {
            panic!("expected original backend error")
        }
        ProductionOutputRouteSinkFailureMappingResult::Mapped(_) => {
            panic!("expected unmapped playback error")
        }
    }
}

#[test]
fn route_adapter_does_not_map_non_output_error() {
    let classifier = OutputSubmitErrorClassifier;
    let neutral_result = classifier
        .classify_submit_frame_error(PlaybackError::Backend("output text but backend".into()));

    match mapper().map_output_submit_classification_result(neutral_result) {
        ProductionOutputRouteSinkFailureMappingResult::Unmapped(PlaybackError::Backend(
            message,
        )) => assert_eq!(message, "output text but backend"),
        ProductionOutputRouteSinkFailureMappingResult::Unmapped(_) => {
            panic!("expected original backend error")
        }
        ProductionOutputRouteSinkFailureMappingResult::Mapped(_) => {
            panic!("expected unmapped playback error")
        }
    }
}

#[test]
fn route_adapter_result_supports_exhaustive_match() {
    let classification =
        OutputSubmitErrorClassification::new(OutputSubmitOperation::SubmitFrame, "mapped".into());

    let description = match mapper().map_output_submit_classification(classification) {
        ProductionOutputRouteSinkFailureMappingResult::Mapped(_) => "mapped",
        ProductionOutputRouteSinkFailureMappingResult::Unmapped(_) => "unmapped",
    };

    assert_eq!(description, "mapped");
}

#[test]
fn route_adapter_mapper_is_stateless_zst() {
    assert_eq!(
        std::mem::size_of::<ProductionOutputRouteSinkFailureMapper>(),
        0
    );
}
