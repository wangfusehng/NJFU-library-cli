use crate::utils::def;
use crate::utils::time;
use anyhow::Context;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// State struct
/// State struct is used to store the information of the user's state.
#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub id: String,
    pub site: String,
    pub start_time: String,
    pub end_time: String,
    pub state: String,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}  {}  {}  {}  {}",
            self.id, self.site, self.start_time, self.end_time, self.state
        )
    }
}

impl State {
    pub fn new(
        id: String,
        site: String,
        start_time: String,
        end_time: String,
        state: String,
    ) -> Self {
        State {
            id,
            site,
            start_time,
            end_time,
            state,
        }
    }

    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn site(&self) -> &str {
        self.site.as_ref()
    }

    pub fn set_site(&mut self, site: String) {
        self.site = site;
    }

    pub fn start_time(&self) -> &str {
        self.start_time.as_ref()
    }

    pub fn set_start_time(&mut self, start_time: String) {
        self.start_time = start_time;
    }

    pub fn end_time(&self) -> &str {
        self.end_time.as_ref()
    }

    pub fn set_end_time(&mut self, end_time: String) {
        self.end_time = end_time;
    }

    pub fn state(&self) -> &str {
        self.state.as_ref()
    }

    pub fn set_state(&mut self, state: String) {
        self.state = state;
    }

    // judge whether the user is able to check in
    pub fn isable_to_check_in(&self) -> Result<bool> {
        let start_time = self.start_time.split(' ').collect::<Vec<&str>>();
        let start_date = start_time[0].split('-').collect::<Vec<&str>>();
        let start_month = start_date[0]
            .parse::<u32>()
            .context("Failed to parse month")?;
        let start_day = start_date[1]
            .parse::<u32>()
            .context("Failed to parse day")?;

        let start_time = start_time[1].split(':').collect::<Vec<&str>>();
        let start_hour = start_time[0]
            .parse::<u32>()
            .context("Failed to parse hour")?;
        let start_minute = start_time[1]
            .parse::<u32>()
            .context("Failed to parse minute")?;

        let reserve_time =
            time::get_utc_timestamp(start_month, start_day, start_hour, start_minute)?;
        let diff = time::get_now_timestamp()? - reserve_time;
        // in 30 minutes before the start time, the user is able to check in
        Ok((0..60 * 30).contains(&diff))
    }
}
