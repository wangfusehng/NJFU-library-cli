#![allow(dead_code)]
mod cli;
mod core;
pub mod def;
mod executor;
mod role;
mod utils;

use crate::cli::opt::Opt;
use anyhow::Result;
use structopt::StructOpt;

fn main() -> Result<()> {
    // Get the command-line arguments.
    let Opt { action } = Opt::from_args();
    println!("{:#?}", core::handle_action(action));
    Ok(())
}
