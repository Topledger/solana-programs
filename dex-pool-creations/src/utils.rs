extern crate chrono;
use borsh::{BorshDeserialize, BorshSerialize};
use chrono::prelude::*;
use substreams_solana::pb::sf::solana::r#type::v1::{InnerInstructions, TokenBalance};

use crate::prepare_input_accounts;

pub fn convert_to_date(ts: i64) -> String {
    let nt = NaiveDateTime::from_timestamp_opt(ts, 0);
    let dt: DateTime<Utc> = DateTime::from_naive_utc_and_offset(nt.unwrap(), Utc);
    let res = dt.format("%Y-%m-%d");
    return res.to_string();
}
