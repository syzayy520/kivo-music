/// Action the consumer loop should take in a single step.
///
/// Pure classification — no WASAPI calls, no buffer reads.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum OutputThreadRenderAction {
    /// Wait before next step (buffer empty or WASAPI buffer full).
    #[default]
    Sleep,
    /// Read and render audio frames.
    RenderAudio,
    /// Fill silence frames (underrun or paused).
    RenderSilence,
    /// Exit the consumer loop.
    Exit,
}

/// Plan for a single consumer loop step.
///
/// Pure data — describes what to do, does not execute it.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct OutputThreadRenderPlan {
    /// The action to take.
    pub action: OutputThreadRenderAction,
    /// Number of audio frames to read from the buffer.
    pub frames_to_read: u32,
    /// Number of silence frames to fill.
    pub silence_frames: u32,
    /// Whether the consumer should sleep before next step.
    pub should_sleep: bool,
    /// Whether the consumer should exit.
    pub should_exit: bool,
}

impl OutputThreadRenderPlan {
    /// Whether this plan renders audio frames.
    #[allow(dead_code)]
    pub(crate) fn is_audio(self) -> bool {
        self.action == OutputThreadRenderAction::RenderAudio
    }

    /// Whether this plan renders silence frames.
    #[allow(dead_code)]
    pub(crate) fn is_silence(self) -> bool {
        self.action == OutputThreadRenderAction::RenderSilence
    }

    /// Whether this plan is idle (sleep).
    #[allow(dead_code)]
    pub(crate) fn is_idle(self) -> bool {
        self.action == OutputThreadRenderAction::Sleep
    }

    /// Whether this plan exits the loop.
    #[allow(dead_code)]
    pub(crate) fn is_exit(self) -> bool {
        self.action == OutputThreadRenderAction::Exit
    }
}
