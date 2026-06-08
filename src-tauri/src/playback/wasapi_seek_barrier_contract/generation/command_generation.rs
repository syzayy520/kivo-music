/// Generation tag on a command entering the runtime queue.
///
/// Commands carry the generation they were issued under.
/// The runtime queue rejects stale commands whose CommandGeneration does not
/// match the active SeekOutputGeneration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CommandGeneration(pub u64);

impl CommandGeneration {
    /// Create a new command generation with the given value.
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Return the raw value.
    pub fn value(self) -> u64 {
        self.0
    }
}
