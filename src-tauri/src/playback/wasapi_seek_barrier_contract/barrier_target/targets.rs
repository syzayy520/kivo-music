use super::target::WasapiBarrierTarget;

/// Ordered set of barrier targets.
///
/// Preserves insertion order. Does not deduplicate.
/// Provides membership semantics via contains().
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WasapiBarrierTargets {
    targets: Vec<WasapiBarrierTarget>,
}

impl WasapiBarrierTargets {
    /// Create a new target set from a slice.
    pub fn new(targets: &[WasapiBarrierTarget]) -> Self {
        Self {
            targets: targets.to_vec(),
        }
    }

    /// Whether the given target is present.
    pub fn contains(&self, target: WasapiBarrierTarget) -> bool {
        self.targets.contains(&target)
    }

    /// Iterate over targets in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = &WasapiBarrierTarget> {
        self.targets.iter()
    }

    /// Number of targets.
    pub fn len(&self) -> usize {
        self.targets.len()
    }

    /// Whether the target set is empty.
    pub fn is_empty(&self) -> bool {
        self.targets.is_empty()
    }
}
