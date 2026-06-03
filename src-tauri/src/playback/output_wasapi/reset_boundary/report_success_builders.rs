// reset_boundary/report_success_builders.rs
//
// Builder methods for successful reset boundary smoke reports.
//
// These builders handle cases where the smoke probe succeeded
// through the full flow including Stop and Reset.

use super::format_fields::FormatFields;
use super::report::WasapiResetBoundarySmokeReport;

impl WasapiResetBoundarySmokeReport {
    /// Create a success report after successful Stop + Reset.
    pub fn success(
        fields: FormatFields,
        buffer_size_frames: u32,
        padding_frames: u32,
        reset_hresult: i32,
    ) -> Self {
        Self {
            skipped: false,
            skipped_reason: None,

            endpoint_available: true,
            client_activated: true,
            mix_format_available: true,
            initialize_attempted: true,
            initialized_audio_client: true,
            get_service_attempted: true,
            render_client_obtained: true,

            get_buffer_size_attempted: true,
            buffer_size_frames: Some(buffer_size_frames),

            prefill_get_buffer_attempted: true,
            prefill_buffer_obtained: true,
            prefill_release_buffer_attempted: true,
            prefill_buffer_released: true,
            prefill_requested_frames: Some(1),
            prefill_released_frames: Some(1),
            prefill_used_silent_flag: true,

            start_attempted: true,
            started_audio_client: true,
            stop_attempted: true,
            stopped_audio_client: true,

            get_current_padding_attempted: true,
            current_padding_frames: Some(padding_frames),

            reset_attempted: true,
            reset_succeeded: true,
            reset_hresult: Some(reset_hresult),

            // Prohibited fields (all false)
            is_format_supported_called: false,
            loop_executed: false,
            audio_produced: false,
            output_sink_connected: false,
            capability_exposed: false,
            thread_created: false,
            async_runtime_created: false,
            callback_registered: false,
            ring_buffer_created: false,
            decoder_connected: false,
            pipeline_connected: false,

            ..Self::base_report_for_windows()
        }
        .with_format_fields(fields)
    }
}
