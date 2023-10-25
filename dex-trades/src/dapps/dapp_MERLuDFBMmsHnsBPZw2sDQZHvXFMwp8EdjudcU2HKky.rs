use substreams_solana::pb::sf::solana::r#type::v1::TokenBalance;

use crate::trade_instruction::TradeInstruction;

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    input_accounts: Vec<String>,
    pre_token_balances: &Vec<TokenBalance>,
    post_token_balances: &Vec<TokenBalance>,
    accounts: &Vec<String>,
) -> Option<TradeInstruction> {
    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let discriminator: u8 = u8::from(disc_bytes[0]);

    let mut result = None;

    match discriminator {
        4 => {
            let address_a = input_accounts.get(4).unwrap().to_string();
            let address_b = input_accounts.get(5).unwrap().to_string();
            let address_c = input_accounts.get(6).unwrap().to_string();

            let obj_a = get_amt(
                &address_a,
                pre_token_balances,
                post_token_balances,
                accounts,
            );
            let obj_b = get_amt(
                &address_b,
                pre_token_balances,
                post_token_balances,
                accounts,
            );
            let obj_c = get_amt(
                &address_c,
                pre_token_balances,
                post_token_balances,
                accounts,
            );

            let (vault_a, vault_b) = if obj_a.0 == 0.0 {
                (obj_b.1.clone(), obj_c.1.clone())
            } else if obj_b.0 == 0.0 {
                (obj_a.1.clone(), obj_c.1.clone())
            } else {
                (obj_a.1.clone(), obj_b.1.clone())
            };

            result = Some(TradeInstruction {
                dapp_address: String::from("MERLuDFBMmsHnsBPZw2sDQZHvXFMwp8EdjudcU2HKky"),
                name: String::from("Exchange"),
                amm: accounts.get(0).unwrap().to_string(),
                vault_a,
                vault_b,
            });
        }
        _ => {}
    }

    return result;
}

fn get_amt(
    address: &String,
    pre_token_balances: &Vec<TokenBalance>,
    post_token_balances: &Vec<TokenBalance>,
    accounts: &Vec<String>,
) -> (f64, String) {
    let index = accounts.iter().position(|r| r == address).unwrap();

    let mut pre_balance: f64 = 0 as f64;
    let mut post_balance: f64 = 0 as f64;

    pre_token_balances
        .iter()
        .filter(|token_balance| token_balance.account_index == index as u32)
        .for_each(|token_balance: &TokenBalance| {
            pre_balance = token_balance.ui_token_amount.clone().unwrap().ui_amount;
        });

    post_token_balances
        .iter()
        .filter(|token_balance| token_balance.account_index == index as u32)
        .for_each(|token_balance: &TokenBalance| {
            post_balance = token_balance.ui_token_amount.clone().unwrap().ui_amount;
        });

    return ((post_balance - pre_balance), address.clone());
}
