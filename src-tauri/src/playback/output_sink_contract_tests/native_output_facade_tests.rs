use super::super::native_output::KivoNativeOutputSink;
use super::super::output::{OutputRuntimeStatus, OutputSink};
use super::fixtures::{null_sink, output_frame, output_settings};

fn assert_same_status(actual: &OutputRuntimeStatus, expected: &OutputRuntimeStatus) {
    assert_eq!(actual.is_open, expected.is_open);
    assert_eq!(actual.is_active, expected.is_active);
    assert_eq!(
        actual.active_device_id.as_deref(),
        expected.active_device_id.as_deref()
    );
    assert_eq!(actual.pending_frames, expected.pending_frames);
    assert_eq!(actual.latency.requested_ms, expected.latency.requested_ms);
    assert_eq!(actual.latency.measured_ms, expected.latency.measured_ms);
    assert_eq!(actual.controls.volume_level, expected.controls.volume_level);
    assert_eq!(actual.controls.muted, expected.controls.muted);
    assert_eq!(actual.gap_count, expected.gap_count);
    assert_eq!(actual.last_error.as_deref(), expected.last_error.as_deref());
}

#[test]
fn native_output_facade_matches_null_sink_submit_contract() {
    let mut native_sink = KivoNativeOutputSink::new();
    let mut null_sink = null_sink();

    let native_status = native_sink
        .submit_frame(output_frame(100))
        .expect("native facade submit should follow current contract");
    let null_status = null_sink
        .submit_frame(output_frame(100))
        .expect("null sink submit should follow current contract");

    assert_same_status(&native_status, &null_status);
    assert_same_status(&native_sink.status(), &null_sink.status());
}

#[test]
fn native_output_facade_matches_null_sink_lifecycle_contract() {
    let mut native_sink = KivoNativeOutputSink::new();
    let mut null_sink = null_sink();
    let settings = output_settings(Some("native-facade-device"));

    assert_same_status(
        &native_sink.open(&settings).expect("native open"),
        &null_sink.open(&settings).expect("null open"),
    );
    assert_same_status(
        &native_sink
            .submit_frame(output_frame(200))
            .expect("native submit"),
        &null_sink
            .submit_frame(output_frame(200))
            .expect("null submit"),
    );
    assert_same_status(
        &native_sink.pause().expect("native pause"),
        &null_sink.pause().expect("null pause"),
    );
    assert_same_status(
        &native_sink.resume().expect("native resume"),
        &null_sink.resume().expect("null resume"),
    );
    assert_same_status(
        &native_sink.flush().expect("native flush"),
        &null_sink.flush().expect("null flush"),
    );
    assert_same_status(
        &native_sink.stop().expect("native stop"),
        &null_sink.stop().expect("null stop"),
    );

    native_sink.close().expect("native close");
    null_sink.close().expect("null close");
    assert_same_status(&native_sink.status(), &null_sink.status());
}
