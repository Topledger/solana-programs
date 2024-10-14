use crate::trade_instruction::TradeInstruction;

const SWAP_DISCRIMINATOR: u64 = u64::from_le_bytes([248, 198, 158, 145, 225, 117, 135, 200]);

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    input_accounts: Vec<String>,
) -> Option<TradeInstruction> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut result = None;

    match discriminator {
        SWAP_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"),
                name: String::from("Swap"),
                amm: input_accounts.get(0).unwrap().to_string(),
                vault_a: input_accounts.get(2).unwrap().to_string(),
                vault_b: input_accounts.get(3).unwrap().to_string(),
            });
        }
        _ => {}
    }

    return result;
}
