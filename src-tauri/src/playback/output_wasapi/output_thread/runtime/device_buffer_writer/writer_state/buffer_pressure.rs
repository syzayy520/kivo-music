//! Buffer pressure level classification.

/// Buffer pressure level based on fill percentage.
///
/// Categorizes the buffer fill state into severity tiers.
/// Pure data — no IO, no real buffer.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum BufferPressure {
    /// Buffer fill is below 50%.
    #[default]
    Relaxed,
    /// Buffer fill is between 50% and 74%.
    Moderate,
    /// Buffer fill is between 75% and 89%.
    High,
    /// Buffer fill is 90% or above.
    Critical,
}

impl BufferPressure {
    /// Returns true if the pressure level indicates the buffer is under significant pressure.
    pub fn is_concerning(&self) -> bool {
        matches!(self, Self::High | Self::Critical)
    }

    /// Returns true if the pressure level is critical.
    pub fn is_critical(&self) -> bool {
        matches!(self, Self::Critical)
    }
}
