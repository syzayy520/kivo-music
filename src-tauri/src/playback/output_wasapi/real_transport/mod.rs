pub(crate) mod channel;
pub(crate) mod command;
pub(crate) mod contract;
pub(crate) mod handle;
pub(crate) mod render_loop;
pub(crate) mod render_once;
pub(crate) mod render_padding_loop;
pub(crate) mod status;
pub(crate) mod thread;
pub(crate) mod thread_error;
pub(crate) mod thread_report;

#[cfg(test)]
mod channel_tests;
#[cfg(test)]
mod command_tests;
#[cfg(test)]
mod contract_tests;
#[cfg(test)]
mod real_output_thread_render_loop_tests;
#[cfg(test)]
mod real_output_thread_render_once_tests;
#[cfg(test)]
mod real_output_thread_render_padding_loop_tests;
#[cfg(test)]
mod real_output_thread_start_stop_tests;
#[cfg(test)]
mod real_output_thread_tests;
#[cfg(test)]
mod status_tests;
