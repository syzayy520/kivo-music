use super::plan_validation::{
    OutputThreadPlanValidationError, OutputThreadPlanValidationResult,
};

#[test]
fn validation_error_is_copy_eq_debug() {
    let a = OutputThreadPlanValidationError::AudioPlanWithoutFrames;
    let b = a;
    assert_eq!(a, b);
    let _ = format!("{:?}", a);
}

#[test]
fn validation_result_accepts_ok() {
    let result: OutputThreadPlanValidationResult = Ok(());
    assert!(result.is_ok());
}

#[test]
fn validation_result_accepts_error() {
    let result: OutputThreadPlanValidationResult =
        Err(OutputThreadPlanValidationError::AudioPlanWithoutFrames);
    assert!(result.is_err());
}
