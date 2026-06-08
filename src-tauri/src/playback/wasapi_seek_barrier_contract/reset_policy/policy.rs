/// Reset policy for WASAPI seek barrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WasapiResetPolicy {
    /// Only flush the pipeline buffer. No device reset.
    FlushOnly,
    /// Reset the audio device (IAudioClient::Reset).
    ResetDevice,
    /// Stop, reset, and restart the device (Stop → Reset → Start).
    StopResetStart,
}

impl WasapiResetPolicy {
    /// Whether this policy requires a device reset ack.
    pub fn requires_device_reset(self) -> bool {
        matches!(self, Self::ResetDevice | Self::StopResetStart)
    }

    /// Whether this policy requires a full stop-reset-start cycle.
    pub fn requires_stop_start_cycle(self) -> bool {
        matches!(self, Self::StopResetStart)
    }

    /// Whether this policy is flush-only (no device reset).
    pub fn is_flush_only(self) -> bool {
        matches!(self, Self::FlushOnly)
    }

    /// Whether this policy can satisfy the Playing seek device-buffer barrier.
    ///
    /// FlushOnly alone is NOT sufficient for Playing seek device-buffer safety.
    /// ResetDevice and StopResetStart are candidates.
    pub fn can_satisfy_playing_seek_device_barrier(self) -> bool {
        matches!(self, Self::ResetDevice | Self::StopResetStart)
    }
}
