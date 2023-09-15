#![allow(dead_code)]
mod cli;
mod core;
mod def;
mod error;
mod executor;
mod njfulib;
mod utils;

use crate::cli::opt::Opt;
use anyhow::{Ok, Result};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    // Get the command-line arguments.
    let Opt { action } = Opt::from_args();

    let resp = core::handle_action(action).await?;
    #[cfg(debug_assertions)]
    println!("{:#?}", resp);
    #[cfg(not(debug_assertions))]
    println!("{}", resp);
    Ok(())
}
