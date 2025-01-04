mod pb;
mod utils;

use pb::sf::solana::block::rewards::v1::{BlockReward, Output};
use substreams::log;

use substreams_solana::pb::sf::solana::r#type::v1::Block;
use utils::convert_to_date;

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Output, substreams::errors::Error> {
    let mut data: Vec<BlockReward> = vec![];

    block.rewards.iter().for_each(|reward| {
        let block_time_option = block.block_time.as_ref();
        let block_date =
            block_time_option.and_then(|block_time| convert_to_date(block_time.timestamp).ok());
        let block_time = block_time_option.map(|block_time| block_time.timestamp as u64);

        let mut block_reward = BlockReward::default();

        block_reward.block_slot = block.slot as u64;
        block_reward.block_date = block_date.unwrap_or_else(|| "2020-03-15".to_string());
        block_reward.block_time = block_time.unwrap_or(0);
        block_reward.pubkey = reward.pubkey.clone();
        block_reward.lamports = reward.lamports;
        block_reward.post_balance = reward.post_balance;
        block_reward.reward_type = reward.reward_type as i64;
        block_reward.commission = reward.commission.clone();

        data.push(block_reward);
    });

    Ok(Output { data })
}
