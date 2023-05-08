use chrono::prelude::*;
use chrono::{DateTime, Utc};

pub fn get_timestamp() -> i64 {
    Utc::now().timestamp()
}

pub fn get_date_with_offset(fmt: &str, day: i64) -> String {
    // Time zone offset
    let offset: chrono::FixedOffset =
        FixedOffset::east_opt(8 * 3600).expect("time zone offset fail");

    let dt = Utc::now().timestamp();
    let n_day = 60 * 60 * 24 * day;
    let now = dt + n_day;
    let local = Utc.timestamp_opt(now, 0);
    let date = match local {
        chrono::LocalResult::None => panic!("tomorrow data is none!"),
        chrono::LocalResult::Single(date) => date,
        chrono::LocalResult::Ambiguous(_, _) => panic!("tomorrow data is ambiguous!"),
    };
    date.with_timezone(&offset).format(fmt).to_string()
}
