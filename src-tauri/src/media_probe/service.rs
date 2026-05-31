use std::sync::Mutex;

use super::backend::ProbeBackend;
use super::backends::ffprobe::FfprobeBackend;
use super::backends::ffprobe_status::ffprobe_status;
use super::errors::MediaProbeResultValue;
use super::path::validate_probe_path;
use super::status::MediaProbeBackendStatus;
use super::types::MediaProbeResult;

#[derive(Clone, Debug, Default)]
pub struct MediaProbeService {
    backend: FfprobeBackend,
}

impl MediaProbeService {
    pub fn new() -> Self {
        Self {
            backend: FfprobeBackend::default(),
        }
    }

    pub fn backend_name(&self) -> &'static str {
        self.backend.name()
    }

    pub fn backend_status(&self) -> MediaProbeBackendStatus {
        ffprobe_status()
    }

    pub fn probe(&self, path: &str) -> MediaProbeResultValue<MediaProbeResult> {
        let path = validate_probe_path(path)?;
        self.backend.probe(path)
    }
}

#[derive(Debug, Default)]
pub struct MediaProbeServiceState {
    service: Mutex<MediaProbeService>,
}

impl MediaProbeServiceState {
    pub fn backend_name(&self) -> &'static str {
        let service = self
            .service
            .lock()
            .expect("media probe service lock failed");
        service.backend_name()
    }

    pub fn backend_status(&self) -> MediaProbeBackendStatus {
        let service = self
            .service
            .lock()
            .expect("media probe service lock failed");
        service.backend_status()
    }

    pub fn probe(&self, path: &str) -> MediaProbeResultValue<MediaProbeResult> {
        let service = self
            .service
            .lock()
            .expect("media probe service lock failed");
        service.probe(path)
    }
}
