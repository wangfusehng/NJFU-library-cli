use super::day::Day;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
///reserve a site
pub struct Reserve {
    /// the site to reserve
    #[structopt(short, long)]
    pub sites: Option<Vec<String>>,

    #[structopt(short, long)]
    pub filter: Option<Vec<String>>,

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

    /// retry times for random reserve
    #[structopt(short, long, default_value = "30")]
    pub retry: u32,
}
