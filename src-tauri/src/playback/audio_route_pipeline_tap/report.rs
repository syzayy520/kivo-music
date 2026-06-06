use super::error::AudioRoutePipelineTapErrorKind;
use super::frame_kind::AudioRouteTapFrameKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioRoutePipelineTapReport {
    pub initialized: bool,
    pub closed: bool,
    pub frame_kind: AudioRouteTapFrameKind,
    pub input_count: u64,
    pub total_accepted_frames: u64,
    pub total_rejected_frames: u64,
    pub backpressure_count: u64,
    pub partial_write_count: u64,
    pub source_closed_seen: bool,
    pub pending_frames: u32,
    pub capacity_frames: u32,
    pub last_error: Option<AudioRoutePipelineTapErrorKind>,
}
