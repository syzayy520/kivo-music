/// Declares the execution order between output flush and decoder seek.
///
/// This is the critical ordering contract that P0-158 PRE identified as
/// missing from the current NativePipeline::flush_output implementation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FlushOrdering {
    /// Flush output before decoder seek.
    /// Used when output must be silenced before seeking to prevent stale frames.
    OutputBeforeDecoderSeek,

    /// Decoder seek before output flush.
    /// Used when decoder position must be set before output can flush correctly.
    DecoderSeekBeforeOutput,

    /// Two-phase barrier: output flush + generation ack, then decoder seek,
    /// then second output flush + generation ack.
    /// Required for seek operations that need stale-frame prevention.
    TwoPhaseBarrier,
}

impl FlushOrdering {
    /// Returns whether this ordering flushes output before decoder seek.
    pub fn flushes_output_before_seek(&self) -> bool {
        matches!(self, Self::OutputBeforeDecoderSeek | Self::TwoPhaseBarrier)
    }

    /// Returns whether this ordering flushes output after decoder seek.
    pub fn flushes_output_after_seek(&self) -> bool {
        matches!(self, Self::DecoderSeekBeforeOutput | Self::TwoPhaseBarrier)
    }

    /// Returns whether this ordering requires two-phase barrier.
    pub fn is_two_phase(&self) -> bool {
        matches!(self, Self::TwoPhaseBarrier)
    }
}
