use crate::playback::decoder::AudioStreamInfo;
use crate::playback::output::AudioOutputFrame;

#[derive(Clone, Copy, Debug)]
pub(crate) struct ProductionOutputRouteFrameInput<'a> {
    frame: &'a AudioOutputFrame,
}

impl<'a> ProductionOutputRouteFrameInput<'a> {
    pub(crate) fn from_frame(frame: &'a AudioOutputFrame) -> Self {
        Self { frame }
    }

    pub(crate) fn stream(&self) -> &'a AudioStreamInfo {
        &self.frame.stream
    }

    pub(crate) fn position_ms(&self) -> u64 {
        self.frame.position_ms
    }

    pub(crate) fn sample_count(&self) -> usize {
        self.frame.samples.len()
    }
}
