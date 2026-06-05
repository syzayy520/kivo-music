use super::id::{OutputThreadRuntimeGeneration, OutputThreadRuntimeId};
use super::report::OutputThreadRuntimeReportSummary;
use super::super::output_thread_state::{OutputThreadReport, OutputThreadStats};

#[test]
fn report_summary_from_thread_report_preserves_stats() {
    let mut stats = OutputThreadStats::default();
    stats.rendered_frames = 500;
    stats.silence_filled_frames = 50;
    stats.consumed_frames = 450;

    let report = OutputThreadReport {
        stats: stats.clone(),
        error: None,
        panicked: false,
    };

    let summary = OutputThreadRuntimeReportSummary::from_thread_report(
        OutputThreadRuntimeId::new(1),
        OutputThreadRuntimeGeneration::new(0),
        report,
    );

    assert_eq!(summary.stats.rendered_frames, 500);
    assert_eq!(summary.stats.silence_filled_frames, 50);
    assert_eq!(summary.stats.consumed_frames, 450);
}

#[test]
fn report_summary_detects_error() {
    let report = OutputThreadReport {
        stats: OutputThreadStats::default(),
        error: Some("device lost".to_string()),
        panicked: false,
    };

    let summary = OutputThreadRuntimeReportSummary::from_thread_report(
        OutputThreadRuntimeId::new(1),
        OutputThreadRuntimeGeneration::new(0),
        report,
    );

    assert!(summary.has_error());
    assert!(!summary.panicked());
}

#[test]
fn report_summary_detects_panic() {
    let report = OutputThreadReport {
        stats: OutputThreadStats::default(),
        error: None,
        panicked: true,
    };

    let summary = OutputThreadRuntimeReportSummary::from_thread_report(
        OutputThreadRuntimeId::new(1),
        OutputThreadRuntimeGeneration::new(0),
        report,
    );

    assert!(summary.has_error());
    assert!(summary.panicked());
}

#[test]
fn rendered_frames_accessor_returns_stats_value() {
    let mut stats = OutputThreadStats::default();
    stats.rendered_frames = 1234;

    let summary = OutputThreadRuntimeReportSummary::new(
        OutputThreadRuntimeId::new(1),
        OutputThreadRuntimeGeneration::new(0),
        stats,
        None,
        false,
    );

    assert_eq!(summary.rendered_frames(), 1234);
}

#[test]
fn silence_frames_accessor_returns_stats_value() {
    let mut stats = OutputThreadStats::default();
    stats.silence_filled_frames = 567;

    let summary = OutputThreadRuntimeReportSummary::new(
        OutputThreadRuntimeId::new(1),
        OutputThreadRuntimeGeneration::new(0),
        stats,
        None,
        false,
    );

    assert_eq!(summary.silence_frames(), 567);
}
