mod pb;
mod utils;

use std::collections::HashMap;

use pb::sf::solana::account_sol_balance::v1::{AccountStats, Output};
use substreams_solana::pb::sf::solana::r#type::v1::{
    Block, ConfirmedTransaction, TransactionStatusMeta,
};
use utils::convert_to_date;

const VOTE_ACCOUNT: &str = "Vote111111111111111111111111111111111111111";

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Output, substreams::errors::Error> {
    let block_slot = block.slot;

    let block_date = match block.block_time.as_ref() {
        Some(block_time) => match convert_to_date(block_time.timestamp) {
            Ok(date) => date,
            Err(_) => "Error converting block time to date".to_string(),
        },
        None => "Block time is not available".to_string(),
    };

    let decoded_vote_account = bs58::decode(VOTE_ACCOUNT)
        .into_vec()
        .expect("Failed to decode vote account");

    let mut latest_stats: HashMap<String, AccountStats> = HashMap::new();

    block.transactions.iter().for_each(|trx| {
        if let (Some(meta), false) = (
            &trx.meta,
            transaction_contains_vote_account(trx, &decoded_vote_account),
        ) {
            if meta.err.is_none() && !meta.post_token_balances.is_empty() {
                let accounts = trx.resolved_accounts_as_strings();
                update_latest_stats(&mut latest_stats, meta, &accounts, block_slot, &block_date);
            }
        }
    });

    Ok(Output {
        data: latest_stats.into_values().collect(),
    })
}

fn transaction_contains_vote_account(
    trx: &ConfirmedTransaction,
    decoded_vote_account: &Vec<u8>,
) -> bool {
    trx.transaction.as_ref().map_or(false, |t| {
        t.message
            .as_ref()
            .map_or(false, |m| m.account_keys.contains(decoded_vote_account))
    })
}

fn update_latest_stats(
    latest_stats: &mut HashMap<String, AccountStats>,
    meta: &TransactionStatusMeta,
    accounts: &[String],
    block_slot: u64,
    block_date: &str,
) {
    meta.post_balances
        .iter()
        .enumerate()
        .for_each(|(index, post_balance)| {
            let account = &accounts[index].to_string();

            latest_stats
                .entry(account.clone())
                .and_modify(|stats| stats.post_balance = post_balance.clone())
                .or_insert_with(|| AccountStats {
                    block_slot: block_slot.clone(),
                    block_date: block_date.to_string(),
                    account: account.clone(),
                    post_balance: post_balance.clone(),
                });
        });
}
