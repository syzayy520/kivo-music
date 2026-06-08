/// A single barrier target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WasapiBarrierTarget {
    /// The ring buffer between producer and render thread.
    RingBuffer,
    /// The runtime command queue.
    RuntimeQueue,
    /// The output thread command queue.
    OutputThreadCommandQueue,
    /// The render plan (next action decision).
    RenderPlan,
    /// Render thread local state (epoch, buffer pointers).
    RenderThreadLocalState,
    /// The IAudioRenderClient buffer (GetBuffer/ReleaseBuffer region).
    AudioRenderClientBuffer,
    /// The IAudioClient device buffer (hardware-level).
    AudioClientDeviceBuffer,
}
