mod pb;
mod utils;

use std::collections::HashMap;

use pb::sf::solana::account_balance::v1::{AccountStats, Output};
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionStatusMeta};
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

    let mut latest_stats: HashMap<(String, String), AccountStats> = HashMap::new();

    block.transactions.iter().for_each(|trx| {
        if let (Some(meta), false) = (&trx.meta, transaction_contains_vote_account(trx, &decoded_vote_account)) {
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

fn transaction_contains_vote_account(trx: &ConfirmedTransaction, decoded_vote_account: &Vec<u8>) -> bool {
    trx.transaction
        .as_ref()
        .map_or(false, |t| t.message.as_ref().map_or(false, |m| m.account_keys.contains(decoded_vote_account)))
}

fn update_latest_stats(
    latest_stats: &mut HashMap<(String, String), AccountStats>,
    meta: &TransactionStatusMeta,
    accounts: &[String],
    block_slot: u64,
    block_date: &str,
) {
    meta.post_token_balances.iter().for_each(|token_balance| {
        let account = &accounts[token_balance.account_index as usize];
        let key = (account.clone(), token_balance.mint.to_string());
        let ui_amount = token_balance.ui_token_amount.as_ref().map_or(0.0, |amt| amt.ui_amount);
        let owner = token_balance.owner.to_string();

        latest_stats
            .entry(key)
            .and_modify(|stats| stats.post_balance = ui_amount)
            .or_insert_with(|| AccountStats {
                block_slot,
                block_date: block_date.to_string(),
                token_account: account.clone(),
                owner,
                mint: token_balance.mint.to_string(),
                post_balance: ui_amount,
            });
    });
}
