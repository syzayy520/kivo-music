/// Render epoch — the active generation for the render thread.
///
/// Distinguishes which render-thread lifetime is currently active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RenderEpoch(pub u64);

impl RenderEpoch {
    /// Create a new render epoch with the given value.
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Return the raw value.
    pub fn value(self) -> u64 {
        self.0
    }

    /// Return the next epoch (current value + 1).
    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }

    /// Whether this is the initial (zero) epoch.
    pub fn is_initial(self) -> bool {
        self.0 == 0
    }

    /// Whether this epoch matches the other.
    pub fn matches(self, other: RenderEpoch) -> bool {
        self.0 == other.0
    }
}
