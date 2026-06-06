#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceToRingBufferBridgeReport {
    pub requested_frames: u32,
    pub accepted_frames: u32,
    pub rejected_frames: u32,
    pub bytes_written: usize,
    pub frames_written: u32,
    pub pending_frames_after_write: u32,
    pub source_closed: bool,
    pub ring_buffer_full: bool,
    pub format_validated: bool,
    pub partial_write: bool,
}
