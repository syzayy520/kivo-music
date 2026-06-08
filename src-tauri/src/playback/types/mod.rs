use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PlaybackStatus {
    Idle,
    Loading,
    Playing,
    Paused,
    Stopped,
    Failed,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum RepeatMode {
    #[default]
    Off,
    One,
    All,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TrackId(pub String);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlaybackTrack {
    pub id: TrackId,
    pub title: String,
    pub artist: String,
    pub source_path: String,
}
