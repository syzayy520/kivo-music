use super::audio_route_pipeline_tap::{
    AudioRoutePipelineTap, AudioRoutePipelineTapError, AudioRoutePipelineTapReport,
};
use super::native_pipeline::NativePipeline;
use super::output::AudioOutputFrame;

impl NativePipeline {
    pub fn attach_audio_route_pipeline_tap(
        &mut self,
        tap: AudioRoutePipelineTap,
    ) -> Option<AudioRoutePipelineTap> {
        self.route_tap.replace(tap)
    }

    pub fn detach_audio_route_pipeline_tap(&mut self) -> Option<AudioRoutePipelineTap> {
        self.route_tap.take()
    }

    pub fn audio_route_pipeline_tap_report(&self) -> Option<AudioRoutePipelineTapReport> {
        self.route_tap.as_ref().map(AudioRoutePipelineTap::report)
    }

    pub fn reset_audio_route_pipeline_tap(
        &mut self,
    ) -> Result<Option<AudioRoutePipelineTapReport>, AudioRoutePipelineTapError> {
        self.route_tap
            .as_mut()
            .map(AudioRoutePipelineTap::reset)
            .transpose()
    }

    pub(in crate::playback) fn tap_audio_route_output_frame_non_fatal(
        &mut self,
        frame: &AudioOutputFrame,
    ) {
        if let Some(tap) = self.route_tap.as_mut() {
            tap.tap_output_frame(frame);
        }
    }
}
