use structopt::StructOpt;
use strum::{EnumString, EnumVariantNames, VariantNames};

#[derive(EnumString, EnumVariantNames, Debug)]
#[strum(serialize_all = "kebab_case")]
pub enum Day {
    Today,
    Tomorrow,
}

#[derive(Debug, StructOpt)]
///reserve a site
pub struct Reserve {
    /// the site to reserve
    #[structopt(short, long)]
    pub site: String,

    /// the day to reserve
    #[structopt(
        short,
        long,
        possible_values = &["today","tomorrow"],
        case_insensitive = true,
        default_value = "today"
    )]
    pub day: Day,

    /// the start time to reserve
    #[structopt(long)]
    pub start: String,

    /// the end time to reserve
    #[structopt(long)]
    pub end: String,
}

///Command line arguments
#[derive(Debug, StructOpt)]
pub enum Action {
    ///Query library site or student
    #[structopt(alias = "q")]
    Query {
        /// the day to query
        #[structopt(
        short,
        long,
        possible_values = &["today","tomorrow"],
        case_insensitive = true,
        default_value = "today"
        )]
        day: Day,

        /// the name to query
        #[structopt(short, long)]
        name: Option<String>,

        /// the site to query
        #[structopt(short, long)]
        site: Option<String>,
    },

    ///Login library
    #[structopt(alias = "l")]
    Login {
        /// username to login
        #[structopt(short, long)]
        username: String,

        /// password to login
        #[structopt(short, long)]
        password: String,
    },
    ///list personal status
    #[structopt(alias = "s")]
    State {},

    ///cancel the reservation
    #[structopt(alias = "c")]
    Cancel {
        /// the id of the reservation to cancel
        id: String,
    },

    ///reserve a site
    #[structopt(alias = "r")]
    Reserve(Reserve),

    /// check in (not support yet)
    #[structopt(alias = "i")]
    In { site: String },

    /// check out
    #[structopt(alias = "o")]
    Out { id: String },
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
