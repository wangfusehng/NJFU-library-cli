#![allow(dead_code)]
mod action;
mod cli;
mod opt;
mod reserve;

use anyhow::{Ok, Result};
use opt::Opt;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    // Get the command-line arguments.
    let Opt { action } = Opt::from_args();

    let resp = cli::handle_action(action).await?;
    #[cfg(debug_assertions)]
    println!("{:#?}", resp);
    #[cfg(not(debug_assertions))]
    println!("{}", resp);
    Ok(())
}
