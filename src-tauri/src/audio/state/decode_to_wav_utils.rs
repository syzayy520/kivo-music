use std::{
    path::PathBuf,
    thread::{self, JoinHandle},
    time::Instant,
};

use crate::audio::{
    buffer::PcmRingBuffer,
    decoder_ffmpeg::PcmChunkF32,
    errors::{AudioError, AudioResult},
    logging::AudioLog,
};

use super::wav_writer::{WavFloat32Writer, WavWriterInfo};

const WRITE_CHUNK_FRAMES: usize = 4096;

pub fn spawn_wav_writer_thread(
    out_path: PathBuf,
    ring: PcmRingBuffer,
    sample_rate: u32,
    channels: u16,
) -> JoinHandle<AudioResult<()>> {
    thread::spawn(move || {
        let t0 = Instant::now();
        let info = WavWriterInfo {
            sample_rate,
            channels,
        };
        let mut w = WavFloat32Writer::create(out_path.as_path(), info)?;

        let chunk_samples = WRITE_CHUNK_FRAMES.saturating_mul(channels as usize).max(1);

        let mut scratch: Vec<f32> = vec![0.0; chunk_samples];
        loop {
            let n = ring.read_blocking(&mut scratch)?;
            if n == 0 {
                break;
            }
            w.write_f32_interleaved(&scratch[..n])?;
        }

        w.finalize()?;
        AudioLog::info(&format!(
            "wav_writer: done in {} ms",
            t0.elapsed().as_millis()
        ));
        Ok(())
    })
}

pub fn join_writer(handle: JoinHandle<AudioResult<()>>) -> AudioResult<()> {
    match handle.join() {
        Ok(r) => r,
        Err(_) => Err(AudioError::ThreadPanic("wav writer thread panicked")),
    }
}

/// Write a decoded PCM chunk into the ring buffer, respecting `max_frames_total` if provided.
///
/// Returns `Ok(true)` if the caller should stop decoding (max reached).
pub fn write_chunk_limited(
    ring: &PcmRingBuffer,
    chunk: &PcmChunkF32,
    sample_rate_hz: u32,
    channels: u16,
    max_frames_total: Option<u64>,
    frames_written: &mut u64,
    seconds_written: &mut f64,
) -> AudioResult<bool> {
    let ch = channels as usize;
    if ch == 0 {
        return Err(AudioError::InvalidInput("channels=0".into()));
    }
    if chunk.channels != channels {
        return Err(AudioError::InvalidInput(format!(
            "decoder channels changed: {} -> {}",
            channels, chunk.channels
        )));
    }

    let frames_in_chunk = (chunk.data.len() / ch) as u64;

    // Determine how many frames to write from this chunk.
    let frames_to_write = if let Some(max_frames) = max_frames_total {
        if *frames_written >= max_frames {
            return Ok(true);
        }
        let remain = max_frames.saturating_sub(*frames_written);
        frames_in_chunk.min(remain)
    } else {
        frames_in_chunk
    };

    if frames_to_write == 0 {
        return Ok(true);
    }

    let samples_to_write = (frames_to_write as usize)
        .saturating_mul(ch)
        .min(chunk.data.len());

    ring.write_blocking(&chunk.data[..samples_to_write])?;

    *frames_written = frames_written.saturating_add(frames_to_write);
    if sample_rate_hz > 0 {
        *seconds_written = (*frames_written as f64) / (sample_rate_hz as f64);
    }

    if let Some(max_frames) = max_frames_total {
        Ok(*frames_written >= max_frames)
    } else {
        Ok(false)
    }
}
