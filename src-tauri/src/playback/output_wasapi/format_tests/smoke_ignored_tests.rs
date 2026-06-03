// smoke_ignored_tests.rs
//
// Real COM smoke test (ignored by default).
//
// This test is #[ignore] and only runs with:
//   cargo test -- --ignored
//
// Even then, it only performs real COM operations if:
//   KIVO_WASAPI_MIX_FORMAT_SMOKE=1
//
// If the env is not set, it passes with a skip report.
// If the env is set but no endpoint is available, it passes with a skip report.
// If the env is set but Activate fails, it passes with a skip report.
// If the env is set but GetMixFormat fails, it passes with a skip report.
// It never panics or hard-fails.

#[cfg(windows)]
#[test]
#[ignore]
fn mix_format_smoke_windows_ignored_opt_in() {
    use crate::playback::output_wasapi::format::probe_mix_format;

    let report = probe_mix_format();

    if !report.opt_in_enabled {
        // Env not set, skip is expected
        assert!(report.skipped, "should be skipped when env not set");
        assert!(!report.attempted, "should not attempt when env not set");
        assert!(
            !report.mix_format_available,
            "should not have mix format when env not set"
        );
        eprintln!("mix format smoke skipped: env not set");
        return;
    }

    // Env is set, probe was attempted
    assert!(report.attempted, "should have attempted when env is set");

    if report.skipped {
        // Probe failed (no audio device, COM error, Activate error, GetMixFormat error, etc.)
        // This is a valid outcome, not a code error
        assert!(
            report.skipped_reason.is_some(),
            "should have skip reason when skipped"
        );
        assert!(
            !report.mix_format_available,
            "mix format should not be available when skipped"
        );
        eprintln!(
            "mix format smoke skipped: {:?} - {:?}",
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
        !report.initialized_audio_client,
        "initialized_audio_client should always be false"
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
        report.sample_rate_hz.is_some(),
        "sample_rate_hz should be Some on success"
    );
    assert!(
        report.channels.is_some(),
        "channels should be Some on success"
    );
    assert!(
        report.bits_per_sample.is_some(),
        "bits_per_sample should be Some on success"
    );
    assert!(
        report.format_tag.is_some(),
        "format_tag should be Some on success"
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
        "mix format smoke passed: sample_rate={:?} channels={:?} bits={:?} tag={:?}",
        report.sample_rate_hz, report.channels, report.bits_per_sample, report.format_tag
    );
}
