use crate::playback::errors::PlaybackError;
use crate::playback::native_pipeline::NativePipeline;
use crate::playback::playback_worker_command::PlaybackWorkerCommand;
use crate::playback::playback_worker_state::{PlaybackWorkerPhase, PlaybackWorkerState};
use crate::playback::types::{PlaybackTrack, TrackId};

fn track() -> PlaybackTrack {
    PlaybackTrack {
        id: TrackId("route-track-1".to_string()),
        title: "Route Track".to_string(),
        artist: "Route Artist".to_string(),
        source_path: "C:/Music/route-track-1.wav".to_string(),
    }
}

mod general_route_tests;
mod load_route_tests;
mod pause_route_tests;
mod play_route_tests;
mod resume_route_tests;
mod seek_route_tests;
mod set_muted_route_tests;
mod set_volume_route_tests;
mod shutdown_route_tests;
mod stop_route_tests;
