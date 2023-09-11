use crate::cli::reserve::Reserve;
use structopt::StructOpt;

///Command line arguments
#[derive(StructOpt)]
pub enum Action {
    ///Query library site or student
    Query {
        /// the day to query
        #[structopt(short, long, default_value = "0")]
        day: u32,

        /// the name to query
        #[structopt(short, long)]
        name: Option<String>,

        /// the site to query
        #[structopt(short, long)]
        site: Option<String>,
    },

    ///Login library
    Login {
        #[structopt(short, long, env = "NJFU_USERNAME")]
        username: String,
        #[structopt(short, long, env = "NJFU_PASSWORD", hide_env_values = true)]
        password: String,
        #[structopt(short, long, env = "NJFU_COOKIE", hide_env_values = true)]
        cookie: String,
    },

    ///list personal status
    Status {},

    ///cancel the reservation
    Cancel {
        /// the id of the reservation to cancel
        #[structopt(short, long)]
        uuid: String,
    },

    ///reserve a site
    Reserve(Reserve),
}
