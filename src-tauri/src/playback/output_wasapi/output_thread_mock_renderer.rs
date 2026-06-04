/// Pure in-memory mock renderer capacity for testing.
///
/// No real device, no WASAPI, no thread.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct OutputThreadMockRenderer {
    free_frames: u32,
}

#[allow(dead_code)]
impl OutputThreadMockRenderer {
    /// Create a new mock renderer with explicit free frames.
    pub(crate) fn new(free_frames: u32) -> Self {
        Self { free_frames }
    }

    /// Full renderer (no free frames).
    pub(crate) fn full() -> Self {
        Self { free_frames: 0 }
    }

    /// Renderer with specified free frames.
    pub(crate) fn with_free_frames(frames: u32) -> Self {
        Self {
            free_frames: frames,
        }
    }

    /// Get the number of free frames.
    pub(crate) fn free_frames(self) -> u32 {
        self.free_frames
    }

    /// Whether the renderer has capacity.
    pub(crate) fn has_capacity(self) -> bool {
        self.free_frames > 0
    }
}
