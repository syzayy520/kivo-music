use std::collections::VecDeque;

use super::decoder::DecodedAudioFrame;

#[derive(Clone, Debug)]
pub struct AudioFrameBuffer {
    frames: VecDeque<DecodedAudioFrame>,
    capacity_frames: usize,
}

impl AudioFrameBuffer {
    pub fn new(capacity_frames: usize) -> Self {
        Self {
            frames: VecDeque::new(),
            capacity_frames,
        }
    }

    pub fn capacity_frames(&self) -> usize {
        self.capacity_frames
    }

    pub fn len(&self) -> usize {
        self.frames.len()
    }

    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    pub fn clear(&mut self) {
        self.frames.clear();
    }

    pub fn push(&mut self, frame: DecodedAudioFrame) -> Option<DecodedAudioFrame> {
        if self.capacity_frames == 0 {
            return Some(frame);
        }

        self.frames.push_back(frame);

        if self.frames.len() > self.capacity_frames {
            self.frames.pop_front()
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<DecodedAudioFrame> {
        self.frames.pop_front()
    }
}
