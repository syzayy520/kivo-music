// output_wasapi_root_tests/status_tests.rs
//
// Tests for WasapiOutputSink status reporting.

use crate::playback::output::{OutputSettings, OutputSink};
use crate::playback::output_wasapi::sink::WasapiOutputSink;

#[test]
fn wasapi_stub_status_never_reports_real_device_open() {
    let mut sink = WasapiOutputSink::new();

    // Initial status
    let status = sink.status();
    assert!(!status.is_open, "initial status should not be open");
    assert!(!status.is_active, "initial status should not be active");

    // After open attempt
    let _ = sink.open(&OutputSettings::default());
    let status = sink.status();
    assert!(!status.is_open, "status after open should not be open");
    assert!(!status.is_active, "status after open should not be active");

    // With device ID set
    let mut settings = OutputSettings::default();
    settings.selected_device_id = Some("fake-device".to_string());
    let _ = sink.open(&settings);
    let status = sink.status();
    assert!(!status.is_open, "status with device ID should not be open");
    assert_eq!(
        status.active_device_id.as_deref(),
        Some("fake-device"),
        "active_device_id should reflect config"
    );
}
