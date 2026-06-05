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

    // Lazy-create ring buffer if absent
    if ring_buffer.is_none() {
        lazy_create_ring_buffer(&frame, ring_buffer)?;
    }

    // Convert f32 samples to native-endian bytes
    let bytes = f32_samples_to_ne_bytes(&frame.samples);

    // Write to ring buffer
    let rb = ring_buffer.as_mut().unwrap();
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

/// Lazy-create a ring buffer from the frame's stream info.
fn lazy_create_ring_buffer(
    frame: &AudioOutputFrame,
    ring_buffer: &mut Option<RingBuffer>,
) -> PlaybackResult<()> {
    let format = ring_buffer_format_from_stream(&frame.stream)
        .map_err(|e| PlaybackError::UnsupportedFormat(format!("frame format error: {e:?}")))?;
    let rb = RingBuffer::new(format, DEFAULT_CAPACITY_FRAMES)
        .map_err(|e| PlaybackError::Output(format!("ring buffer creation failed: {e:?}")))?;
    *ring_buffer = Some(rb);
    Ok(())
}
