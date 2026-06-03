// start_stop_tests/smoke_ignored_tests.rs
//
// Windows-only ignored smoke test for the WASAPI start/stop probe.
// This test only runs with `cargo test -- --ignored` and only performs
// real COM operations when KIVO_WASAPI_START_STOP_SMOKE=1.

use crate::playback::output_wasapi::start_stop::probe_start_stop;

#[cfg(windows)]
#[test]
#[ignore]
fn start_stop_smoke_windows_ignored_opt_in() {
    // This test is #[ignore] and only runs with:
    //   cargo test -- --ignored
    //
    // Even then, it only performs real COM operations if:
    //   KIVO_WASAPI_START_STOP_SMOKE=1
    //
    // If the env is not set, it passes with a skip report.
    // It never panics or hard-fails.
    //
    // It does NOT:
    // - Call IsFormatSupported
    // - Call GetCurrentPadding
    // - Call Reset
    // - Produce sound
    // - Connect OutputSink
    // - Expose PlaybackCapabilities

    let report = probe_start_stop();

    if !report.opt_in_enabled {
        assert!(report.skipped, "should be skipped when env not set");
        assert!(!report.attempted, "should not attempt when env not set");
        assert!(
            !report.start_attempted,
            "should not attempt start when env not set"
        );
        assert!(
            !report.stop_attempted,
            "should not attempt stop when env not set"
        );
        eprintln!("start_stop smoke skipped: env not set");
        return;
    }

    assert!(report.attempted, "should have attempted when env is set");

    if report.skipped {
        assert!(
            report.skipped_reason.is_some(),
            "should have skip reason when skipped"
        );
        assert!(!report.stopped_audio_client, "should not stop when skipped");
        eprintln!(
            "start_stop smoke skipped: {:?} - {:?}",
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
        report.get_service_attempted,
        "get_service should have been attempted on success"
    );
    assert!(
        report.render_client_obtained,
        "render_client should have been obtained on success"
    );
    assert!(
        report.get_buffer_size_attempted,
        "get_buffer_size should have been attempted on success"
    );
    assert!(
        report.buffer_size_frames.is_some(),
        "buffer_size_frames should be Some on success"
    );
    assert!(
        report.buffer_size_frames.unwrap() >= 1,
        "buffer_size_frames should be >= 1 on success"
    );
    assert!(
        report.get_buffer_attempted,
        "get_buffer should have been attempted on success"
    );
    assert!(
        report.buffer_obtained,
        "buffer should have been obtained on success"
    );
    assert!(
        report.release_buffer_attempted,
        "release_buffer should have been attempted on success"
    );
    assert!(
        report.buffer_released,
        "buffer should have been released on success"
    );
    assert!(
        report.used_silent_flag,
        "should have used silent flag on success"
    );

    // Key assertions: requested_frames = 1
    assert_eq!(
        report.requested_frames,
        Some(1),
        "requested_frames should be Some(1)"
    );
    assert_eq!(
        report.released_frames,
        Some(1),
        "released_frames should be Some(1)"
    );

    // Start/Stop assertions
    assert!(
        report.start_attempted,
        "start should have been attempted on success"
    );
    assert!(
        report.started_audio_client,
        "start should have succeeded on success"
    );
    assert!(
        report.stop_attempted,
        "stop should have been attempted on success"
    );
    assert!(
        report.stopped_audio_client,
        "stop should have succeeded on success"
    );
    assert_eq!(
        report.wait_duration_ms,
        Some(0),
        "wait_duration_ms should be Some(0)"
    );

    // Prohibited operations
    assert!(
        !report.is_format_supported_called,
        "is_format_supported_called should always be false"
    );
    assert!(
        !report.get_current_padding_called,
        "get_current_padding_called should always be false"
    );
    assert!(
        !report.reset_audio_client,
        "reset_audio_client should always be false"
    );
    assert!(
        !report.audio_produced,
        "audio_produced should always be false"
    );
    assert!(
        !report.output_sink_connected,
        "output_sink_connected should always be false"
    );
    assert!(
        !report.capability_exposed,
        "capability_exposed should always be false"
    );
    assert!(
        report.skipped_reason.is_none(),
        "should not have skip reason on success"
    );
    assert!(
        report.error_message.is_none(),
        "should not have error message on success"
    );

    eprintln!(
        "start_stop smoke passed: Start+Stop succeeded, buffer_size={} frames",
        report.buffer_size_frames.unwrap()
    );
}
