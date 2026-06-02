use super::errors::PlaybackResult;
use super::native_null_output::KivoNullOutputSink;
use super::output::{AudioOutputFrame, OutputRuntimeStatus, OutputSettings, OutputSink};

/// Native output facade for the native audio pipeline.
///
/// Currently delegates to [`KivoNullOutputSink`] as the output boundary.
/// This keeps the pipeline decoupled from any specific output implementation.
#[derive(Clone, Debug, Default)]
pub struct KivoNativeOutputSink {
    inner: KivoNullOutputSink,
}

impl KivoNativeOutputSink {
    pub fn new() -> Self {
        Self {
            inner: KivoNullOutputSink::new(),
        }
    }
}

impl OutputSink for KivoNativeOutputSink {
    fn open(&mut self, settings: &OutputSettings) -> PlaybackResult<OutputRuntimeStatus> {
        self.inner.open(settings)
    }

    fn submit_frame(&mut self, frame: AudioOutputFrame) -> PlaybackResult<OutputRuntimeStatus> {
        self.inner.submit_frame(frame)
    }

    fn pause(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        self.inner.pause()
    }

    fn resume(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        self.inner.resume()
    }

    fn flush(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        self.inner.flush()
    }

    fn stop(&mut self) -> PlaybackResult<OutputRuntimeStatus> {
        self.inner.stop()
    }

    fn status(&self) -> OutputRuntimeStatus {
        self.inner.status()
    }

    fn close(&mut self) -> PlaybackResult<()> {
        self.inner.close()
    }
}
