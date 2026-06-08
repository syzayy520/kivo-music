pub(crate) mod authority;
pub(crate) mod config;
mod failure;
mod input;
pub(crate) mod input_admission;
pub(crate) mod lifecycle;
pub(crate) mod sink_failure_mapping;

pub(crate) use failure::{
    ProductionOutputRouteBackpressure, ProductionOutputRouteFailure,
    ProductionOutputRouteFailureClass, ProductionOutputRouteFormatDescriptor,
    ProductionOutputRouteFormatMismatch, ProductionOutputRouteRouteClosed,
    ProductionOutputRouteRouteClosedReason, ProductionOutputRouteSinkFailure,
    ProductionOutputRouteStreamFormat, ProductionOutputRouteUnderrun,
};
pub(crate) use input::ProductionOutputRouteFrameInput;

#[allow(dead_code)]
fn production_output_route_contract_lint_anchor(frame: &crate::playback::output::AudioOutputFrame) {
    use std::num::NonZeroU64;

    use authority::{ProductionOutputRouteConfigAuthority, ProductionOutputRouteIdentity};

    let input = ProductionOutputRouteFrameInput::from_frame(frame);
    let underrun = ProductionOutputRouteUnderrun::new(1, 0);
    let sink_failure = ProductionOutputRouteSinkFailure::new("contract", "unattached");
    let route_closed =
        ProductionOutputRouteRouteClosed::new(ProductionOutputRouteRouteClosedReason::NotOpened);
    let classes: [ProductionOutputRouteFailureClass; 3] = [
        ProductionOutputRouteFailure::from(underrun).class(),
        ProductionOutputRouteFailure::from(sink_failure.clone()).class(),
        ProductionOutputRouteFailure::from(route_closed).class(),
    ];

    // P0-105: identity + config authority contract symbols
    let _identity = ProductionOutputRouteIdentity::new(NonZeroU64::new(1).unwrap());
    let _identity_try = ProductionOutputRouteIdentity::try_new(2);
    let _config_authority_type = std::any::type_name::<ProductionOutputRouteConfigAuthority>();

    // P0-110: lifecycle input gate contract symbols
    use lifecycle::{ProductionOutputRouteLifecycleInputGate, ProductionOutputRouteLifecycleState};
    let _lifecycle_gate = ProductionOutputRouteLifecycleInputGate::new();
    let _lifecycle_state = ProductionOutputRouteLifecycleState::AcceptingInput;

    // P0-112: lifecycle transition matrix contract symbols
    use lifecycle::transition::{
        ProductionOutputRouteLifecycleTransitionDecision,
        ProductionOutputRouteLifecycleTransitionMatrix,
    };
    let _transition_matrix = ProductionOutputRouteLifecycleTransitionMatrix::new();
    let _transition_decision = _transition_matrix.validate_transition(
        ProductionOutputRouteLifecycleState::NotReadyForInput,
        ProductionOutputRouteLifecycleState::AcceptingInput,
    );
    let _decision_type = std::any::type_name::<ProductionOutputRouteLifecycleTransitionDecision>();

    // P0-114: lifecycle state owner contract symbols
    use lifecycle::{
        ProductionOutputRouteLifecycleStateCell,
        ProductionOutputRouteLifecycleStateOwnerUpdateDecision,
    };
    let _state_cell = ProductionOutputRouteLifecycleStateCell::new(
        ProductionOutputRouteLifecycleState::NotReadyForInput,
    );
    let _update_decision_type =
        std::any::type_name::<ProductionOutputRouteLifecycleStateOwnerUpdateDecision>();

    // P0-116: lifecycle close authority contract symbols
    use lifecycle::{
        ProductionOutputRouteLifecycleCloseAuthority, ProductionOutputRouteLifecycleCloseDecision,
    };
    let _close_authority = ProductionOutputRouteLifecycleCloseAuthority;
    let _close_decision_type = std::any::type_name::<ProductionOutputRouteLifecycleCloseDecision>();

    // P0-119: input admission contract symbols
    use input_admission::{
        ProductionOutputRouteInputAdmission, ProductionOutputRouteInputAdmissionResult,
    };
    let _input_admission = ProductionOutputRouteInputAdmission;
    let _input_admission_result_type =
        std::any::type_name::<ProductionOutputRouteInputAdmissionResult<'_>>();

    let _ = (
        input.position_ms(),
        input.sample_count(),
        underrun.requested_frames(),
        underrun.available_frames(),
        underrun.missing_frames(),
        sink_failure.operation(),
        sink_failure.message(),
        route_closed.reason(),
        classes,
        ProductionOutputRouteRouteClosedReason::ExplicitClose,
        ProductionOutputRouteRouteClosedReason::AlreadyClosed,
        ProductionOutputRouteRouteClosedReason::RejectedAfterClose,
        _identity.value(),
        _identity_try.is_some(),
        _config_authority_type,
        _lifecycle_gate,
        _lifecycle_state,
        _transition_matrix,
        _transition_decision,
        _decision_type,
        _state_cell,
        _update_decision_type,
        _close_authority,
        _close_decision_type,
        _input_admission,
        _input_admission_result_type,
    );
}

#[cfg(test)]
mod tests;
