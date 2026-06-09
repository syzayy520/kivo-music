//! Render source adapter test helpers.
//!
//! Shared mock implementations for render source adapter tests.

use crate::playback::output_wasapi::output_thread::sink_boundary::consumer::consumer_contract::SinkConsumer;
use crate::playback::output_wasapi::output_thread::sink_boundary::render_source::{
    RenderSource, RenderSourceError, RenderSourceRequest, RenderSourceResult, SourceCursor,
    SourceSnapshot,
};
use crate::playback::output_wasapi::output_thread::sink_boundary::{
    ConsumerSnapshot, SinkError, SinkRequest, SinkResult,
};

/// Mock render source for testing.
pub(crate) struct MockRenderSource {
    ready: bool,
    exhausted: bool,
    next_result: Option<Result<RenderSourceResult, RenderSourceError>>,
    requests_accepted: u64,
    packets_provided: u64,
    frames_read: u64,
    bytes_read: u64,
    errors: u64,
}

impl MockRenderSource {
    pub(crate) fn new() -> Self {
        Self {
            ready: true,
            exhausted: false,
            next_result: None,
            requests_accepted: 0,
            packets_provided: 0,
            frames_read: 0,
            bytes_read: 0,
            errors: 0,
        }
    }

    pub(crate) fn with_result(
        mut self,
        result: Result<RenderSourceResult, RenderSourceError>,
    ) -> Self {
        self.next_result = Some(result);
        self
    }

    pub(crate) fn with_ready(mut self, ready: bool) -> Self {
        self.ready = ready;
        self
    }
}

impl RenderSource for MockRenderSource {
    fn process_request(
        &mut self,
        _request: &RenderSourceRequest,
    ) -> Result<RenderSourceResult, RenderSourceError> {
        self.requests_accepted += 1;
        let result = match self.next_result.take() {
            Some(r) => r,
            None => {
                self.errors += 1;
                return Err(RenderSourceError::Internal {
                    description: "no result configured".to_string(),
                });
            }
        };
        match &result {
            Ok(RenderSourceResult::Packet {
                frames_provided,
                bytes_read,
            }) => {
                self.packets_provided += 1;
                self.frames_read += frames_provided;
                self.bytes_read += bytes_read;
            }
            Ok(RenderSourceResult::Exhausted) => self.exhausted = true,
            Err(_) => self.errors += 1,
            _ => {}
        }
        result
    }

    fn snapshot(&self) -> SourceSnapshot {
        SourceSnapshot {
            requests_accepted: self.requests_accepted,
            packets_provided: self.packets_provided,
            frames_read: self.frames_read,
            bytes_read: self.bytes_read,
            errors: self.errors,
            is_exhausted: self.exhausted,
            is_ready: self.ready,
        }
    }

    fn cursor(&self) -> SourceCursor {
        SourceCursor::default()
    }

    fn is_ready(&self) -> bool {
        self.ready && !self.exhausted
    }

    fn is_exhausted(&self) -> bool {
        self.exhausted
    }

    fn reset(&mut self) {
        self.ready = true;
        self.exhausted = false;
        self.next_result = None;
        self.requests_accepted = 0;
        self.packets_provided = 0;
        self.frames_read = 0;
        self.bytes_read = 0;
        self.errors = 0;
    }
}

/// Mock sink consumer for testing.
pub(crate) struct MockSinkConsumer {
    ready: bool,
    next_result: Option<Result<SinkResult, SinkError>>,
}

impl MockSinkConsumer {
    pub(crate) fn new() -> Self {
        Self {
            ready: true,
            next_result: None,
        }
    }

    pub(crate) fn with_result(mut self, result: Result<SinkResult, SinkError>) -> Self {
        self.next_result = Some(result);
        self
    }
}

impl SinkConsumer for MockSinkConsumer {
    fn process_request(&mut self, _request: &SinkRequest) -> Result<SinkResult, SinkError> {
        self.next_result.take().unwrap_or(Err(SinkError::Internal {
            description: "no result configured".to_string(),
        }))
    }

    fn snapshot(&self) -> ConsumerSnapshot {
        ConsumerSnapshot {
            is_ready: self.ready,
            ..Default::default()
        }
    }

    fn is_ready(&self) -> bool {
        self.ready
    }

    fn reset(&mut self) {
        self.ready = true;
        self.next_result = None;
    }
}
