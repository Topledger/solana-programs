use crate::trade_instruction::TradeInstruction;

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    accounts: Vec<String>,
) -> Option<TradeInstruction> {
    let (disc_bytes, rest) = bytes_stream.split_at(2);
    let disc_bytes_arr: [u8; 2] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u16 = u16::from_be_bytes(disc_bytes_arr);

    let mut result = None;

    match discriminator {
        5 => {
            result = Some(TradeInstruction {
                dapp_address: String::from("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin"),
                name: String::from("SettleFunds"),
                amm: accounts.get(0).unwrap().to_string(),
                vault_a: accounts.get(3).unwrap().to_string(),
                vault_b: accounts.get(4).unwrap().to_string(),
            });
        }
        _ => {}
    }

    return result;
}
