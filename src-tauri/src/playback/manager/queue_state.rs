use crate::playback::errors::{PlaybackError, PlaybackResult};
use crate::playback::queue::PlaybackQueue;
use crate::playback::queue_policy::{decide_queue_step, QueueStepReason};
use crate::playback::types::{PlaybackTrack, RepeatMode};

pub fn append(queue: &mut PlaybackQueue, track: PlaybackTrack) -> PlaybackQueue {
    queue.append(track);
    queue.clone()
}

pub fn remove(queue: &mut PlaybackQueue, index: usize) -> PlaybackResult<PlaybackQueue> {
    queue.remove(index)?;
    Ok(queue.clone())
}

pub fn set_repeat_mode(queue: &mut PlaybackQueue, repeat_mode: RepeatMode) -> PlaybackQueue {
    queue.repeat_mode = repeat_mode;
    queue.clone()
}

pub fn set_shuffle(queue: &mut PlaybackQueue, shuffle: bool) -> PlaybackQueue {
    queue.shuffle = shuffle;
    queue.clone()
}

pub fn select_track(queue: &mut PlaybackQueue, index: usize) -> PlaybackResult<PlaybackTrack> {
    queue.set_current_index(index)?;
    queue
        .current_track()
        .ok_or_else(|| PlaybackError::Queue("queue has no current track".to_string()))
}

pub fn select_next_track(queue: &mut PlaybackQueue) -> PlaybackResult<PlaybackTrack> {
    let decision = decide_queue_step(queue, QueueStepReason::Next);
    let index = decision
        .target_index
        .ok_or_else(|| PlaybackError::Queue("queue reached end".to_string()))?;

    select_track(queue, index)
}

pub fn select_previous_track(queue: &mut PlaybackQueue) -> PlaybackResult<PlaybackTrack> {
    let decision = decide_queue_step(queue, QueueStepReason::Previous);
    let index = decision
        .target_index
        .ok_or_else(|| PlaybackError::Queue("queue has no previous track".to_string()))?;

    select_track(queue, index)
}
