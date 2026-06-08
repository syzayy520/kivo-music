/// Ordering for WASAPI seek barrier phases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WasapiSeekBarrierOrdering {
    /// Drain the runtime queue before resetting the ring buffer.
    QueueDrainBeforeRingBufferReset,
    /// Reset the ring buffer before advancing the render epoch.
    RingBufferResetBeforeRenderEpochAdvance,
    /// Advance the render epoch before resetting the device.
    RenderEpochAdvanceBeforeDeviceReset,
    /// Reset the device before resuming render.
    DeviceResetBeforeRenderResume,
    /// Two-phase barrier: render-side barrier, then external decoder seek.
    TwoPhaseRenderBarrier,
}

impl WasapiSeekBarrierOrdering {
    /// Whether this ordering is render-side only (no decoder involvement).
    ///
    /// Even render-side-only ack does NOT mean public timeline may be committed.
    pub fn is_render_side_only(self) -> bool {
        !matches!(self, Self::TwoPhaseRenderBarrier)
    }

    /// Whether this ordering requires external decoder seek success
    /// before the barrier is fully satisfied.
    pub fn requires_external_decoder_seek_success(self) -> bool {
        matches!(self, Self::TwoPhaseRenderBarrier)
    }

    /// Whether this ordering requires future orchestration to complete
    /// the full seek (e.g., public timeline commit).
    pub fn requires_future_orchestration(self) -> bool {
        true
    }
}
