use super::super::service::{MediaProbeError, MediaProbeResultValue, ProbeBackend};
use super::super::types::MediaProbeResult;

#[derive(Clone, Debug, Default)]
pub struct FfprobeBackend;

impl ProbeBackend for FfprobeBackend {
    fn name(&self) -> &'static str {
        "ffprobe"
    }

    fn probe(&self, path: &str) -> MediaProbeResultValue<MediaProbeResult> {
        Err(MediaProbeError::UnsupportedOperation(format!(
            "ffprobe probing is not implemented yet for {path}"
        )))
    }
}
