use chrono::prelude::*;
use chrono::{DateTime, Utc};

pub fn get_timestamp() -> i64 {
    Utc::now().timestamp()
}

/// get format date
pub fn get_today_time(fmt: &str) -> String {
    // Time zone offset
    let offset: chrono::FixedOffset =
        FixedOffset::east_opt(8 * 3600).expect("time zone offset fail");

    Utc::now().with_timezone(&offset).format(fmt).to_string()
}

pub fn get_tomorrow_time(fmt: &str) -> String {
    // Time zone offset
    let offset: chrono::FixedOffset =
        FixedOffset::east_opt(8 * 3600).expect("time zone offset fail");

    let dt = Utc::now().timestamp();
    let n_day = 60 * 60 * 24;
    let now = dt + n_day;
    let local = Utc.timestamp_opt(now, 0);
    let date = match local {
        chrono::LocalResult::None => panic!("tomorrow data is none!"),
        chrono::LocalResult::Single(date) => date,
        chrono::LocalResult::Ambiguous(_, _) => panic!("tomorrow data is ambiguous!"),
    };
    date.with_timezone(&offset).format(fmt).to_string()
}
