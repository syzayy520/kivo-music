#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ProductionOutputRouteUnderrun {
    requested_frames: u32,
    available_frames: u32,
}

impl ProductionOutputRouteUnderrun {
    pub(crate) fn new(requested_frames: u32, available_frames: u32) -> Self {
        Self {
            requested_frames,
            available_frames,
        }
    }

    pub(crate) fn requested_frames(&self) -> u32 {
        self.requested_frames
    }

    pub(crate) fn available_frames(&self) -> u32 {
        self.available_frames
    }

    pub(crate) fn missing_frames(&self) -> u32 {
        self.requested_frames.saturating_sub(self.available_frames)
    }
}
