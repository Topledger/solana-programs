use substreams_solana::pb::sf::solana::r#type::v1::{InnerInstructions, TokenBalance};

use crate::{
    pb::sf::solana::liquidity::providers::v1::TradeData,
    utils::{get_mint_address_for, get_token_transfer},
};

const CreatePosition: u64 = u64::from_le_bytes([48, 215, 197, 153, 96, 203, 180, 133]);
const RemovePosition: u64 = u64::from_le_bytes([219, 24, 236, 110, 138, 80, 129, 6]);

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
        CreatePosition => {
            td.instruction_type = "CreatePosition".to_string();
            td.pool = input_accounts.get(2).unwrap().to_string();
            td.account_a = input_accounts.get(14).unwrap().to_string();
            td.account_b = input_accounts.get(15).unwrap().to_string();
            td.account_c = "".to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);
            td.mint_c = get_mint_address_for(&td.account_c, post_token_balances, accounts);

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

            result = Some(td);
        }
        RemovePosition => {
            td.instruction_type = "RemovePosition".to_string();
            td.pool = input_accounts.get(4).unwrap().to_string();
            td.account_a = input_accounts.get(13).unwrap().to_string();
            td.account_b = input_accounts.get(14).unwrap().to_string();
            td.account_c = "".to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);
            td.mint_c = get_mint_address_for(&td.account_c, post_token_balances, accounts);

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

            result = Some(td);
        }
        _ => {}
    }

    return result;
}
