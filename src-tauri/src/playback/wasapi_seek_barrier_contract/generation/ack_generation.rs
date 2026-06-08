/// Generation tag carried by an ack.
///
/// Ack structs carry AckGeneration, not SeekOutputGeneration.
/// Barrier satisfaction methods compare AckGeneration.value() against
/// expected SeekOutputGeneration.value().
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AckGeneration(pub u64);

impl AckGeneration {
    /// Create a new ack generation with the given value.
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Return the raw value.
    pub fn value(self) -> u64 {
        self.0
    }
}
