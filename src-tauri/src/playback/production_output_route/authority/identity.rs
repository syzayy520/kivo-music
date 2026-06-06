use std::num::NonZeroU64;

/// Explicit deterministic route identity contract.
///
/// Backed by `NonZeroU64` — zero identity cannot exist.
/// Created from caller-provided deterministic value only.
/// No global counter / timestamp / UUID / random / runtime generator.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct ProductionOutputRouteIdentity {
    value: NonZeroU64,
}

impl ProductionOutputRouteIdentity {
    /// Create identity from an explicit `NonZeroU64` value.
    #[allow(dead_code)]
    pub(crate) fn new(value: NonZeroU64) -> Self {
        Self { value }
    }

    /// Try to create identity from a `u64`.
    /// Returns `None` if `value` is zero.
    #[allow(dead_code)]
    pub(crate) fn try_new(value: u64) -> Option<Self> {
        NonZeroU64::new(value).map(|nz| Self { value: nz })
    }

    /// Returns the underlying `NonZeroU64` value.
    #[allow(dead_code)]
    pub(crate) fn value(&self) -> NonZeroU64 {
        self.value
    }
}
