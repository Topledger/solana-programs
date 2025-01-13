use substreams_solana::pb::sf::solana::r#type::v1::{InnerInstructions, TokenBalance};

use crate::pb::sf::solana::dex::pool::creations::v1::TradeData;

const TokenMint: u64 = u64::from_le_bytes([3, 44, 164, 184, 123, 13, 245, 179]);

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
        Create => {
            td.dapp = "MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG".to_string();
            td.pool = input_accounts.get(2).unwrap().to_string();
            td.associated_account = input_accounts.get(5).unwrap().to_string();
            td.base_mint = input_accounts.get(3).unwrap().to_string();
            td.quote_mint = "So11111111111111111111111111111111111111112".to_string();
            result = Some(td);
        }
        _ => {}
    }

    return result;
}
