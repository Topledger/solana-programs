mod pb;
extern crate chrono;
use chrono::prelude::*;

use pb::sf::solana::transactions::v1::{Output, TransactionStats};

use substreams_solana::pb::sf::solana::r#type::v1::{
    Block
};

const VOTE_ACCOUNT: &str = "Vote111111111111111111111111111111111111111";

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Output, substreams::errors::Error> {
    let mut data = Vec::new();

    let block_time = block.block_time.as_ref();
    let block_date = match block_time {
        Some(block_time) => match convert_to_date(block_time.timestamp) {
            Ok(date) => date,
            Err(_) => "Error converting block time to date".to_string(),
        },
        None => "Block time is not available".to_string(),
    };

    let decoded_vote_account = bs58::decode(VOTE_ACCOUNT)
        .into_vec()
        .expect("Failed to decode vote account");

    for (index, trx) in block.transactions.iter().enumerate() {
        let meta = match trx.meta.as_ref() {
            Some(meta) => meta,
            None => continue,
        };

        let transaction = match trx.transaction.as_ref() {
            Some(transaction) => transaction,
            None => continue,
        };

        let message = transaction.message.as_ref().expect("Message is missing");

        // Skip Vote Transactions
        if message.account_keys.contains(&decoded_vote_account) {
            continue;
        }

        let accounts = trx.resolved_accounts_as_strings();
        let header = message.header.as_ref().expect("Header is missing");

        let mut transaction_stats = TransactionStats::default();
        transaction_stats.block_date = block_date.clone();
        transaction_stats.block_slot = block.slot;
        transaction_stats.block_time = block_time.unwrap().timestamp as u64;
        transaction_stats.fees = meta.fee;
        transaction_stats.success = !meta.err.is_some();
        transaction_stats.signer = accounts[0].to_string();
        transaction_stats.required_signatures = header.num_required_signatures;

        data.push(transaction_stats);
    }

    Ok(Output { data })
}

pub fn convert_to_date(ts: i64) -> Result<String, &'static str> {
    let nt = NaiveDateTime::from_timestamp_opt(ts, 0).ok_or("Invalid timestamp")?;

    let dt: DateTime<Utc> = Utc.from_utc_datetime(&nt);
    Ok(dt.format("%Y-%m-%d").to_string())
}
