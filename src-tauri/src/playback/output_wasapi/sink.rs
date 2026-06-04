use crate::playback::decoder::AudioStreamInfo;
use crate::playback::errors::{PlaybackError, PlaybackResult};
use crate::playback::output::{AudioOutputFrame, OutputRuntimeStatus, OutputSettings, OutputSink};

use super::config::WasapiOutputConfig;
use super::errors::wasapi_unsupported;
use super::frame_bridge::{ring_buffer_format_from_stream, FrameBridgeError};
use super::platform::WasapiCompileBoundary;
use super::ring_buffer::buffer::RingBuffer;
use super::ring_buffer::errors::RingBufferError;
use super::status::WasapiOutputStatus;

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

/// WASAPI output sink scaffold for the native audio pipeline.
///
/// This is a lifecycle-only scaffold that does NOT:
/// - Open real audio devices
/// - Produce audible output
/// - Create output threads
/// - Use Windows audio APIs
/// - Write to ring buffers
///
/// State-transition methods (`open`, `stop`, `flush`, `close`, `set_volume`,
/// `set_muted`) succeed and update internal lifecycle state.
/// `submit_frame` returns typed `UnsupportedOperation` — this sink cannot
/// accept audio frames until a real WASAPI backend is wired in.
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
    pub fn prepare_ring_buffer_for_stream(
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
}

impl OutputSink for WasapiOutputSink {
    fn open(&mut self, settings: &OutputSettings) -> PlaybackResult<OutputRuntimeStatus> {
        self.config = WasapiOutputConfig::from_output_settings(settings);
        self.status.is_open_attempted = true;
        self.status.is_real_device_open = false;
        self.runtime.active_device_id = settings.selected_device_id.clone();
        self.runtime.is_open = true;
        self.runtime.is_active = true;
        self.runtime.last_error = None;
        Ok(self.runtime.clone())
    }

    fn submit_frame(&mut self, _frame: AudioOutputFrame) -> PlaybackResult<OutputRuntimeStatus> {
        self.runtime.last_error = Some(wasapi_unsupported("submit_frame"));
        Err(PlaybackError::UnsupportedOperation(wasapi_unsupported(
            "submit_frame",
        )))
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
        if let Some(mut rb) = self.ring_buffer.take() {
            rb.close();
        }
        self.runtime = OutputRuntimeStatus::default();
        self.status = WasapiOutputStatus::default();
        self.config = WasapiOutputConfig::default();
        Ok(())
    }
}
