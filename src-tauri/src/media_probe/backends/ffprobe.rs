use super::ffprobe_command::read_probe_json;
use super::ffprobe_json::parse_probe_result;
use super::super::backend::ProbeBackend;
use super::super::errors::MediaProbeResultValue;
use super::super::types::MediaProbeResult;

#[derive(Clone, Debug, Default)]
pub struct FfprobeBackend;

impl ProbeBackend for FfprobeBackend {
    fn name(&self) -> &'static str {
        "ffprobe"
    }

    fn probe(&self, path: &str) -> MediaProbeResultValue<MediaProbeResult> {
        let root = read_probe_json(path)?;
        Ok(parse_probe_result(path, &root))
    }
}
