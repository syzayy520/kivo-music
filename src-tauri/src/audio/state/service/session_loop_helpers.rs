use crate::audio::{
    errors::{AudioError, AudioResult},
    output_wasapi::pack::pack_samples_into,
};
use crossbeam_channel::Receiver;
use ringbuf::traits::Consumer;
use std::sync::{Arc, Mutex};

use super::lock_poison::lock_poison;
use super::{
    session_buffers::SessionBuffers,
    types::{AudioSessionState, AudioSessionStatus, ServiceCmd},
    windows_device::WasapiSharedDevice,
};

const SEEK_END_EPS_SECONDS: f64 = 0.1;

pub(super) fn start_output_stream(
    dev: &WasapiSharedDevice,
    base_pos_seconds: f64,
    status: &Arc<Mutex<AudioSessionStatus>>,
) -> AudioResult<()> {
    if dev.buffer_frames > 0 {
        let silence = vec![0u8; dev.buffer_frames.saturating_mul(dev.block_align)];
        dev.render_client
            .write_to_device(dev.buffer_frames, &silence, None)
            .map_err(|e| AudioError::unsupported(format!("wasapi: prime write failed: {e}")))?;
    }
    dev.audio_client
        .start_stream()
        .map_err(|e| AudioError::unsupported(format!("wasapi: start stream failed: {e}")))?;

    {
        let mut s = lock_poison(status);
        s.state = AudioSessionState::Playing;
        s.pos_seconds = base_pos_seconds;
        s.ended = false;
    }
    Ok(())
}

pub(super) fn drain_commands(
    cmd_rx: &Receiver<ServiceCmd>,
    status: &Arc<Mutex<AudioSessionStatus>>,
    stopping: &mut bool,
    paused: &mut bool,
    pending_seek: &mut Option<f64>,
    volume: &mut f32,
) {
    while let Ok(cmd) = cmd_rx.try_recv() {
        match cmd {
            ServiceCmd::Stop => *stopping = true,
            ServiceCmd::Pause(p) => *paused = p,
            ServiceCmd::Seek { seconds } => *pending_seek = Some(seconds),
            ServiceCmd::SetVolume(v) => {
                *volume = v.clamp(0.0, 1.0);
                let mut s = lock_poison(status);
                s.volume = *volume;
            }
        }
    }
}

pub(super) fn apply_pause_state(status: &Arc<Mutex<AudioSessionStatus>>, paused: bool) {
    let mut s = lock_poison(status);
    if paused {
        if s.state != AudioSessionState::Paused {
            s.state = AudioSessionState::Paused;
        }
    } else if s.state == AudioSessionState::Paused {
        s.state = AudioSessionState::Playing;
    }
}

pub(super) enum SeekPlan {
    StopAtEof(f64),
    SeekTo(f64),
}

pub(super) fn plan_seek(requested_seek: f64, duration_opt: Option<f64>) -> SeekPlan {
    let mut seek_to = if requested_seek.is_finite() {
        requested_seek
    } else {
        0.0
    };
    if seek_to < 0.0 {
        seek_to = 0.0;
    }

    if let Some(duration) = duration_opt.filter(|d| d.is_finite() && *d > 0.0) {
        if seek_to >= duration {
            return SeekPlan::StopAtEof(duration);
        }

        let eps = SEEK_END_EPS_SECONDS.min(duration).max(0.0);
        let clamp_hi = (duration - eps).max(0.0);
        if seek_to > clamp_hi {
            seek_to = clamp_hi;
        }
    }

    SeekPlan::SeekTo(seek_to)
}

pub(super) struct RenderStep {
    pub(super) popped: usize,
    pub(super) fill_frames: u64,
    pub(super) capacity_frames: u64,
    pub(super) underrun_events: u64,
    pub(super) underrun_frames: u64,
}

pub(super) struct RenderConfig<'a> {
    pub(super) dev: &'a WasapiSharedDevice,
    pub(super) ring_cap: usize,
    pub(super) paused: bool,
    pub(super) volume: f32,
}

pub(super) fn render_step(
    cons: &mut impl Consumer<Item = f32>,
    buffers: &mut SessionBuffers,
    cfg: RenderConfig<'_>,
    mut underrun_events: u64,
    mut underrun_frames: u64,
) -> AudioResult<Option<RenderStep>> {
    let _ = cfg.dev.handle.wait_for_event(50);
    let frames = cfg
        .dev
        .audio_client
        .get_available_space_in_frames()
        .map_err(|e| AudioError::unsupported(format!("get available frames failed: {e}")))?
        as usize;

    if frames == 0 {
        return Ok(None);
    }

    let need = frames.saturating_mul(cfg.dev.device_ch as usize);
    if need > buffers.samples_f32().len() {
        buffers.samples_f32_mut().resize(need, 0.0);
    }

    let mut popped = 0usize;
    {
        let slice = &mut buffers.samples_f32_mut()[..need];
        if !cfg.paused {
            popped = cons.pop_slice(slice);
        }

        if !cfg.paused && popped < need {
            let ch = (cfg.dev.device_ch as usize).max(1);
            let missing_samples = need - popped;
            let missing_frames = missing_samples.div_ceil(ch) as u64;
            underrun_events = underrun_events.saturating_add(1);
            underrun_frames = underrun_frames.saturating_add(missing_frames);
        }
        if popped < need {
            slice[popped..].fill(0.0);
        }

        if cfg.volume != 1.0 {
            for s in slice.iter_mut() {
                *s *= cfg.volume;
            }
        }
    }

    let mut packed_buf = std::mem::take(buffers.packed_mut());
    let samples_slice = &buffers.samples_f32()[..need];
    pack_samples_into(
        &mut packed_buf,
        samples_slice,
        cfg.dev.sample_type,
        cfg.dev.bits_per_sample,
        cfg.dev.valid_bits,
    );
    *buffers.packed_mut() = packed_buf;

    let packed_data = buffers.packed();
    cfg.dev
        .render_client
        .write_to_device(frames, packed_data, None)
        .map_err(|e| AudioError::unsupported(format!("render write failed: {e}")))?;

    let ch = (cfg.dev.device_ch as usize).max(1);
    let fill_frames = (cons.occupied_len() / ch) as u64;
    let capacity_frames = (cfg.ring_cap / ch) as u64;
    Ok(Some(RenderStep {
        popped,
        fill_frames,
        capacity_frames,
        underrun_events,
        underrun_frames,
    }))
}
