use crate::playback::audio_route_pipeline_tap::AudioRouteTapFrameKind;

use super::fixtures::{output_frame, tap};

#[test]
fn tap_accepts_output_frame_by_borrow_and_updates_report() -> Result<(), String> {
    let frame = output_frame(vec![0.0, 0.1, 0.2, 0.3]);
    let mut tap = tap(4).map_err(|error| format!("{error:?}"))?;
    let report = tap.tap_output_frame(&frame);

    assert_eq!(frame.samples.len(), 4);
    assert_eq!(report.frame_kind, AudioRouteTapFrameKind::OutputFrame);
    assert_eq!(report.total_accepted_frames, 2);
    assert_eq!(report.pending_frames, 2);
    assert_eq!(report.input_count, 1);
    assert!(report.last_error.is_none());
    Ok(())
}

#[test]
fn tap_report_records_accepted_frames_without_moving_frame() -> Result<(), String> {
    let frame = output_frame(vec![0.0, 0.1]);
    let mut tap = tap(2).map_err(|error| format!("{error:?}"))?;
    let report = tap.tap_output_frame(&frame);

    assert_eq!(report.total_accepted_frames, 1);
    assert_eq!(frame.position_ms, 100);
    assert_eq!(frame.samples.len(), 2);
    Ok(())
}
