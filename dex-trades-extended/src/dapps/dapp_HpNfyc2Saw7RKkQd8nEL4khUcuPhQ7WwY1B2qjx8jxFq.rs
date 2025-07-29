use crate::trade_instruction::TradeInstruction;

const SWAP_DISCRIMINATOR: u64 = u64::from_le_bytes([248, 198, 158, 145, 225, 117, 135, 200]);
const SWAP_V2_DISCRIMINATOR: u64 = u64::from_le_bytes([43, 4, 237, 11, 26, 201, 30, 98]);

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
                dapp_address: String::from("HpNfyc2Saw7RKkQd8nEL4khUcuPhQ7WwY1B2qjx8jxFq"),
                name: String::from("Swap"),
                amm: accounts.get(2).unwrap().to_string(),
                vault_a: accounts.get(5).unwrap().to_string(),
                vault_b: accounts.get(6).unwrap().to_string(),
                ..Default::default()
            });
        }
        SWAP_V2_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("HpNfyc2Saw7RKkQd8nEL4khUcuPhQ7WwY1B2qjx8jxFq"),
                name: String::from("SwapV2"),
                amm: accounts.get(2).unwrap().to_string(),
                vault_a: accounts.get(5).unwrap().to_string(),
                vault_b: accounts.get(6).unwrap().to_string(),
                ..Default::default()
            });
        }
        _ => {}
    }

    return result;
} 