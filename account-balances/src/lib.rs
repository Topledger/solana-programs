mod pb;
mod utils;

use std::collections::HashMap;

use pb::sf::solana::account_balance::v1::{AccountStats, Output};

use substreams_solana::pb::sf::solana::r#type::v1::{
    Block, ConfirmedTransaction, TransactionStatusMeta,
};
use utils::convert_to_date;

const VOTE_ACCOUNT: &str = "Vote111111111111111111111111111111111111111";
const SOL_MINT: &str = "So11111111111111111111111111111111111111111";

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

    for trx in &block.transactions {
        if let Some(meta) = &trx.meta {
            if transaction_contains_vote_account(&trx, &decoded_vote_account) {
                continue;
            }

            let accounts = trx.resolved_accounts_as_strings();
            update_latest_stats(&mut latest_stats, meta, &accounts, block_slot, &block_date);
        }
    }

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
    latest_stats: &mut HashMap<(String, String), AccountStats>,
    meta: &TransactionStatusMeta,
    accounts: &[String],
    block_slot: u64,
    block_date: &str,
) {
    // Update for SOL balances
    for (idx, pre_balance) in meta.pre_balances.iter().enumerate() {
        let key = (accounts[idx].clone(), SOL_MINT.to_string());
        let post_balance = meta.post_balances[idx] as f64;
        latest_stats.insert(
            key,
            create_account_stats(
                block_slot,
                block_date,
                &accounts[idx],
                SOL_MINT,
                *pre_balance as f64,
                post_balance,
            ),
        );
    }
    // Update for token balances
    update_balance_changes(latest_stats, meta, accounts, block_slot, block_date, false);
    update_balance_changes(latest_stats, meta, accounts, block_slot, block_date, true);
}

fn update_balance_changes(
    latest_stats: &mut HashMap<(String, String), AccountStats>,
    meta: &TransactionStatusMeta,
    accounts: &[String],
    block_slot: u64,
    block_date: &str,
    is_post: bool,
) {
    let token_balances = if is_post {
        &meta.post_token_balances
    } else {
        &meta.pre_token_balances
    };

    for token_balance in token_balances {
        let account = &accounts[token_balance.account_index as usize];
        let key = (account.clone(), token_balance.mint.to_string());
        let ui_amount = token_balance.ui_token_amount.clone().unwrap().ui_amount;

        latest_stats
            .entry(key)
            .and_modify(|stats| update_account_stats(stats, ui_amount, is_post))
            .or_insert_with(|| {
                create_account_stats(
                    block_slot,
                    block_date,
                    account,
                    &token_balance.mint,
                    0.0,
                    ui_amount,
                )
            });
    }
}

fn update_account_stats(stats: &mut AccountStats, balance: f64, is_post: bool) {
    if is_post {
        stats.post_balance = balance;
    } else {
        stats.pre_balance = balance;
    }
    stats.balance_change = stats.post_balance - stats.pre_balance;
}

fn create_account_stats(
    block_slot: u64,
    block_date: &str,
    account_key: &str,
    mint: &str,
    pre_balance: f64,
    post_balance: f64,
) -> AccountStats {
    AccountStats {
        block_slot,
        block_date: block_date.to_string(),
        account_key: account_key.to_string(),
        mint: mint.to_string(),
        pre_balance,
        post_balance,
        balance_change: post_balance - pre_balance,
    }
}
