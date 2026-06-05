use super::id::{OutputThreadRuntimeGeneration, OutputThreadRuntimeId};
use super::status::OutputThreadRuntimeStatus;
use super::super::output_thread_core::state::OutputThreadStats;

/// Snapshot of the output thread runtime state.
///
/// Pure data — no real thread, no real queue, no real device, no real buffer.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct OutputThreadRuntimeSnapshot {
    /// Unique identifier for this runtime instance.
    pub id: OutputThreadRuntimeId,
    /// Generation counter for successive lifetimes.
    pub generation: OutputThreadRuntimeGeneration,
    /// Current runtime status.
    pub status: OutputThreadRuntimeStatus,
    /// Accumulated statistics.
    pub stats: OutputThreadStats,
}

impl OutputThreadRuntimeSnapshot {
    /// Create a new snapshot with explicit values.
    #[allow(dead_code)]
    pub(crate) fn new(
        id: OutputThreadRuntimeId,
        generation: OutputThreadRuntimeGeneration,
        status: OutputThreadRuntimeStatus,
        stats: OutputThreadStats,
    ) -> Self {
        Self {
            id,
            generation,
            status,
            stats,
        }
    }

    /// Create an empty snapshot (all defaults).
    #[allow(dead_code)]
    pub(crate) fn empty() -> Self {
        Self {
            id: OutputThreadRuntimeId::default(),
            generation: OutputThreadRuntimeGeneration::default(),
            status: OutputThreadRuntimeStatus::inactive(),
            stats: OutputThreadStats::default(),
        }
    }

    /// Whether the runtime is currently active (running).
    #[allow(dead_code)]
    pub(crate) fn is_active(&self) -> bool {
        self.status.is_running()
    }

    /// Whether the runtime can accept audio frames right now.
    #[allow(dead_code)]
    pub(crate) fn can_accept_frames(&self) -> bool {
        self.status.can_accept_frames()
    }

    /// Get the generation counter.
    #[allow(dead_code)]
    pub(crate) fn generation(&self) -> OutputThreadRuntimeGeneration {
        self.generation
    }
}
