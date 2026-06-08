use serde::{Deserialize, Serialize};

use crate::playback::queue::PlaybackQueue;
use crate::playback::types::RepeatMode;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum QueueStepReason {
    Next,
    Previous,
    NaturalAdvance,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct QueueStepDecision {
    pub target_index: Option<usize>,
    pub reached_end: bool,
}

pub fn decide_queue_step(queue: &PlaybackQueue, reason: QueueStepReason) -> QueueStepDecision {
    if queue.items.is_empty() {
        return QueueStepDecision {
            target_index: None,
            reached_end: true,
        };
    }

    let current = queue.current_index.unwrap_or(0).min(queue.items.len() - 1);

    match reason {
        QueueStepReason::Previous => QueueStepDecision {
            target_index: Some(current.saturating_sub(1)),
            reached_end: false,
        },
        QueueStepReason::Next => {
            let next = current + 1;

            if next < queue.items.len() {
                QueueStepDecision {
                    target_index: Some(next),
                    reached_end: false,
                }
            } else {
                QueueStepDecision {
                    target_index: None,
                    reached_end: true,
                }
            }
        }
        QueueStepReason::NaturalAdvance => match queue.repeat_mode {
            RepeatMode::One => QueueStepDecision {
                target_index: Some(current),
                reached_end: false,
            },
            RepeatMode::All => {
                let next = current + 1;
                if next < queue.items.len() {
                    QueueStepDecision {
                        target_index: Some(next),
                        reached_end: false,
                    }
                } else {
                    QueueStepDecision {
                        target_index: Some(0),
                        reached_end: false,
                    }
                }
            }
            RepeatMode::Off => {
                let next = current + 1;
                if next < queue.items.len() {
                    QueueStepDecision {
                        target_index: Some(next),
                        reached_end: false,
                    }
                } else {
                    QueueStepDecision {
                        target_index: None,
                        reached_end: true,
                    }
                }
            }
        },
    }
}
