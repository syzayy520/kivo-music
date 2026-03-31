use std::sync::{Arc, Mutex};

use crate::audio::errors::{AudioError, AudioResult};
use crate::audio::logging::AudioLog;

use super::lock_poison::lock_poison;
use super::types::{AudioSessionState, AudioSessionStatus};

use wasapi::{DeviceEnumerator, Direction, Handle, StreamMode};

pub(super) struct WasapiSharedDevice {
    #[allow(dead_code)]
    pub device_name: String,
    pub device_sr: u32,
    pub device_ch: u16,
    pub bits_per_sample: usize,
    pub valid_bits: usize,
    pub block_align: usize,
    pub sample_type: wasapi::SampleType,
    pub buffer_frames: usize,
    pub audio_client: wasapi::AudioClient,
    pub render_client: wasapi::AudioRenderClient,
    pub handle: Handle,
}

impl WasapiSharedDevice {
    pub fn init(
        status: Arc<Mutex<AudioSessionStatus>>,
        requested_device_id: Option<&str>,
    ) -> AudioResult<Self> {
        let enumerator = DeviceEnumerator::new()
            .map_err(|e| AudioError::unsupported(format!("wasapi: enumerator failed: {e}")))?;
        let default_device = enumerator
            .get_default_device(&Direction::Render)
            .map_err(|e| {
                AudioError::unsupported(format!("wasapi: get default render device failed: {e}"))
            })?;

        let (device, device_name) = match requested_device_id {
            Some(req_id) if !req_id.trim().is_empty() => {
                let req_id = req_id.trim();
                match try_find_render_device_by_id(&enumerator, req_id) {
                    Some(dev) => {
                        let name = dev
                            .get_friendlyname()
                            .unwrap_or_else(|_| "<unknown device>".to_string());
                        (dev, name)
                    }
                    None => {
                        let fallback_name = default_device
                            .get_friendlyname()
                            .unwrap_or_else(|_| "<unknown device>".to_string());
                        let default_id = default_device
                            .get_id()
                            .unwrap_or_else(|_| "<unknown id>".to_string());
                        AudioLog::warn(&format!(
                            "wasapi: requested device_id=\"{req_id}\" not found; falling back to default device name=\"{fallback_name}\" id=\"{default_id}\""
                        ));
                        (default_device, fallback_name)
                    }
                }
            }
            _ => {
                let name = default_device
                    .get_friendlyname()
                    .unwrap_or_else(|_| "<unknown device>".to_string());
                (default_device, name)
            }
        };

        let mut audio_client = device.get_iaudioclient().map_err(|e| {
            AudioError::unsupported(format!("wasapi: get IAudioClient failed: {e}"))
        })?;
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

        {
            let mut s = lock_poison(&status);
            s.device_name = Some(device_name.clone());
            s.sample_rate = device_sr;
            s.channels = device_ch;
            s.state = AudioSessionState::Starting;
        }

        let mode = StreamMode::EventsShared {
            autoconvert: true,
            buffer_duration_hns: 1_000_000, // 100ms
        };
        audio_client
            .initialize_client(&mix_format, &Direction::Render, &mode)
            .map_err(|e| AudioError::unsupported(format!("wasapi: initialize failed: {e}")))?;

        let handle = audio_client.set_get_eventhandle().map_err(|e| {
            AudioError::unsupported(format!("wasapi: set event handle failed: {e}"))
        })?;
        let render_client = audio_client.get_audiorenderclient().map_err(|e| {
            AudioError::unsupported(format!("wasapi: get render client failed: {e}"))
        })?;
        let buffer_frames = audio_client
            .get_buffer_size()
            .map_err(|e| AudioError::unsupported(format!("wasapi: get buffer size failed: {e}")))?
            as usize;

        Ok(Self {
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
}

fn try_find_render_device_by_id(
    enumerator: &DeviceEnumerator,
    requested_id: &str,
) -> Option<wasapi::Device> {
    let collection = enumerator.get_device_collection(&Direction::Render).ok()?;
    // wasapi::DeviceCollection implements IntoIterator for &DeviceCollection.
    for d in &collection {
        let device = match d {
            Ok(v) => v,
            Err(e) => {
                AudioLog::warn(&format!("wasapi: enumerate device failed: {e}"));
                continue;
            }
        };
        let id = match device.get_id() {
            Ok(v) => v,
            Err(e) => {
                AudioLog::warn(&format!("wasapi: get device id failed: {e}"));
                continue;
            }
        };
        if id == requested_id {
            return Some(device);
        }
    }
    None
}
