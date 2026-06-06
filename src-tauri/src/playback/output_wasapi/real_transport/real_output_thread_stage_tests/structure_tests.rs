const THREAD: &str = include_str!("../thread.rs");
const THREAD_STAGES: &str = include_str!("../thread_stages.rs");
const CONFIG: &str = include_str!("../thread_stages/config.rs");
const ERROR_MAPPING: &str = include_str!("../thread_stages/error_mapping.rs");
const OUTCOMES: &str = include_str!("../thread_stages/outcomes.rs");
const REPORT_MAPPING: &str = include_str!("../thread_stages/report_mapping.rs");
const RUNNER: &str = include_str!("../thread_stages/runner.rs");
const VALIDATION: &str = include_str!("../thread_stages/validation.rs");

#[test]
fn production_stage_files_remain_below_line_limit() {
    let files = [
        ("thread.rs", THREAD),
        ("thread_stages.rs", THREAD_STAGES),
        ("config.rs", CONFIG),
        ("error_mapping.rs", ERROR_MAPPING),
        ("outcomes.rs", OUTCOMES),
        ("report_mapping.rs", REPORT_MAPPING),
        ("runner.rs", RUNNER),
        ("validation.rs", VALIDATION),
    ];

    for (name, source) in files {
        assert!(source.lines().count() <= 220, "{name} exceeded 220 lines");
    }
}

#[test]
fn orchestration_does_not_accept_or_build_an_external_audio_source() {
    let orchestration = format!("{THREAD}\n{THREAD_STAGES}\n{CONFIG}\n{RUNNER}");
    for forbidden in [
        "RingBuffer::new",
        "NativePipeline",
        "OutputSink",
        "track_id",
        "file_path",
        "std::thread::sleep",
        "Duration::",
        "Instant::",
    ] {
        assert!(
            !orchestration.contains(forbidden),
            "found forbidden orchestration dependency: {forbidden}"
        );
    }
}

#[test]
fn boundary_error_is_resolved_before_missing_worker_report() {
    let boundary_index = OUTCOMES
        .find("let render_ring_buffer_boundary_outcome")
        .unwrap();
    let worker_index = OUTCOMES.find("WorkerDidNotReport").unwrap();

    assert!(boundary_index < worker_index);
}
