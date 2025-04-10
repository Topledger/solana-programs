use crate::trade_instruction::TradeInstruction;

const SWAP_DISCRIMINATOR: u8 = 7;

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    accounts: Vec<String>,
) -> Option<TradeInstruction> {
    let mut result = None;

    if bytes_stream.len() < 8 {
        return result;
    }

    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let disc_bytes_arr: [u8; 1] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u8 = u8::from_le_bytes(disc_bytes_arr);

    match discriminator {
        SWAP_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe"),
                name: String::from("Swap"),
                amm: accounts.get(1).unwrap().to_string(),
                vault_a: accounts.get(2).unwrap().to_string(),
                vault_b: accounts.get(3).unwrap().to_string(),
                ..Default::default()
            });
        }
        _ => {}
    }

    return result;
}
