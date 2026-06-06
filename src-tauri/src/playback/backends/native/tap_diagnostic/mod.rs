mod close;
mod config;
mod open;
#[cfg(test)]
mod query;
mod seek;
mod state;

pub(super) use close::{close_on_shutdown, close_on_stop};
pub(super) use config::state_from_config;
pub(super) use open::open_after_decoder_open;
pub(super) use seek::reset_after_seek_success;
pub(super) use state::TapDiagnosticState;
