// output_wasapi_root_tests/status_tests.rs
//
// Tests for WasapiOutputSink status reporting.

use crate::playback::output::{OutputSettings, OutputSink};
use crate::playback::output_wasapi::sink::WasapiOutputSink;

#[test]
fn wasapi_scaffold_status_reports_lifecycle_state() {
    let mut sink = WasapiOutputSink::new();

    // Initial status: not open
    let status = sink.status();
    assert!(!status.is_open, "initial status should not be open");
    assert!(!status.is_active, "initial status should not be active");

    // After open: scaffold reports open and active
    sink.open(&OutputSettings::default()).unwrap();
    let status = sink.status();
    assert!(status.is_open, "scaffold status after open should be open");
    assert!(
        status.is_active,
        "scaffold status after open should be active"
    );

    // With device ID set
    let mut settings = OutputSettings::default();
    settings.selected_device_id = Some("fake-device".to_string());
    sink.open(&settings).unwrap();
    let status = sink.status();
    assert!(
        status.is_open,
        "scaffold status with device ID should be open"
    );
    assert_eq!(
        status.active_device_id.as_deref(),
        Some("fake-device"),
        "active_device_id should reflect config"
    );
}
