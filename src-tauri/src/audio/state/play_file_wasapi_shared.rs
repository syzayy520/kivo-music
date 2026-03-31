use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicU64},
        mpsc, Arc,
    },
    time::{Duration, Instant},
};

use serde::Serialize;

#[cfg(windows)]
use crate::audio::output_wasapi::pack::pack_samples_into;

use crate::audio::{
    decoder_ffmpeg::FfmpegLoadOptions,
    errors::{AudioError, AudioResult},
    logging::AudioLog,
};

#[path = "play_file_wasapi_shared_consumer.rs"]
mod consumer;
#[path = "play_file_wasapi_shared_producer.rs"]
mod producer;
#[path = "play_file_wasapi_shared_runtime.rs"]
mod runtime;
#[path = "play_file_wasapi_shared_validation.rs"]
mod validation;

use consumer::{run_consumer_loop, ConsumerLoopConfig};
use producer::{spawn_producer, ProducerConfig, ProducerMsg};
use runtime::{
    build_play_file_result, init_wasapi_runtime, prime_and_start_stream, recv_producer_meta,
    PlayFileResultParts,
};
use validation::{
    create_diag_log_path, validate_input_path, validate_max_seconds_or_default,
    validate_start_seconds,
};

#[derive(Debug, Clone)]
pub struct PlayFileArgs {
    pub input_path: String,
    pub start_seconds: Option<f64>,
    pub max_seconds: Option<f64>,
    pub ffmpeg_dir: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayFileResult {
    pub input_path: String,
    pub device_name: String,
    pub ffmpeg_version: String,
    pub input_sample_rate_hz: u32,
    pub input_channels: u16,
    pub device_sample_rate_hz: u32,
    pub device_channels: u16,
    pub seconds_played: f64,
    pub frames_played: u64,
    pub underrun_events: u64,
    pub diag_log_path: Option<String>,
}

pub fn run_ffmpeg_play_file_wasapi_shared(args: PlayFileArgs) -> AudioResult<PlayFileResult> {
    #[cfg(not(windows))]
    {
        let _ = args;
        return Err(AudioError::unsupported(
            "wasapi shared playback is Windows-only",
        ));
    }

    #[cfg(windows)]
    {
        run_ffmpeg_play_file_wasapi_shared_windows(args)
    }
}

#[cfg(windows)]
fn run_ffmpeg_play_file_wasapi_shared_windows(args: PlayFileArgs) -> AudioResult<PlayFileResult> {
    let input = validate_input_path(&args.input_path)?;
    let start = validate_start_seconds(args.start_seconds)?;
    let max = validate_max_seconds_or_default(args.max_seconds)?;

    let load_opt = FfmpegLoadOptions {
        dll_dir: args.ffmpeg_dir.as_deref().map(PathBuf::from),
    };

    let mut runtime = init_wasapi_runtime()?;
    let device_name = runtime.device_name.clone();
    let device_sr = runtime.device_sr;
    let device_ch = runtime.device_ch;
    let bits_per_sample = runtime.bits_per_sample;
    let valid_bits = runtime.valid_bits;
    let block_align = runtime.block_align;
    let sample_type = runtime.sample_type;
    let buffer_frames = runtime.buffer_frames;

    // Producer metadata channel.
    let (meta_tx, meta_rx) = mpsc::sync_channel::<ProducerMsg>(1);

    // ringbuf SPSC for interleaved f32 at device sample rate/channels.
    let ring_capacity_samples = (device_sr as usize)
        .saturating_mul(device_ch as usize)
        .saturating_mul(2)
        .max(1);
    let (prod, mut cons) = crate::audio::ring::make_f32_spsc(ring_capacity_samples);

    // diag log (best-effort).
    let diag_log_path = create_diag_log_path();
    if let Some(p) = diag_log_path.as_ref() {
        AudioLog::info(&format!("play_file: diag_log={}", p.display()));
    }

    let done = Arc::new(AtomicBool::new(false));
    let underrun_events = Arc::new(AtomicU64::new(0));

    let producer_join = spawn_producer(
        ProducerConfig {
            input_path: input.clone(),
            start,
            max,
            load_opt,
            device_sr,
            device_ch,
            diag_log_path: diag_log_path.clone(),
            done: done.clone(),
            meta_tx: meta_tx.clone(),
        },
        prod,
    );

    // Fail fast if producer reports a fatal issue (missing DLL / unsupported input).
    let (meta, producer_join) = recv_producer_meta(&meta_rx, producer_join, &input, device_sr)?;

    // Prime endpoint buffer with silence.
    prime_and_start_stream(&mut runtime)?;

    let started = Instant::now();
    let deadline = started + Duration::from_secs_f64(max.max(0.1));

    let mut packed: Vec<u8> = Vec::with_capacity(
        buffer_frames
            .max(1)
            .saturating_mul(device_ch as usize)
            .saturating_mul(block_align),
    );
    let mut wait_for_event = || {
        let _ = runtime.handle.wait_for_event(250);
    };
    let mut get_available_frames = || {
        runtime
            .audio_client
            .get_available_space_in_frames()
            .map(|v| v as usize)
            .map_err(|e| {
                AudioError::unsupported(format!("wasapi: get available frames failed: {e}"))
            })
    };
    let mut render_samples = |available_frames, slice: &[f32]| {
        pack_samples_into(&mut packed, slice, sample_type, bits_per_sample, valid_bits);
        runtime
            .render_client
            .write_to_device(available_frames, &packed, None)
            .map_err(|e| AudioError::unsupported(format!("wasapi: render write failed: {e}")))
    };
    let mut log_line = |frames, elapsed, avg_fps, ring_fill_samples| {
        AudioLog::info(&format!(
            "play_file: frames={} elapsed={:.2}s avg_fps={:.0} ring_fill_samples={}",
            frames, elapsed, avg_fps, ring_fill_samples
        ));
    };

    let total_frames_played = run_consumer_loop(
        &mut cons,
        consumer::ConsumerLoopRuntime {
            done: &done,
            underrun_events: &underrun_events,
            cfg: ConsumerLoopConfig {
                device_ch,
                started,
                deadline,
                log_every: Duration::from_millis(1000),
            },
        },
        consumer::ConsumerLoopCallbacks {
            wait_for_event: &mut wait_for_event,
            get_available_frames: &mut get_available_frames,
            render_samples: &mut render_samples,
            log_line: &mut log_line,
        },
    )?;

    runtime.audio_client.stop_stream().ok();
    let _ = producer_join.join();

    let elapsed = started.elapsed().as_secs_f64().max(0.0);
    Ok(build_play_file_result(
        PlayFileResultParts {
            input,
            device_name,
            meta,
            device_sr,
            device_ch,
            seconds_played: elapsed,
            frames_played: total_frames_played,
            diag_log_path,
        },
        &underrun_events,
    ))
}
