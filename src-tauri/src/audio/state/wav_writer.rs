use std::{
    fs::File,
    io::{Seek, SeekFrom, Write},
    path::Path,
};

use crate::audio::errors::{AudioError, AudioResult};

#[derive(Debug, Clone, Copy)]
pub struct WavWriterInfo {
    pub sample_rate: u32,
    pub channels: u16,
}

/// Minimal WAV writer for 32-bit float PCM (WAVE_FORMAT_IEEE_FLOAT).
///
/// On little-endian targets (Windows/x86_64), we can write the `f32` slice as raw bytes
/// without per-sample conversion.
pub struct WavFloat32Writer {
    file: File,
    data_bytes: u32,
}

impl WavFloat32Writer {
    pub fn create(path: &Path, info: WavWriterInfo) -> AudioResult<Self> {
        let mut file = File::create(path).map_err(|e| AudioError::io("creating wav file", e))?;
        write_wav_header_placeholder(&mut file, info)?;
        Ok(Self {
            file,
            data_bytes: 0,
        })
    }

    pub fn write_f32_interleaved(&mut self, samples: &[f32]) -> AudioResult<()> {
        if samples.is_empty() {
            return Ok(());
        }

        #[cfg(target_endian = "little")]
        {
            let bytes = unsafe {
                std::slice::from_raw_parts(
                    samples.as_ptr() as *const u8,
                    samples.len().saturating_mul(4),
                )
            };
            self.file
                .write_all(bytes)
                .map_err(|e| AudioError::io("writing wav samples", e))?;

            let add = (samples.len() as u64).saturating_mul(4);
            self.data_bytes = self
                .data_bytes
                .saturating_add(add.min(u32::MAX as u64) as u32);
            Ok(())
        }

        #[cfg(not(target_endian = "little"))]
        {
            // Portable fallback: convert each sample to little-endian bytes.
            let mut bytes = Vec::<u8>::with_capacity(samples.len().saturating_mul(4));
            for s in samples {
                bytes.extend_from_slice(&s.to_le_bytes());
            }
            self.file
                .write_all(&bytes)
                .map_err(|e| AudioError::io("writing wav samples", e))?;

            let add = bytes.len() as u64;
            self.data_bytes = self
                .data_bytes
                .saturating_add(add.min(u32::MAX as u64) as u32);
            Ok(())
        }
    }

    pub fn finalize(&mut self) -> AudioResult<()> {
        // Patch RIFF size (file size - 8) and data chunk size.
        let riff_size = 36u32.saturating_add(self.data_bytes);

        self.file
            .seek(SeekFrom::Start(4))
            .map_err(|e| AudioError::io("seeking wav header", e))?;
        self.file
            .write_all(&riff_size.to_le_bytes())
            .map_err(|e| AudioError::io("patching riff size", e))?;

        self.file
            .seek(SeekFrom::Start(40))
            .map_err(|e| AudioError::io("seeking wav data size", e))?;
        self.file
            .write_all(&self.data_bytes.to_le_bytes())
            .map_err(|e| AudioError::io("patching data size", e))?;

        self.file
            .flush()
            .map_err(|e| AudioError::io("flushing wav", e))?;
        Ok(())
    }
}

fn write_wav_header_placeholder(file: &mut File, info: WavWriterInfo) -> AudioResult<()> {
    file.write_all(b"RIFF")
        .map_err(|e| AudioError::io("writing wav header", e))?;
    file.write_all(&0u32.to_le_bytes())
        .map_err(|e| AudioError::io("writing wav header", e))?;
    file.write_all(b"WAVE")
        .map_err(|e| AudioError::io("writing wav header", e))?;

    // fmt chunk
    file.write_all(b"fmt ")
        .map_err(|e| AudioError::io("writing wav fmt", e))?;
    file.write_all(&16u32.to_le_bytes())
        .map_err(|e| AudioError::io("writing wav fmt", e))?;

    let audio_format: u16 = 3; // IEEE float
    let channels = info.channels.max(1);
    let sample_rate = info.sample_rate.max(1);
    let bits_per_sample: u16 = 32;
    let block_align: u16 = channels.saturating_mul((bits_per_sample / 8).max(1));
    let byte_rate: u32 = sample_rate.saturating_mul(block_align as u32);

    file.write_all(&audio_format.to_le_bytes())
        .map_err(|e| AudioError::io("writing wav fmt", e))?;
    file.write_all(&channels.to_le_bytes())
        .map_err(|e| AudioError::io("writing wav fmt", e))?;
    file.write_all(&sample_rate.to_le_bytes())
        .map_err(|e| AudioError::io("writing wav fmt", e))?;
    file.write_all(&byte_rate.to_le_bytes())
        .map_err(|e| AudioError::io("writing wav fmt", e))?;
    file.write_all(&block_align.to_le_bytes())
        .map_err(|e| AudioError::io("writing wav fmt", e))?;
    file.write_all(&bits_per_sample.to_le_bytes())
        .map_err(|e| AudioError::io("writing wav fmt", e))?;

    // data chunk
    file.write_all(b"data")
        .map_err(|e| AudioError::io("writing wav data header", e))?;
    file.write_all(&0u32.to_le_bytes())
        .map_err(|e| AudioError::io("writing wav data header", e))?;

    Ok(())
}
