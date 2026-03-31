//! FFmpeg decoder (LGPL dynamic linking).
//!
//! This module intentionally loads FFmpeg shared libraries at runtime.
//! That keeps our binary LGPL-compatible and avoids bundling GPL/nonfree builds.

pub mod header_probe;
pub mod media_probe;
pub mod probe;

mod api;
mod decoder;
mod dll_dirs;
mod dll_names;
mod dll_set;
mod dynlib;
mod ffi;
mod media_probe_abi;
mod resample;
mod resample_frame;
mod types;

pub use api::FfmpegLoadOptions;
pub use decoder::FfmpegDecoder;
pub use media_probe::{probe_media, FfmpegMediaInfo, FfmpegMediaTags};
pub use types::PcmChunkF32;
