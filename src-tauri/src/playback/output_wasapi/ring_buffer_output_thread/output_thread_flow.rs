// ring_buffer_output_thread/output_thread_flow.rs
//
// WASAPI lifecycle flow for ring buffer output thread smoke.
// COM init → endpoint → activate → format → initialize → getService
// → bufferSize → RingBuffer → read_frames_or_silence → GetBuffer
// → ReleaseBuffer(SILENT) → Start → Padding → Stop → Reset

use super::format_fields::extract_format_fields;
use super::guards::{ComApartment, StartedClientGuard};
use super::output_thread_steps;
use super::report::WasapiRingBufferOutputThreadSmokeReport;
use crate::playback::output_wasapi::ring_buffer::{RingBuffer, RingBufferFormat};

/// Run the full WASAPI lifecycle probe with ring buffer inside the output thread.
pub fn run_ring_buffer_output_thread_flow() -> WasapiRingBufferOutputThreadSmokeReport {
    let mut r = WasapiRingBufferOutputThreadSmokeReport {
        attempted: true,
        ..Default::default()
    };

    // COM initialization
    let _com = match ComApartment::initialize() {
        Ok(c) => { r.com_initialized = true; c }
        Err(e) => { r.error_message = Some(format!("COM init: {e}")); return r; }
    };

    // Device → endpoint → client → format
    let enumerator = match output_thread_steps::create_device_enumerator() {
        Ok(v) => v,
        Err(e) => { r.error_message = Some(e); return r; }
    };
    let endpoint = match output_thread_steps::get_default_render_endpoint(&enumerator) {
        Ok(v) => { r.endpoint_available = true; v }
        Err(e) => { r.error_message = Some(e); return r; }
    };
    let audio_client = match output_thread_steps::activate_audio_client(&endpoint) {
        Ok(v) => { r.client_activated = true; v }
        Err(e) => { r.error_message = Some(e); return r; }
    };
    let guard = match output_thread_steps::get_mix_format(&audio_client) {
        Ok(v) => { r.mix_format_available = true; v }
        Err(e) => { r.error_message = Some(e); return r; }
    };

    let fields = unsafe { extract_format_fields(guard.ptr) };
    r.sample_rate_hz = Some(fields.sample_rate_hz);
    r.channels = Some(fields.channels);
    r.bits_per_sample = Some(fields.bits_per_sample);
    r.block_align = Some(fields.block_align);

    // Initialize → getService → bufferSize
    r.initialize_attempted = true;
    if let Err(e) = output_thread_steps::initialize_shared_client(&audio_client, guard.ptr) {
        r.error_message = Some(format!("Initialize: {e}"));
        return r;
    }
    r.initialized_audio_client = true;

    r.get_service_attempted = true;
    let render_client = match output_thread_steps::get_render_client_service(&audio_client) {
        Ok(v) => { r.render_client_obtained = true; v }
        Err(e) => { r.error_message = Some(format!("GetService: {e}")); return r; }
    };

    r.get_buffer_size_attempted = true;
    let buffer_size = match output_thread_steps::get_buffer_size(&audio_client) {
        Ok(v) => { r.buffer_size_frames = Some(v); v }
        Err(e) => { r.error_message = Some(format!("GetBufferSize: {e}")); return r; }
    };
    if buffer_size == 0 {
        r.error_message = Some("buffer size is zero".into());
        return r;
    }

    // Create ring buffer and read silence from empty buffer
    let rb_fmt = RingBufferFormat {
        sample_rate_hz: fields.sample_rate_hz,
        channels: fields.channels,
        bits_per_sample: fields.bits_per_sample,
        block_align: fields.block_align,
    };
    let mut rb = match RingBuffer::new(rb_fmt, buffer_size) {
        Ok(v) => {
            r.ring_buffer_created = true;
            r.ring_buffer_capacity_frames = Some(v.capacity_frames());
            v
        }
        Err(e) => { r.error_message = Some(format!("RingBuffer: {e:?}")); return r; }
    };

    let frame_bytes = (buffer_size as usize) * (fields.block_align as usize);
    let mut silence = vec![0u8; frame_bytes];
    if let Err(e) = rb.read_frames_or_silence(&mut silence) {
        r.error_message = Some(format!("read_frames_or_silence: {e:?}"));
        rb.close(); r.ring_buffer_closed = true;
        r.apply_ring_buffer_stats(&rb.stats());
        return r;
    }
    r.apply_ring_buffer_stats(&rb.stats());
    r.ring_buffer_available_frames = Some(rb.available_frames());

    // GetBuffer + ReleaseBuffer(SILENT)
    let mut buf = match output_thread_steps::get_buffer_sized(&render_client, buffer_size) {
        Ok(v) => { r.wasapi_buffer_obtained = true; v }
        Err(e) => {
            r.error_message = Some(e);
            rb.close(); r.ring_buffer_closed = true;
            return r;
        }
    };
    r.used_silent_flag = true;
    if let Err(e) = buf.release_silent() {
        r.error_message = Some(e);
        rb.close(); r.ring_buffer_closed = true;
        return r;
    }
    r.wasapi_buffer_released = true;

    // Start → Padding → Stop → Reset
    r.start_attempted = true;
    if let Err(e) = output_thread_steps::start_audio_client(&audio_client) {
        r.error_message = Some(format!("Start: {e}"));
        rb.close(); r.ring_buffer_closed = true;
        return r;
    }
    r.started_audio_client = true;

    let mut stop_guard = StartedClientGuard::new(audio_client.clone());

    r.get_current_padding_attempted = true;
    match output_thread_steps::get_current_padding(&audio_client) {
        Ok(p) => { r.current_padding_frames = Some(p); }
        Err(e) => {
            r.error_message = Some(format!("Padding: {e}"));
            let _ = stop_guard.stop();
            r.stopped_audio_client = stop_guard.is_stopped();
            rb.close(); r.ring_buffer_closed = true;
            return r;
        }
    }

    r.stop_attempted = true;
    if let Err(e) = stop_guard.stop() {
        r.error_message = Some(format!("Stop: {e}"));
        rb.close(); r.ring_buffer_closed = true;
        return r;
    }
    r.stopped_audio_client = true;

    r.reset_attempted = true;
    if let Err((_, e)) = output_thread_steps::reset_audio_client(&audio_client) {
        r.error_message = Some(format!("Reset: {e}"));
        rb.close(); r.ring_buffer_closed = true;
        return r;
    }
    r.reset_succeeded = true;

    rb.close();
    r.ring_buffer_closed = true;
    r.apply_ring_buffer_stats(&rb.stats());
    r.ring_buffer_available_frames = Some(rb.available_frames());
    r
}
