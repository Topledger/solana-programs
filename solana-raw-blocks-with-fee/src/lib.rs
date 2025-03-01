mod pb;
mod utils;

use pb::sf::solana::raw::blocks::v1::{BlockStat, Output};

use substreams_solana::pb::sf::solana::r#type::v1::Block;
use utils::convert_to_date;

const VOTE_ACCOUNT: &str = "Vote111111111111111111111111111111111111111";

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Output, substreams::errors::Error> {
    let result = process_block(block);
    Ok(Output { data: vec![result] })
}

fn process_block(block: Block) -> BlockStat {
    let block_time_option = block.block_time.as_ref();
    let block_date =
        block_time_option.and_then(|block_time| convert_to_date(block_time.timestamp).ok());

    let block_time = block_time_option.map(|block_time| block_time.timestamp as u64);
    let mut block_stat = BlockStat::default();

    block_stat.block_slot = block.slot as u64;
    block_stat.block_date = block_date.unwrap_or_else(|| "2020-03-15".to_string());
    block_stat.block_time = block_time.unwrap_or(0);
    block_stat.block_height = block.block_height.unwrap_or_default().block_height;
    block_stat.block_hash = block.blockhash;
    block_stat.parent_slot = block.parent_slot;
    block_stat.previous_block_hash = block.previous_blockhash;

    // Initialize transaction-related stats
    block_stat.total_transactions = 0;
    block_stat.successful_transactions = 0;
    block_stat.failed_transactions = 0;
    block_stat.total_vote_transactions = 0;
    block_stat.total_non_vote_transactions = 0;
    block_stat.successful_vote_transactions = 0;
    block_stat.successful_non_vote_transactions = 0;
    block_stat.failed_vote_transactions = 0;
    block_stat.failed_non_vote_transactions = 0;

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

        block_stat.total_transactions += 1;

        if is_vote_transaction {
            block_stat.total_vote_transactions += 1;
        } else {
            block_stat.total_non_vote_transactions += 1;
        }

        if let Some(meta) = trx.meta.as_ref() {
            block_stat.total_fee += meta.fee;

            if meta.err.is_none() {
                // Successful transaction
                block_stat.successful_transactions += 1;

                if is_vote_transaction {
                    block_stat.successful_vote_transactions += 1;
                    block_stat.successful_vote_transactions_fee += meta.fee;
                } else {
                    block_stat.successful_non_vote_transactions += 1;
                    block_stat.successful_non_vote_transactions_fee += meta.fee;
                    block_stat.successful_non_vote_transactions_priority_fee += (meta.fee
                        - 5000
                            * transaction
                                .message
                                .clone()
                                .unwrap()
                                .header
                                .unwrap()
                                .num_required_signatures as u64);
                }
            } else {
                // Failed transaction
                block_stat.failed_transactions += 1;

                if is_vote_transaction {
                    block_stat.failed_vote_transactions += 1;
                    block_stat.failed_vote_transactions_fee += meta.fee;
                } else {
                    block_stat.failed_non_vote_transactions += 1;
                    block_stat.failed_non_vote_transactions_fee += meta.fee;
                }
            }
        }
    }

    block_stat
}
