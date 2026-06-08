use super::seek_output_generation::SeekOutputGeneration;

/// Describes an expected/actual generation mismatch for seek output.
///
/// This is the mismatch value type. It does NOT replace RenderEpochMismatch.
/// Ack stale-generation states may carry StaleGeneration when representing
/// seek-output-generation mismatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StaleGeneration {
    /// The generation that was expected.
    pub expected: SeekOutputGeneration,
    /// The generation that was actually observed.
    pub actual: SeekOutputGeneration,
}

impl StaleGeneration {
    /// Create a new StaleGeneration mismatch.
    pub fn new(expected: SeekOutputGeneration, actual: SeekOutputGeneration) -> Self {
        Self { expected, actual }
    }
}
