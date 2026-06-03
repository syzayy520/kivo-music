// report_defaults.rs
//
// Default report values for WasapiRenderClientSmokeReport.
//
// This file provides base constructors that fill all report fields
// with safe defaults (all prohibited ops = false, no format info).

use super::report::{WasapiRenderClientSmokeReport, WASAPI_RENDER_CLIENT_SMOKE_ENV};

impl WasapiRenderClientSmokeReport {
    /// Create a base report with Windows platform defaults.
    ///
    /// All prohibited operations are false, no format info, no error.
    pub(crate) fn base_report_for_windows() -> Self {
        Self {
            platform: "windows",
            opt_in_env: WASAPI_RENDER_CLIENT_SMOKE_ENV,
            opt_in_enabled: true,
            attempted: true,
            skipped: false,
            skipped_reason: None,
            endpoint_available: false,
            client_activated: false,
            mix_format_available: false,
            initialize_attempted: false,
            initialized_audio_client: false,
            get_service_attempted: false,
            render_client_obtained: false,
            is_format_supported_called: false,
            get_buffer_called: false,
            release_buffer_called: false,
            started_audio_client: false,
            stopped_audio_client: false,
            reset_audio_client: false,
            audio_produced: false,
            sample_rate_hz: None,
            channels: None,
            bits_per_sample: None,
            block_align: None,
            avg_bytes_per_sec: None,
            format_tag: None,
            cb_size: None,
            share_mode: "shared",
            stream_flags: 0,
            buffer_duration_hns: 0,
            periodicity_hns: 0,
            error_message: None,
        }
    }

    /// Create a base report with non-Windows platform defaults.
    pub(crate) fn base_report_for_non_windows() -> Self {
        Self {
            platform: "non-windows",
            opt_in_env: WASAPI_RENDER_CLIENT_SMOKE_ENV,
            ..Self::base_report_for_windows()
        }
    }

    /// Apply format fields from GetMixFormat to this report.
    pub(crate) fn with_format_fields(mut self, fields: super::format_fields::FormatFields) -> Self {
        self.sample_rate_hz = Some(fields.sample_rate_hz);
        self.channels = Some(fields.channels);
        self.bits_per_sample = Some(fields.bits_per_sample);
        self.block_align = Some(fields.block_align);
        self.avg_bytes_per_sec = Some(fields.avg_bytes_per_sec);
        self.format_tag = Some(fields.format_tag);
        self.cb_size = Some(fields.cb_size);
        self
    }
}
