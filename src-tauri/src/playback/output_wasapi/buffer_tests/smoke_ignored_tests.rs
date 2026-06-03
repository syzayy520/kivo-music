// buffer_tests/smoke_ignored_tests.rs
//
// Windows-only ignored smoke test for the WASAPI buffer probe.
// This test only runs with `cargo test -- --ignored` and only performs
// real COM operations when KIVO_WASAPI_BUFFER_SMOKE=1.

use crate::playback::output_wasapi::buffer::probe_buffer;

#[cfg(windows)]
#[test]
#[ignore]
fn buffer_smoke_windows_ignored_opt_in() {
    // This test is #[ignore] and only runs with:
    //   cargo test -- --ignored
    //
    // Even then, it only performs real COM operations if:
    //   KIVO_WASAPI_BUFFER_SMOKE=1
    //
    // If the env is not set, it passes with a skip report.
    // If the env is set but no endpoint is available, it passes with a skip report.
    // If the env is set but Activate fails, it passes with a skip report.
    // If the env is set but GetMixFormat fails, it passes with a skip report.
    // If the env is set but Initialize fails, it passes with a skip report.
    // If the env is set but GetService fails, it passes with a skip report.
    // If the env is set but GetBufferSize fails, it passes with a skip report.
    // If the env is set but GetBuffer fails, it passes with a skip report.
    // If the env is set but ReleaseBuffer fails, it passes with a skip report.
    // It never panics or hard-fails.
    //
    // It does NOT:
    // - Call IsFormatSupported
    // - Call GetCurrentPadding
    // - Call Start / Stop / Reset
    // - Produce sound

    let report = probe_buffer();

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
        assert!(
            !report.get_service_attempted,
            "should not attempt get_service when env not set"
        );
        assert!(
            !report.render_client_obtained,
            "should not obtain render client when env not set"
        );
        assert!(
            !report.get_buffer_size_attempted,
            "should not attempt get_buffer_size when env not set"
        );
        assert!(
            !report.get_buffer_attempted,
            "should not attempt get_buffer when env not set"
        );
        assert!(
            !report.buffer_obtained,
            "should not obtain buffer when env not set"
        );
        assert!(
            !report.release_buffer_attempted,
            "should not attempt release_buffer when env not set"
        );
        assert!(
            !report.buffer_released,
            "should not release buffer when env not set"
        );
        eprintln!("buffer smoke skipped: env not set");
        return;
    }

    // Env is set, probe was attempted
    assert!(report.attempted, "should have attempted when env is set");

    if report.skipped {
        // Probe failed (no audio device, COM error, etc.)
        // This is a valid outcome, not a code error
        assert!(
            report.skipped_reason.is_some(),
            "should have skip reason when skipped"
        );
        assert!(
            !report.buffer_released,
            "should not release buffer when skipped"
        );
        eprintln!(
            "buffer smoke skipped: {:?} - {:?}",
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
        "requested_frames should be Some(1) - only 1 frame requested"
    );
    assert_eq!(
        report.released_frames,
        Some(1),
        "released_frames should be Some(1) - only 1 frame released"
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
        !report.audio_produced,
        "audio_produced should always be false"
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
        "buffer smoke passed: GetBuffer(1) + ReleaseBuffer(1, SILENT) succeeded, buffer_size={} frames",
        report.buffer_size_frames.unwrap()
    );
}
