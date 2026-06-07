use super::super::types::{PlaybackTrack, TrackId};

pub(super) fn track() -> PlaybackTrack {
    PlaybackTrack {
        id: TrackId("track-1".to_string()),
        title: "Track 1".to_string(),
        artist: "Artist".to_string(),
        source_path: "C:/Music/track-1.flac".to_string(),
    }
}

pub(super) fn wav_track() -> (PlaybackTrack, String) {
    let path = std::env::temp_dir().join(format!(
        "kivo-native-engine-load-{}-{}.wav",
        std::process::id(),
        unique_suffix()
    ));
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 44_100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(&path, spec).expect("create wav file");

    for sample in [0_i16, 1000, -1000, 2000] {
        writer.write_sample(sample).expect("write wav sample");
    }
    writer.finalize().expect("finalize wav file");

    let source_path = path.to_string_lossy().into_owned();
    (
        PlaybackTrack {
            id: TrackId("track-wav-load".to_string()),
            title: "WAV Load".to_string(),
            artist: "Artist".to_string(),
            source_path: source_path.clone(),
        },
        source_path,
    )
}

pub(super) fn multi_frame_wav_track() -> (PlaybackTrack, String) {
    let path = std::env::temp_dir().join(format!(
        "kivo-native-engine-play-{}-{}.wav",
        std::process::id(),
        unique_suffix()
    ));
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 44_100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(&path, spec).expect("create wav file");

    for sample in 0..4096_i16 {
        writer.write_sample(sample).expect("write wav sample");
    }
    writer.finalize().expect("finalize wav file");

    let source_path = path.to_string_lossy().into_owned();
    (
        PlaybackTrack {
            id: TrackId("track-wav-play".to_string()),
            title: "WAV Play".to_string(),
            artist: "Artist".to_string(),
            source_path: source_path.clone(),
        },
        source_path,
    )
}

fn unique_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos()
}

mod control_tests;
mod lifecycle_tests;
mod load_tests;
mod play_tests;
mod stop_tests;
