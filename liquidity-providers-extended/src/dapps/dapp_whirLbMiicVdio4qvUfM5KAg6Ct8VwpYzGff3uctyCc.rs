use borsh::{BorshDeserialize, BorshSerialize};
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

const OpenPosition: u64 = u64::from_le_bytes([135, 128, 47, 77, 15, 152, 240, 49]);
const OpenPositionWithMetadata: u64 = u64::from_le_bytes([242, 29, 134, 48, 58, 110, 14, 60]);
const OpenBundledPosition: u64 = u64::from_le_bytes([169, 113, 126, 171, 213, 172, 212, 49]);
const OpenPositionWithTokenExtensions: u64 =
    u64::from_le_bytes([212, 47, 95, 92, 114, 102, 131, 250]);

const ClosePosition: u64 = u64::from_le_bytes([123, 134, 81, 0, 49, 68, 98, 98]);
const CloseBundledPosition: u64 = u64::from_le_bytes([41, 36, 216, 245, 27, 85, 103, 67]);
const ClosePositionWithTokenExtensions: u64 = u64::from_le_bytes([1, 182, 135, 59, 155, 25, 99, 223]);

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct OpenPositionBumpsLayout {
    positionBump: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct OpenPositionLayout {
    openPositionBumps: OpenPositionBumpsLayout,
    tickLowerIndex: i32,
    tickUpperIndex: i32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct OpenPositionWithMetadataBumpsLayout {
    positionBump: u8,
    metadataBump: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct OpenPositionWithMetadataLayout {
    openPositionWithMetadataBumps: OpenPositionWithMetadataBumpsLayout,
    tickLowerIndex: i32,
    tickUpperIndex: i32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct OpenBundledPositionLayout {
    bundleIndex: u16,
    tickLowerIndex: i32,
    tickUpperIndex: i32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct OpenPositionWithTokenExtensionsLayout {
    tickLowerIndex: i32,
    tickUpperIndex: i32,
    withTokenMetadataExtension: bool,
}

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
        OpenPosition => {
            td.instruction_type = "OpenPosition".to_string();
            td.pool = input_accounts.get(5).unwrap().to_string();
            td.lp_wallet = signer.to_string();
            td.position = input_accounts.get(2).unwrap().to_string();

            let data = OpenPositionLayout::deserialize(&mut rest.clone()).unwrap();
            td.tick_lower_index = data.tickLowerIndex;
            td.tick_upper_index = data.tickUpperIndex;

            result = Some(td);
        }
        OpenPositionWithMetadata => {
            td.instruction_type = "OpenPositionWithMetadata".to_string();
            td.pool = input_accounts.get(6).unwrap().to_string();
            td.lp_wallet = signer.to_string();
            td.position = input_accounts.get(2).unwrap().to_string();

            let data = OpenPositionWithMetadataLayout::deserialize(&mut rest.clone()).unwrap();
            td.tick_lower_index = data.tickLowerIndex;
            td.tick_upper_index = data.tickUpperIndex;

            result = Some(td);
        }
        OpenBundledPosition => {
            td.instruction_type = "OpenBundledPosition".to_string();
            td.pool = input_accounts.get(4).unwrap().to_string();
            td.lp_wallet = signer.to_string();
            td.position = input_accounts.get(1).unwrap().to_string();
            td.bundled_position = input_accounts.get(0).unwrap().to_string();

            let data = OpenBundledPositionLayout::deserialize(&mut rest.clone()).unwrap();
            td.tick_lower_index = data.tickLowerIndex;
            td.tick_upper_index = data.tickUpperIndex;

            result = Some(td);
        }
        OpenPositionWithTokenExtensions => {
            td.instruction_type = "OpenPositionWithTokenExtensions".to_string();
            td.pool = input_accounts.get(5).unwrap().to_string();
            td.lp_wallet = signer.to_string();
            td.position = input_accounts.get(2).unwrap().to_string();

            let data =
                OpenPositionWithTokenExtensionsLayout::deserialize(&mut rest.clone()).unwrap();
            td.tick_lower_index = data.tickLowerIndex;
            td.tick_upper_index = data.tickUpperIndex;

            result = Some(td);
        }
        ClosePosition => {
            td.instruction_type = "ClosePosition".to_string();
            td.lp_wallet = signer.to_string();
            td.position = input_accounts.get(2).unwrap().to_string();

            result = Some(td);
        }
        CloseBundledPosition => {
            td.instruction_type = "CloseBundledPosition".to_string();
            td.lp_wallet = signer.to_string();
            td.position = input_accounts.get(1).unwrap().to_string();
            td.bundled_position = input_accounts.get(0).unwrap().to_string();

            result = Some(td);
        }
        ClosePositionWithTokenExtensions => {
            td.instruction_type = "ClosePositionWithTokenExtensions".to_string();
            td.lp_wallet = signer.to_string();
            td.position = input_accounts.get(2).unwrap().to_string();

            result = Some(td);
        }
        _ => {}
    }

    return result;
}
