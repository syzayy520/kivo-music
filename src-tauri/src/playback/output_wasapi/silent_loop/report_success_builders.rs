// silent_loop/report_success_builders.rs
//
// Builder methods for successful silent loop smoke reports.
//
// These builders handle cases where the smoke probe succeeded
// through the full silent loop and Stop.

use super::format_fields::FormatFields;
use super::report::WasapiSilentLoopSmokeReport;

/// Loop statistics collected during the silent loop execution.
pub struct LoopStats {
    pub loop_iterations_completed: u32,
    pub zero_available_count: u32,
    pub current_padding_success_count: u32,
    pub first_padding_frames: u32,
    pub last_padding_frames: u32,
    pub min_padding_observed: u32,
    pub max_padding_observed: u32,
    pub last_available_frames: u32,
    pub last_writable_frames: u32,
    pub loop_get_buffer_success_count: u32,
    pub loop_release_buffer_success_count: u32,
}

impl WasapiSilentLoopSmokeReport {
    /// Create a success report after successful silent loop.
    pub fn success(fields: FormatFields, buffer_size_frames: u32, stats: LoopStats) -> Self {
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

            loop_iterations_configured: Some(3),
            loop_iterations_completed: Some(stats.loop_iterations_completed),
            small_frame_count: Some(1),
            zero_available_count: Some(stats.zero_available_count),
            get_current_padding_attempted: true,
            current_padding_success_count: Some(stats.current_padding_success_count),
            first_padding_frames: Some(stats.first_padding_frames),
            last_padding_frames: Some(stats.last_padding_frames),
            min_padding_observed: Some(stats.min_padding_observed),
            max_padding_observed: Some(stats.max_padding_observed),
            last_available_frames: Some(stats.last_available_frames),
            last_writable_frames: Some(stats.last_writable_frames),
            loop_get_buffer_attempted: true,
            loop_get_buffer_success_count: Some(stats.loop_get_buffer_success_count),
            loop_release_buffer_attempted: true,
            loop_release_buffer_success_count: Some(stats.loop_release_buffer_success_count),
            loop_all_releases_silent: true,

            // Prohibited fields (all false)
            is_format_supported_called: false,
            reset_audio_client: false,
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
