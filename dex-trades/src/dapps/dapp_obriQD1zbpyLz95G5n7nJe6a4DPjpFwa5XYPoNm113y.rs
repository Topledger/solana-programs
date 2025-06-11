use crate::trade_instruction::TradeInstruction;

const SWAP_DISCRIMINATOR: u64 = u64::from_le_bytes([248, 198, 158, 145, 225, 117, 135, 200]);
const SWAP2_DISCRIMINATOR: u64 = u64::from_le_bytes([65, 75, 63, 76, 235, 91, 91, 136]);

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
                dapp_address: String::from("obriQD1zbpyLz95G5n7nJe6a4DPjpFwa5XYPoNm113y"),
                name: String::from("Swap"),
                amm: accounts.get(0).unwrap().to_string(),
                vault_a: accounts.get(3).unwrap().to_string(),
                vault_b: accounts.get(4).unwrap().to_string(),
                ..Default::default()
            });
        }
        SWAP2_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("obriQD1zbpyLz95G5n7nJe6a4DPjpFwa5XYPoNm113y"),
                name: String::from("Swap2"),
                amm: accounts.get(0).unwrap().to_string(),
                vault_a: accounts.get(3).unwrap().to_string(),
                vault_b: accounts.get(4).unwrap().to_string(),
                ..Default::default()
            });
        }
        _ => {}
    }

    return result;
}
