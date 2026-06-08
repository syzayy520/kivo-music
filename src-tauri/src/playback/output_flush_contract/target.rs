/// Describes which output pipeline buffer/queue must be flushed.
///
/// The contract layer selects targets based on `OutputFlushRequest`; the
/// orchestrator then executes flush operations on each target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FlushTarget {
    /// NativePipelineBuffer — VecDeque<AudioOutputFrame> FIFO.
    PipelineBuffer,

    /// OutputSink pending_frames counter.
    OutputSinkPendingFrames,

    /// ProductionOutputRoute input admission gate.
    ProductionOutputRouteInput,

    /// WasapiOutputSink RingBuffer (lock-free ring buffer).
    WasapiRingBuffer,

    /// Output thread command queue (OutputThreadCommand::Flush).
    RenderThreadQueue,

    /// IAudioClient device render buffer (WASAPI shared-mode buffer).
    DeviceRenderBuffer,
}

impl FlushTarget {
    /// Returns whether this target is a buffer-level flush (pipeline/sink counters).
    pub fn is_buffer_level(&self) -> bool {
        matches!(self, Self::PipelineBuffer | Self::OutputSinkPendingFrames)
    }

    /// Returns whether this target is a device-level flush (WASAPI/device).
    pub fn is_device_level(&self) -> bool {
        matches!(self, Self::WasapiRingBuffer | Self::DeviceRenderBuffer)
    }

    /// Returns whether this target requires render-thread coordination.
    pub fn requires_render_thread(&self) -> bool {
        matches!(self, Self::RenderThreadQueue | Self::DeviceRenderBuffer)
    }
}
