use crate::playback::audio_route_pipeline_tap::AudioRoutePipelineTapErrorKind;

use super::fixtures::{output_frame, tap};

#[test]
fn closed_tap_failure_is_recorded_without_panic() -> Result<(), String> {
    let frame = output_frame(vec![0.0, 0.1]);
    let mut tap = tap(2).map_err(|error| format!("{error:?}"))?;
    tap.close();
    let report = tap.tap_output_frame(&frame);

    assert!(report.closed);
    assert_eq!(
        report.last_error,
        Some(AudioRoutePipelineTapErrorKind::IntegrationClosed)
    );
    assert_eq!(frame.samples.len(), 2);
    Ok(())
}

#[test]
fn reset_after_success_clears_side_error_state() -> Result<(), String> {
    let frame = output_frame(vec![0.0, 0.1]);
    let mut tap = tap(2).map_err(|error| format!("{error:?}"))?;
    tap.tap_output_frame(&frame);
    let report = tap.reset().map_err(|error| format!("{error:?}"))?;

    assert_eq!(report.input_count, 0);
    assert_eq!(report.pending_frames, 0);
    assert!(report.last_error.is_none());
    Ok(())
}
