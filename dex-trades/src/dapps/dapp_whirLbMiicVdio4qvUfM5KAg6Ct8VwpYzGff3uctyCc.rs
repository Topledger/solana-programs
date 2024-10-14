use crate::trade_instruction::TradeInstruction;

const SWAP_DISCRIMINATOR: u64 = 14449647541112719096;
const SWAP_V2_DISCRIMINATOR: u64 = 7070309578724672555;

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    accounts: Vec<String>,
) -> Option<TradeInstruction> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut result = None;

    match discriminator {
        SWAP_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"),
                name: String::from("Swap"),
                amm: accounts.get(2).unwrap().to_string(),
                vault_a: accounts.get(4).unwrap().to_string(),
                vault_b: accounts.get(6).unwrap().to_string(),
            });
        }
        SWAP_V2_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"),
                name: String::from("SwapV2"),
                amm: accounts.get(4).unwrap().to_string(),
                vault_a: accounts.get(8).unwrap().to_string(),
                vault_b: accounts.get(10).unwrap().to_string(),
            });
        }
        _ => {}
    }

    return result;
}
