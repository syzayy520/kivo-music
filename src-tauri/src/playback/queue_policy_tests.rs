use super::queue::PlaybackQueue;
use super::queue_policy::{decide_queue_step, QueueStepReason};
use super::types::{PlaybackTrack, RepeatMode, TrackId};

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

#[test]
fn next_moves_to_following_index_when_available() {
    let mut queue = PlaybackQueue::default();
    queue.append(track(1));
    queue.append(track(2));
    queue.append(track(3));
    queue.set_current_index(1).expect("index should be valid");

    let decision = decide_queue_step(&queue, QueueStepReason::Next);

    assert_eq!(decision.target_index, Some(2));
    assert!(!decision.reached_end);
}

#[test]
fn next_from_last_reaches_end() {
    let queue = queue_with_current_last(RepeatMode::Off);

    let decision = decide_queue_step(&queue, QueueStepReason::Next);

    assert!(decision.target_index.is_none());
    assert!(decision.reached_end);
}

#[test]
fn previous_from_middle_moves_back_one() {
    let mut queue = PlaybackQueue::default();
    queue.append(track(1));
    queue.append(track(2));
    queue.append(track(3));
    queue.set_current_index(2).expect("index should be valid");

    let decision = decide_queue_step(&queue, QueueStepReason::Previous);

    assert_eq!(decision.target_index, Some(1));
    assert!(!decision.reached_end);
}

#[test]
fn previous_from_start_stays_at_zero() {
    let mut queue = PlaybackQueue::default();
    queue.append(track(1));
    queue.append(track(2));
    queue.set_current_index(0).expect("index should be valid");

    let decision = decide_queue_step(&queue, QueueStepReason::Previous);

    assert_eq!(decision.target_index, Some(0));
    assert!(!decision.reached_end);
}

#[test]
fn empty_queue_always_reports_end() {
    let queue = PlaybackQueue::default();

    let next = decide_queue_step(&queue, QueueStepReason::Next);
    let previous = decide_queue_step(&queue, QueueStepReason::Previous);
    let natural = decide_queue_step(&queue, QueueStepReason::NaturalAdvance);

    assert!(next.target_index.is_none());
    assert!(previous.target_index.is_none());
    assert!(natural.target_index.is_none());
    assert!(next.reached_end);
    assert!(previous.reached_end);
    assert!(natural.reached_end);
}
