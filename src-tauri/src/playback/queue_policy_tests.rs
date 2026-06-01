use super::queue::PlaybackQueue;
use super::queue_policy::{decide_queue_step, QueueStepDecision, QueueStepReason};
use super::types::{PlaybackTrack, RepeatMode, TrackId};

fn track(index: usize) -> PlaybackTrack {
    PlaybackTrack {
        id: TrackId(format!("track-{index}")),
        title: format!("Track {index}"),
        artist: "Artist".to_string(),
        source_path: format!("local-track-{index}.wav"),
    }
}

fn assert_decision(
    decision: QueueStepDecision,
    expected_target_index: Option<usize>,
    expected_reached_end: bool,
) {
    assert_eq!(decision.target_index, expected_target_index);
    assert_eq!(decision.reached_end, expected_reached_end);
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

    assert_decision(decision, None, true);
}

#[test]
fn natural_advance_repeat_one_stays_on_current_track() {
    let queue = queue_with_current_last(RepeatMode::One);
    let decision = decide_queue_step(&queue, QueueStepReason::NaturalAdvance);

    assert_decision(decision, Some(1), false);
}

#[test]
fn natural_advance_repeat_all_wraps_to_first_track() {
    let queue = queue_with_current_last(RepeatMode::All);
    let decision = decide_queue_step(&queue, QueueStepReason::NaturalAdvance);

    assert_decision(decision, Some(0), false);
}

#[test]
fn next_moves_to_following_index_when_available() {
    let mut queue = PlaybackQueue::default();
    queue.append(track(1));
    queue.append(track(2));
    queue.append(track(3));
    queue.set_current_index(1).expect("index should be valid");

    let decision = decide_queue_step(&queue, QueueStepReason::Next);

    assert_decision(decision, Some(2), false);
}

#[test]
fn next_from_last_reaches_end() {
    let queue = queue_with_current_last(RepeatMode::Off);

    let decision = decide_queue_step(&queue, QueueStepReason::Next);

    assert_decision(decision, None, true);
}

#[test]
fn previous_from_middle_moves_back_one() {
    let mut queue = PlaybackQueue::default();
    queue.append(track(1));
    queue.append(track(2));
    queue.append(track(3));
    queue.set_current_index(2).expect("index should be valid");

    let decision = decide_queue_step(&queue, QueueStepReason::Previous);

    assert_decision(decision, Some(1), false);
}

#[test]
fn previous_from_start_stays_at_zero() {
    let mut queue = PlaybackQueue::default();
    queue.append(track(1));
    queue.append(track(2));
    queue.set_current_index(0).expect("index should be valid");

    let decision = decide_queue_step(&queue, QueueStepReason::Previous);

    assert_decision(decision, Some(0), false);
}

#[test]
fn empty_queue_always_reports_end() {
    let queue = PlaybackQueue::default();

    let next = decide_queue_step(&queue, QueueStepReason::Next);
    let previous = decide_queue_step(&queue, QueueStepReason::Previous);
    let natural = decide_queue_step(&queue, QueueStepReason::NaturalAdvance);

    assert_decision(next, None, true);
    assert_decision(previous, None, true);
    assert_decision(natural, None, true);
}

#[test]
fn missing_current_index_defaults_to_first_track() {
    let mut queue = PlaybackQueue::default();
    queue.items.push(track(1));
    queue.items.push(track(2));
    queue.current_index = None;

    let next = decide_queue_step(&queue, QueueStepReason::Next);
    let previous = decide_queue_step(&queue, QueueStepReason::Previous);

    assert_decision(next, Some(1), false);
    assert_decision(previous, Some(0), false);
}

#[test]
fn out_of_range_current_index_is_clamped_to_last_track() {
    let mut queue = PlaybackQueue::default();
    queue.append(track(1));
    queue.append(track(2));
    queue.current_index = Some(99);

    let next = decide_queue_step(&queue, QueueStepReason::Next);
    let previous = decide_queue_step(&queue, QueueStepReason::Previous);
    let natural = decide_queue_step(&queue, QueueStepReason::NaturalAdvance);

    assert_decision(next, None, true);
    assert_decision(previous, Some(0), false);
    assert_decision(natural, None, true);
}
