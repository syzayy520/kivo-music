/// Decision for a frame or command based on render epoch comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderEpochDecision {
    /// The frame/command matches the active epoch — accept it.
    AcceptCurrent,
    /// The frame/command is from a stale epoch — reject it.
    RejectStale,
    /// The frame is stale; render silence instead.
    RenderSilence,
    /// A barrier is pending; await it before proceeding.
    AwaitBarrier,
}
