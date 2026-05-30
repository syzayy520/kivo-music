use super::backend_types::{PlaybackBackendDescriptor, PlaybackBackendKind};
use super::super::capabilities::PlaybackCapabilities;

pub fn descriptor() -> PlaybackBackendDescriptor {
    PlaybackBackendDescriptor {
        kind: PlaybackBackendKind::Mpv,
        name: "mpv".to_string(),
        capabilities: PlaybackCapabilities {
            can_seek: true,
            can_select_output_device: false,
            can_use_exclusive_output: false,
            can_probe_metadata: false,
            can_gapless: false,
            can_replaygain: false,
        },
        is_primary: false,
    }
}
