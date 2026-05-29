#[derive(Debug, Clone)]
pub struct PlaybackState {
    pub current_track_id: Option<String>,
    pub is_playing: bool,
    pub progress_ms: u64,
    pub duration_ms: u64,
    pub volume: f32,
    pub output_device: Option<String>,
    pub quality_label: Option<String>,
}
