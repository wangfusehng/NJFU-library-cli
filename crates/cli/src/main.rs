#![allow(dead_code)]
mod action;
mod cli;
mod opt;
mod reserve;

use anyhow::{Ok, Result};
use env_logger;
use opt::Opt;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    // Get the command-line arguments.
    let Opt { action } = Opt::from_args();

    let resp = cli::handle_action(action).await?;
    #[cfg(debug_assertions)]
    println!("{:#?}", resp);
    #[cfg(not(debug_assertions))]
    println!("{}", resp);
    Ok(())
}
