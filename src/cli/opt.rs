use super::action::Action;
use structopt::StructOpt;

/// A command line connect NJFU library written in Rust
#[derive(StructOpt)]
#[structopt(
    name = "NJFU-library-cli",
    long_about = "A command line connect NJFU library written in Rust",
    after_help = r##"EXAMPLES:
    njfulib login -u <username> -p <password> -c <cookie>
    njfulib query -n <name> [-f <filter>]
    njfulib query -s <site>
    njfulib statue
    njfulib reserve [-s <site> ...] [-f <filter> ...] [-d <day>]--start <start time> --end <end time> -r 30
    njfulib cancel -u <uuid>
"##
)]
pub struct Opt {
    #[structopt(subcommand)]
    pub action: Action,
}
