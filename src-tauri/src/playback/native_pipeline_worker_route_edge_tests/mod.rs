use super::native_pipeline::NativePipeline;
use super::playback_worker_command::PlaybackWorkerCommand;
use super::playback_worker_state::{PlaybackWorkerPhase, PlaybackWorkerState};
use super::types::{PlaybackTrack, TrackId};

fn edge_track(id: &str) -> PlaybackTrack {
    PlaybackTrack {
        id: TrackId(id.to_string()),
        title: "Edge Track".to_string(),
        artist: "Edge Artist".to_string(),
        source_path: format!("C:/Music/{id}.wav"),
    }
}

mod load_edge_tests;
mod pause_edge_tests;
mod play_edge_tests;
mod resume_edge_tests;
mod seek_edge_tests;
mod set_muted_edge_tests;
mod set_volume_edge_tests;
mod shutdown_edge_tests;
mod stop_edge_tests;
