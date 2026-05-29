#[derive(Debug, Clone)]
pub enum PlaybackCommand {
    Play,
    Pause,
    Seek { position_ms: u64 },
    Next,
    Previous,
    SetQueue { track_ids: Vec<String> },
}
