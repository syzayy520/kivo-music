use super::super::ProductionOutputRouteBackpressure;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[allow(dead_code)]
pub(crate) struct ProductionOutputRouteCapacity {
    capacity_frames: usize,
}

#[allow(dead_code)]
impl ProductionOutputRouteCapacity {
    pub(crate) fn new(capacity_frames: usize) -> Option<Self> {
        (capacity_frames > 0).then_some(Self { capacity_frames })
    }

    pub(crate) fn capacity_frames(&self) -> usize {
        self.capacity_frames
    }

    pub(crate) fn accept(
        &self,
        pending_frames: usize,
        incoming_frame_count: usize,
    ) -> Result<(), ProductionOutputRouteBackpressure> {
        let projected_pending_frames = pending_frames.saturating_add(incoming_frame_count);
        let backpressure =
            ProductionOutputRouteBackpressure::new(projected_pending_frames, self.capacity_frames);
        let is_over_capacity = backpressure.pending_frames() > backpressure.capacity_frames();

        if is_over_capacity {
            debug_assert!(backpressure.is_capacity_reached());
            Err(backpressure)
        } else {
            Ok(())
        }
    }
}
