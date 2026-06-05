//! Buffered submit_frame logic.
//!
//! Writes AudioOutputFrame samples into a local RingBuffer.
//! Does NOT write to WASAPI device buffer.
//! Does NOT call IAudioClient::Start.
//! Does NOT call GetBuffer/ReleaseBuffer.

use crate::playback::errors::{PlaybackError, PlaybackResult};
use crate::playback::output::{AudioOutputFrame, OutputRuntimeStatus};
use crate::playback::output_wasapi::errors::wasapi_unsupported;
use crate::playback::output_wasapi::frame_bridge::ring_buffer_format_from_stream;
use crate::playback::output_wasapi::ring_buffer::buffer::RingBuffer;
use crate::playback::output_wasapi::status::WasapiOutputStatus;

use super::sample_bytes::f32_samples_to_ne_bytes;

/// Default ring buffer capacity for lazy creation (≈93 ms at 44100 Hz).
const DEFAULT_CAPACITY_FRAMES: u32 = 4096;

/// Submit an AudioOutputFrame to a local RingBuffer.
///
/// Guards:
/// - Device must be real-open.
/// - Render client must be acquired.
///
/// Behavior:
/// - Empty frames are a no-op (returns Ok).
/// - If no ring buffer exists, one is lazy-created from the frame's stream.
/// - Samples are converted to native-endian bytes and written.
/// - `submitted_frames` and `pending_frames` are updated on success.
pub(crate) fn submit_frame_to_ring_buffer(
    frame: AudioOutputFrame,
    status: &mut WasapiOutputStatus,
    ring_buffer: &mut Option<RingBuffer>,
    runtime: &mut OutputRuntimeStatus,
) -> PlaybackResult<OutputRuntimeStatus> {
    // Guard: real device must be open
    if !status.is_real_device_open {
        runtime.last_error = Some(wasapi_unsupported("submit_frame"));
        return Err(PlaybackError::UnsupportedOperation(wasapi_unsupported(
            "submit_frame",
        )));
    }

    // Guard: render client must be acquired
    if !status.is_render_client_acquired {
        runtime.last_error = Some(wasapi_unsupported(
            "submit_frame: render client not acquired",
        ));
        return Err(PlaybackError::UnsupportedOperation(wasapi_unsupported(
            "submit_frame: render client not acquired",
        )));
    }

    // Empty frame is a no-op
    if frame.samples.is_empty() {
        runtime.last_error = None;
        return Ok(runtime.clone());
    }

    // Compute expected format from frame stream
    let expected_format = ring_buffer_format_from_stream(&frame.stream).map_err(|e| {
        let msg = format!("frame format error: {e:?}");
        runtime.last_error = Some(msg.clone());
        PlaybackError::UnsupportedFormat(msg)
    })?;

    // Lazy-create ring buffer if absent; validate format if present
    match ring_buffer {
        None => {
            let rb = RingBuffer::new(expected_format, DEFAULT_CAPACITY_FRAMES).map_err(|e| {
                let msg = format!("ring buffer creation failed: {e:?}");
                runtime.last_error = Some(msg.clone());
                PlaybackError::Output(msg)
            })?;
            *ring_buffer = Some(rb);
        }
        Some(rb) => {
            if rb.format() != expected_format {
                let msg = format!(
                    "ring buffer format mismatch: expected {:?}, got {:?}",
                    expected_format,
                    rb.format()
                );
                runtime.last_error = Some(msg.clone());
                return Err(PlaybackError::UnsupportedFormat(msg));
            }
        }
    }

    // Convert f32 samples to native-endian bytes
    let bytes = f32_samples_to_ne_bytes(&frame.samples);

    // Write to ring buffer (safe: guaranteed Some after above logic)
    let Some(rb) = ring_buffer.as_mut() else {
        let msg = "ring buffer unavailable after lazy create".to_string();
        runtime.last_error = Some(msg.clone());
        return Err(PlaybackError::Output(msg));
    };

    match rb.write_frames(&bytes) {
        Ok(written) => {
            status.submitted_frames += written as u64;
            runtime.pending_frames += written as usize;
            runtime.last_error = None;
            Ok(runtime.clone())
        }
        Err(e) => {
            let msg = format!("ring buffer write failed: {e:?}");
            runtime.last_error = Some(msg.clone());
            Err(PlaybackError::Output(msg))
        }
    }
}
