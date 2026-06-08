//! Thread channel boundary types.
//!
//! Wraps std::sync::mpsc for command/event communication
//! between the output thread and external callers.

pub mod command_receiver;
pub mod command_sender;
pub mod event_receiver;
pub mod event_sender;

pub use command_receiver::CommandReceiver;
pub use command_sender::{command_channel, CommandSender};
pub use event_receiver::EventReceiver;
pub use event_sender::{event_channel, EventSender};
