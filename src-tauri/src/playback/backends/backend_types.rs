use serde::{Deserialize, Serialize};

use super::super::capabilities::PlaybackCapabilities;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PlaybackBackendKind {
    Native,
    Mpv,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlaybackBackendDescriptor {
    pub kind: PlaybackBackendKind,
    pub name: String,
    pub capabilities: PlaybackCapabilities,
    pub is_primary: bool,
}
