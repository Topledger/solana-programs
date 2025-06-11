use crate::trade_instruction::TradeInstruction;

const BUY_EXACT_OUT_DISCRIMINATOR: u64 = u64::from_le_bytes([24, 211, 116, 40, 105, 3, 153, 56]);
const BUY_EXACT_IN_DISCRIMINATOR: u64 = u64::from_le_bytes([250, 234, 13, 123, 213, 156, 19, 236]);
const SELL_EXACT_IN_DISCRIMINATOR: u64 = u64::from_le_bytes([149, 39, 222, 155, 211, 124, 152, 26]);
const SELL_EXACT_OUT_DISCRIMINATOR: u64 = u64::from_le_bytes([95, 200, 71, 34, 8, 9, 11, 166]);

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    accounts: Vec<String>,
) -> Option<TradeInstruction> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut result = None;

    match discriminator {
        BUY_EXACT_OUT_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj"),
                name: String::from("BuyExactOut"),
                amm: accounts.get(4).unwrap().to_string(),
                vault_a: accounts.get(7).unwrap().to_string(),
                vault_b: accounts.get(8).unwrap().to_string(),
                ..Default::default()
            });
        }
        BUY_EXACT_IN_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj"),
                name: String::from("BuyExactIn"),
                amm: accounts.get(4).unwrap().to_string(),
                vault_a: accounts.get(7).unwrap().to_string(),
                vault_b: accounts.get(8).unwrap().to_string(),
                ..Default::default()
            });
        }
        SELL_EXACT_IN_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj"),
                name: String::from("SellExactIn"),
                amm: accounts.get(4).unwrap().to_string(),
                vault_a: accounts.get(7).unwrap().to_string(),
                vault_b: accounts.get(8).unwrap().to_string(),
                ..Default::default()
            });
        }
        SELL_EXACT_OUT_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj"),
                name: String::from("SellExactOut"),
                amm: accounts.get(4).unwrap().to_string(),
                vault_a: accounts.get(7).unwrap().to_string(),
                vault_b: accounts.get(8).unwrap().to_string(),
                ..Default::default()
            });
        }
        _ => {}
    }

    return result;
} 