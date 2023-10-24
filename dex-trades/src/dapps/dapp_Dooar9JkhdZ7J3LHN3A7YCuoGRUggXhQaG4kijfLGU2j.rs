use crate::trade_instruction::TradeInstruction;

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    accounts: Vec<String>,
) -> Option<TradeInstruction> {
    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let discriminator: u8 = u8::from(disc_bytes[0]);

    let mut result = None;

    match discriminator {
        0 => {
            result = Some(TradeInstruction {
                dapp_address: String::from("Dooar9JkhdZ7J3LHN3A7YCuoGRUggXhQaG4kijfLGU2j"),
                name: String::from("Swap"),
                amm: accounts.get(0).unwrap().to_string(),
                vault_a: accounts.get(4).unwrap().to_string(),
                vault_b: accounts.get(5).unwrap().to_string(),
            });
        }
        _ => {}
    }

    return result;
}
