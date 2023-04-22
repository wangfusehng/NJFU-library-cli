mod cli;
mod context;
mod utils;

use cli::{Action::*, CommandLineArgs};
use log::*;
use std::env::set_var;
use structopt::StructOpt;


#[tokio::main]
async fn main() {
    set_var("RUST_LOG", "debug");
    env_logger::init();
    // Get the command-line arguments.
    let CommandLineArgs { action } = CommandLineArgs::from_args();

    debug!("Action: {:?}", action);
    // Perform the action.
    match action {
        Query { name, site } => {
            let context = context::Context::new();
            if name.is_some() {
                context.query_by_name(name).unwrap();
            } else if site.is_some() {
                context.query_by_site(site).unwrap();
            }
        }
    };
}
