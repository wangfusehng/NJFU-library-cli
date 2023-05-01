use chrono::prelude::*;
use chrono::{DateTime, Utc};

/// get format date
pub fn get_date_today(fmt: &str) -> String {
    // Time zone offset
    let offset: chrono::FixedOffset =
        FixedOffset::east_opt(8 * 3600).expect("time zone offset fail");

    Utc::now().with_timezone(&offset).format(fmt).to_string()
}

pub fn get_date_tomorrow(fmt: &str) -> String {
    // Time zone offset
    let offset: chrono::FixedOffset =
        FixedOffset::east_opt(8 * 3600).expect("time zone offset fail");

    let dt = Utc::now().timestamp_millis();
    let n_day = 1000 * 60 * 60 * 24;
    let now = dt + n_day;
    let local = Utc.timestamp_millis_opt(now);
    let date = match local {
        chrono::LocalResult::None => panic!("tomorrow data is none!"),
        chrono::LocalResult::Single(date) => date,
        chrono::LocalResult::Ambiguous(_, _) => panic!("tomorrow data is ambiguous!"),
    };
    date.with_timezone(&offset).format(fmt).to_string()
}
