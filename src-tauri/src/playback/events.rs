#[derive(Debug, Clone)]
pub enum PlaybackEvent {
    StateChanged,
    QueueChanged,
    TrackEnded,
    ErrorRaised,
}
