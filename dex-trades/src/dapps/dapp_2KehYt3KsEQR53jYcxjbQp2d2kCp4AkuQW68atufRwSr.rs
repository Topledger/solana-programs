use crate::trade_instruction::TradeInstruction;

const SWAP_FUND_TOKENS_DISCRIMINATOR: u64 =
    u64::from_le_bytes([112, 246, 21, 136, 172, 62, 27, 20]);

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    accounts: Vec<String>,
) -> Option<TradeInstruction> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut result = None;

    match discriminator {
        SWAP_FUND_TOKENS_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("2KehYt3KsEQR53jYcxjbQp2d2kCp4AkuQW68atufRwSr"),
                name: String::from("SwapFundTokens"),
                amm: accounts.get(1).unwrap().to_string(),
                vault_a: accounts.get(3).unwrap().to_string(),
                vault_b: accounts.get(5).unwrap().to_string(),
            });
        }
        _ => {}
    }

    return result;
}
