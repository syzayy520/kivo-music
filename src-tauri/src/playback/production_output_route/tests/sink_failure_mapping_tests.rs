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

#[test]
fn route_mapper_maps_borrowed_classification_ref() {
    let classification = OutputSubmitErrorClassification::new(
        OutputSubmitOperation::SubmitFrame,
        "borrowed classification".into(),
    );

    match mapper().map_output_submit_classification_ref(&classification) {
        ProductionOutputRouteSinkFailureMappingResult::Mapped(failure) => {
            assert_eq!(failure.operation(), "submit_frame");
            assert_eq!(failure.message(), "borrowed classification");
        }
        ProductionOutputRouteSinkFailureMappingResult::Unmapped(_) => {
            panic!("expected mapped classification")
        }
    }
    assert_eq!(classification.message(), "borrowed classification");
}

#[test]
fn route_mapper_maps_borrowed_classified_output_submit_result_ref() {
    let result =
        OutputSubmitErrorClassificationResult::Classified(OutputSubmitErrorClassification::new(
            OutputSubmitOperation::SubmitFrame,
            "borrowed result".into(),
        ));

    assert!(matches!(
        mapper().map_output_submit_classification_result_ref(&result),
        ProductionOutputRouteSinkFailureMappingResult::Mapped(failure)
            if failure.operation() == "submit_frame" && failure.message() == "borrowed result"
    ));
    assert!(matches!(
        result,
        OutputSubmitErrorClassificationResult::Classified(_)
    ));
}

#[test]
fn route_mapper_maps_borrowed_unmapped_result_without_consuming_result() {
    let result = OutputSubmitErrorClassificationResult::Unmapped(PlaybackError::Backend(
        "borrowed backend".into(),
    ));

    assert!(matches!(
        mapper().map_output_submit_classification_result_ref(&result),
        ProductionOutputRouteSinkFailureMappingResult::Unmapped(
            PlaybackError::Backend(message)
        ) if message == "borrowed backend"
    ));
    assert!(matches!(
        result,
        OutputSubmitErrorClassificationResult::Unmapped(PlaybackError::Backend(ref message))
            if message == "borrowed backend"
    ));
}

#[test]
fn route_mapper_owned_unmapped_result_still_moves_original_error_by_value() {
    let result =
        OutputSubmitErrorClassificationResult::Unmapped(PlaybackError::Queue("owned queue".into()));

    assert!(matches!(
        mapper().map_output_submit_classification_result(result),
        ProductionOutputRouteSinkFailureMappingResult::Unmapped(PlaybackError::Queue(message))
            if message == "owned queue"
    ));
}

#[test]
fn route_mapper_borrowed_unmapped_does_not_rerun_classifier_or_remap_output_error() {
    let result = OutputSubmitErrorClassificationResult::Unmapped(PlaybackError::Output(
        "explicitly unmapped".into(),
    ));

    assert!(matches!(
        mapper().map_output_submit_classification_result_ref(&result),
        ProductionOutputRouteSinkFailureMappingResult::Unmapped(PlaybackError::Output(message))
            if message == "explicitly unmapped"
    ));
}

#[test]
fn route_mapper_preserves_existing_owned_classified_mapping() {
    let result =
        OutputSubmitErrorClassificationResult::Classified(OutputSubmitErrorClassification::new(
            OutputSubmitOperation::SubmitFrame,
            "owned classified".into(),
        ));

    assert!(matches!(
        mapper().map_output_submit_classification_result(result),
        ProductionOutputRouteSinkFailureMappingResult::Mapped(failure)
            if failure.operation() == "submit_frame" && failure.message() == "owned classified"
    ));
}
