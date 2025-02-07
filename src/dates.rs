use chrono::{DateTime, Datelike, TimeZone, Utc};
use log::debug;

pub fn get_last_month_time_box_utc() -> (DateTime<Utc>, DateTime<Utc>) {
    let now = Utc::now();
    let year = now.year();
    let month = now.month();

    let (prev_year, prev_month) = if month == 1 {
        (year - 1, 12)
    } else {
        (year, month - 1)
    };

    let start_of_month = Utc.with_ymd_and_hms(prev_year, prev_month, 1, 0, 0, 0).unwrap();
    let end_of_month = Utc.with_ymd_and_hms(year, month, 1, 0, 0, 0).unwrap();
    debug!("start: {:?} and end: {:?}", start_of_month, end_of_month);

    (start_of_month, end_of_month)
}
