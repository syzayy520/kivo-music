use super::fixtures::{output_frame, tap};

#[test]
fn report_reflects_latest_successful_tap() -> Result<(), String> {
    let first = output_frame(vec![0.0, 0.1]);
    let second = output_frame(vec![0.2, 0.3]);
    let mut tap = tap(4).map_err(|error| format!("{error:?}"))?;
    tap.tap_output_frame(&first);
    tap.tap_output_frame(&second);
    let report = tap.report();

    assert!(report.initialized);
    assert_eq!(report.input_count, 2);
    assert_eq!(report.total_accepted_frames, 2);
    assert_eq!(report.pending_frames, 2);
    Ok(())
}

#[test]
fn close_marks_report_closed() -> Result<(), String> {
    let mut tap = tap(4).map_err(|error| format!("{error:?}"))?;
    let report = tap.close();

    assert!(report.closed);
    assert!(tap.is_closed());
    Ok(())
}
