use super::super::{
    ProductionOutputRouteFailure, ProductionOutputRouteFormatDescriptor,
    ProductionOutputRouteFrameInput,
};
use super::capacity::ProductionOutputRouteCapacity;
use super::format::ProductionOutputRouteExpectedFormat;

const ROUTE_INPUT_FRAME_COUNT: usize = 1;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[allow(dead_code)]
pub(crate) struct ProductionOutputRouteInputAcceptancePolicy {
    capacity: ProductionOutputRouteCapacity,
    expected_format: ProductionOutputRouteExpectedFormat,
}

#[allow(dead_code)]
impl ProductionOutputRouteInputAcceptancePolicy {
    pub(crate) fn new(
        capacity: ProductionOutputRouteCapacity,
        expected_format: ProductionOutputRouteExpectedFormat,
    ) -> Self {
        Self {
            capacity,
            expected_format,
        }
    }

    pub(crate) fn accept(
        &self,
        input: ProductionOutputRouteFrameInput<'_>,
        pending_frames: usize,
    ) -> Result<(), ProductionOutputRouteFailure> {
        let actual_format = ProductionOutputRouteFormatDescriptor::from_stream(input.stream());
        self.expected_format.accept(actual_format)?;
        self.capacity
            .accept(pending_frames, ROUTE_INPUT_FRAME_COUNT)?;
        Ok(())
    }
}
