use super::errors::MediaProbeResultValue;
use super::types::MediaProbeResult;

pub trait ProbeBackend {
    fn name(&self) -> &'static str;
    fn probe(&self, path: &str) -> MediaProbeResultValue<MediaProbeResult>;
}
