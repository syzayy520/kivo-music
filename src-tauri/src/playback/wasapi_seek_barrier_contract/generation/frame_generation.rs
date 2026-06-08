/// Generation tag on a frame flowing through the render pipeline.
///
/// Frames carry the generation they were produced under.
/// The render thread rejects stale frames whose FrameGeneration does not
/// match the active RenderEpoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FrameGeneration(pub u64);

impl FrameGeneration {
    /// Create a new frame generation with the given value.
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Return the raw value.
    pub fn value(self) -> u64 {
        self.0
    }
}
