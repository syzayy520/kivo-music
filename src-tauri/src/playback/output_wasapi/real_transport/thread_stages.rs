mod config;
mod error_mapping;
mod outcomes;
mod report_mapping;
mod runner;
mod validation;

pub(crate) use config::RealOutputThreadStageConfigs;
pub(crate) use outcomes::resolve_thread_stage_results;
pub(crate) use report_mapping::{
    build_real_output_thread_report, RealOutputThreadLifecycleReportFields,
};
pub(crate) use runner::{run_post_start_stages, run_render_once_stage};
pub(crate) use validation::validate_thread_stage_configs;

#[cfg(test)]
pub(crate) use error_mapping::map_render_ring_buffer_boundary_error;
#[cfg(test)]
pub(crate) use outcomes::{PostStartStageResults, ResolvedThreadStageResults};
#[cfg(test)]
pub(crate) use runner::run_post_start_stages_with;
