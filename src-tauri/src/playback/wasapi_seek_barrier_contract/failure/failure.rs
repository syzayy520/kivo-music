use super::super::generation::StaleGeneration;

/// Failure modes for a WASAPI seek barrier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WasapiSeekBarrierFailure {
    /// Queue drain failed.
    QueueDrainFailed(String),
    /// Ring buffer reset failed.
    RingBufferResetFailed(String),
    /// Render thread rejected the barrier.
    RenderThreadRejected(String),
    /// Render thread failed during barrier.
    RenderThreadFailed(String),
    /// Device reset failed.
    DeviceResetFailed(String),
    /// Audio device was lost.
    DeviceLost(String),
    /// Barrier target or policy is unsupported.
    UnsupportedBarrier(String),
    /// Generation mismatch — stale barrier.
    StaleGeneration(StaleGeneration),
    /// Unknown failure.
    Unknown(String),
}

impl WasapiSeekBarrierFailure {
    /// Whether this failure is a stale generation mismatch.
    pub fn is_stale_generation(&self) -> bool {
        matches!(self, Self::StaleGeneration(_))
    }

    /// Whether this failure indicates device loss.
    pub fn is_device_lost(&self) -> bool {
        matches!(self, Self::DeviceLost(_))
    }
}
