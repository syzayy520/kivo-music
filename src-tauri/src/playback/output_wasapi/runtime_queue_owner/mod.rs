pub(crate) mod contract;
pub(crate) mod state;
pub(crate) mod report;
pub(crate) mod matrix;
pub(crate) mod fixed_slots;
pub(crate) mod slot_state;
pub(crate) mod slot_projection;
pub(crate) mod slot_report;
pub(crate) mod slot_matrix;

#[cfg(test)]
mod contract_tests;
#[cfg(test)]
mod state_tests;
#[cfg(test)]
mod report_tests;
#[cfg(test)]
mod matrix_tests;
#[cfg(test)]
mod fixed_slots_tests;
#[cfg(test)]
mod slot_state_tests;
#[cfg(test)]
mod slot_projection_tests;
#[cfg(test)]
mod slot_report_tests;
#[cfg(test)]
mod slot_matrix_tests;
