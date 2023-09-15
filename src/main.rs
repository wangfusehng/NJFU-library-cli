#![allow(dead_code)]
mod cli;
mod core;
pub mod def;
mod executor;
mod njfulib;
mod utils;

use crate::cli::opt::Opt;
use anyhow::Result;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    // Get the command-line arguments.
    let Opt { action } = Opt::from_args();

    match core::handle_action(action).await {
        Ok(resp) => {
            #[cfg(debug_assertions)]
            println!("{:#?}", resp);
            #[cfg(not(debug_assertions))]
            println!("{}", resp);
        }
        Err(e) => {
            #[cfg(debug_assertions)]
            println!("{:#?}", e);
            #[cfg(not(debug_assertions))]
            println!("{}", e);
        }
    }

    Ok(())
}
