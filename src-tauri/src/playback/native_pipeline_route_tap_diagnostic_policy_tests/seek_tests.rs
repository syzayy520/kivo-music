use crate::playback::native_pipeline::NativePipeline;

use super::fixtures::{float_stream, opened, output_frame, policy};

#[test]
fn seek_without_tap_is_a_no_op() -> Result<(), String> {
    let mut policy = policy(2)?;
    let mut pipeline = NativePipeline::new();

    let report = policy
        .reset_on_seek(&mut pipeline)
        .map_err(|error| format!("{error:?}"))?;

    assert!(report.is_none());
    Ok(())
}

#[test]
fn seek_reset_clears_tap_counters_pending_and_error() -> Result<(), String> {
    let (mut policy, mut pipeline) = opened(1)?;
    pipeline.tap_audio_route_output_frame_non_fatal(&output_frame(vec![0.0, 0.1, 0.2, 0.3]));
    pipeline.tap_audio_route_output_frame_non_fatal(&output_frame(vec![0.4, 0.5]));

    let report = policy
        .reset_on_seek(&mut pipeline)
        .map_err(|error| format!("{error:?}"))?
        .ok_or_else(|| "reset report missing".to_string())?;

    assert_eq!(report.input_count, 0);
    assert_eq!(report.pending_frames, 0);
    assert_eq!(report.total_accepted_frames, 0);
    assert_eq!(report.total_rejected_frames, 0);
    assert_eq!(report.backpressure_count, 0);
    assert_eq!(report.partial_write_count, 0);
    assert!(!report.source_closed_seen);
    assert!(report.last_error.is_none());
    assert!(!report.closed);
    Ok(())
}

#[test]
fn seek_reset_keeps_current_tap_attached() -> Result<(), String> {
    let (mut policy, mut pipeline) = opened(2)?;

    policy
        .reset_on_seek(&mut pipeline)
        .map_err(|error| format!("{error:?}"))?;

    assert!(pipeline.audio_route_pipeline_tap_report().is_some());
    assert!(policy.current_stream_is_compatible(&float_stream()));
    assert!(policy.last_detached_report().is_none());
    Ok(())
}
