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

        /// the floor of querying site
        #[structopt(short, long)]
        filter: Option<Vec<String>>,
    },

    ///Login library
    Login {
        // library username
        #[structopt(short, long, env = "NJFU_USERNAME")]
        username: String,
        // library password
        #[structopt(short, long, env = "NJFU_PASSWORD", hide_env_values = true)]
        password: String,
        // library cookie
        #[structopt(short, long, env = "NJFU_COOKIE", hide_env_values = true)]
        cookie: String,
    },

    ///list personal status
    Status {
        // the day number to show status
        #[structopt(short, long, default_value = "5")]
        day: u32,
    },

    ///cancel the reservation
    Cancel {
        /// the id of the reservation to cancel
        #[structopt(short, long)]
        uuid: String,
    },

    ///reserve a site
    Reserve(Reserve),
}
