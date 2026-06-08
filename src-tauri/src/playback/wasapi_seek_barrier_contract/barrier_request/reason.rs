/// Reason for requesting a WASAPI seek barrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WasapiSeekBarrierReason {
    /// Barrier for a Paused seek.
    PausedSeek,
    /// Barrier for a Playing seek.
    PlayingSeek,
    /// Barrier for a Stop command.
    Stop,
    /// Manual barrier request.
    Manual,
}
