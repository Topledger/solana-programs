use crate::trade_instruction::TradeInstruction;

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    accounts: Vec<String>,
) -> Option<TradeInstruction> {
    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let discriminator: u8 = u8::from(disc_bytes[0]);

    let mut result = None;

    match discriminator {
        1 => {
            result = Some(TradeInstruction {
                dapp_address: String::from("CTMAxxk34HjKWxQ3QLZK1HpaLXmBveao3ESePXbiyfzh"),
                name: String::from("Swap"),
                amm: accounts.get(1).unwrap().to_string(),
                vault_a: accounts.get(5).unwrap().to_string(),
                vault_b: accounts.get(6).unwrap().to_string(),
            });
        }
        _ => {}
    }

    return result;
}
