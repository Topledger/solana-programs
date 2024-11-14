use crate::trade_instruction::TradeInstruction;

const SWAP_EXACT_AMOUNT_IN_DISCRIMINATOR: u64 =
    u64::from_le_bytes([8, 151, 245, 76, 172, 203, 144, 39]);

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    accounts: Vec<String>,
) -> Option<TradeInstruction> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut result = None;

    match discriminator {
        SWAP_EXACT_AMOUNT_IN_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("DEXYosS6oEGvk8uCDayvwEZz4qEyDJRf9nFgYCaqPMTm"),
                name: String::from("SwapExactAmountIn"),
                amm: accounts.get(1).unwrap().to_string(),
                vault_a: accounts.get(3).unwrap().to_string(),
                vault_b: accounts.get(4).unwrap().to_string(),
                ..Default::default()
            });
        }
        _ => {}
    }

    return result;
}
