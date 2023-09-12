use anyhow::Context;
use anyhow::Result;
use chrono::prelude::*;
use chrono::NaiveDate;
use chrono::Utc;

pub fn get_utc_timestamp(month: u32, day: u32, hour: u32, minute: u32) -> Result<i64> {
    // Time zone offset
    let offset: chrono::FixedOffset =
        FixedOffset::east_opt(8 * 3600).expect("time zone offset fail");
    let time = Utc::now().with_timezone(&offset);
    let year = time.year();

    let date_time = NaiveDate::from_ymd_opt(year, month, day)
        .context("time error")?
        .and_hms_opt(hour, minute, 0)
        .context("time error")?;
    Ok(date_time.timestamp() - 60 * 60 * 8)
}

pub fn get_now_timestamp() -> Result<i64> {
    // Time zone offset
    let offset: chrono::FixedOffset =
        FixedOffset::east_opt(8 * 3600).expect("time zone offset fail");
    let time = Utc::now().with_timezone(&offset);
    Ok(time.timestamp())
}

pub fn get_date_with_offset(fmt: &str, day: u32) -> String {
    // Time zone offset
    let offset: chrono::FixedOffset =
        FixedOffset::east_opt(8 * 3600).expect("time zone offset fail");

    let dt = Utc::now().timestamp();
    let n_day = 60 * 60 * 24 * day as i64;
    let now = dt + n_day;
    let local = Utc.timestamp_opt(now, 0);
    let date = match local {
        chrono::LocalResult::None => panic!("tomorrow data is none!"),
        chrono::LocalResult::Single(date) => date,
        chrono::LocalResult::Ambiguous(_, _) => panic!("tomorrow data is ambiguous!"),
    };
    date.with_timezone(&offset).format(fmt).to_string()
}

pub fn get_date_with_time_stamp(time_stamp: u64) -> String {
    // Time zone offset
    let offset: chrono::FixedOffset =
        FixedOffset::east_opt(8 * 3600).expect("time zone offset fail");
    let datetime = NaiveDateTime::from_timestamp_opt(time_stamp as i64, 0).unwrap();
    let offset_datetime = offset.from_utc_datetime(&datetime);

    offset_datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
