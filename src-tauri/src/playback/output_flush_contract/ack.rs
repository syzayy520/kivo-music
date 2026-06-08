/// Acknowledgment that a flush operation completed.
///
/// For `TwoPhaseBarrier` ordering, this includes the generation counter that
/// the render thread advanced to, confirming stale frames are drained.
#[derive(Clone, Debug)]
pub enum OutputFlushAck {
    /// Flush completed successfully.
    Completed(OutputFlushCompletedDetails),

    /// Flush was rejected (e.g. route closed, sink not open).
    Rejected(OutputFlushRejectedReason),

    /// Flush failed with an error.
    Failed(OutputFlushFailedReason),

    /// Flush is not supported for this request/target combination.
    Unsupported,

    /// Flush completed but generation counter is stale
    /// (render thread did not advance to expected generation).
    StaleGeneration(OutputFlushStaleGenerationDetails),
}

/// Details for a Completed ack.
#[derive(Clone, Debug)]
pub struct OutputFlushCompletedDetails {
    /// Number of frames cleared from the pipeline buffer.
    pub buffer_frames_cleared: usize,
    /// Number of pending frames discarded by the sink.
    pub sink_discarded_frames: usize,
    /// The generation counter the render thread advanced to (if applicable).
    pub generation: Option<u64>,
}

/// Reasons a flush may be rejected.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OutputFlushRejectedReason {
    /// Output route is in a closed/failed state.
    RouteClosed,
    /// Output sink is not open.
    SinkNotOpen,
}

/// Reasons a flush may fail.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OutputFlushFailedReason {
    /// Sink-level flush failed.
    SinkFailure(String),
    /// Render thread did not respond.
    RenderThreadFailure(String),
    /// Device reset failed.
    DeviceResetFailure(String),
    /// Unknown error.
    Unknown(String),
}

/// Details for a StaleGeneration ack.
#[derive(Clone, Debug)]
pub struct OutputFlushStaleGenerationDetails {
    /// The expected generation counter.
    pub expected_generation: u64,
    /// The actual generation counter.
    pub actual_generation: u64,
}

impl OutputFlushAck {
    /// Returns the generation counter if this is a Completed ack.
    pub fn generation(&self) -> Option<u64> {
        match self {
            Self::Completed(details) => details.generation,
            _ => None,
        }
    }

    /// Returns whether this ack indicates success.
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Completed(_))
    }
}
