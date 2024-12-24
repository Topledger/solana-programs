use substreams_solana::pb::sf::solana::r#type::v1::{InnerInstructions, TokenBalance};

use crate::{
    pb::sf::solana::liquidity::providers::v1::TradeData,
    utils::{get_mint_address_for, get_token_transfer},
};

const IncreaseLiquidity: u64 = u64::from_le_bytes([46, 156, 243, 118, 13, 205, 251, 178]);
const DecreaseLiquidity: u64 = u64::from_le_bytes([160, 38, 208, 111, 104, 91, 44, 1]);
const IncreaseLiquidityV2: u64 = u64::from_le_bytes([133, 29, 89, 223, 69, 238, 176, 10]);
const DecreaseLiquidityV2: u64 = u64::from_le_bytes([58, 127, 188, 62, 79, 82, 196, 96]);

const CollectFees: u64 = u64::from_le_bytes([164, 152, 207, 99, 30, 186, 19, 182]);
const CollectFeesV2: u64 = u64::from_le_bytes([207, 117, 95, 191, 229, 180, 226, 15]);
const CollectReward: u64 = u64::from_le_bytes([70, 5, 132, 87, 86, 235, 177, 34]);
const CollectRewardV2: u64 = u64::from_le_bytes([177, 107, 37, 180, 160, 19, 49, 209]);

pub fn parse_trade_instruction(
    signer: &String,
    bytes_stream: Vec<u8>,
    accounts: &Vec<String>,
    input_accounts: Vec<String>,
    pre_token_balances: &Vec<TokenBalance>,
    post_token_balances: &Vec<TokenBalance>,
    inner_idx: u32,
    inner_instructions: &Vec<InnerInstructions>,
) -> Option<TradeData> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut td = TradeData::default();
    let mut result = None;

    match discriminator {
        IncreaseLiquidity => {
            td.instruction_type = "IncreaseLiquidity".to_string();
            td.pool = input_accounts.get(0).unwrap().to_string();
            td.account_a = input_accounts.get(7).unwrap().to_string();
            td.account_b = input_accounts.get(8).unwrap().to_string();

            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.position = input_accounts.get(3).unwrap().to_string();
            let (parsed_value, _) = rest.split_at(16);
            let parsed_value_arr: [u8; 16] = parsed_value.to_vec().try_into().unwrap();
            td.liquidity_index = u128::from_le_bytes(parsed_value_arr).to_string();

            result = Some(td);
        }
        DecreaseLiquidity => {
            td.instruction_type = "DecreaseLiquidity".to_string();
            td.pool = input_accounts.get(0).unwrap().to_string();
            td.account_a = input_accounts.get(7).unwrap().to_string();
            td.account_b = input_accounts.get(8).unwrap().to_string();

            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.position = input_accounts.get(3).unwrap().to_string();
            let (parsed_value, _) = rest.split_at(16);
            let parsed_value_arr: [u8; 16] = parsed_value.to_vec().try_into().unwrap();
            td.liquidity_index = u128::from_le_bytes(parsed_value_arr).to_string();

            result = Some(td);
        }
        IncreaseLiquidityV2 => {
            td.instruction_type = "IncreaseLiquidityV2".to_string();
            td.pool = input_accounts.get(0).unwrap().to_string();
            td.account_a = input_accounts.get(11).unwrap().to_string();
            td.account_b = input_accounts.get(12).unwrap().to_string();

            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.position = input_accounts.get(5).unwrap().to_string();
            let (parsed_value, _) = rest.split_at(16);
            let parsed_value_arr: [u8; 16] = parsed_value.to_vec().try_into().unwrap();
            td.liquidity_index = u128::from_le_bytes(parsed_value_arr).to_string();

            result = Some(td);
        }
        DecreaseLiquidityV2 => {
            td.instruction_type = "DecreaseLiquidityV2".to_string();
            td.pool = input_accounts.get(0).unwrap().to_string();
            td.account_a = input_accounts.get(11).unwrap().to_string();
            td.account_b = input_accounts.get(12).unwrap().to_string();

            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.position = input_accounts.get(5).unwrap().to_string();
            let (parsed_value, _) = rest.split_at(16);
            let parsed_value_arr: [u8; 16] = parsed_value.to_vec().try_into().unwrap();
            td.liquidity_index = u128::from_le_bytes(parsed_value_arr).to_string();

            result = Some(td);
        }
        CollectFees => {
            td.instruction_type = "CollectFees".to_string();
            td.pool = input_accounts.get(0).unwrap().to_string();
            td.account_a = input_accounts.get(5).unwrap().to_string();
            td.account_b = input_accounts.get(7).unwrap().to_string();

            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.position = input_accounts.get(2).unwrap().to_string();

            result = Some(td);
        }
        CollectFeesV2 => {
            td.instruction_type = "CollectFeesV2".to_string();
            td.pool = input_accounts.get(0).unwrap().to_string();
            td.account_a = input_accounts.get(7).unwrap().to_string();
            td.account_b = input_accounts.get(9).unwrap().to_string();

            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.position = input_accounts.get(2).unwrap().to_string();

            result = Some(td);
        }
        CollectReward => {
            td.instruction_type = "CollectReward".to_string();
            td.pool = input_accounts.get(0).unwrap().to_string();
            td.account_a = input_accounts.get(5).unwrap().to_string();
            td.account_b = "".to_string();

            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.position = input_accounts.get(2).unwrap().to_string();

            result = Some(td);
        }
        CollectRewardV2 => {
            td.instruction_type = "CollectRewardV2".to_string();
            td.pool = input_accounts.get(0).unwrap().to_string();
            td.account_a = input_accounts.get(6).unwrap().to_string();
            td.account_b = "".to_string();

            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.position = input_accounts.get(2).unwrap().to_string();

            result = Some(td);
        }
        _ => {}
    }

    return result;
}
