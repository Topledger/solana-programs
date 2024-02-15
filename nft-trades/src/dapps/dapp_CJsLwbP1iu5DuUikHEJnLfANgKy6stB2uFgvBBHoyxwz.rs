use std::str::FromStr;

use crate::pb::sf::solana::nft::trades::v1::TradeData;

const BUY_DISCRIMINATOR: u8 = 5;

pub fn parse_logs(log_messages: &Vec<String>, amount: f64, amm_fee: f64) -> Option<(f64, f64)> {
    let mut result: Option<(f64, f64)> = None;

    for log_message in log_messages {
        if log_message.starts_with("Program log: ")
            & (amm_fee > 0.0)
            & log_message.contains(&amm_fee.to_string())
        {
            let val_str = log_message
                .replace("Program log: ", "")
                .replacen(&amm_fee.to_string(), "", 1)
                .replace("  ", " ")
                .trim()
                .to_string();
            let string_items: Vec<&str> = val_str.split(" ").collect();

            let amount: f64 = f64::from_str(string_items.get(0).unwrap()).unwrap();
            let royalty: f64 = f64::from_str(string_items.get(1).unwrap()).unwrap();

            result = Some((amount, royalty));
        }
    }

    return result;
}

pub fn enrich_with_logs_data(trade_data: &mut TradeData, log_messages: &Vec<String>) -> () {
    let log_data = parse_logs(log_messages, trade_data.amount, trade_data.amm_fee);
    if log_data.is_some() {
        let log_data_unwraped = log_data.unwrap();
        trade_data.amount = log_data_unwraped.0;
        trade_data.royalty = log_data_unwraped.1;
    }
}

pub fn is_cancelled_transaction(log_messages: &Vec<String>) -> bool {
    for log_message in log_messages {
        if log_message
            .to_lowercase()
            .contains("sale cancelled by seller")
        {
            return true;
        }
    }
    false
}

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    input_accounts: Vec<String>,
    accounts: &Vec<String>,
    pre_balances: &Vec<u64>,
    post_balances: &Vec<u64>,
    log_messages: &Vec<String>,
) -> Option<TradeData> {
    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let disc_bytes_arr: [u8; 1] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u8 = u8::from_le_bytes(disc_bytes_arr);

    let signer = accounts.get(0).unwrap().to_string();

    let mut result = None;
    let mut trade_data: TradeData;

    if is_cancelled_transaction(log_messages) {
        return None;
    }

    match discriminator {
        BUY_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "Buy".to_string();
            trade_data.platform = "solanart".to_string();

            trade_data.currency_mint = "So11111111111111111111111111111111111111112".to_string();
            trade_data.taker_fee = 0.0;
            trade_data.maker_fee = 0.0;
            trade_data.amm_fee = get_sol_balance_change(
                &"39fEpihLATXPJCQuSiXLUSiCbGchGYjeL39eyXh3KbyT".to_string(),
                accounts,
                pre_balances,
                post_balances,
            );

            trade_data.mint = input_accounts.get(10).unwrap().to_string();
            trade_data.buyer = input_accounts.get(0).unwrap().to_string();
            trade_data.seller = input_accounts.get(3).unwrap().to_string();

            if signer.eq(&trade_data.buyer.to_string()) {
                trade_data.category = "buy".to_string();
            } else {
                trade_data.category = "sell".to_string();
            }

            trade_data.amount =
                get_sol_balance_change(&trade_data.seller, accounts, pre_balances, post_balances);
            enrich_with_logs_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        _ => {}
    }

    return result;
}

fn get_sol_balance_change(
    address: &String,
    accounts: &Vec<String>,
    pre_balances: &Vec<u64>,
    post_balances: &Vec<u64>,
) -> f64 {
    let index = accounts.iter().position(|r| r == address).unwrap();
    let pre_balance = pre_balances[index];
    let post_balance = post_balances[index];
    return post_balance as f64 - pre_balance as f64;
}
