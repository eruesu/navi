#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate anyhow;

mod cheatsh;
mod cmds;
mod common;
mod display;
mod env_vars;
mod fetcher;
mod filesystem;
mod finder;
mod handler;
mod parser;
mod structures;
mod tldr;
mod welcome;

pub use common::file_issue::FileAnIssue;
pub use handler::handle_config;
pub use structures::config::{config_from_env, config_from_iter};
