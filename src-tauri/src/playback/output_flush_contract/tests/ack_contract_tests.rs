use super::super::ack::{
    OutputFlushAck, OutputFlushCompletedDetails, OutputFlushFailedReason,
    OutputFlushRejectedReason, OutputFlushStaleGenerationDetails,
};

#[test]
fn ack_completed_is_success_with_generation() {
    let ack = OutputFlushAck::Completed(OutputFlushCompletedDetails {
        buffer_frames_cleared: 10,
        sink_discarded_frames: 5,
        generation: Some(42),
    });
    assert!(ack.is_success());
    assert_eq!(ack.generation(), Some(42));
}

#[test]
fn ack_completed_without_generation() {
    let ack = OutputFlushAck::Completed(OutputFlushCompletedDetails {
        buffer_frames_cleared: 10,
        sink_discarded_frames: 5,
        generation: None,
    });
    assert!(ack.is_success());
    assert_eq!(ack.generation(), None);
}

#[test]
fn ack_rejected_is_not_success() {
    let ack = OutputFlushAck::Rejected(OutputFlushRejectedReason::RouteClosed);
    assert!(!ack.is_success());
    assert_eq!(ack.generation(), None);
}

#[test]
fn ack_failed_is_not_success() {
    let ack = OutputFlushAck::Failed(OutputFlushFailedReason::SinkFailure("error".into()));
    assert!(!ack.is_success());
    assert_eq!(ack.generation(), None);
}

#[test]
fn ack_unsupported_is_not_success() {
    let ack = OutputFlushAck::Unsupported;
    assert!(!ack.is_success());
    assert_eq!(ack.generation(), None);
}

#[test]
fn ack_stale_generation_is_not_success() {
    let ack = OutputFlushAck::StaleGeneration(OutputFlushStaleGenerationDetails {
        expected_generation: 10,
        actual_generation: 9,
    });
    assert!(!ack.is_success());
    assert_eq!(ack.generation(), None);
}
