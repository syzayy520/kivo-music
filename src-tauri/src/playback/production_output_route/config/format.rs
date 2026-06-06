use crate::playback::decoder::AudioStreamInfo;

use super::super::{
    ProductionOutputRouteFormatDescriptor, ProductionOutputRouteFormatMismatch,
    ProductionOutputRouteStreamFormat,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[allow(dead_code)]
pub(crate) struct ProductionOutputRouteExpectedFormat {
    descriptor: ProductionOutputRouteFormatDescriptor,
}

#[allow(dead_code)]
impl ProductionOutputRouteExpectedFormat {
    pub(crate) fn from_stream(stream: &AudioStreamInfo) -> Self {
        Self::from_descriptor(ProductionOutputRouteFormatDescriptor::from_stream(stream))
    }

    pub(crate) fn from_descriptor(descriptor: ProductionOutputRouteFormatDescriptor) -> Self {
        Self { descriptor }
    }

    pub(crate) fn descriptor(&self) -> ProductionOutputRouteFormatDescriptor {
        self.descriptor
    }

    pub(crate) fn accept(
        &self,
        actual: ProductionOutputRouteFormatDescriptor,
    ) -> Result<(), ProductionOutputRouteFormatMismatch> {
        let mismatch = ProductionOutputRouteFormatMismatch::new(self.descriptor, actual);
        let expected_fields = descriptor_fields(mismatch.expected());
        let actual_fields = descriptor_fields(mismatch.actual());
        let fields_mismatch = expected_fields != actual_fields;

        debug_assert_eq!(mismatch.is_mismatch(), fields_mismatch);
        if fields_mismatch {
            Err(mismatch)
        } else {
            Ok(())
        }
    }
}

fn descriptor_fields(
    descriptor: ProductionOutputRouteFormatDescriptor,
) -> (u32, u16, ProductionOutputRouteStreamFormat) {
    (
        descriptor.sample_rate_hz(),
        descriptor.channels(),
        descriptor.sample_format(),
    )
}
