mod close;
mod config;
mod open;
#[cfg_attr(not(test), allow(dead_code))]
mod query;
mod state;

pub(super) use close::{close_on_shutdown, close_on_stop};
pub(super) use config::state_from_config;
pub(super) use open::open_after_decoder_open;
pub(super) use state::TapDiagnosticState;
