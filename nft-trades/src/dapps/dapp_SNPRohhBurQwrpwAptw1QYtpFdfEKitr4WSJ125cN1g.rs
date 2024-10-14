use std::str::FromStr;

use crate::pb::sf::solana::nft::trades::v1::TradeData;

const EXECUTE_SOL_NFT_ORDER_DISCRIMINATOR: u64 =
    u64::from_le_bytes([169, 124, 36, 17, 155, 122, 9, 90]);

pub fn parse_logs(log_messages: &Vec<String>) -> Option<(f64, f64)> {
    let mut result: Option<(f64, f64)> = None;

    for log_message in log_messages {
        if log_message.starts_with("Program log: ") & log_message.contains("royalties") {
            let val_str = log_message
                .replace("Program log: ", "")
                .replace("fees ", "")
                .trim()
                .to_string();
            let string_items: Vec<&str> = val_str.split(" ").collect();

            let amm_fee: f64 = f64::from_str(string_items.get(0).unwrap()).unwrap();
            let royalty: f64 =
                f64::from_str(string_items.get(string_items.len() - 1).unwrap()).unwrap();

            result = Some((amm_fee, royalty));
        }
    }

    return result;
}

pub fn enrich_with_logs_data(trade_data: &mut TradeData, log_messages: &Vec<String>) -> () {
    let log_data = parse_logs(log_messages);
    if log_data.is_some() {
        let log_data_unwraped = log_data.unwrap();
        trade_data.amm_fee = log_data_unwraped.0;
        trade_data.royalty = log_data_unwraped.1;
    }
}

pub fn get_amount_from_logs(log_messages: &Vec<String>) -> Option<f64> {
    let mut result: Option<f64> = None;
    for log_message in log_messages {
        if log_message.starts_with("Program log: ")
            & log_message.contains("fee_adjusted_maker_price")
        {
            let amount_str = log_message
                .replace("Program log: ", "")
                .replace("fee_adjusted_maker_price: ", "")
                .trim()
                .to_string();
            let string_items: Vec<&str> = amount_str.split(",").collect();
            let amount: f64 = f64::from_str(string_items.get(0).unwrap()).unwrap();
            result = Some(amount);
        }
    }
    result
}

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    input_accounts: Vec<String>,
    accounts: &Vec<String>,
    pre_balances: &Vec<u64>,
    post_balances: &Vec<u64>,
    log_messages: &Vec<String>,
) -> Option<TradeData> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let signer = accounts.get(0).unwrap().to_string();

    let mut result = None;
    let mut trade_data: TradeData;

    match discriminator {
        EXECUTE_SOL_NFT_ORDER_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "ExecuteSolNftOrder".to_string();
            trade_data.platform = "sniper".to_string();

            trade_data.currency_mint = "So11111111111111111111111111111111111111112".to_string();
            trade_data.taker_fee = 0.0;
            trade_data.maker_fee = 0.0;

            trade_data.mint = input_accounts.get(6).unwrap().to_string();
            trade_data.buyer = input_accounts.get(0).unwrap().to_string();
            trade_data.seller = input_accounts.get(5).unwrap().to_string();

            if signer.eq(&trade_data.buyer.to_string()) {
                trade_data.category = "buy".to_string();
            } else {
                trade_data.category = "sell".to_string();
            }
            trade_data.amount = get_amount_from_logs(log_messages).unwrap_or_default();
            enrich_with_logs_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        _ => {}
    }

    return result;
}
