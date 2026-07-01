//! Core application code for the `mailghost` command-line client.

mod account;
mod app;
mod cli;

pub use app::run;
pub use cli::Cli;
