/// Validation errors for render plan shape and state transitions.
///
/// Pure enum — no external error traits, no platform types.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputThreadPlanValidationError {
    /// Audio plan must have frames_to_read > 0.
    AudioPlanWithoutFrames,
    /// Audio plan must not have silence_frames > 0.
    AudioPlanWithSilenceFrames,
    /// Silence plan must have silence_frames > 0.
    SilencePlanWithoutFrames,
    /// Silence plan must not have frames_to_read > 0.
    SilencePlanWithAudioFrames,
    /// Sleep plan must have no work (frames_to_read == 0, silence_frames == 0).
    SleepPlanHasWork,
    /// Exit plan must have no work (frames_to_read == 0, silence_frames == 0).
    ExitPlanHasWork,
    /// When shutdown is requested, plan must be Exit.
    ShutdownPlanMustExit,
    /// When buffer is closed and empty, plan must be Exit.
    ClosedExhaustedPlanMustExit,
    /// When paused, plan must not be RenderAudio.
    PausedPlanMustNotRenderAudio,
    /// When flush is requested, plan must not be RenderAudio.
    FlushPlanMustNotRenderAudio,
    /// When state is not Running (and not shutdown/closed-exhausted), plan must be Sleep.
    NonRunningPlanMustSleep,
    /// Audio plan frames_to_read exceeds available frames.
    AudioPlanExceedsAvailable,
}

/// Result type for plan validation.
#[allow(dead_code)]
pub(crate) type OutputThreadPlanValidationResult = Result<(), OutputThreadPlanValidationError>;
