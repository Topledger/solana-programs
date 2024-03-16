mod pb;
mod utils;

use pb::sf::{
    solana::block_stats::v1::{BlockStats, Reward},
    substreams::sink::files::v1::Lines,
};

use serde_json::json;

use substreams_solana::pb::sf::solana::r#type::v1::Block;
use utils::convert_to_date;

const VOTE_ACCOUNT: &str = "Vote111111111111111111111111111111111111111";

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<BlockStats, substreams::errors::Error> {
    let block_stats = process_block(block);
    Ok(block_stats)
}

#[substreams::handlers::map]
fn jsonl_out(block: Block) -> Result<Lines, substreams::errors::Error> {
    let block_stats = process_block(block);
    Ok(Lines {
        lines: vec![json!(block_stats).to_string()],
    })
}

fn process_block(block: Block) -> BlockStats {
    let block_time_option = block.block_time.as_ref();
    let block_date =
        block_time_option.and_then(|block_time| convert_to_date(block_time.timestamp).ok());

    let block_time = block_time_option.map(|block_time| block_time.timestamp as u64);

    let mut block_stats = BlockStats::default();

    // Setting initial block properties
    block_stats.block_slot = block.slot as u64;
    block_stats.block_date = block_date.unwrap_or_else(|| "2020-03-15".to_string());
    block_stats.block_time = block_time.unwrap_or(0);
    block_stats.height = block.block_height.unwrap_or_default().block_height;
    block_stats.hash = block.blockhash;
    block_stats.parent_slot = block.parent_slot;
    block_stats.previous_block_hash = block.previous_blockhash;

    // Initialize transaction-related stats
    block_stats.total_transactions = 0;
    block_stats.successful_transactions = 0;
    block_stats.failed_transactions = 0;
    block_stats.total_vote_transactions = 0;
    block_stats.total_non_vote_transactions = 0;
    block_stats.successful_vote_transactions = 0;
    block_stats.successful_non_vote_transactions = 0;
    block_stats.failed_non_vote_transactions = 0;

    let decoded_vote_account = bs58::decode(VOTE_ACCOUNT)
        .into_vec()
        .expect("Failed to decode vote account");

    for trx in block.transactions.iter() {
        let transaction = match trx.transaction.as_ref() {
            Some(transaction) => transaction,
            None => continue,
        };
        let message = transaction.message.as_ref().expect("Message is missing");
        let is_vote_transaction = message.account_keys.contains(&decoded_vote_account);

        block_stats.total_transactions += 1;

        if is_vote_transaction {
            block_stats.total_vote_transactions += 1;
        } else {
            block_stats.total_non_vote_transactions += 1;
        }

        if let Some(meta) = trx.meta.as_ref() {
            if meta.err.is_none() {
                // Successful transaction
                block_stats.successful_transactions += 1;
                if is_vote_transaction {
                    block_stats.successful_vote_transactions += 1;
                } else {
                    block_stats.successful_non_vote_transactions += 1;
                }
            } else {
                // Failed transaction
                block_stats.failed_transactions += 1;
                if !is_vote_transaction {
                    block_stats.failed_non_vote_transactions += 1;
                }
            }
        }
    }

    // Assuming `Reward` structure exists and matches the expected format.
    block_stats.block_rewards = block
        .rewards
        .iter()
        .map(|reward| Reward {
            pubkey: reward.pubkey.to_string(),
            lamports: reward.lamports as u64,
            post_balance: reward.post_balance,
            reward_type: reward.reward_type as u64,
            commission: reward.commission.to_string(),
        })
        .collect();
    return block_stats;
}
