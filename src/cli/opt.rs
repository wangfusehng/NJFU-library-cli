use super::action::Action;
use structopt::StructOpt;

/// A command line connect NJFU library written in Rust
#[derive(Debug, StructOpt)]
#[structopt(
    name = "NJFU-library-cli",
    about = "A command line connect NJFU library written in Rust",
    after_help = r##"EXAMPLES:
    njfulib login -u <username> -p <password>
    njfulib query -n <your name>
    njfulib statue
    njfulib reserve [-s <site>...] [-f <floor>...] --start <start time> --end <end time>
    njfulib cancel <id>"##
)]
pub struct Opt {
    #[structopt(subcommand)]
    pub action: Action,
}
