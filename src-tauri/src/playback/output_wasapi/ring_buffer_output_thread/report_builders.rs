// ring_buffer_output_thread/report_builders.rs
//
// Builder methods for WasapiRingBufferOutputThreadSmokeReport.

use super::report::WasapiRingBufferOutputThreadSmokeReport;
use crate::playback::output_wasapi::ring_buffer::RingBufferStats;

impl WasapiRingBufferOutputThreadSmokeReport {
    pub fn skipped_env_missing() -> Self {
        Self {
            attempted: false,
            skipped: true,
            skipped_reason: Some("env opt-in not enabled"),
            ..Default::default()
        }
    }

    pub fn skipped_non_windows() -> Self {
        Self {
            attempted: false,
            skipped: true,
            skipped_reason: Some("non-windows platform"),
            ..Default::default()
        }
    }

    pub fn scaffold_ready_report() -> Self {
        Self {
            attempted: true,
            skipped: false,
            ring_buffer_created: false,
            output_thread_spawned: false,
            ..Default::default()
        }
    }

    /// Apply ring buffer stats to the report.
    pub fn apply_ring_buffer_stats(&mut self, stats: &RingBufferStats) {
        self.ring_buffer_underrun_count = stats.underrun_count;
        self.ring_buffer_overrun_count = stats.overrun_count;
        self.total_frames_written = stats.total_frames_written;
        self.total_frames_read = stats.total_frames_read;
        self.total_silence_frames_filled = stats.total_silence_frames_filled;
    }

    /// Merge fields from another report into this one.
    ///
    /// Used when the output thread sends a report back via channel.
    pub fn merge_from(&mut self, other: WasapiRingBufferOutputThreadSmokeReport) {
        // Thread lifecycle
        self.thread_report_recv_attempted = true;
        self.thread_report_received = other.thread_report_received;
        self.thread_recv_timeout_ms = other.thread_recv_timeout_ms;
        self.output_thread_join_attempted = other.output_thread_join_attempted;
        self.output_thread_joined = other.output_thread_joined;
        self.output_thread_join_failed = other.output_thread_join_failed;
        self.thread_panic_caught = other.thread_panic_caught;
        self.thread_panic_message = other.thread_panic_message;
        self.thread_recv_timed_out = other.thread_recv_timed_out;
        self.thread_duration_ms = other.thread_duration_ms;

        // WASAPI lifecycle
        self.com_initialized = other.com_initialized;
        self.endpoint_available = other.endpoint_available;
        self.client_activated = other.client_activated;
        self.mix_format_available = other.mix_format_available;
        self.sample_rate_hz = other.sample_rate_hz;
        self.channels = other.channels;
        self.bits_per_sample = other.bits_per_sample;
        self.block_align = other.block_align;
        self.initialize_attempted = other.initialize_attempted;
        self.initialized_audio_client = other.initialized_audio_client;
        self.get_service_attempted = other.get_service_attempted;
        self.render_client_obtained = other.render_client_obtained;
        self.get_buffer_size_attempted = other.get_buffer_size_attempted;
        self.buffer_size_frames = other.buffer_size_frames;
        self.wasapi_buffer_obtained = other.wasapi_buffer_obtained;
        self.wasapi_buffer_released = other.wasapi_buffer_released;
        self.used_silent_flag = other.used_silent_flag;
        self.start_attempted = other.start_attempted;
        self.started_audio_client = other.started_audio_client;
        self.get_current_padding_attempted = other.get_current_padding_attempted;
        self.current_padding_frames = other.current_padding_frames;
        self.stop_attempted = other.stop_attempted;
        self.stopped_audio_client = other.stopped_audio_client;
        self.reset_attempted = other.reset_attempted;
        self.reset_succeeded = other.reset_succeeded;

        // Ring buffer
        self.ring_buffer_created = other.ring_buffer_created;
        self.ring_buffer_capacity_frames = other.ring_buffer_capacity_frames;
        self.ring_buffer_available_frames = other.ring_buffer_available_frames;
        self.ring_buffer_closed = other.ring_buffer_closed;
        self.ring_buffer_underrun_count = other.ring_buffer_underrun_count;
        self.ring_buffer_overrun_count = other.ring_buffer_overrun_count;
        self.total_frames_written = other.total_frames_written;
        self.total_frames_read = other.total_frames_read;
        self.total_silence_frames_filled = other.total_silence_frames_filled;

        // Prohibited (copy from other)
        self.output_sink_connected = other.output_sink_connected;
        self.capability_exposed = other.capability_exposed;
        self.decoder_connected = other.decoder_connected;
        self.pipeline_connected = other.pipeline_connected;
        self.manager_connected = other.manager_connected;
        self.real_pcm_produced = other.real_pcm_produced;
        self.non_silent_data_written = other.non_silent_data_written;
        self.audio_produced = other.audio_produced;
        self.playback_capability_enabled = other.playback_capability_enabled;

        // Error fields (copy from other if present)
        if other.error_message.is_some() {
            self.error_message = other.error_message;
        }
        if other.skipped_reason.is_some() {
            self.skipped_reason = other.skipped_reason;
        }
        self.skipped = other.skipped;
        self.attempted = other.attempted;
    }
}
