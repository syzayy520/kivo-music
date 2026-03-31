//! WASAPI device enumeration helpers.
//!
//! Scope: enumerate **render** (output) endpoint devices.
//!
//! Notes:
//! - This module intentionally does **not** touch the playback pipeline.
//! - We keep the API small so Window3 can later wire `device_id` into the player.

use serde::Serialize;

use wasapi::{initialize_mta, DeviceEnumerator, Direction};

use crate::audio::{error::AudioResult, logging::AudioLog};

/// Minimal output device descriptor (stable shape for UI/CLI later).
#[derive(Debug, Clone, Serialize)]
pub struct RenderDeviceInfo {
    pub id: String,
    pub name: String,
    pub is_default: bool,
}

/// Best-effort list of active render devices.
///
/// This is the surface API requested by the ticket: `list_render_devices() -> Vec<{id,name,is_default}>`.
///
/// If WASAPI enumeration fails, this returns an empty vec and prints a diagnostic line.
pub fn list_render_devices() -> Vec<RenderDeviceInfo> {
    match try_list_render_devices() {
        Ok(v) => v,
        Err(e) => {
            AudioLog::warn(&format!(
                "wasapi: list_render_devices failed: {e}. hint: run on Windows, and avoid UI thread"
            ));
            Vec::new()
        }
    }
}

/// Fallible variant of [`list_render_devices`].
pub fn try_list_render_devices() -> AudioResult<Vec<RenderDeviceInfo>> {
    // Initialize COM for WASAPI. If already initialized with a different mode, we still
    // continue and let subsequent calls fail with a clear error.
    let _ = initialize_mta();

    let enumerator = DeviceEnumerator::new()?;
    let default_id = enumerator
        .get_default_device(&Direction::Render)
        .ok()
        .and_then(|d| d.get_id().ok());

    let collection = enumerator.get_device_collection(&Direction::Render)?;

    let mut out = Vec::<RenderDeviceInfo>::new();
    for d in &collection {
        let device = match d {
            Ok(v) => v,
            Err(e) => {
                AudioLog::warn(&format!("wasapi: enumerate device failed: {e}"));
                continue;
            }
        };

        let id = device
            .get_id()
            .unwrap_or_else(|_| "<unknown id>".to_string());
        let name = device
            .get_friendlyname()
            .unwrap_or_else(|_| "<unknown device>".to_string());

        let is_default = match default_id.as_deref() {
            Some(did) => did == id,
            None => false,
        };

        out.push(RenderDeviceInfo {
            id,
            name,
            is_default,
        });
    }

    // Prefer stable ordering for UX: default first, then name.
    out.sort_by(|a, b| {
        b.is_default
            .cmp(&a.is_default)
            .then_with(|| a.name.cmp(&b.name))
    });

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Smoke-only: print the render device list (never fail CI on empty list).
    #[test]
    fn print_render_devices_smoke() {
        let devices = list_render_devices();
        eprintln!("wasapi render devices: {}", devices.len());
        for d in devices {
            let tag = if d.is_default { "default" } else { "" };
            eprintln!("- {tag}\tname=\"{}\"\tid=\"{}\"", d.name, d.id);
        }
    }
}
