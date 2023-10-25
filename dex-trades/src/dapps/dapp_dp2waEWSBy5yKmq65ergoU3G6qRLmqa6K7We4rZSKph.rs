use crate::trade_instruction::TradeInstruction;

const CREATE_ORDER_DISCRIMINATOR: u64 = u64::from_le_bytes([141, 54, 37, 207, 237, 210, 250, 215]);

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    accounts: Vec<String>,
) -> Option<TradeInstruction> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut result = None;

    match discriminator {
        CREATE_ORDER_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("dp2waEWSBy5yKmq65ergoU3G6qRLmqa6K7We4rZSKph"),
                name: String::from("CreateOrder"),
                amm: accounts.get(0).unwrap().to_string(),
                vault_a: accounts.get(7).unwrap().to_string(),
                vault_b: accounts.get(8).unwrap().to_string(),
            });
        }
        _ => {}
    }

    return result;
}
