use crate::utils::def;
use serde::{Deserialize, Serialize};

/// # State struct
/// State struct is used to store the information of the user's state.
#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub id: String,
    pub site: String,
    pub start_time: String,
    pub end_time: String,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\t{}\t\t{}\t\t{}",
            self.id, self.site, self.start_time, self.end_time
        )
    }
}

impl State {
    pub fn new(id: String, site: String, start_time: String, end_time: String) -> Self {
        State {
            id,
            site,
            start_time,
            end_time,
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
}
