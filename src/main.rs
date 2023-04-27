#![allow(dead_code, unused_imports)]
mod cli;
mod context;
mod core;
mod role;
mod utils;

use structopt::StructOpt;

use crate::cli::opt::Opt;
use crate::core::handle_action;

fn main() {
    // Get the command-line arguments.
    let Opt { action } = Opt::from_args();
    handle_action(action);
}
