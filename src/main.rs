#![allow(dead_code, unused_imports)]
mod cli;
mod core;
mod executor;
mod role;
mod utils;

use crate::cli::opt::Opt;
use crate::core::handle_action;
use anyhow::Result;
use structopt::StructOpt;

fn main() -> Result<()> {
    // Get the command-line arguments.
    let Opt { action } = Opt::from_args();
    handle_action(action)
}
