mod decode_to_wav;
mod decode_to_wav_utils;
mod play_file_wasapi_shared;
pub mod service;
mod wav_writer;

pub use decode_to_wav::{run_ffmpeg_decode_to_wav, DecodeToWavArgs, DecodeToWavResult};
pub use play_file_wasapi_shared::{
    run_ffmpeg_play_file_wasapi_shared, PlayFileArgs, PlayFileResult,
};

pub use service::{
    AudioService, AudioSessionState, AudioSessionStatus, PlayStartOptions, QueueSnapshot,
};
