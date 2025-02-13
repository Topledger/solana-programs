use substreams_solana::pb::sf::solana::r#type::v1::{InnerInstructions, TokenBalance};

use crate::{
    pb::sf::solana::liquidity::providers::v1::TradeData,
    utils::{get_mint_address_for, get_token_transfer},
};

const Deposit: u8 = 3;
const Withdraw: u8 = 4;
const Initialize: u8 = 0;
const Initialize2: u8 = 1;

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
        Deposit => {
            td.instruction_type = "Deposit".to_string();
            td.pool = input_accounts.get(1).unwrap().to_string();
            td.account_a = input_accounts.get(6).unwrap().to_string();
            td.account_b = input_accounts.get(7).unwrap().to_string();
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
        Withdraw => {
            td.instruction_type = "Withdraw".to_string();
            td.pool = input_accounts.get(1).unwrap().to_string();
            td.account_a = input_accounts.get(6).unwrap().to_string();
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

            result = Some(td);
        }
        Initialize => {
            td.instruction_type = "Initialize".to_string();
            td.pool = input_accounts.get(4).unwrap().to_string();
            td.account_a = input_accounts.get(10).unwrap().to_string();
            td.account_b = input_accounts.get(11).unwrap().to_string();
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
        Initialize2 => {
            td.instruction_type = "Initialize2".to_string();
            td.pool = input_accounts.get(4).unwrap_or(&"".to_string()).to_string();
            td.account_a = input_accounts
                .get(10)
                .unwrap_or(&"".to_string())
                .to_string();

            if input_accounts.get(11).is_some() {
                td.account_b = input_accounts.get(11).unwrap().to_string();
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
        }
        _ => {}
    }

    return result;
}
