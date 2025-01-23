mod pb;
mod utils;

use pb::sf::solana::raw::blocks::v1::{BlockStat, Reward, Output} ;

use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams_solana::pb::sf::solana::r#type::v1::RewardType;
use utils::convert_to_date;

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Output, substreams::errors::Error> {
    let result = process_block(block);
    Ok(Output{ data: vec![result]})
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

    block_stat.rewards = block
        .rewards
        .iter()
        .map(|reward| Reward {
            pubkey: reward.pubkey.to_string(),
            lamports: reward.lamports as u64,
            post_balance: reward.post_balance,
            reward_type: reward_type_name(reward.reward_type).to_string(),
            commission: reward.commission.to_string(),
        })
        .collect();
    block_stat
}


fn reward_type_name(value: i32) -> &'static str {
    match RewardType::from_i32(value) {
        Some(RewardType::Unspecified) => "UNSPECIFIED",
        Some(RewardType::Fee)         => "FEE",
        Some(RewardType::Rent)        => "RENT",
        Some(RewardType::Staking)     => "STAKING",
        Some(RewardType::Voting)      => "VOTING",
        None                          => "UNKNOWN", // handle invalid values
    }
}
