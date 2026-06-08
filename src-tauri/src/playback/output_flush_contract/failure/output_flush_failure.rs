/// Failure modes for an output flush operation.
///
/// Each variant captures a distinct failure origin so the caller can
/// decide whether to retry, rollback, or escalate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OutputFlushFailure {
    /// The output route is closed or not opened.
    RouteClosed(String),

    /// The sink-level flush (IAudioClient / RingBuffer reset) failed.
    SinkFailure(String),

    /// The output route is experiencing backpressure.
    Backpressure(String),

    /// The flush target is not supported for this request.
    UnsupportedTarget(String),

    /// The render thread did not advance to the expected generation
    /// within the timeout window.
    StaleGeneration {
        expected_generation: u64,
        actual_generation: u64,
    },

    /// The render thread did not respond or crashed.
    RenderThreadFailure(String),

    /// The device reset (IAudioClient::Reset) failed.
    DeviceResetFailure(String),

    /// Unknown error.
    Unknown(String),
}

impl OutputFlushFailure {
    /// Returns whether this failure indicates data loss.
    pub fn is_data_loss(&self) -> bool {
        matches!(self, Self::SinkFailure(_) | Self::DeviceResetFailure(_))
    }

    /// Returns whether this failure is retryable.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::Backpressure(_) | Self::StaleGeneration { .. } | Self::RenderThreadFailure(_)
        )
    }

    /// Returns whether this failure is fatal (not retryable, not data loss).
    pub fn is_fatal(&self) -> bool {
        matches!(
            self,
            Self::RouteClosed(_) | Self::UnsupportedTarget(_) | Self::Unknown(_)
        )
    }
}
