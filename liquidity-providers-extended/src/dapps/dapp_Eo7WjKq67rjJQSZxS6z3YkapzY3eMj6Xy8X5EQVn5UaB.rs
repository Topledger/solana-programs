use substreams_solana::pb::sf::solana::r#type::v1::{InnerInstructions, TokenBalance};

use crate::{
    pb::sf::solana::liquidity::providers::v1::TradeData,
    utils::{get_mint_address_for, get_token_transfer},
};

const AddBalanceLiquidity: u64 = u64::from_le_bytes([168, 227, 50, 62, 189, 171, 84, 176]);
const RemoveBalanceLiquidity: u64 = u64::from_le_bytes([133, 109, 44, 179, 56, 238, 114, 33]);
const RemoveLiquiditySingleSide: u64 = u64::from_le_bytes([84, 84, 177, 66, 254, 185, 10, 251]);
const BootstrapLiquidity: u64 = u64::from_le_bytes([4, 228, 215, 71, 225, 253, 119, 206]);
const AddImbalanceLiquidity: u64 = u64::from_le_bytes([79, 35, 122, 84, 173, 15, 93, 191]);

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
        AddBalanceLiquidity => {
            td.instruction_type = "AddBalanceLiquidity".to_string();
            td.pool = input_accounts.get(0).unwrap().to_string();
            td.account_a = input_accounts.get(9).unwrap().to_string();
            td.account_b = input_accounts.get(10).unwrap().to_string();
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

            result = Some(td);
        }
        RemoveBalanceLiquidity => {
            td.instruction_type = "RemoveBalanceLiquidity".to_string();
            td.pool = input_accounts.get(0).unwrap().to_string();
            td.account_a = input_accounts.get(9).unwrap().to_string();
            td.account_b = input_accounts.get(10).unwrap().to_string();
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

            result = Some(td);
        }
        RemoveLiquiditySingleSide => {
            td.instruction_type = "RemoveLiquiditySingleSide".to_string();
            td.pool = input_accounts.get(0).unwrap().to_string();
            td.account_a = input_accounts.get(9).unwrap().to_string();
            td.account_b = input_accounts.get(10).unwrap().to_string();
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

            result = Some(td);
        }
        BootstrapLiquidity => {
            td.instruction_type = "BootstrapLiquidity".to_string();
            td.pool = input_accounts.get(0).unwrap().to_string();
            td.account_a = input_accounts.get(9).unwrap().to_string();
            td.account_b = input_accounts.get(10).unwrap().to_string();
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

            result = Some(td);
        }
        AddImbalanceLiquidity => {
            td.instruction_type = "AddImbalanceLiquidity".to_string();
            td.pool = input_accounts.get(0).unwrap().to_string();
            td.account_a = input_accounts.get(9).unwrap().to_string();
            td.account_b = input_accounts.get(10).unwrap().to_string();
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

            result = Some(td);
        }
        _ => {}
    }

    return result;
}
