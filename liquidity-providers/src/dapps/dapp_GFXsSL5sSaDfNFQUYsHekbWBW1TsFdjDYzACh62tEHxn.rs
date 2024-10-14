use substreams_solana::pb::sf::solana::r#type::v1::{InnerInstructions, TokenBalance};

use crate::{
    pb::sf::solana::liquidity::providers::v1::TradeData,
    utils::{get_mint_address_for, get_token_transfer},
};

const Deposit: u64 = u64::from_le_bytes([242, 35, 198, 137, 82, 225, 242, 182]);
const Withdraw: u64 = u64::from_le_bytes([183, 18, 70, 156, 148, 109, 161, 34]);

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
        Deposit => {
            td.instruction_type = "Deposit".to_string();
            td.pool = input_accounts.get(0).unwrap().to_string();
            td.account_a = input_accounts.get(4).unwrap().to_string();
            td.account_b = "".to_string();
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
        Withdraw => {
            td.instruction_type = "Withdraw".to_string();
            td.pool = input_accounts.get(0).unwrap().to_string();
            td.account_a = input_accounts.get(4).unwrap().to_string();
            td.account_b = "".to_string();
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
