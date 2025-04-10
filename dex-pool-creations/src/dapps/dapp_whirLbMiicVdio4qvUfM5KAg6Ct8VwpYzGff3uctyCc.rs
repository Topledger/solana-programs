use substreams_solana::pb::sf::solana::r#type::v1::{InnerInstructions, TokenBalance};

use crate::pb::sf::solana::dex::pool::creations::v1::TradeData;

const InitializePool: u64 = u64::from_le_bytes([95, 180, 10, 172, 84, 174, 232, 40]);
const InitializePoolV2: u64 = u64::from_le_bytes([207, 45, 87, 242, 27, 63, 204, 67]);

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
        InitializePool => {
            td.dapp = "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc".to_string();
            td.pool = input_accounts.get(4).unwrap().to_string();
            td.base_mint = input_accounts.get(1).unwrap().to_string();
            td.quote_mint = input_accounts.get(2).unwrap().to_string();
            result = Some(td);
        }
        InitializePoolV2 => {
            td.dapp = "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc".to_string();
            td.pool = input_accounts.get(6).unwrap().to_string();
            result = Some(td);
        }
        _ => {}
    }

    return result;
}
