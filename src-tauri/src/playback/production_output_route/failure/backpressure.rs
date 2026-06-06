#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ProductionOutputRouteBackpressure {
    pending_frames: usize,
    capacity_frames: usize,
}

impl ProductionOutputRouteBackpressure {
    pub(crate) fn new(pending_frames: usize, capacity_frames: usize) -> Self {
        Self {
            pending_frames,
            capacity_frames,
        }
    }

    pub(crate) fn pending_frames(&self) -> usize {
        self.pending_frames
    }

    pub(crate) fn capacity_frames(&self) -> usize {
        self.capacity_frames
    }

    pub(crate) fn is_capacity_reached(&self) -> bool {
        self.pending_frames >= self.capacity_frames
    }
}
