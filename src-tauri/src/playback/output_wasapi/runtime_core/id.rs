/// Unique identifier for an output thread runtime instance.
///
/// Pure value type — no randomness, no time, no global state.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub(crate) struct OutputThreadRuntimeId(pub u64);

impl OutputThreadRuntimeId {
    /// Create a new runtime ID with the given value.
    #[allow(dead_code)]
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    /// Whether this ID is the zero (empty) value.
    #[allow(dead_code)]
    pub(crate) fn is_empty(self) -> bool {
        self.0 == 0
    }
}

/// Monotonic generation counter for an output thread runtime.
///
/// Each time a new runtime is created for the same logical sink,
/// the generation increments to distinguish successive lifetimes.
///
/// Pure value type — no randomness, no time, no global state.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub(crate) struct OutputThreadRuntimeGeneration(pub u64);

impl OutputThreadRuntimeGeneration {
    /// Create a new generation with the given value.
    #[allow(dead_code)]
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    /// Return the next generation (current value + 1).
    #[allow(dead_code)]
    pub(crate) fn next(self) -> Self {
        Self(self.0 + 1)
    }

    /// Whether this is the initial (zero) generation.
    #[allow(dead_code)]
    pub(crate) fn is_initial(self) -> bool {
        self.0 == 0
    }
}
