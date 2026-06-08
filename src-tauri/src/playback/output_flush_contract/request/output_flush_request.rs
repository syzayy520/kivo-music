/// Describes why an output flush is being requested.
///
/// Each variant captures a distinct control-flow origin; the contract layer
/// uses this to decide ordering, ack requirements, and failure policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OutputFlushRequest {
    /// Seek to a new playback position (ms).
    /// Requires full barrier: buffer clear + sink flush + generation ack.
    Seek { position_ms: u64 },

    /// Transition from Playing → Paused.
    /// Requires sink-level flush to silence output before pause completes.
    Pause,

    /// Transition from Playing → Stopped.
    /// Requires full barrier to silence output and reset state.
    Stop,

    /// Manual / explicit flush command (e.g. user-initiated buffer clear).
    /// Uses SinkOnly target by default; caller may override.
    Manual,
}

impl OutputFlushRequest {
    /// Returns whether this request is a seek operation.
    pub fn is_seek(&self) -> bool {
        matches!(self, Self::Seek { .. })
    }
}
