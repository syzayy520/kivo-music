//! Render event type.
//!
//! Defines events representing render cycle completions in the output thread.

/// Events emitted after render cycle completions.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum RenderEvent {
    /// A batch of frames was rendered to the output device.
    FramesRendered {
        /// Number of frames rendered.
        frame_count: u64,
        /// Number of bytes written.
        bytes_written: u64,
    },
    /// Silent frames were written (underrun fill).
    SilenceWritten {
        /// Number of silent frames.
        frame_count: u64,
    },
    /// Render cycle completed with no output.
    #[default]
    EmptyRender,
}
