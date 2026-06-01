use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WindowsAudioPlan {
    pub device_id: Option<String>,
    pub dedicated_mode: bool,
    pub exact_format_requested: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WindowsAudioStatus {
    pub available: bool,
    pub active_device_id: Option<String>,
    pub note: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WindowsAudioDevice {
    pub id: String,
    pub name: String,
    pub is_default: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WindowsAudioDeviceSnapshot {
    pub devices: Vec<WindowsAudioDevice>,
    pub status: WindowsAudioStatus,
}

pub fn query_windows_audio_devices(
    plan: &WindowsAudioPlan,
) -> super::errors::PlaybackResult<WindowsAudioDeviceSnapshot> {
    platform::query_windows_audio_devices(plan)
}

#[cfg(not(windows))]
mod platform {
    use super::{WindowsAudioDeviceSnapshot, WindowsAudioPlan, WindowsAudioStatus};

    pub fn query_windows_audio_devices(
        _plan: &WindowsAudioPlan,
    ) -> super::super::errors::PlaybackResult<WindowsAudioDeviceSnapshot> {
        Ok(WindowsAudioDeviceSnapshot {
            devices: Vec::new(),
            status: WindowsAudioStatus {
                available: false,
                active_device_id: None,
                note: Some(
                    "windows audio device enumeration is only available on Windows".to_string(),
                ),
            },
        })
    }
}

#[cfg(windows)]
mod platform {
    use std::ffi::c_void;

    use super::super::errors::PlaybackResult;
    use super::super::windows_audio_com::platform::{windows_audio_error, WindowsComScope};
    use super::super::windows_audio_endpoint::platform::{
        create_device_enumerator, default_render_endpoint,
    };
    use super::{
        WindowsAudioDevice, WindowsAudioDeviceSnapshot, WindowsAudioPlan, WindowsAudioStatus,
    };
    use windows::Win32::Media::Audio::{
        eRender, IMMDevice, IMMDeviceEnumerator, DEVICE_STATE_ACTIVE,
    };
    use windows::Win32::System::Com::CoTaskMemFree;

    pub fn query_windows_audio_devices(
        plan: &WindowsAudioPlan,
    ) -> PlaybackResult<WindowsAudioDeviceSnapshot> {
        let _com = WindowsComScope::initialize()?;
        let enumerator = create_device_enumerator()?;

        let default_device_id = default_render_device_id(&enumerator).ok();
        let collection = unsafe { enumerator.EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE) }
            .map_err(|error| windows_audio_error("enumerate render endpoints", error))?;
        let count = unsafe { collection.GetCount() }
            .map_err(|error| windows_audio_error("read endpoint count", error))?;

        let mut devices = Vec::with_capacity(count as usize);
        for index in 0..count {
            let device = unsafe { collection.Item(index) }
                .map_err(|error| windows_audio_error("read render endpoint", error))?;
            let id = device_id(&device)?;
            let is_default = default_device_id.as_deref() == Some(id.as_str());
            let name = if is_default {
                "Default Windows audio device".to_string()
            } else {
                format!("Windows audio device {}", index + 1)
            };

            devices.push(WindowsAudioDevice {
                id,
                name,
                is_default,
            });
        }

        let active_device_id = plan.device_id.clone().or(default_device_id);
        let note = if devices.is_empty() {
            Some("no active Windows render endpoints were found".to_string())
        } else {
            None
        };

        Ok(WindowsAudioDeviceSnapshot {
            status: WindowsAudioStatus {
                available: !devices.is_empty(),
                active_device_id,
                note,
            },
            devices,
        })
    }

    fn default_render_device_id(enumerator: &IMMDeviceEnumerator) -> PlaybackResult<String> {
        let device = default_render_endpoint(enumerator)?;
        device_id(&device)
    }

    fn device_id(device: &IMMDevice) -> PlaybackResult<String> {
        let raw_id = unsafe { device.GetId() }
            .map_err(|error| windows_audio_error("read endpoint id", error))?;
        let text = unsafe { raw_id.to_string() }
            .map_err(|error| windows_audio_error("convert endpoint id", error.into()))?;
        unsafe { CoTaskMemFree(Some(raw_id.0.cast::<c_void>())) };
        Ok(text)
    }
}
