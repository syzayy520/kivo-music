use std::collections::VecDeque;

use super::output::AudioOutputFrame;

/// Pipeline buffer boundary for decoded output frames.
///
/// Stores `AudioOutputFrame` in FIFO order for drain to output sink.
/// This buffer does NOT:
/// - know about decoders
/// - know about output sinks
/// - know about engine/manager/UI
/// - connect to real audio devices
/// - produce sound
#[derive(Clone, Debug, Default)]
pub(in crate::playback) struct NativePipelineBuffer {
    frames: VecDeque<AudioOutputFrame>,
}

impl NativePipelineBuffer {
    pub(in crate::playback) fn new() -> Self {
        Self::default()
    }

    pub(in crate::playback) fn enqueue_frame(&mut self, frame: AudioOutputFrame) {
        self.frames.push_back(frame);
    }

    pub(in crate::playback) fn drain_next_frame(&mut self) -> Option<AudioOutputFrame> {
        self.frames.pop_front()
    }

    pub(in crate::playback) fn clear(&mut self) {
        self.frames.clear();
    }

    pub(in crate::playback) fn len(&self) -> usize {
        self.frames.len()
    }

    #[allow(dead_code)]
    pub(in crate::playback) fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }
}
