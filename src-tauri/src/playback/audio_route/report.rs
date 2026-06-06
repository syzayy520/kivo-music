use crate::playback::audio_bridge::SourceToRingBufferBridgeReport;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioRouteReport {
    pub initialized: bool,
    pub capacity_frames: u32,
    pub pending_frames: u32,
    pub total_requested_frames: u64,
    pub total_accepted_frames: u64,
    pub total_rejected_frames: u64,
    pub last_requested_frames: u32,
    pub last_accepted_frames: u32,
    pub last_rejected_frames: u32,
    pub last_source_closed: bool,
    pub last_ring_buffer_full: bool,
    pub last_partial_write: bool,
}

impl AudioRouteReport {
    pub(super) fn initialized(capacity_frames: u32) -> Self {
        Self {
            initialized: true,
            capacity_frames,
            pending_frames: 0,
            total_requested_frames: 0,
            total_accepted_frames: 0,
            total_rejected_frames: 0,
            last_requested_frames: 0,
            last_accepted_frames: 0,
            last_rejected_frames: 0,
            last_source_closed: false,
            last_ring_buffer_full: false,
            last_partial_write: false,
        }
    }

    pub(super) fn record_bridge_report(&mut self, bridge: &SourceToRingBufferBridgeReport) {
        self.pending_frames = bridge.pending_frames_after_write;
        self.total_requested_frames += u64::from(bridge.requested_frames);
        self.total_accepted_frames += u64::from(bridge.accepted_frames);
        self.total_rejected_frames += u64::from(bridge.rejected_frames);
        self.last_requested_frames = bridge.requested_frames;
        self.last_accepted_frames = bridge.accepted_frames;
        self.last_rejected_frames = bridge.rejected_frames;
        self.last_source_closed = bridge.source_closed;
        self.last_ring_buffer_full = bridge.ring_buffer_full;
        self.last_partial_write = bridge.partial_write;
    }

    pub(super) fn reset(&mut self) {
        *self = Self::initialized(self.capacity_frames);
    }
}
