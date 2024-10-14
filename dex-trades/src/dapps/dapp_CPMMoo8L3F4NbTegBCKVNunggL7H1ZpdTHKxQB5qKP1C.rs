use crate::trade_instruction::TradeInstruction;

const SWAP_BASE_INPUT_PARTNER_DISCRIMINATOR: u64 =
    u64::from_le_bytes([143, 190, 90, 218, 196, 30, 51, 222]);
const SWAP_BASE_OUTPUT_PARTNER_DISCRIMINATOR: u64 =
    u64::from_le_bytes([55, 217, 98, 86, 163, 74, 180, 173]);

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    accounts: Vec<String>,
) -> Option<TradeInstruction> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut result = None;

    match discriminator {
        SWAP_BASE_INPUT_PARTNER_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C"),
                name: String::from("SwapBaseInput"),
                amm: accounts.get(3).unwrap().to_string(),
                vault_a: accounts.get(6).unwrap().to_string(),
                vault_b: accounts.get(7).unwrap().to_string(),
            });
        }
        SWAP_BASE_OUTPUT_PARTNER_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C"),
                name: String::from("SwapBaseOutput"),
                amm: accounts.get(3).unwrap().to_string(),
                vault_a: accounts.get(6).unwrap().to_string(),
                vault_b: accounts.get(7).unwrap().to_string(),
            });
        }
        _ => {}
    }

    return result;
}
