pub mod aggregate;
pub mod command;
pub mod command_handler;
pub mod event;
pub mod query_handler;
pub mod snapshot;

extern crate custom_error;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
