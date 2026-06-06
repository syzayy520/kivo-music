use std::mem;

use crate::playback::errors::PlaybackError;

use super::super::sink_failure_mapping::{
    mapper::ProductionOutputRouteSinkFailureMapper,
    operation::ProductionOutputRouteSinkFailureOperation,
    result::ProductionOutputRouteSinkFailureMappingResult,
};

fn map_submit_frame_error(error: PlaybackError) -> ProductionOutputRouteSinkFailureMappingResult {
    let mapper = ProductionOutputRouteSinkFailureMapper;
    mapper.map_submit_frame_error(error)
}

fn assert_unmapped(
    result: ProductionOutputRouteSinkFailureMappingResult,
    expected_variant: PlaybackError,
    expected_message: &str,
) {
    let actual = match result {
        ProductionOutputRouteSinkFailureMappingResult::Unmapped(error) => error,
        ProductionOutputRouteSinkFailureMappingResult::Mapped(_) => {
            panic!("expected unmapped playback error")
        }
    };

    assert_eq!(
        mem::discriminant(&actual),
        mem::discriminant(&expected_variant)
    );
    let message = match actual {
        PlaybackError::Backend(message)
        | PlaybackError::Path(message)
        | PlaybackError::Queue(message)
        | PlaybackError::Output(message)
        | PlaybackError::UnsupportedFormat(message)
        | PlaybackError::UnsupportedOperation(message)
        | PlaybackError::Playback(message) => message,
    };
    assert_eq!(message, expected_message);
}

#[test]
fn sink_failure_mapping_maps_submit_frame_output_error() {
    let result = map_submit_frame_error(PlaybackError::Output("sink unavailable".into()));

    match result {
        ProductionOutputRouteSinkFailureMappingResult::Mapped(failure) => {
            assert_eq!(
                failure.operation(),
                ProductionOutputRouteSinkFailureOperation::SubmitFrame.as_str()
            );
            assert_eq!(failure.message(), "sink unavailable");
            assert_ne!(failure.message(), "output error: sink unavailable");
        }
        ProductionOutputRouteSinkFailureMappingResult::Unmapped(_) => {
            panic!("expected mapped sink failure")
        }
    }
}

#[test]
fn sink_failure_mapping_operation_is_typed_submit_frame() {
    assert_eq!(
        ProductionOutputRouteSinkFailureOperation::SubmitFrame.as_str(),
        "submit_frame"
    );
}

#[test]
fn sink_failure_mapping_unmaps_backend_error_even_with_output_text() {
    let message = "output error text from backend";
    assert_unmapped(
        map_submit_frame_error(PlaybackError::Backend(message.into())),
        PlaybackError::Backend(String::new()),
        message,
    );
}

#[test]
fn sink_failure_mapping_unmaps_queue_error() {
    assert_unmapped(
        map_submit_frame_error(PlaybackError::Queue("queue rejected".into())),
        PlaybackError::Queue(String::new()),
        "queue rejected",
    );
}

#[test]
fn sink_failure_mapping_unmaps_path_error() {
    assert_unmapped(
        map_submit_frame_error(PlaybackError::Path("missing path".into())),
        PlaybackError::Path(String::new()),
        "missing path",
    );
}

#[test]
fn sink_failure_mapping_unmaps_unsupported_format() {
    assert_unmapped(
        map_submit_frame_error(PlaybackError::UnsupportedFormat("codec".into())),
        PlaybackError::UnsupportedFormat(String::new()),
        "codec",
    );
}

#[test]
fn sink_failure_mapping_unmaps_unsupported_operation() {
    assert_unmapped(
        map_submit_frame_error(PlaybackError::UnsupportedOperation("operation".into())),
        PlaybackError::UnsupportedOperation(String::new()),
        "operation",
    );
}

#[test]
fn sink_failure_mapping_unmaps_playback_error() {
    assert_unmapped(
        map_submit_frame_error(PlaybackError::Playback("playback stopped".into())),
        PlaybackError::Playback(String::new()),
        "playback stopped",
    );
}

#[test]
fn sink_failure_mapping_result_supports_exhaustive_matching() {
    let description = match map_submit_frame_error(PlaybackError::Output("mapped".into())) {
        ProductionOutputRouteSinkFailureMappingResult::Mapped(_) => "mapped",
        ProductionOutputRouteSinkFailureMappingResult::Unmapped(_) => "unmapped",
    };

    assert_eq!(description, "mapped");
}

#[test]
fn sink_failure_mapping_mapper_is_stateless_zst() {
    assert_eq!(
        std::mem::size_of::<ProductionOutputRouteSinkFailureMapper>(),
        0
    );
}
