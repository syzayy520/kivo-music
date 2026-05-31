use serde::{Deserialize, Serialize};

use super::types::PlaybackTrack;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PlaybackWorkerCommand {
    Load { track: PlaybackTrack },
    Play,
    Pause,
    Resume,
    Stop,
    Seek { position_ms: u64 },
    SetVolume { level: f32 },
    SetMuted { muted: bool },
    Shutdown,
}
