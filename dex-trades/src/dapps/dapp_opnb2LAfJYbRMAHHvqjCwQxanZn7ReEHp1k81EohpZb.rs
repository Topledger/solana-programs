use crate::trade_instruction::TradeInstruction;

const PLACE_TAKE_ORDER_DISCRIMINATOR: u64 = u64::from_le_bytes([3, 44, 71, 3, 26, 199, 203, 85]);

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    accounts: Vec<String>,
) -> Option<TradeInstruction> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut result = None;

    match discriminator {
        PLACE_TAKE_ORDER_DISCRIMINATOR => {
            result = Some(TradeInstruction {
                dapp_address: String::from("opnb2LAfJYbRMAHHvqjCwQxanZn7ReEHp1k81EohpZb"),
                name: String::from("PlaceTakeOrder"),
                amm: accounts.get(2).unwrap().to_string(),
                vault_a: accounts.get(6).unwrap().to_string(),
                vault_b: accounts.get(7).unwrap().to_string(),
            });
        }
        _ => {}
    }

    return result;
}
