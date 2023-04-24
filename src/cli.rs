use structopt::StructOpt;

///Command line arguments
#[derive(Debug, StructOpt)]
pub enum Action {
    ///Query library site or student
    Query {
        #[structopt(short, long)]
        name: Option<String>,

        #[structopt(short, long)]
        site: Option<String>,
    },

    ///Login library site
    Login {
        #[structopt(short, long)]
        username: String,

        #[structopt(short, long)]
        password: String,
    },
    ///Logout personal status
    State {},

    ///cancel the reservation
    Cancel {
        #[structopt(short, long)]
        id: String,
    },

    Reserve {
        #[structopt(short, long)]
        site: String,

        #[structopt(short, long)]
        day: String,

        #[structopt(long)]
        start: String,

        #[structopt(long)]
        end: String,
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
