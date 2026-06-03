// report_failure_builders.rs
//
// Builder methods for failed padding query smoke reports.
//
// These builders handle cases where the smoke probe attempted some
// WASAPI operations but ultimately failed before or during GetCurrentPadding.

use super::report::WasapiPaddingQuerySmokeReport;

impl WasapiPaddingQuerySmokeReport {
    /// Create a report when endpoint is available but activation failed.
    pub fn endpoint_available_but_activate_failed(error: String) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("audio client activation failed"),
            endpoint_available: true,
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
    }

    /// Create a report when client activated but GetMixFormat failed.
    pub fn client_activated_but_mix_format_failed(error: String) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("mix format unavailable"),
            endpoint_available: true,
            client_activated: true,
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
    }

    /// Create a report when mix format obtained but Initialize failed.
    pub fn mix_format_obtained_but_initialize_failed(
        fields: super::format_fields::FormatFields,
        error: String,
    ) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("initialize failed"),
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }

    /// Create a report when Initialize succeeded but GetService failed.
    pub fn initialized_but_get_service_failed(
        fields: super::format_fields::FormatFields,
        error: String,
    ) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("get service failed"),
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            get_service_attempted: true,
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }

    /// Create a report when GetService succeeded but GetBufferSize failed.
    pub fn render_client_obtained_but_get_buffer_size_failed(
        fields: super::format_fields::FormatFields,
        error: String,
    ) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("get buffer size failed"),
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            get_service_attempted: true,
            render_client_obtained: true,
            get_buffer_size_attempted: true,
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }

    /// Create a report when GetBufferSize returned zero.
    pub fn buffer_size_zero(fields: super::format_fields::FormatFields) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("buffer size is zero"),
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            get_service_attempted: true,
            render_client_obtained: true,
            get_buffer_size_attempted: true,
            buffer_size_frames: Some(0),
            error_message: Some("GetBufferSize returned 0".to_string()),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }

    /// Create a report when GetBuffer failed.
    pub fn get_buffer_failed(
        fields: super::format_fields::FormatFields,
        buffer_size_frames: u32,
        requested_frames: u32,
        error: String,
    ) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("get buffer failed"),
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            get_service_attempted: true,
            render_client_obtained: true,
            get_buffer_size_attempted: true,
            buffer_size_frames: Some(buffer_size_frames),
            get_buffer_attempted: true,
            prefill_requested_frames: Some(requested_frames),
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }

    /// Create a report when ReleaseBuffer failed.
    pub fn release_buffer_failed(
        fields: super::format_fields::FormatFields,
        buffer_size_frames: u32,
        requested_frames: u32,
        error: String,
    ) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("release buffer failed"),
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            get_service_attempted: true,
            render_client_obtained: true,
            get_buffer_size_attempted: true,
            buffer_size_frames: Some(buffer_size_frames),
            get_buffer_attempted: true,
            buffer_obtained: true,
            release_buffer_attempted: true,
            prefill_requested_frames: Some(requested_frames),
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }

    /// Create a report when Start failed.
    pub fn start_failed(
        fields: super::format_fields::FormatFields,
        buffer_size_frames: u32,
        error: String,
    ) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("start failed"),
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            get_service_attempted: true,
            render_client_obtained: true,
            get_buffer_size_attempted: true,
            buffer_size_frames: Some(buffer_size_frames),
            get_buffer_attempted: true,
            buffer_obtained: true,
            release_buffer_attempted: true,
            buffer_released: true,
            prefill_requested_frames: Some(1),
            prefill_released_frames: Some(1),
            prefill_used_silent_flag: true,
            start_attempted: true,
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }

    /// Create a report when GetCurrentPadding failed.
    pub fn get_current_padding_failed(
        fields: super::format_fields::FormatFields,
        buffer_size_frames: u32,
        error: String,
    ) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("get current padding failed"),
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            get_service_attempted: true,
            render_client_obtained: true,
            get_buffer_size_attempted: true,
            buffer_size_frames: Some(buffer_size_frames),
            get_buffer_attempted: true,
            buffer_obtained: true,
            release_buffer_attempted: true,
            buffer_released: true,
            prefill_requested_frames: Some(1),
            prefill_released_frames: Some(1),
            prefill_used_silent_flag: true,
            start_attempted: true,
            started_audio_client: true,
            get_current_padding_attempted: true,
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }

    /// Create a report when Start succeeded but Stop failed.
    pub fn stop_failed(
        fields: super::format_fields::FormatFields,
        buffer_size_frames: u32,
        padding_frames: Option<u32>,
        error: String,
    ) -> Self {
        Self {
            skipped: true,
            skipped_reason: Some("stop failed"),
            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            get_service_attempted: true,
            render_client_obtained: true,
            get_buffer_size_attempted: true,
            buffer_size_frames: Some(buffer_size_frames),
            get_buffer_attempted: true,
            buffer_obtained: true,
            release_buffer_attempted: true,
            buffer_released: true,
            prefill_requested_frames: Some(1),
            prefill_released_frames: Some(1),
            prefill_used_silent_flag: true,
            start_attempted: true,
            started_audio_client: true,
            get_current_padding_attempted: true,
            padding_frames,
            stop_attempted: true,
            error_message: Some(error),
            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }
}
