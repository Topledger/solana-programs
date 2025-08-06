mod pb;
mod utils;

use pb::sf::solana::{
    r#type::v1::RewardType,
    raw::blocks::rewards::v1::{BlockReward, Output},
};

use substreams_solana::pb::sf::solana::r#type::v1::Block;
use utils::convert_to_date;

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Output, substreams::errors::Error> {
    let result = process_block(block);
    Ok(Output { data: result })
}

fn process_block(block: Block) -> Vec<BlockReward> {
    let block_time_option = block.block_time.as_ref();
    let block_date = block_time_option
        .and_then(|block_time| convert_to_date(block_time.timestamp).ok())
        .unwrap_or_else(|| "2020-03-15".to_string());

    let block_time = block_time_option.map_or(0, |block_time| block_time.timestamp as u64);
    let block_hash: String = block.blockhash;

    let mut block_rewards: Vec<BlockReward> = Vec::new();

    block.rewards.iter().for_each(|reward| {
        let mut block_reward = BlockReward::default();

        block_reward.block_date = block_date.clone();
        block_reward.block_slot = block.slot as u64;
        block_reward.block_timestamp = block_time;
        block_reward.pubkey = reward.pubkey.to_string();
        block_reward.lamports = reward.lamports;
        block_reward.post_balance = reward.post_balance;
        block_reward.reward_type = reward_type_name(reward.reward_type).to_string();
        block_reward.commission = match reward.commission.parse::<u64>() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Invalid commission “{}”: {}", reward.commission, e);
                0  // or some sensible default
            }
        };
        block_reward.block_hash = block_hash.to_string();

        block_rewards.push(block_reward);
    });

    block_rewards
}

fn reward_type_name(value: i32) -> &'static str {
    match RewardType::from_i32(value) {
        Some(RewardType::Unspecified) => "UNSPECIFIED",
        Some(RewardType::Fee) => "FEE",
        Some(RewardType::Rent) => "RENT",
        Some(RewardType::Staking) => "STAKING",
        Some(RewardType::Voting) => "VOTING",
        None => "UNKNOWN",
    }
}
