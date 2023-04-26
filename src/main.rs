#![allow(dead_code, unused_imports)]
mod cli;
mod client;
mod context;
mod core;
mod role;
mod utils;

use crate::core::handle_action;
use cli::CommandLineArgs;
use std::env::set_var;
use structopt::StructOpt;

fn main() {
    // Get the command-line arguments.
    let CommandLineArgs { action } = CommandLineArgs::from_args();
    handle_action(action);
}
