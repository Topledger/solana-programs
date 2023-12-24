extern crate serde;
extern crate serde_json;

extern crate chrono;
use chrono::prelude::*;

pub fn convert_to_date(ts: i64) -> Result<String, &'static str> {
    let nt = NaiveDateTime::from_timestamp_opt(ts, 0).ok_or("Invalid timestamp")?;

    let dt: DateTime<Utc> = Utc.from_utc_datetime(&nt);
    Ok(dt.format("%Y-%m-%d").to_string())
}
