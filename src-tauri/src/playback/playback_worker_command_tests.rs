use super::playback_worker_command::PlaybackWorkerCommand;
use super::types::{PlaybackTrack, TrackId};

fn track() -> PlaybackTrack {
    PlaybackTrack {
        id: TrackId("track-42".to_string()),
        title: "Track 42".to_string(),
        artist: "Artist".to_string(),
        source_path: "C:/Music/track-42.flac".to_string(),
    }
}

#[test]
fn load_command_keeps_track_payload() {
    let command = PlaybackWorkerCommand::Load { track: track() };

    match command {
        PlaybackWorkerCommand::Load { track } => {
            assert_eq!(track.id.0, "track-42");
            assert_eq!(track.source_path, "C:/Music/track-42.flac");
        }
        _ => panic!("expected load command"),
    }
}

#[test]
fn seek_command_keeps_position_payload() {
    let command = PlaybackWorkerCommand::Seek {
        position_ms: 12_000,
    };

    match command {
        PlaybackWorkerCommand::Seek { position_ms } => assert_eq!(position_ms, 12_000),
        _ => panic!("expected seek command"),
    }
}

#[test]
fn simple_control_commands_are_constructible() {
    let commands = vec![
        PlaybackWorkerCommand::Play,
        PlaybackWorkerCommand::Pause,
        PlaybackWorkerCommand::Resume,
        PlaybackWorkerCommand::Stop,
        PlaybackWorkerCommand::SetVolume { level: 0.8 },
        PlaybackWorkerCommand::SetMuted { muted: true },
        PlaybackWorkerCommand::Shutdown,
    ];

    assert_eq!(commands.len(), 7);
}
