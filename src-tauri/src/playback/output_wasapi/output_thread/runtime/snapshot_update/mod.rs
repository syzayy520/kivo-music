//! Runtime snapshot update helpers.
//!
//! Pure functions for updating ThreadSnapshot fields.

pub mod buffer_update;
pub mod lifecycle_update;
pub mod render_update;

pub use buffer_update::update_buffer_state;
pub use lifecycle_update::update_lifecycle;
pub use render_update::update_render_activity;
