pub(crate) mod lifecycle;
pub(crate) mod handle_contract;
pub(crate) mod shutdown;
pub(crate) mod plan;
pub(crate) mod matrix;

#[cfg(test)]
mod lifecycle_tests;
#[cfg(test)]
mod handle_contract_tests;
#[cfg(test)]
mod shutdown_tests;
#[cfg(test)]
mod plan_tests;
#[cfg(test)]
mod matrix_tests;
