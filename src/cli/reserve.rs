use super::day::Day;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
///reserve a site
pub struct Reserve {
    /// the site list to try reserve
    #[structopt(short, long)]
    pub sites: Option<Vec<String>>,

    /// filter the site by a list of floor
    #[structopt(short, long)]
    pub filter: Option<Vec<String>>,

    /// the user to reserve, work when reserve space
    #[structopt(short, long)]
    pub user: Option<Vec<String>>,

    /// the day to reserve
    #[structopt(
        short,
        long,
        possible_values = &["today","tomorrow","overmorrow"],
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
    #[structopt(short, long, default_value = "50")]
    pub retry: u32,
}
