use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    ///Query library site or student
    #[structopt(alias = "q")]
    Query {
        #[structopt(short, long)]
        name: Option<String>,

        #[structopt(short, long)]
        site: Option<String>,
    },

    #[structopt(alias = "q")]
    Login {
        #[structopt(short, long)]
        username: String,

        #[structopt(short, long)]
        password: String,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "NJFU-library-cli",
    about = "A command line connect NJFU library written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
}
