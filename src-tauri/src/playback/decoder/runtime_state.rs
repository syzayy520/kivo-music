use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum DecoderRuntimePhase {
    Idle,
    Opening,
    Open,
    Draining,
    Closed,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DecoderRuntimeState {
    pub phase: DecoderRuntimePhase,
    pub last_error: Option<String>,
}

impl DecoderRuntimeState {
    pub fn idle() -> Self {
        Self {
            phase: DecoderRuntimePhase::Idle,
            last_error: None,
        }
    }

    pub fn begin_opening(&mut self) {
        self.phase = DecoderRuntimePhase::Opening;
        self.last_error = None;
    }

    pub fn mark_open(&mut self) {
        self.phase = DecoderRuntimePhase::Open;
        self.last_error = None;
    }

    pub fn begin_draining(&mut self) {
        self.phase = DecoderRuntimePhase::Draining;
    }

    pub fn mark_closed(&mut self) {
        self.phase = DecoderRuntimePhase::Closed;
    }

    pub fn mark_failed(&mut self, message: impl Into<String>) {
        self.phase = DecoderRuntimePhase::Failed;
        self.last_error = Some(message.into());
    }
}
