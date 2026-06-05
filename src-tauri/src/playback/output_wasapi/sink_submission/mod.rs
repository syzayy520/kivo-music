//! Local buffered submit_frame path for WasapiOutputSink.
//!
//! Converts AudioOutputFrame samples to bytes and writes them into
//! an in-memory RingBuffer. Does NOT write to WASAPI device buffer,
//! does NOT call IAudioClient::Start, does NOT call GetBuffer/ReleaseBuffer.

mod buffered;
mod sample_bytes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_format;

pub(crate) use buffered::submit_frame_to_ring_buffer;
