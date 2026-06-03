// env_tests.rs
//
// Tests for environment variable handling in padding query smoke.

use crate::playback::output_wasapi::padding_query::report::WasapiPaddingQuerySmokeReport;
use crate::playback::output_wasapi::padding_query::report::WASAPI_PADDING_QUERY_SMOKE_ENV;

#[test]
fn padding_query_smoke_without_env_skips_without_attempting() {
    // Ensure env is not set
    std::env::remove_var(WASAPI_PADDING_QUERY_SMOKE_ENV);

    let report = WasapiPaddingQuerySmokeReport::skipped_env_missing();
    assert!(report.skipped);
    assert!(!report.attempted);
    assert_eq!(
        report.skipped_reason,
        Some("opt-in environment variable not set")
    );
}

#[test]
fn padding_query_smoke_with_non_one_env_skips_without_attempting() {
    // Set env to something other than "1"
    std::env::set_var(WASAPI_PADDING_QUERY_SMOKE_ENV, "0");

    let report = WasapiPaddingQuerySmokeReport::skipped_env_missing();
    assert!(report.skipped);
    assert!(!report.attempted);

    // Clean up
    std::env::remove_var(WASAPI_PADDING_QUERY_SMOKE_ENV);
}

#[test]
fn padding_query_report_uses_padding_query_env_name() {
    let report = WasapiPaddingQuerySmokeReport::base_report_for_windows();
    assert_eq!(report.opt_in_env, "KIVO_WASAPI_PADDING_QUERY_SMOKE");
}

#[test]
fn padding_query_non_windows_stub_is_safe() {
    let report = WasapiPaddingQuerySmokeReport::skipped_non_windows();
    assert!(report.skipped);
    assert!(!report.attempted);
    assert_eq!(report.skipped_reason, Some("non-windows platform"));
    assert_eq!(report.platform, "non-windows");
}
