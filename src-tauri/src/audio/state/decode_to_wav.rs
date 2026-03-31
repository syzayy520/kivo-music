use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::audio::{
    buffer::PcmRingBuffer,
    decoder_ffmpeg::{header_probe, FfmpegDecoder, FfmpegLoadOptions},
    errors::{AudioError, AudioResult},
    logging::AudioLog,
};

use super::decode_to_wav_utils::{join_writer, spawn_wav_writer_thread, write_chunk_limited};

#[derive(Debug, Clone)]
pub struct DecodeToWavArgs {
    pub input_path: String,
    pub output_path: Option<String>,
    pub start_seconds: Option<f64>,
    pub max_seconds: Option<f64>,
    pub ffmpeg_dir: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DecodeToWavResult {
    pub output_path: String,
    pub ffmpeg_version: String,
    pub sample_rate: u32,
    pub channels: u16,
    pub seconds_written: f64,
    pub frames_written: u64,
}

pub fn run_ffmpeg_decode_to_wav(args: DecodeToWavArgs) -> AudioResult<DecodeToWavResult> {
    let input = PathBuf::from(args.input_path.trim());
    if args.input_path.trim().is_empty() {
        return Err(AudioError::InvalidInput("input_path is empty".to_string()));
    }

    let out = resolve_output_path(&input, args.output_path.as_deref())?;
    let start = args.start_seconds.unwrap_or(0.0).max(0.0);
    let max = args.max_seconds.filter(|v| *v > 0.0);

    let load_opt = FfmpegLoadOptions {
        dll_dir: args.ffmpeg_dir.as_deref().map(PathBuf::from),
    };

    AudioLog::info(&format!(
        "decode_to_wav: in={} out={} start={:.3} max={:?}",
        input.display(),
        out.display(),
        start,
        max,
    ));

    let mut decoder = FfmpegDecoder::open(&input, &load_opt)?;
    let ffmpeg_version = decoder.ffmpeg_version();

    if start > 0.0 {
        decoder.seek_seconds(start)?;
    }

    let sample_rate = header_probe::probe_sample_rate(&input).unwrap_or_else(|| {
        AudioLog::warn("sample rate probe failed; defaulting to 44100");
        44_100
    });

    let max_frames_total: Option<u64> = max.map(|limit_seconds| {
        // Floor to ensure we never write more than requested.
        (limit_seconds * (sample_rate as f64)).floor() as u64
    });

    // Decode first chunk to learn channel count, then start the consumer.
    let first = decoder
        .next_pcm_f32()?
        .ok_or_else(|| AudioError::unsupported("no audio frames decoded"))?;

    let channels = first.channels.max(1);
    let ring = PcmRingBuffer::new((sample_rate as usize) * (channels as usize) * 2);

    let writer_handle = spawn_wav_writer_thread(out.clone(), ring.clone(), sample_rate, channels);

    // Producer: write first chunk then the rest.
    let mut frames_written: u64 = 0;
    let mut seconds_written: f64 = 0.0;

    let hit_limit = write_chunk_limited(
        &ring,
        &first,
        sample_rate,
        channels,
        max_frames_total,
        &mut frames_written,
        &mut seconds_written,
    )?;

    if !hit_limit {
        loop {
            if let Some(max_frames) = max_frames_total {
                if frames_written >= max_frames {
                    break;
                }
            }

            let Some(chunk) = decoder.next_pcm_f32()? else {
                break;
            };
            if chunk.channels != channels {
                AudioLog::warn(
                    "channel count changed mid-stream; continuing with initial channels",
                );
            }
            if write_chunk_limited(
                &ring,
                &chunk,
                sample_rate,
                channels,
                max_frames_total,
                &mut frames_written,
                &mut seconds_written,
            )? {
                break;
            }
        }
    }

    ring.close();
    join_writer(writer_handle)?;

    Ok(DecodeToWavResult {
        output_path: out.to_string_lossy().to_string(),
        ffmpeg_version,
        sample_rate,
        channels,
        seconds_written,
        frames_written,
    })
}

fn resolve_output_path(input: &Path, output_arg: Option<&str>) -> AudioResult<PathBuf> {
    if let Some(p) = output_arg {
        let p = p.trim();
        if p.is_empty() {
            return Err(AudioError::InvalidInput("output_path is empty".to_string()));
        }
        return Ok(PathBuf::from(p));
    }
    let mut out = input.to_path_buf();
    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("decoded");
    out.set_file_name(format!("{stem}.decoded.wav"));
    Ok(out)
}
