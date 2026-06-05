use crate::playback::decoder::AudioStreamInfo;
use crate::playback::errors::PlaybackResult;
use crate::playback::output::{AudioOutputFrame, OutputRuntimeStatus, OutputSettings, OutputSink};

use super::config::WasapiOutputConfig;
use super::frame_bridge::{ring_buffer_format_from_stream, FrameBridgeError};
use super::platform::WasapiCompileBoundary;
use super::ring_buffer::{buffer::RingBuffer, errors::RingBufferError};
use super::sink_submission::submit_frame_to_ring_buffer;
use super::status::WasapiOutputStatus;
use super::wasapi_context::WasapiContext;

/// Errors from preparing a ring buffer for a stream.
#[derive(Debug)]
pub enum WasapiRingBufferPrepareError {
    /// Frame bridge format-mapping failed.
    Bridge(FrameBridgeError),
    /// Ring buffer creation failed.
    Buffer(RingBufferError),
}

impl From<FrameBridgeError> for WasapiRingBufferPrepareError {
    fn from(value: FrameBridgeError) -> Self {
        Self::Bridge(value)
    }
}

impl From<RingBufferError> for WasapiRingBufferPrepareError {
    fn from(value: RingBufferError) -> Self {
        Self::Buffer(value)
    }
}

/// WASAPI output sink for the native audio pipeline.
///
/// Manages real WASAPI device lifecycle on Windows:
/// - `open()` initializes COM, enumerates devices, activates IAudioClient,
///   initializes in shared mode, and acquires IAudioRenderClient.
/// - `close()` releases all WASAPI resources.
///
/// **This sink does NOT:**
/// - Write PCM to WASAPI device buffer (local ring buffer only)
/// - Start IAudioClient
/// - Produce audible output
/// - Create output threads
/// - Use RingBuffer for audible output
#[derive(Debug, Default)]
pub struct WasapiOutputSink {
    /// Configuration populated by `open`.
    config: WasapiOutputConfig,
    /// WASAPI-specific metadata.
    status: WasapiOutputStatus,
    /// Generic lifecycle state for `OutputSink` trait contract.
    runtime: OutputRuntimeStatus,
    /// Optional ring buffer owned by this sink.
    ring_buffer: Option<RingBuffer>,
    /// WASAPI device context (real on Windows, empty on other platforms).
    context: WasapiContext,
}
impl WasapiOutputSink {
    /// Create a new WASAPI output sink scaffold.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get current configuration.
    pub fn config(&self) -> &WasapiOutputConfig {
        &self.config
    }

    /// Get WASAPI-specific status.
    pub fn wasapi_status(&self) -> &WasapiOutputStatus {
        &self.status
    }

    /// Get the WASAPI compile boundary for this platform.
    pub fn compile_boundary(&self) -> WasapiCompileBoundary {
        super::platform::wasapi_compile_boundary()
    }

    /// Prepare a ring buffer for the given stream format.
    ///
    /// If a ring buffer already exists, it is closed and dropped before
    /// creating the new one. The ring buffer is NOT populated with data.
    #[allow(dead_code)]
    pub(crate) fn prepare_ring_buffer_for_stream(
        &mut self,
        stream: &AudioStreamInfo,
        capacity_frames: u32,
    ) -> Result<(), WasapiRingBufferPrepareError> {
        if let Some(mut rb) = self.ring_buffer.take() {
            rb.close();
        }
        let format = ring_buffer_format_from_stream(stream)?;
        let rb = RingBuffer::new(format, capacity_frames)?;
        self.ring_buffer = Some(rb);
        Ok(())
    }

    /// Test-only: check if a ring buffer is present.
    #[cfg(test)]
    pub(crate) fn has_ring_buffer(&self) -> bool {
        self.ring_buffer.is_some()
    }

    /// Test-only: get ring buffer available frames if present.
    #[cfg(test)]
    pub(crate) fn ring_buffer_available_frames(&self) -> Option<u32> {
        self.ring_buffer.as_ref().map(|rb| rb.available_frames())
    }

    /// Release stale WASAPI device resources before a new `open()` attempt.
    ///
    /// Closes the WASAPI device context and resets wasapi-specific status
    /// fields. Ring buffer is intentionally preserved — it is an independent
    /// sink-level resource that can survive across open/close cycles.
    /// Does NOT change `runtime` or `config` — those are set by `open()`.
    fn reset_context_for_open_attempt(&mut self) {
        self.context.close();
        self.status.mark_closed();
    }
}

impl OutputSink for WasapiOutputSink {
    fn open(&mut self, settings: &OutputSettings) -> PlaybackResult<OutputRuntimeStatus> {
        self.config = WasapiOutputConfig::from_output_settings(settings);
        self.runtime.active_device_id = settings.selected_device_id.clone();

        // Release old context before attempting new open
        self.reset_context_for_open_attempt();

        // Attempt real WASAPI device open
        match self.context.open() {
            Ok(()) => {
                self.status.mark_opened_with_render_client();
                self.runtime.is_open = true;
                self.runtime.is_active = true;
                self.runtime.last_error = None;
            }
            Err(e) => {
                self.status.mark_open_failed(e.to_message());
                self.runtime.is_open = false;
                self.runtime.is_active = false;
                self.runtime.last_error = Some(e.to_message());
            }
        }

        Ok(self.runtime.clone())
    }

    fn submit_frame(&mut self, frame: AudioOutputFrame) -> PlaybackResult<OutputRuntimeStatus> {
        submit_frame_to_ring_buffer(
            frame,
            &mut self.status,
            &mut self.ring_buffer,
            &mut self.runtime,
        )
    }

    fn pause(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        self.runtime.last_error = None;
        Ok(self.runtime.clone())
    }

    fn resume(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        self.runtime.last_error = None;
        Ok(self.runtime.clone())
    }

    fn flush(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        if let Some(ref mut rb) = self.ring_buffer {
            rb.reset();
        }
        self.runtime.pending_frames = 0;
        self.runtime.last_error = None;
        Ok(self.runtime.clone())
    }

    fn stop(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        self.runtime.is_active = false;
        self.runtime.pending_frames = 0;
        self.runtime.last_error = None;
        Ok(self.runtime.clone())
    }

    fn set_volume(&mut self, level: f32) -> PlaybackResult<OutputRuntimeStatus> {
        self.runtime.controls.volume_level = level.clamp(0.0, 1.0);
        self.runtime.last_error = None;
        Ok(self.runtime.clone())
    }

    fn set_muted(&mut self, muted: bool) -> PlaybackResult<OutputRuntimeStatus> {
        self.runtime.controls.muted = muted;
        self.runtime.last_error = None;
        Ok(self.runtime.clone())
    }

    fn status(&self) -> OutputRuntimeStatus {
        self.runtime.clone()
    }

    fn close(&mut self) -> PlaybackResult<()> {
        // Release ring buffer if present
        if let Some(mut rb) = self.ring_buffer.take() {
            rb.close();
        }

        // Release WASAPI context
        self.context.close();

        // Reset status
        self.status.mark_closed();

        // Reset runtime state
        self.runtime = OutputRuntimeStatus::default();
        self.config = WasapiOutputConfig::default();

        Ok(())
    }
}
