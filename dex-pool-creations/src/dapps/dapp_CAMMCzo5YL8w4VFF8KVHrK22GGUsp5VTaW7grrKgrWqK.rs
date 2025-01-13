use substreams_solana::pb::sf::solana::r#type::v1::{InnerInstructions, TokenBalance};

use crate::pb::sf::solana::dex::pool::creations::v1::TradeData;

const CreatePool: u64 = u64::from_le_bytes([233, 146, 209, 142, 207, 104, 64, 188]);

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    input_accounts: Vec<String>,
) -> Option<TradeData> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut td = TradeData::default();
    let mut result = None;

    match discriminator {
        CreatePool => {
            td.dapp = "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK".to_string();
            td.pool = input_accounts.get(1).unwrap().to_string();
            td.base_mint = input_accounts.get(3).unwrap().to_string();
            td.quote_mint = input_accounts.get(4).unwrap().to_string();
            result = Some(td);
        }
        _ => {}
    }

    return result;
}
