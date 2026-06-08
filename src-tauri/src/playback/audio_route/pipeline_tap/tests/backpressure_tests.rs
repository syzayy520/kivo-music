use crate::playback::audio_route_pipeline_tap::AudioRoutePipelineTapErrorKind;

use super::fixtures::{output_frame, tap};

#[test]
fn partial_write_is_recorded_as_side_report() -> Result<(), String> {
    let frame = output_frame(vec![0.0, 0.1, 0.2, 0.3]);
    let mut tap = tap(1).map_err(|error| format!("{error:?}"))?;
    let report = tap.tap_output_frame(&frame);

    assert_eq!(report.total_accepted_frames, 1);
    assert_eq!(report.total_rejected_frames, 1);
    assert_eq!(report.partial_write_count, 1);
    assert_eq!(report.backpressure_count, 1);
    assert!(report.last_error.is_none());
    Ok(())
}

#[test]
fn full_buffer_error_is_non_fatal_side_report() -> Result<(), String> {
    let first = output_frame(vec![0.0, 0.1]);
    let second = output_frame(vec![0.2, 0.3]);
    let mut tap = tap(1).map_err(|error| format!("{error:?}"))?;
    tap.tap_output_frame(&first);
    let report = tap.tap_output_frame(&second);

    assert_eq!(report.total_accepted_frames, 1);
    assert_eq!(report.backpressure_count, 1);
    assert_eq!(
        report.last_error,
        Some(AudioRoutePipelineTapErrorKind::IntegrationFailed)
    );
    Ok(())
}
