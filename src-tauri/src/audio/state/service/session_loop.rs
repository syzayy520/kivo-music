use crate::audio::{decoder_ffmpeg::FfmpegLoadOptions, errors::AudioResult};
use crossbeam_channel::Receiver;
use ringbuf::traits::Observer;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use super::lock_poison::lock_poison;
use super::session_buffers::SessionBuffers;
use super::session_loop_helpers::{apply_pause_state, drain_commands, render_step, RenderConfig};
use super::session_loop_runtime::{
    feed_ring_from_decoder, finish_session, handle_pending_seek, mark_stopping,
    open_decoder_with_start_seek, playback_is_drained, rebuild_after_successful_seek,
    reset_after_successful_seek, update_playback_status, SeekOutcome, SeekResetState, StatusUpdate,
};
use super::session_output_runtime::{
    recover_output_after_render_error, start_output_with_retries, RebuildState, StartupResult,
};
use super::{
    types::{AudioSessionStatus, PlayStartOptions, ServiceCmd},
    windows_device::WasapiSharedDevice,
};

pub(super) fn run_session_loop(
    _session_id: u64,
    input_path: &Path,
    opt: PlayStartOptions,
    status: Arc<Mutex<AudioSessionStatus>>,
    cmd_rx: Receiver<ServiceCmd>,
    dev: WasapiSharedDevice,
) -> AudioResult<()> {
    let mut dev = dev;
    let mut base_seconds = opt.start_seconds.unwrap_or(0.0).max(0.0);
    let max_frames_total = opt
        .max_seconds
        .map(|m| (m.max(0.0) * dev.device_sr as f64).round() as u64);
    let load_opt = FfmpegLoadOptions {
        dll_dir: opt.ffmpeg_dir.as_deref().map(PathBuf::from),
    };

    let (mut decoder, resolved_base_seconds) =
        open_decoder_with_start_seek(input_path, &load_opt, base_seconds, &status)?;
    base_seconds = resolved_base_seconds;
    let StartupResult {
        dev: started_dev,
        mut input_sr,
        mut resampler,
        mut ring_cap,
    } = start_output_with_retries(
        &status,
        opt.device_id.as_deref(),
        input_path,
        dev,
        base_seconds,
    )?;
    dev = started_dev;
    let (mut prod, mut cons) = crate::audio::ring::make_f32_spsc(ring_cap);
    let mut paused = false;
    let mut stopping = false;
    let mut pending_seek: Option<f64> = None;
    let mut volume: f32 = { lock_poison(&status).volume }.clamp(0.0, 1.0);
    let mut natural_end = false;
    let mut eof = false;
    let mut pending_out: Vec<f32> = Vec::new();
    let mut pending_off: usize = 0;
    let mut media_frames_played: u64 = 0;
    let mut underrun_events: u64 = 0;
    let mut underrun_frames: u64 = 0;
    let mut buffers = SessionBuffers::new();
    buffers.resize_for_device(&dev);
    let mut last_status = Instant::now();
    let status_every = Duration::from_millis(100);
    loop {
        drain_commands(
            &cmd_rx,
            &status,
            &mut stopping,
            &mut paused,
            &mut pending_seek,
            &mut volume,
        );
        if stopping {
            mark_stopping(&status);
            break;
        }
        apply_pause_state(&status, paused);

        if let Some(requested_seek) = pending_seek.take() {
            match handle_pending_seek(requested_seek, &status, &mut decoder)? {
                SeekOutcome::StopAtEof => break,
                SeekOutcome::Seeked { seek_to } => {
                    (resampler, prod, cons) =
                        rebuild_after_successful_seek(input_sr, &dev, ring_cap);
                    reset_after_successful_seek(
                        &status,
                        SeekResetState {
                            pending_out: &mut pending_out,
                            pending_off: &mut pending_off,
                            eof: &mut eof,
                            media_frames_played: &mut media_frames_played,
                            base_seconds: &mut base_seconds,
                            underrun_events: &mut underrun_events,
                            underrun_frames: &mut underrun_frames,
                        },
                        seek_to,
                    );
                }
                SeekOutcome::Failed => {}
            }
        }

        if !paused && !eof {
            feed_ring_from_decoder(
                &mut prod,
                &mut decoder,
                &mut resampler,
                &mut buffers,
                &dev,
                &mut eof,
                &mut pending_out,
                &mut pending_off,
            )?;
        }

        let step = match render_step(
            &mut cons,
            &mut buffers,
            RenderConfig {
                dev: &dev,
                ring_cap,
                paused,
                volume,
            },
            underrun_events,
            underrun_frames,
        ) {
            Ok(v) => v,
            Err(e) => {
                let startup = recover_output_after_render_error(
                    &status,
                    opt.device_id.as_deref(),
                    input_path,
                    &dev,
                    e.to_string(),
                    RebuildState {
                        pending_out: &mut pending_out,
                        pending_off: &mut pending_off,
                        media_frames_played: &mut media_frames_played,
                        base_seconds: &mut base_seconds,
                        buffers: &mut buffers,
                    },
                )?;
                dev = startup.dev;
                input_sr = startup.input_sr;
                resampler = startup.resampler;
                ring_cap = startup.ring_cap;
                (prod, cons) = crate::audio::ring::make_f32_spsc(ring_cap);
                continue;
            }
        };

        let Some(step) = step else {
            if playback_is_drained(eof, cons.occupied_len()) {
                natural_end = true;
                break;
            }
            continue;
        };
        underrun_events = step.underrun_events;
        underrun_frames = step.underrun_frames;

        if !paused {
            media_frames_played = media_frames_played
                .saturating_add((step.popped / (dev.device_ch as usize).max(1)) as u64);
        }
        if last_status.elapsed() >= status_every {
            update_playback_status(
                &status,
                StatusUpdate {
                    base_seconds,
                    media_frames_played,
                    device_sr: dev.device_sr,
                    fill_frames: step.fill_frames,
                    capacity_frames: step.capacity_frames,
                    underrun_events,
                    underrun_frames,
                },
                &mut last_status,
            );
        }

        if let Some(max_frames) = max_frames_total {
            if media_frames_played >= max_frames {
                break;
            }
        }
        if playback_is_drained(eof, cons.occupied_len()) {
            natural_end = true;
            break;
        }
    }
    finish_session(&status, &dev, natural_end);
    Ok(())
}
