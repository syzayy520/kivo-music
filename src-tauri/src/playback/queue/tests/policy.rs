use crate::playback::queue::PlaybackQueue;
use crate::playback::queue_policy::{decide_queue_step, QueueStepReason};
use crate::playback::types::{PlaybackTrack, RepeatMode, TrackId};

fn track(index: usize) -> PlaybackTrack {
    PlaybackTrack {
        id: TrackId(format!("track-{index}")),
        title: format!("Track {index}"),
        artist: "Artist".to_string(),
        source_path: format!("C:/Music/track-{index}.wav"),
    }
}

fn queue_with_current_last(repeat_mode: RepeatMode) -> PlaybackQueue {
    let mut queue = PlaybackQueue::default();
    queue.repeat_mode = repeat_mode;
    queue.append(track(1));
    queue.append(track(2));
    queue.set_current_index(1).expect("index should be valid");
    queue
}

#[test]
fn natural_advance_repeat_off_reaches_end() {
    let queue = queue_with_current_last(RepeatMode::Off);
    let decision = decide_queue_step(&queue, QueueStepReason::NaturalAdvance);

    assert!(decision.target_index.is_none());
    assert!(decision.reached_end);
}

#[test]
fn natural_advance_repeat_one_stays_on_current_track() {
    let queue = queue_with_current_last(RepeatMode::One);
    let decision = decide_queue_step(&queue, QueueStepReason::NaturalAdvance);

    assert_eq!(decision.target_index, Some(1));
    assert!(!decision.reached_end);
}

#[test]
fn natural_advance_repeat_all_wraps_to_first_track() {
    let queue = queue_with_current_last(RepeatMode::All);
    let decision = decide_queue_step(&queue, QueueStepReason::NaturalAdvance);

    assert_eq!(decision.target_index, Some(0));
    assert!(!decision.reached_end);
}
