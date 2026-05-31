use std::fs::File;
use std::io::BufReader;

use hound::WavReader;

use crate::playback::decoder::{AudioDecoder, AudioStreamInfo, DecodedAudioFrame};
use crate::playback::decoders::wav_format::{map_sample_kind, WavSampleKind};
use crate::playback::decoders::wav_samples::normalize_int_sample;
use crate::playback::errors::{PlaybackError, PlaybackResult};

const WAV_FRAME_CHUNK_SIZE: usize = 1024;

#[derive(Default)]
pub struct WavDecoder {
    reader: Option<WavReader<BufReader<File>>>,
    stream: Option<AudioStreamInfo>,
    sample_kind: Option<WavSampleKind>,
    samples_read: u64,
}

impl WavDecoder {
    fn ensure_open(
        &mut self,
    ) -> PlaybackResult<(
        &mut WavReader<BufReader<File>>,
        AudioStreamInfo,
        WavSampleKind,
    )> {
        let reader = self
            .reader
            .as_mut()
            .ok_or_else(|| PlaybackError::Backend("decoder is not open".to_string()))?;
        let stream = self
            .stream
            .clone()
            .ok_or_else(|| PlaybackError::Backend("decoder stream is missing".to_string()))?;
        let sample_kind = self
            .sample_kind
            .clone()
            .ok_or_else(|| PlaybackError::Backend("decoder sample kind is missing".to_string()))?;

        Ok((reader, stream, sample_kind))
    }
}

impl AudioDecoder for WavDecoder {
    fn open(&mut self, path: &str) -> PlaybackResult<AudioStreamInfo> {
        let reader =
            WavReader::open(path).map_err(|error| PlaybackError::Backend(error.to_string()))?;
        let spec = reader.spec();
        let (sample_kind, sample_format) =
            map_sample_kind(spec.sample_format, spec.bits_per_sample)?;

        let stream = AudioStreamInfo {
            sample_rate_hz: spec.sample_rate,
            channels: spec.channels,
            sample_format,
        };

        self.reader = Some(reader);
        self.stream = Some(stream.clone());
        self.sample_kind = Some(sample_kind);
        self.samples_read = 0;

        Ok(stream)
    }

    fn next_frame(&mut self) -> PlaybackResult<Option<DecodedAudioFrame>> {
        let start_samples = self.samples_read;
        let (reader, stream, sample_kind) = self.ensure_open()?;
        let channels = stream.channels as usize;
        let target_samples = WAV_FRAME_CHUNK_SIZE.saturating_mul(channels);
        let mut samples = Vec::with_capacity(target_samples);

        match sample_kind {
            WavSampleKind::Float32 => {
                let mut values = reader.samples::<f32>();
                for _ in 0..target_samples {
                    match values.next() {
                        Some(Ok(value)) => samples.push(value.clamp(-1.0, 1.0)),
                        Some(Err(error)) => return Err(PlaybackError::Backend(error.to_string())),
                        None => break,
                    }
                }
            }
            WavSampleKind::Int16 => {
                let mut values = reader.samples::<i16>();
                for _ in 0..target_samples {
                    match values.next() {
                        Some(Ok(value)) => samples.push(normalize_int_sample(value as i32, 16)),
                        Some(Err(error)) => return Err(PlaybackError::Backend(error.to_string())),
                        None => break,
                    }
                }
            }
            WavSampleKind::Int24 => {
                let mut values = reader.samples::<i32>();
                for _ in 0..target_samples {
                    match values.next() {
                        Some(Ok(value)) => samples.push(normalize_int_sample(value, 24)),
                        Some(Err(error)) => return Err(PlaybackError::Backend(error.to_string())),
                        None => break,
                    }
                }
            }
            WavSampleKind::Int32 => {
                let mut values = reader.samples::<i32>();
                for _ in 0..target_samples {
                    match values.next() {
                        Some(Ok(value)) => samples.push(normalize_int_sample(value, 32)),
                        Some(Err(error)) => return Err(PlaybackError::Backend(error.to_string())),
                        None => break,
                    }
                }
            }
        }

        if samples.is_empty() {
            return Ok(None);
        }

        self.samples_read += samples.len() as u64;
        let position_ms = start_samples
            .saturating_mul(1000)
            .saturating_div(stream.channels as u64)
            .saturating_div(stream.sample_rate_hz as u64);

        Ok(Some(DecodedAudioFrame {
            stream,
            position_ms,
            samples,
        }))
    }

    fn seek(&mut self, position_ms: u64) -> PlaybackResult<()> {
        let (reader, stream, _) = self.ensure_open()?;
        let sample_index = position_ms
            .saturating_mul(stream.sample_rate_hz as u64)
            .saturating_div(1000);

        let target = u32::try_from(sample_index)
            .map_err(|_| PlaybackError::Backend("seek target exceeds wav range".to_string()))?;

        reader
            .seek(target)
            .map_err(|error| PlaybackError::Backend(error.to_string()))?;

        self.samples_read = sample_index.saturating_mul(stream.channels as u64);
        Ok(())
    }

    fn close(&mut self) -> PlaybackResult<()> {
        self.reader = None;
        self.stream = None;
        self.sample_kind = None;
        self.samples_read = 0;
        Ok(())
    }
}
