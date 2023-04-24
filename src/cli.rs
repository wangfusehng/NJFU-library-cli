use structopt::StructOpt;

///Command line arguments
#[derive(Debug, StructOpt)]
pub enum Action {
    ///Query library site or student
    Query {
        /// the name to query
        #[structopt(short, long)]
        name: Option<String>,
        
        /// the site to query
        #[structopt(short, long)]
        site: Option<String>,
    },

    ///Login library site
    Login {
        /// username to login
        #[structopt(short, long)]
        username: String,

        /// password to login
        #[structopt(short, long)]
        password: String,
    },
    ///list personal status
    State {},

    ///cancel the reservation
    Cancel {
        /// the id of the reservation to cancel
        #[structopt(short, long)]
        id: String,
    },

    Reserve {
        /// the site to reserve
        #[structopt(short, long)]
        site: String,
        
        /// the day to reserve
        #[structopt(short, long)]
        day: String,

        /// the start time to reserve
        #[structopt(long)]
        start: String,

        /// the end time to reserve
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
