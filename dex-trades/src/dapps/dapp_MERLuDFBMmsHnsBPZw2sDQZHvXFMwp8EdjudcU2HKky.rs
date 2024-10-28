use substreams_solana::pb::sf::solana::r#type::v1::{InnerInstructions, TokenBalance};

use crate::utils::get_amt;

use crate::trade_instruction::TradeInstruction;

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    input_accounts: Vec<String>,
    pre_token_balances: &Vec<TokenBalance>,
    post_token_balances: &Vec<TokenBalance>,
    accounts: &Vec<String>,
    input_inner_idx: u32,
    inner_instructions: &Vec<InnerInstructions>,
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
                input_inner_idx,
                inner_instructions,
                accounts,
                post_token_balances,
            );
            let obj_b = get_amt(
                &address_b,
                input_inner_idx,
                inner_instructions,
                accounts,
                post_token_balances,
            );
            let obj_c = get_amt(
                &address_c,
                input_inner_idx,
                inner_instructions,
                accounts,
                post_token_balances,
            );

            let (vault_a, vault_b) = if obj_a == 0.0 {
                (address_b.clone(), address_c.clone())
            } else if obj_b == 0.0 {
                (address_a.clone(), address_c.clone())
            } else {
                (address_a.clone(), address_b.clone())
            };

            result = Some(TradeInstruction {
                dapp_address: String::from("MERLuDFBMmsHnsBPZw2sDQZHvXFMwp8EdjudcU2HKky"),
                name: String::from("Exchange"),
                amm: input_accounts.get(0).unwrap().to_string(),
                vault_a,
                vault_b,
                ..Default::default()
            });
        }
        _ => {}
    }

    return result;
}
