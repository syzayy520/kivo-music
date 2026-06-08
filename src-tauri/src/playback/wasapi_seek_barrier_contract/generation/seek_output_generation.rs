/// Monotonic generation counter for seek output.
///
/// Each seek barrier request targets a specific SeekOutputGeneration.
/// Acks carry their own observed generation for comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SeekOutputGeneration(pub u64);

impl SeekOutputGeneration {
    /// Create a new generation with the given value.
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Return the raw value.
    pub fn value(self) -> u64 {
        self.0
    }

    /// Return the next generation (current value + 1).
    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }

    /// Whether this is the initial (zero) generation.
    pub fn is_initial(self) -> bool {
        self.0 == 0
    }

    /// Whether this generation matches the other.
    pub fn matches(self, other: SeekOutputGeneration) -> bool {
        self.0 == other.0
    }
}
