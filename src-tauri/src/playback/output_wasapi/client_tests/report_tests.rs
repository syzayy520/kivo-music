// client_tests/report_tests.rs
//
// Report field semantics tests for the WASAPI client activate smoke boundary.
// Validates that report fields have correct invariants.

use std::env;

use crate::playback::output_wasapi::client::{
    probe_client_activate, WASAPI_CLIENT_ACTIVATE_SMOKE_ENV,
};

#[test]
fn client_activate_smoke_does_not_initialize_audio_client() {
    env::remove_var(WASAPI_CLIENT_ACTIVATE_SMOKE_ENV);

    let report = probe_client_activate();

    assert!(
        !report.initialized_audio_client,
        "initialized_audio_client should always be false"
    );
}

#[test]
fn client_activate_smoke_does_not_get_render_client() {
    env::remove_var(WASAPI_CLIENT_ACTIVATE_SMOKE_ENV);

    let report = probe_client_activate();

    assert!(
        !report.render_client_available,
        "render_client_available should always be false"
    );
}
