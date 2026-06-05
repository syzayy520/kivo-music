//! Core drain helper for RingBuffer to render buffer.
//!
//! Provides a pure helper function that can be tested without real WASAPI devices.

use crate::playback::output_wasapi::ring_buffer::buffer::RingBuffer;
use crate::playback::output_wasapi::wasapi_context::{
    WasapiRenderWriteError, WasapiRenderWriteReport,
};

use super::error::WasapiRingBufferDrainError;
use super::report::WasapiRingBufferDrainReport;

/// Drain frames from RingBuffer to render buffer in one shot.
///
/// # Arguments
/// * `ring_buffer` - The RingBuffer to drain from.
/// * `pending_frames` - Mutable reference to pending_frames counter.
/// * `requested_frames` - Maximum frames to drain this call.
/// * `writer` - Closure that writes frames to render buffer.
///
/// # Returns
/// `WasapiRingBufferDrainReport` on success, or error.
///
/// # Safety
/// - Never consumes RingBuffer on writer failure.
/// - Never decrements pending_frames on writer failure.
/// - Checks pending_frames >= peeked_frames before consume.
pub fn drain_ring_buffer_once_with_writer<F>(
    ring_buffer: &mut RingBuffer,
    pending_frames: &mut usize,
    requested_frames: u32,
    writer: F,
) -> Result<WasapiRingBufferDrainReport, WasapiRingBufferDrainError>
where
    F: FnOnce(u32, &[u8]) -> Result<WasapiRenderWriteReport, WasapiRenderWriteError>,
{
    let pending_before = *pending_frames;

    // No-op if requested_frames == 0
    if requested_frames == 0 {
        return Ok(noop_report(0, pending_before));
    }

    // No-op if ring buffer empty
    let available = ring_buffer.available_frames();
    if available == 0 {
        return Ok(noop_report(requested_frames, pending_before));
    }

    // Calculate frames to peek (min of requested and available)
    let frames_to_peek = requested_frames.min(available);
    let format = ring_buffer.format();
    let block_align = format.block_align as usize;
    let bytes_len = frames_to_peek as usize * block_align;

    // Allocate buffer and peek
    let mut bytes = vec![0u8; bytes_len];
    let peeked_frames = ring_buffer.peek_frames(&mut bytes)?;

    if peeked_frames == 0 {
        return Ok(noop_report(requested_frames, pending_before));
    }

    // Only use the bytes we actually peeked
    let peeked_bytes_len = peeked_frames as usize * block_align;
    let bytes_slice = &bytes[..peeked_bytes_len];

    // Call writer
    let write_report = writer(peeked_frames, bytes_slice).map_err(|e| {
        // Writer failed: do not consume, do not decrement pending
        WasapiRingBufferDrainError::RenderWrite(e)
    })?;

    // Verify rendered frames match peeked frames
    if write_report.frames_written != peeked_frames {
        return Err(WasapiRingBufferDrainError::RenderedFrameMismatch {
            peeked: peeked_frames,
            rendered: write_report.frames_written,
        });
    }

    // Check pending_frames >= peeked_frames before consume
    if pending_before < peeked_frames as usize {
        return Err(WasapiRingBufferDrainError::PendingFrameUnderflow {
            pending: pending_before,
            consumed: peeked_frames,
        });
    }

    // Consume from ring buffer
    let consumed_frames = ring_buffer.consume_frames(peeked_frames)?;

    // Decrement pending_frames
    *pending_frames -= consumed_frames as usize;

    Ok(WasapiRingBufferDrainReport {
        requested_frames,
        peeked_frames,
        rendered_frames: write_report.frames_written,
        consumed_frames,
        bytes_rendered: peeked_bytes_len,
        pending_before,
        pending_after: *pending_frames,
    })
}

/// Create a no-op report (no data drained).
fn noop_report(requested_frames: u32, pending_before: usize) -> WasapiRingBufferDrainReport {
    WasapiRingBufferDrainReport {
        requested_frames,
        peeked_frames: 0,
        rendered_frames: 0,
        consumed_frames: 0,
        bytes_rendered: 0,
        pending_before,
        pending_after: pending_before,
    }
}

/// Drain frames from WasapiOutputSink's RingBuffer to its render buffer.
///
/// # Arguments
/// * `sink` - The WasapiOutputSink to drain from.
/// * `requested_frames` - Maximum frames to drain this call.
///
/// # Returns
/// `WasapiRingBufferDrainReport` on success, or error.
///
/// # Behavior
/// - If no ring buffer present, returns no-op report.
/// - If context not open, writer returns NotOpen error, no consume.
/// - Does NOT automatically drain during playback.
/// - Does NOT call IAudioClient::Start.
#[allow(dead_code)] // temporary until P0-074E/P0-074D drain wiring
pub fn drain_wasapi_output_sink_once(
    sink: &mut crate::playback::output_wasapi::sink::WasapiOutputSink,
    requested_frames: u32,
) -> Result<WasapiRingBufferDrainReport, WasapiRingBufferDrainError> {
    let pending_before = sink.runtime.pending_frames;

    // No ring buffer: return no-op
    let ring_buffer = match sink.ring_buffer.as_mut() {
        Some(rb) => rb,
        None => return Ok(noop_report(requested_frames, pending_before)),
    };

    // Create writer closure that uses sink's context
    let context = &sink.context;
    let writer =
        |frames: u32, data: &[u8]| -> Result<WasapiRenderWriteReport, WasapiRenderWriteError> {
            context.write_render_buffer_bytes(frames, data)
        };

    // Call core drain helper
    drain_ring_buffer_once_with_writer(
        ring_buffer,
        &mut sink.runtime.pending_frames,
        requested_frames,
        writer,
    )
}
