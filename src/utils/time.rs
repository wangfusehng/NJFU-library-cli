use chrono::prelude::*;
use chrono::{DateTime, Local};

/// get format date
pub fn get_date_today(fmt: &str) -> String {
    let utc: DateTime<_> = Local::now();
    utc.format(fmt).to_string()
}

pub fn get_date_tomorrow(fmt: &str) -> String {
    let dt = Local::now().timestamp_millis();
    let n_day = 1000 * 60 * 60 * 24;
    let now = dt + n_day;
    let local = Local.timestamp_millis_opt(now);
    let date = match local {
        chrono::LocalResult::None => panic!("tomorrow data is none!"),
        chrono::LocalResult::Single(date) => date,
        chrono::LocalResult::Ambiguous(_, _) => panic!("tomorrow data is ambiguous!"),
    };
    date.format(fmt).to_string()
}
