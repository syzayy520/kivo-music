use super::decoder::{AudioSampleFormat, AudioStreamInfo};
use super::decoder_request::AudioDecoderOpenRequest;
use super::decoder_session::DecoderSession;
use super::types::{PlaybackTrack, TrackId};

fn open_request() -> AudioDecoderOpenRequest {
    let track = PlaybackTrack {
        id: TrackId("track-9".to_string()),
        title: "Track 9".to_string(),
        artist: "Artist".to_string(),
        source_path: "C:/Music/track-9.wav".to_string(),
    };

    AudioDecoderOpenRequest::from_track(&track)
}

fn stream_info() -> AudioStreamInfo {
    AudioStreamInfo {
        sample_rate_hz: 44_100,
        channels: 2,
        sample_format: AudioSampleFormat::Signed16,
    }
}

#[test]
fn from_open_request_builds_session_with_initial_counters() {
    let session = DecoderSession::from_open_request(&open_request(), stream_info(), 1_000);

    assert_eq!(session.track_id, "track-9");
    assert_eq!(session.source_path, "C:/Music/track-9.wav");
    assert_eq!(session.opened_at_ms, 1_000);
    assert_eq!(session.last_position_ms, 0);
    assert_eq!(session.decoded_frame_count, 0);
}

#[test]
fn update_position_replaces_last_position() {
    let mut session = DecoderSession::from_open_request(&open_request(), stream_info(), 2_000);

    session.update_position(8_500);

    assert_eq!(session.last_position_ms, 8_500);
}

#[test]
fn count_frame_increments_decoded_frame_count() {
    let mut session = DecoderSession::from_open_request(&open_request(), stream_info(), 3_000);

    session.count_frame();
    session.count_frame();

    assert_eq!(session.decoded_frame_count, 2);
}
