use crate::playback::decoder_request::AudioDecoderOpenRequest;
use crate::playback::native_pipeline::NativePipeline;

pub fn unique_suffix() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos()
}

pub fn write_test_wav() -> String {
    let path = std::env::temp_dir().join(format!(
        "kivo-pipeline-loop-{}-{}.wav",
        std::process::id(),
        unique_suffix()
    ));
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 48_000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(&path, spec).expect("create wav test file");

    for sample in [0_i16, 1200, -1200, 2400, -2400, 3600, -3600, 0] {
        writer.write_sample(sample).expect("write wav sample");
    }
    writer.finalize().expect("finalize wav test file");

    path.to_string_lossy().into_owned()
}

pub fn write_long_test_wav() -> String {
    let path = std::env::temp_dir().join(format!(
        "kivo-pipeline-loop-long-{}-{}.wav",
        std::process::id(),
        unique_suffix()
    ));
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 48_000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(&path, spec).expect("create long wav test file");

    for sample in 0..512_i16 {
        writer.write_sample(sample).expect("write wav sample");
    }
    writer.finalize().expect("finalize long wav test file");

    path.to_string_lossy().into_owned()
}

pub fn open_wav_pipeline(path: &str) -> NativePipeline {
    let mut pipeline = NativePipeline::new();
    let request = AudioDecoderOpenRequest {
        track_id: "loop-test".to_string(),
        source_path: path.to_string(),
    };
    pipeline.open_decoder(request, 0).expect("open decoder");
    pipeline
}
