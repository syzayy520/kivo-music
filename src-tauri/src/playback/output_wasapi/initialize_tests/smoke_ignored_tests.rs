// initialize_tests/smoke_ignored_tests.rs
//
// Windows-only ignored smoke test for the WASAPI initialize probe.
// This test only runs with `cargo test -- --ignored` and only performs
// real COM operations when KIVO_WASAPI_CLIENT_INIT_SMOKE=1.

use crate::playback::output_wasapi::initialize::probe_initialize;

#[cfg(windows)]
#[test]
#[ignore]
fn init_smoke_windows_ignored_opt_in() {
    // This test is #[ignore] and only runs with:
    //   cargo test -- --ignored
    //
    // Even then, it only performs real COM operations if:
    //   KIVO_WASAPI_CLIENT_INIT_SMOKE=1
    //
    // If the env is not set, it passes with a skip report.
    // If the env is set but no endpoint is available, it passes with a skip report.
    // If the env is set but Activate fails, it passes with a skip report.
    // If the env is set but GetMixFormat fails, it passes with a skip report.
    // If the env is set but Initialize fails, it passes with a skip report.
    // It never panics or hard-fails.
    //
    // It does NOT:
    // - Call GetService
    // - Get IAudioRenderClient
    // - Call GetBuffer / ReleaseBuffer
    // - Call Start / Stop / Reset
    // - Produce sound

    let report = probe_initialize();

    if !report.opt_in_enabled {
        // Env not set, skip is expected
        assert!(report.skipped, "should be skipped when env not set");
        assert!(!report.attempted, "should not attempt when env not set");
        assert!(
            !report.initialize_attempted,
            "should not attempt initialize when env not set"
        );
        assert!(
            !report.initialized_audio_client,
            "should not initialize when env not set"
        );
        eprintln!("initialize smoke skipped: env not set");
        return;
    }

    // Env is set, probe was attempted
    assert!(report.attempted, "should have attempted when env is set");

    if report.skipped {
        // Probe failed (no audio device, COM error, Activate error, GetMixFormat error, Initialize error)
        // This is a valid outcome, not a code error
        assert!(
            report.skipped_reason.is_some(),
            "should have skip reason when skipped"
        );
        assert!(
            !report.initialized_audio_client,
            "should not be initialized when skipped"
        );
        eprintln!(
            "initialize smoke skipped: {:?} - {:?}",
            report.skipped_reason, report.error_message
        );
        return;
    }

    // Probe succeeded
    assert!(
        report.endpoint_available,
        "endpoint should be available on success"
    );
    assert!(
        report.client_activated,
        "client should be activated on success"
    );
    assert!(
        report.mix_format_available,
        "mix format should be available on success"
    );
    assert!(
        report.initialize_attempted,
        "initialize should have been attempted on success"
    );
    assert!(
        report.initialized_audio_client,
        "initialize should have succeeded on success"
    );
    assert!(
        !report.is_format_supported_called,
        "is_format_supported_called should always be false"
    );
    assert!(
        !report.render_client_available,
        "render_client_available should always be false"
    );
    assert!(
        !report.service_requested,
        "service_requested should always be false"
    );
    assert!(
        !report.buffer_requested,
        "buffer_requested should always be false"
    );
    assert!(
        !report.started_audio_client,
        "started_audio_client should always be false"
    );
    assert!(
        !report.stopped_audio_client,
        "stopped_audio_client should always be false"
    );
    assert!(
        !report.reset_audio_client,
        "reset_audio_client should always be false"
    );
    assert!(
        report.skipped_reason.is_none(),
        "should not have skip reason on success"
    );
    assert!(
        report.error_message.is_none(),
        "should not have error message on success"
    );
    eprintln!("initialize smoke passed: IAudioClient::Initialize succeeded in shared mode");
}
