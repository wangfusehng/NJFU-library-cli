#![allow(dead_code, unused_imports)]
mod cli;
mod client;
mod context;
mod core;
mod utils;
mod role;

use crate::core::handle_action;
use cli::CommandLineArgs;
use std::env::set_var;
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    set_var("RUST_LOG", "info");
    env_logger::init();

    // Get the command-line arguments.
    let CommandLineArgs { action } = CommandLineArgs::from_args();
    handle_action(action);
}
