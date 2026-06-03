// output_wasapi_root_tests/config_tests.rs
//
// Tests for WasapiOutputSink configuration behavior.

use crate::playback::output::{OutputSettings, OutputSink};
use crate::playback::output_wasapi::sink::WasapiOutputSink;

#[test]
fn wasapi_config_is_created_from_output_settings() {
    let mut settings = OutputSettings::default();
    settings.selected_device_id = Some("test-device".to_string());
    settings.exclusive_mode = true;
    settings.bit_perfect_mode = true;

    let mut sink = WasapiOutputSink::new();
    let _ = sink.open(&settings);

    let config = sink.config();
    assert_eq!(config.selected_device_id.as_deref(), Some("test-device"));
    assert!(config.exclusive_mode);
    assert!(config.bit_perfect_mode);
}
