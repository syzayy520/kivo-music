use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use ringbuf::traits::Producer;

use crate::audio::{
    decoder_ffmpeg::{FfmpegDecoder, FfmpegLoadOptions},
    errors::{AudioError, AudioResult},
    logging::AudioLog,
    pipeline::{channel_map::map_channels_interleaved, linear_resampler::LinearResampler},
    ring::{F32Consumer, F32Producer},
};
use std::path::Path;

use super::lock_poison::lock_poison;
use super::{
    session_buffers::SessionBuffers,
    session_loop_helpers::{plan_seek, SeekPlan},
    types::{AudioSessionState, AudioSessionStatus},
    windows_device::WasapiSharedDevice,
};

pub(super) enum SeekOutcome {
    StopAtEof,
    Seeked { seek_to: f64 },
    Failed,
}

pub(super) fn open_decoder_with_start_seek(
    input_path: &Path,
    load_opt: &FfmpegLoadOptions,
    base_seconds: f64,
    status: &Arc<Mutex<AudioSessionStatus>>,
) -> AudioResult<(FfmpegDecoder, f64)> {
    let mut decoder = FfmpegDecoder::open(input_path, load_opt).map_err(|e| {
        AudioError::unsupported(format!(
            "ffmpeg open failed: {e}. {}",
            AudioLog::ffmpeg_dir_hint(load_opt.dll_dir.as_deref())
        ))
    })?;

    let mut resolved_base_seconds = base_seconds;
    if resolved_base_seconds > 0.0 {
        let requested = resolved_base_seconds;
        if let Err(e) = decoder.seek_seconds(requested) {
            resolved_base_seconds = 0.0;
            let mut s = lock_poison(status);
            s.pos_seconds = 0.0;
            s.set_last_error_message(format!("[SEEK_FAILED] start_seconds={requested:.3}: {e}"));
        }
    }

    Ok((decoder, resolved_base_seconds))
}

pub(super) fn handle_pending_seek(
    requested_seek: f64,
    status: &Arc<Mutex<AudioSessionStatus>>,
    decoder: &mut FfmpegDecoder,
) -> AudioResult<SeekOutcome> {
    let duration_opt = { lock_poison(status).duration_seconds };
    match plan_seek(requested_seek, duration_opt) {
        SeekPlan::StopAtEof(duration) => {
            let mut s = lock_poison(status);
            s.state = AudioSessionState::Stopping;
            s.pos_seconds = duration;
            s.last_error = None;
            Ok(SeekOutcome::StopAtEof)
        }
        SeekPlan::SeekTo(seek_to) => match decoder.seek_seconds(seek_to) {
            Ok(()) => Ok(SeekOutcome::Seeked { seek_to }),
            Err(e) => {
                let mut s = lock_poison(status);
                s.set_last_error_message(format!("[SEEK_FAILED] seek_to={seek_to:.3}: {e}"));
                Ok(SeekOutcome::Failed)
            }
        },
    }
}

pub(super) struct SeekResetState<'a> {
    pub(super) pending_out: &'a mut Vec<f32>,
    pub(super) pending_off: &'a mut usize,
    pub(super) eof: &'a mut bool,
    pub(super) media_frames_played: &'a mut u64,
    pub(super) base_seconds: &'a mut f64,
    pub(super) underrun_events: &'a mut u64,
    pub(super) underrun_frames: &'a mut u64,
}

pub(super) fn reset_after_successful_seek(
    status: &Arc<Mutex<AudioSessionStatus>>,
    state: SeekResetState<'_>,
    seek_to: f64,
) {
    state.pending_out.clear();
    *state.pending_off = 0;
    *state.eof = false;
    *state.media_frames_played = 0;
    *state.base_seconds = seek_to;
    *state.underrun_events = 0;
    *state.underrun_frames = 0;

    let mut s = lock_poison(status);
    s.pos_seconds = *state.base_seconds;
    s.last_error = None;
    s.ended = false;
    s.underrun_events = 0;
    s.underrun_frames = 0;
}

pub(super) fn rebuild_after_successful_seek(
    input_sr: u32,
    dev: &WasapiSharedDevice,
    ring_cap: usize,
) -> (LinearResampler, F32Producer, F32Consumer) {
    let (prod, cons) = crate::audio::ring::make_f32_spsc(ring_cap);
    (
        LinearResampler::new(input_sr, dev.device_sr, dev.device_ch as usize),
        prod,
        cons,
    )
}

pub(super) fn mark_stopping(status: &Arc<Mutex<AudioSessionStatus>>) {
    lock_poison(status).state = AudioSessionState::Stopping;
}

#[allow(clippy::too_many_arguments)]
pub(super) fn feed_ring_from_decoder(
    prod: &mut impl Producer<Item = f32>,
    decoder: &mut FfmpegDecoder,
    resampler: &mut LinearResampler,
    buffers: &mut SessionBuffers,
    dev: &WasapiSharedDevice,
    eof: &mut bool,
    pending_out: &mut Vec<f32>,
    pending_off: &mut usize,
) -> AudioResult<()> {
    for _ in 0..2 {
        if prod.vacant_len() < (dev.device_ch as usize).saturating_mul(256) {
            break;
        }

        if *pending_off < pending_out.len() {
            let n = (pending_out.len() - *pending_off).min(prod.vacant_len());
            let wrote = prod.push_slice(&pending_out[*pending_off..*pending_off + n]);
            *pending_off += wrote;
            if *pending_off >= pending_out.len() {
                pending_out.clear();
                *pending_off = 0;
            }
            break;
        }

        let Some(chunk) = decoder.next_pcm_f32()? else {
            *eof = true;
            break;
        };
        let src_ch = (chunk.channels as usize).max(1);
        let dst_ch = (dev.device_ch as usize).max(1);

        map_channels_interleaved(&chunk.data, src_ch, dst_ch, buffers.tmp_mapped_mut());

        let mapped_data = std::mem::take(buffers.tmp_mapped_mut());
        resampler.resample_interleaved(&mapped_data, buffers.tmp_resampled_mut());
        *buffers.tmp_mapped_mut() = mapped_data;

        let tmp_resampled = buffers.tmp_resampled_mut();
        if !tmp_resampled.is_empty() {
            *pending_out = std::mem::take(tmp_resampled);
            *pending_off = 0;
        }
    }

    Ok(())
}

pub(super) struct StatusUpdate {
    pub(super) base_seconds: f64,
    pub(super) media_frames_played: u64,
    pub(super) device_sr: u32,
    pub(super) fill_frames: u64,
    pub(super) capacity_frames: u64,
    pub(super) underrun_events: u64,
    pub(super) underrun_frames: u64,
}

pub(super) fn update_playback_status(
    status: &Arc<Mutex<AudioSessionStatus>>,
    update: StatusUpdate,
    last_status: &mut Instant,
) {
    *last_status = Instant::now();
    let pos = update.base_seconds
        + (update.media_frames_played as f64) / (update.device_sr as f64).max(1.0);

    let mut s = lock_poison(status);
    s.pos_seconds = pos;
    s.buffer_fill_frames = update.fill_frames;
    s.buffer_capacity_frames = update.capacity_frames;
    s.underrun_events = update.underrun_events;
    s.underrun_frames = update.underrun_frames;
}

pub(super) fn finish_session(
    status: &Arc<Mutex<AudioSessionStatus>>,
    dev: &WasapiSharedDevice,
    natural_end: bool,
) {
    dev.audio_client.stop_stream().ok();

    let mut s = lock_poison(status);
    s.state = AudioSessionState::Stopped;
    if natural_end {
        s.ended = true;
    }
}

pub(super) fn playback_is_drained(eof: bool, occupied_len: usize) -> bool {
    eof && occupied_len == 0
}
