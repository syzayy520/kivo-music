use std::{path::Path, sync::atomic::AtomicU64};

use crate::audio::{
    errors::{AudioError, AudioResult},
    logging::AudioLog,
};

use super::{producer::ProducerMeta, PlayFileResult};

#[cfg(windows)]
use wasapi::{
    initialize_mta, AudioClient, AudioRenderClient, DeviceEnumerator, Direction, Handle, StreamMode,
};

pub(super) struct WasapiPlaybackRuntime {
    pub(super) device_name: String,
    pub(super) device_sr: u32,
    pub(super) device_ch: u16,
    pub(super) bits_per_sample: usize,
    pub(super) valid_bits: usize,
    pub(super) block_align: usize,
    pub(super) sample_type: wasapi::SampleType,
    pub(super) buffer_frames: usize,
    pub(super) audio_client: AudioClient,
    pub(super) render_client: AudioRenderClient,
    pub(super) handle: Handle,
}

pub(super) fn recv_producer_meta(
    meta_rx: &std::sync::mpsc::Receiver<super::producer::ProducerMsg>,
    producer_join: std::thread::JoinHandle<()>,
    input: &Path,
    device_sr: u32,
) -> Result<(ProducerMeta, std::thread::JoinHandle<()>), AudioError> {
    let mut meta = ProducerMeta {
        ffmpeg_version: "<unknown>".into(),
        input_sample_rate_hz: crate::audio::decoder_ffmpeg::header_probe::probe_sample_rate(input)
            .unwrap_or(device_sr),
        input_channels: 0,
    };
    if let Ok(msg) = meta_rx.recv_timeout(std::time::Duration::from_secs(2)) {
        match msg {
            super::producer::ProducerMsg::Meta(m) => meta = m,
            super::producer::ProducerMsg::Fatal(e) => {
                let _ = producer_join.join();
                return Err(AudioError::unsupported(e));
            }
        }
    }
    Ok((meta, producer_join))
}

#[cfg(windows)]
pub(super) fn init_wasapi_runtime() -> AudioResult<WasapiPlaybackRuntime> {
    let init = initialize_mta();
    if init.is_err() {
        AudioLog::warn(&format!("wasapi: initialize_mta failed: {:?}", init));
    }

    let enumerator = DeviceEnumerator::new()
        .map_err(|e| AudioError::unsupported(format!("wasapi: enumerator failed: {e}")))?;
    let device = enumerator
        .get_default_device(&Direction::Render)
        .map_err(|e| {
            AudioError::unsupported(format!("wasapi: get default render device failed: {e}"))
        })?;

    let device_name = device
        .get_friendlyname()
        .unwrap_or_else(|_| "<unknown device>".to_string());

    let mut audio_client = device
        .get_iaudioclient()
        .map_err(|e| AudioError::unsupported(format!("wasapi: get IAudioClient failed: {e}")))?;

    let mix_format = audio_client
        .get_mixformat()
        .map_err(|e| AudioError::unsupported(format!("wasapi: get mix format failed: {e}")))?;

    let device_sr = mix_format.get_samplespersec();
    let device_ch = mix_format.get_nchannels();
    let bits_per_sample = mix_format.get_bitspersample() as usize;
    let valid_bits = mix_format.get_validbitspersample() as usize;
    let block_align = mix_format.get_blockalign() as usize;
    let sample_type = mix_format
        .get_subformat()
        .map_err(|e| AudioError::unsupported(format!("wasapi: mix subformat failed: {e}")))?;

    let mode = StreamMode::EventsShared {
        autoconvert: true,
        buffer_duration_hns: 1_000_000,
    };
    audio_client
        .initialize_client(&mix_format, &Direction::Render, &mode)
        .map_err(|e| AudioError::unsupported(format!("wasapi: initialize failed: {e}")))?;

    let handle = audio_client
        .set_get_eventhandle()
        .map_err(|e| AudioError::unsupported(format!("wasapi: set event handle failed: {e}")))?;
    let render_client = audio_client
        .get_audiorenderclient()
        .map_err(|e| AudioError::unsupported(format!("wasapi: get render client failed: {e}")))?;
    let buffer_frames = audio_client
        .get_buffer_size()
        .map_err(|e| AudioError::unsupported(format!("wasapi: get buffer size failed: {e}")))?
        as usize;

    AudioLog::info(&format!(
        "play_file: device='{}' mix={}Hz {}ch {:?} bps={} valid_bits={} block_align={} buffer_frames={}",
        device_name, device_sr, device_ch, sample_type, bits_per_sample, valid_bits, block_align, buffer_frames
    ));

    Ok(WasapiPlaybackRuntime {
        device_name,
        device_sr,
        device_ch,
        bits_per_sample,
        valid_bits,
        block_align,
        sample_type,
        buffer_frames,
        audio_client,
        render_client,
        handle,
    })
}

#[cfg(windows)]
pub(super) fn prime_and_start_stream(runtime: &mut WasapiPlaybackRuntime) -> AudioResult<()> {
    if runtime.buffer_frames > 0 {
        let silence_bytes = vec![0u8; runtime.buffer_frames.saturating_mul(runtime.block_align)];
        runtime
            .render_client
            .write_to_device(runtime.buffer_frames, &silence_bytes, None)
            .map_err(|e| AudioError::unsupported(format!("wasapi: prime write failed: {e}")))?;
    }

    runtime
        .audio_client
        .start_stream()
        .map_err(|e| AudioError::unsupported(format!("wasapi: start stream failed: {e}")))?;
    Ok(())
}

pub(super) struct PlayFileResultParts {
    pub(super) input: std::path::PathBuf,
    pub(super) device_name: String,
    pub(super) meta: ProducerMeta,
    pub(super) device_sr: u32,
    pub(super) device_ch: u16,
    pub(super) seconds_played: f64,
    pub(super) frames_played: u64,
    pub(super) diag_log_path: Option<std::path::PathBuf>,
}

pub(super) fn build_play_file_result(
    parts: PlayFileResultParts,
    underrun_events: &AtomicU64,
) -> PlayFileResult {
    PlayFileResult {
        input_path: parts.input.display().to_string(),
        device_name: parts.device_name,
        ffmpeg_version: parts.meta.ffmpeg_version,
        input_sample_rate_hz: parts.meta.input_sample_rate_hz,
        input_channels: parts.meta.input_channels,
        device_sample_rate_hz: parts.device_sr,
        device_channels: parts.device_ch,
        seconds_played: parts.seconds_played,
        frames_played: parts.frames_played,
        underrun_events: underrun_events.load(std::sync::atomic::Ordering::Relaxed),
        diag_log_path: parts.diag_log_path.map(|p| p.display().to_string()),
    }
}
