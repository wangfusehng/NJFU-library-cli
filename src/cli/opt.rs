use super::action::Action;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "NJFU-library-cli",
    about = "A command line connect NJFU library written in Rust"
)]
pub struct Opt {
    #[structopt(subcommand)]
    pub action: Action,
}
