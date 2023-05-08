use crate::cli::day::Day;
use crate::cli::infomation::Infomation;
use crate::cli::reserve::Reserve;
use structopt::StructOpt;

///Command line arguments
#[derive(Debug, StructOpt)]
pub enum Action {
    ///Query library site or student
    Query {
        /// the day to query
        #[structopt(
        short,
        long,
        possible_values = &["today","tomorrow","overmorrow"],
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
    Login {
        /// username to login
        #[structopt(short, long, env = "NJFU_USERNAME")]
        username: String,

        /// password to login
        #[structopt(short, long, env = "NJFU_PASSWORD", hide_env_values = true)]
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

    ///reserve a site
    Reserve(Reserve),

    /// check in
    In {
        // the site to check in
        #[structopt(short, long)]
        site: String,

        // the time to use the site in minutes
        #[structopt(short, long)]
        time: Option<u32>,
    },

    /// check out
    Out {
        /// the reserve id to check out
        #[structopt(short, long)]
        id: String,
    },

    /// show info
    Info {
        #[structopt(
            possible_values = &["floor","author","user"],
            case_insensitive = true,
            default_value = "floor"
        )]
        infomation: Infomation,
    },
}
