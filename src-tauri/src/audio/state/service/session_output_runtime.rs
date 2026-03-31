use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use crate::audio::{
    decoder_ffmpeg::header_probe,
    errors::{AudioError, AudioResult},
    logging::AudioLog,
    pipeline::linear_resampler::LinearResampler,
};

use super::lock_poison::lock_poison;
use super::{
    session_buffers::SessionBuffers,
    session_loop_helpers::start_output_stream,
    types::{AudioSessionState, AudioSessionStatus},
    windows_device::WasapiSharedDevice,
};

pub(super) const OUTPUT_REBUILD_MAX: u32 = 3;
pub(super) const OUTPUT_REBUILD_BACKOFF_MS: u64 = 200;

pub(super) struct RebuildResult {
    pub(super) dev: WasapiSharedDevice,
    pub(super) input_sr: u32,
    pub(super) resampler: LinearResampler,
    pub(super) ring_cap: usize,
    pub(super) base_seconds: f64,
}

pub(super) struct StartupResult {
    pub(super) dev: WasapiSharedDevice,
    pub(super) input_sr: u32,
    pub(super) resampler: LinearResampler,
    pub(super) ring_cap: usize,
}

pub(super) fn rebuild_output_after_failure(
    status: &Arc<Mutex<AudioSessionStatus>>,
    opt_device_id: Option<&str>,
    input_path: &Path,
    reason: String,
) -> AudioResult<RebuildResult> {
    {
        let mut s = lock_poison(status);
        s.output_rebuilds = s.output_rebuilds.saturating_add(1);
        if s.output_rebuilds > OUTPUT_REBUILD_MAX {
            return Err(AudioError::unsupported(format!(
                "wasapi: output rebuild exceeded limit ({OUTPUT_REBUILD_MAX}): {reason}"
            )));
        }
        AudioLog::warn(&format!(
            "wasapi: rebuilding output after failure: {reason} (rebuilds={})",
            s.output_rebuilds
        ));
        s.state = AudioSessionState::Starting;
        s.set_last_output_rebuild_reason(reason);
    }

    let base_seconds = { lock_poison(status).pos_seconds };
    let dev = WasapiSharedDevice::init(status.clone(), opt_device_id)?;
    let input_sr = header_probe::probe_sample_rate(input_path).unwrap_or(dev.device_sr);
    let resampler = LinearResampler::new(input_sr, dev.device_sr, dev.device_ch as usize);
    let ring_cap = (dev.device_sr as usize)
        .saturating_mul(dev.device_ch as usize)
        .saturating_mul(2)
        .max(4096);

    Ok(RebuildResult {
        dev,
        input_sr,
        resampler,
        ring_cap,
        base_seconds,
    })
}

pub(super) fn start_output_with_retries(
    status: &Arc<Mutex<AudioSessionStatus>>,
    opt_device_id: Option<&str>,
    input_path: &Path,
    mut dev: WasapiSharedDevice,
    base_seconds: f64,
) -> AudioResult<StartupResult> {
    let mut input_sr = header_probe::probe_sample_rate(input_path).unwrap_or(dev.device_sr);
    let mut resampler = LinearResampler::new(input_sr, dev.device_sr, dev.device_ch as usize);
    let mut ring_cap = (dev.device_sr as usize)
        .saturating_mul(dev.device_ch as usize)
        .saturating_mul(2)
        .max(1);

    let mut start_attempts: u32 = 0;
    loop {
        match start_output_stream(&dev, base_seconds, status) {
            Ok(()) => {
                return Ok(StartupResult {
                    dev,
                    input_sr,
                    resampler,
                    ring_cap,
                });
            }
            Err(e) => {
                start_attempts = start_attempts.saturating_add(1);
                let reason = format!("[OUTPUT_REBUILD] start output failed: {e}");
                {
                    let mut s = lock_poison(status);
                    s.output_rebuilds = s.output_rebuilds.saturating_add(1);
                    s.set_last_output_rebuild_reason(reason);
                }
                if start_attempts >= OUTPUT_REBUILD_MAX {
                    return Err(e);
                }

                dev.audio_client.stop_stream().ok();
                std::thread::sleep(std::time::Duration::from_millis(OUTPUT_REBUILD_BACKOFF_MS));
                dev = WasapiSharedDevice::init(status.clone(), opt_device_id)?;
                input_sr = header_probe::probe_sample_rate(input_path).unwrap_or(dev.device_sr);
                resampler = LinearResampler::new(input_sr, dev.device_sr, dev.device_ch as usize);
                ring_cap = (dev.device_sr as usize)
                    .saturating_mul(dev.device_ch as usize)
                    .saturating_mul(2)
                    .max(1);
            }
        }
    }
}

pub(super) struct RebuildState<'a> {
    pub(super) pending_out: &'a mut Vec<f32>,
    pub(super) pending_off: &'a mut usize,
    pub(super) media_frames_played: &'a mut u64,
    pub(super) base_seconds: &'a mut f64,
    pub(super) buffers: &'a mut SessionBuffers,
}

pub(super) fn apply_rebuild_result(
    status: &Arc<Mutex<AudioSessionStatus>>,
    rebuild: RebuildResult,
    state: RebuildState<'_>,
) -> AudioResult<StartupResult> {
    let RebuildResult {
        dev,
        input_sr,
        resampler,
        ring_cap,
        base_seconds,
    } = rebuild;

    state.pending_out.clear();
    *state.pending_off = 0;
    *state.media_frames_played = 0;
    *state.base_seconds = base_seconds;
    state.buffers.resize_for_device(&dev);
    start_output_stream(&dev, base_seconds, status)?;

    Ok(StartupResult {
        dev,
        input_sr,
        resampler,
        ring_cap,
    })
}

pub(super) fn recover_output_after_render_error(
    status: &Arc<Mutex<AudioSessionStatus>>,
    opt_device_id: Option<&str>,
    input_path: &Path,
    dev: &WasapiSharedDevice,
    reason: String,
    state: RebuildState<'_>,
) -> AudioResult<StartupResult> {
    dev.audio_client.stop_stream().ok();
    apply_rebuild_result(
        status,
        rebuild_output_after_failure(status, opt_device_id, input_path, reason)?,
        state,
    )
}
