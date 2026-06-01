use super::decoder_request::AudioDecoderOpenRequest;
use super::types::{PlaybackTrack, TrackId};

fn track() -> PlaybackTrack {
    PlaybackTrack {
        id: TrackId("track-1".to_string()),
        title: "Track 1".to_string(),
        artist: "Artist".to_string(),
        source_path: "C:/Music/track-1.flac".to_string(),
    }
}

#[test]
fn open_request_is_created_from_playback_track() {
    let track = track();

    let request = AudioDecoderOpenRequest::from_track(&track);

    assert_eq!(request.track_id, "track-1");
    assert_eq!(request.source_path, "C:/Music/track-1.flac");
}

#[test]
fn open_request_does_not_depend_on_display_metadata() {
    let mut track = track();
    track.title = "Renamed".to_string();
    track.artist = "Another Artist".to_string();

    let request = AudioDecoderOpenRequest::from_track(&track);

    assert_eq!(request.track_id, "track-1");
    assert_eq!(request.source_path, "C:/Music/track-1.flac");
}

#[test]
fn open_request_keeps_owned_values_after_track_changes() {
    let mut track = track();
    let request = AudioDecoderOpenRequest::from_track(&track);

    track.id = TrackId("track-2".to_string());
    track.source_path = "C:/Music/track-2.flac".to_string();

    assert_eq!(request.track_id, "track-1");
    assert_eq!(request.source_path, "C:/Music/track-1.flac");
}

#[test]
fn open_request_preserves_empty_source_path() {
    let mut track = track();
    track.id = TrackId("track-empty".to_string());
    track.source_path = String::new();

    let request = AudioDecoderOpenRequest::from_track(&track);

    assert_eq!(request.track_id, "track-empty");
    assert_eq!(request.source_path, "");
}
