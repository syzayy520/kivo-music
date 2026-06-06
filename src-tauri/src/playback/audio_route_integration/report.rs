#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioRouteIntegrationReport {
    pub initialized: bool,
    pub coordinator_initialized: bool,
    pub closed: bool,
    pub pending_frames: u32,
    pub capacity_frames: u32,
    pub total_requested_frames: u64,
    pub total_accepted_frames: u64,
    pub total_rejected_frames: u64,
    pub input_count: u64,
    pub backpressure_count: u64,
    pub partial_write_count: u64,
    pub source_closed_seen: bool,
}
