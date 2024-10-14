use substreams_solana::pb::sf::solana::r#type::v1::{InnerInstructions, TokenBalance};

use crate::{
    pb::sf::solana::liquidity::providers::v1::TradeData,
    utils::{get_mint_address_for, get_token_transfer},
};

const AddLiquidity: u8 = 1;
const RemoveLiquidity: u8 = 2;
const RemoveLiquidityOneToken: u8 = 3;

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
    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let disc_bytes_arr: [u8; 1] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u8 = u8::from_le_bytes(disc_bytes_arr);

    let mut td = TradeData::default();
    let mut result = None;

    match discriminator {
        AddLiquidity => {
            td.instruction_type = "AddLiquidity".to_string();

            let token_accounts_len = ((input_accounts.len() - 6) / 2) as u8;
            match token_accounts_len {
                1 => {
                    td.account_a = input_accounts.get(4).unwrap().to_string();
                }
                2 => {
                    td.account_a = input_accounts.get(4).unwrap().to_string();
                    td.account_b = input_accounts.get(5).unwrap().to_string();
                }
                3 => {
                    td.account_a = input_accounts.get(4).unwrap().to_string();
                    td.account_b = input_accounts.get(5).unwrap().to_string();
                    td.account_c = input_accounts.get(6).unwrap().to_string();
                }
                4 => {
                    td.account_a = input_accounts.get(4).unwrap().to_string();
                    td.account_b = input_accounts.get(5).unwrap().to_string();
                    td.account_c = input_accounts.get(6).unwrap().to_string();
                    td.account_d = input_accounts.get(7).unwrap().to_string();
                }
                _ => {}
            }

            td.pool = input_accounts.get(0).unwrap().to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);
            td.mint_c = get_mint_address_for(&td.account_c, post_token_balances, accounts);
            td.mint_d = get_mint_address_for(&td.account_d, post_token_balances, accounts);

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
            td.token_c_amount = get_token_transfer(
                &td.account_c,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.token_d_amount = get_token_transfer(
                &td.account_d,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );

            result = Some(td);
        }
        RemoveLiquidity => {
            td.instruction_type = "RemoveLiquidity".to_string();

            let token_accounts_len = ((input_accounts.len() - 6) / 2) as u8;
            match token_accounts_len {
                1 => {
                    td.account_a = input_accounts.get(4).unwrap().to_string();
                }
                2 => {
                    td.account_a = input_accounts.get(4).unwrap().to_string();
                    td.account_b = input_accounts.get(5).unwrap().to_string();
                }
                3 => {
                    td.account_a = input_accounts.get(4).unwrap().to_string();
                    td.account_b = input_accounts.get(5).unwrap().to_string();
                    td.account_c = input_accounts.get(6).unwrap().to_string();
                }
                4 => {
                    td.account_a = input_accounts.get(4).unwrap().to_string();
                    td.account_b = input_accounts.get(5).unwrap().to_string();
                    td.account_c = input_accounts.get(6).unwrap().to_string();
                    td.account_d = input_accounts.get(7).unwrap().to_string();
                }
                _ => {}
            }

            td.pool = input_accounts.get(0).unwrap().to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);
            td.mint_c = get_mint_address_for(&td.account_c, post_token_balances, accounts);
            td.mint_d = get_mint_address_for(&td.account_d, post_token_balances, accounts);

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
            td.token_c_amount = get_token_transfer(
                &td.account_c,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.token_d_amount = get_token_transfer(
                &td.account_d,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );

            result = Some(td);
        }
        RemoveLiquidityOneToken => {
            td.instruction_type = "RemoveLiquidityOneToken".to_string();

            let token_accounts_len: u8 = (input_accounts.len() - 7) as u8;
            match token_accounts_len {
                1 => {
                    td.account_a = input_accounts.get(4).unwrap().to_string();
                }
                2 => {
                    td.account_a = input_accounts.get(4).unwrap().to_string();
                    td.account_b = input_accounts.get(5).unwrap().to_string();
                }
                3 => {
                    td.account_a = input_accounts.get(4).unwrap().to_string();
                    td.account_b = input_accounts.get(5).unwrap().to_string();
                    td.account_c = input_accounts.get(6).unwrap().to_string();
                }
                4 => {
                    td.account_a = input_accounts.get(4).unwrap().to_string();
                    td.account_b = input_accounts.get(5).unwrap().to_string();
                    td.account_c = input_accounts.get(6).unwrap().to_string();
                    td.account_d = input_accounts.get(7).unwrap().to_string();
                }
                _ => {}
            }

            td.pool = input_accounts.get(0).unwrap().to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);
            td.mint_c = get_mint_address_for(&td.account_c, post_token_balances, accounts);
            td.mint_d = get_mint_address_for(&td.account_d, post_token_balances, accounts);

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
            td.token_c_amount = get_token_transfer(
                &td.account_c,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.token_d_amount = get_token_transfer(
                &td.account_d,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );

            result = Some(td);
        }
        _ => {}
    }

    return result;
}
