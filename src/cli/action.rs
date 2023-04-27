use super::day::Day;
use super::reserve::Reserve;
use structopt::StructOpt;

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
    In { _id: String },

    /// check out
    #[structopt(alias = "o")]
    Out { id: String },
}
