use super::super::runtime_core::id::OutputThreadRuntimeGeneration;
use super::super::runtime_core::intent::OutputThreadRuntimeIntent;

/// A single entry in the runtime command queue.
///
/// Pure value type — no real resources, no command sending.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputThreadRuntimeQueueEntry {
    /// The runtime generation this entry targets.
    pub generation: OutputThreadRuntimeGeneration,
    /// The intent (command) to apply.
    pub intent: OutputThreadRuntimeIntent,
    /// Monotonic sequence number assigned at acceptance time.
    pub sequence: u64,
}

#[allow(dead_code)]
impl OutputThreadRuntimeQueueEntry {
    /// Create a new queue entry.
    pub(crate) fn new(
        generation: OutputThreadRuntimeGeneration,
        intent: OutputThreadRuntimeIntent,
        sequence: u64,
    ) -> Self {
        Self {
            generation,
            intent,
            sequence,
        }
    }

    /// Whether this entry targets the given generation.
    pub(crate) fn is_for_generation(self, generation: OutputThreadRuntimeGeneration) -> bool {
        self.generation == generation
    }

    /// Whether this entry requests a shutdown (Stop or Close).
    pub(crate) fn is_shutdown_request(self) -> bool {
        self.intent.requests_shutdown()
    }

    /// Whether this entry clears the buffer (Flush or ResetDevice).
    pub(crate) fn clears_buffer(self) -> bool {
        self.intent.clears_buffer()
    }

    /// Return the next sequence number.
    pub(crate) fn next_sequence(self) -> u64 {
        self.sequence + 1
    }
}
