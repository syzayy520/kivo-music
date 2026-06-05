pub(crate) mod state;
pub(crate) mod buffer_snapshot;
pub(crate) mod control;
pub(crate) mod errors;
pub(crate) mod render_plan;
pub(crate) mod consumer_plan;
pub(crate) mod plan_validation;
pub(crate) mod plan_projection;
pub(crate) mod plan_invariants;
pub(crate) mod transition_validation;

#[cfg(test)]
mod state_tests;
#[cfg(test)]
mod buffer_snapshot_tests;
#[cfg(test)]
mod control_tests;
#[cfg(test)]
mod errors_tests;
#[cfg(test)]
mod render_plan_tests;
#[cfg(test)]
mod consumer_plan_tests;
#[cfg(test)]
mod plan_validation_tests;
#[cfg(test)]
mod plan_projection_tests;
#[cfg(test)]
mod plan_invariants_tests;
#[cfg(test)]
mod transition_validation_tests;
