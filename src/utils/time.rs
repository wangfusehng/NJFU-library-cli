use chrono::{DateTime, Local};

/// get format date
pub fn get_date(fmt: &str) -> String {
    let utc: DateTime<_> = Local::now();
    utc.format(fmt).to_string()
}
