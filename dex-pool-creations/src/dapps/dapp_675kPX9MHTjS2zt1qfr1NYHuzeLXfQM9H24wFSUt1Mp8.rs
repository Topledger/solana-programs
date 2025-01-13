use substreams_solana::pb::sf::solana::r#type::v1::{InnerInstructions, TokenBalance};

use crate::pb::sf::solana::dex::pool::creations::v1::TradeData;

const Initialize2: u8 = 1;

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    input_accounts: Vec<String>,
) -> Option<TradeData> {
    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let disc_bytes_arr: [u8; 1] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u8 = u8::from_le_bytes(disc_bytes_arr);

    let mut td = TradeData::default();
    let mut result = None;

    match discriminator {
        Initialize2 => {
            td.dapp = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8".to_string();
            td.pool = input_accounts.get(4).unwrap().to_string();
            td.base_mint = input_accounts.get(8).unwrap().to_string();
            td.quote_mint = input_accounts.get(9).unwrap().to_string();
            result = Some(td);
        }
        _ => {}
    }

    return result;
}
