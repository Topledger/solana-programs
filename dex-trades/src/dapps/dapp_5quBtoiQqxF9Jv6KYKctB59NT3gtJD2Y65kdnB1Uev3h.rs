use crate::trade_instruction::TradeInstruction;

// Instruction discriminators for Raydium stable AMM (non-anchor)
const SWAP_BASE_IN_DISCRIMINATOR: u8 = 9;
const SWAP_BASE_OUT_DISCRIMINATOR: u8 = 11;

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    accounts: Vec<String>,
) -> Option<TradeInstruction> {
    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let discriminator: u8 = u8::from(disc_bytes[0]);

    let mut result = None;

    match discriminator {
        SWAP_BASE_IN_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("5quBtoiQqxF9Jv6KYKctB59NT3gtJD2Y65kdnB1Uev3h"),
                name: String::from("SwapBaseIn"),
                amm: accounts.get(1).unwrap().to_string(),
                vault_a: accounts.get(4).unwrap().to_string(),
                vault_b: accounts.get(5).unwrap().to_string(),
                ..Default::default()
            });
        }
        SWAP_BASE_OUT_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("5quBtoiQqxF9Jv6KYKctB59NT3gtJD2Y65kdnB1Uev3h"),
                name: String::from("SwapBaseOut"),
                amm: accounts.get(1).unwrap().to_string(),
                vault_a: accounts.get(4).unwrap().to_string(),
                vault_b: accounts.get(5).unwrap().to_string(),
                ..Default::default()
            });
        }
        _ => {}
    }

    return result;
} 