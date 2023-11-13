use serde_json::Value;
use substreams_solana::pb::sf::solana::r#type::v1::InnerInstructions;

use crate::{pb::sf::solana::cnft::trades::v1::TradeData, utils::prepare_input_accounts};

const BUY_NOW_DISCRIMINATOR: u64 = 14733130929291143922;

pub fn parse_logs(log_messages: &Vec<String>) -> Option<(f64, f64, f64)> {
    let mut result: Option<(f64, f64, f64)> = None;

    for log_message in log_messages {
        if log_message.starts_with("Program log: ")
            & log_message.contains("price")
            & log_message.contains("maker_fee")
        {
            let json_str = log_message.replace("Program log: ", "").trim().to_string();
            let mut json_obj: Value = serde_json::from_str(&json_str).unwrap();

            let price_value = json_obj.get_mut("price").unwrap();
            let price_numeric: f64 = price_value.to_string().parse().unwrap();

            let taker_fee_value = json_obj.get_mut("taker_fee").unwrap();
            let taker_fee_numeric = taker_fee_value.to_string().parse().unwrap();

            let maker_fee_value = json_obj.get_mut("maker_fee").unwrap();
            let maker_fee_numeric = maker_fee_value.to_string().parse().unwrap();
            result = Some((price_numeric, taker_fee_numeric, maker_fee_numeric));
        }
    }

    return result;
}

pub fn enrich_with_logs_data(trade_data: &mut TradeData, log_messages: &Vec<String>) -> () {
    let log_data = parse_logs(log_messages);
    if log_data.is_some() {
        let log_data_unwraped = log_data.unwrap();
        trade_data.amount = log_data_unwraped.0;
        trade_data.taker_fee = log_data_unwraped.1;
        trade_data.maker_fee = log_data_unwraped.2;
    }
}

pub fn enrich_with_inner_instructions_data(
    trade_data: &mut TradeData,
    accounts: &Vec<String>,
    inner_instructions: &Vec<InnerInstructions>,
) -> () {
    inner_instructions.iter().for_each(|inner_instruction| {
        inner_instruction
            .instructions
            .iter()
            .enumerate()
            .for_each(|(inner_idx, inner_inst)| {
                let inner_program = &accounts[inner_inst.program_id_index as usize];
                if inner_program
                    .as_str()
                    .eq("BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY")
                    & inner_inst
                        .data
                        .clone()
                        .starts_with(&[163, 52, 200, 231, 140, 3, 69, 186])
                {
                    let input_accounts = prepare_input_accounts(&inner_inst.accounts, accounts);
                    trade_data.leaf_id = input_accounts.get(3).unwrap().to_string();
                    trade_data.merkle_tree = input_accounts.get(4).unwrap().to_string();
                }
            })
    });
}

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    input_accounts: Vec<String>,
    accounts: &Vec<String>,
    log_messages: &Vec<String>,
    inner_instructions: &Vec<InnerInstructions>,
) -> Option<TradeData> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut result = None;
    let mut trade_data: TradeData;

    match discriminator {
        BUY_NOW_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "BuyNow".to_string();
            trade_data.platform = "magiceden".to_string();
            trade_data.currency = "SOL".to_string();

            trade_data.signer = input_accounts.get(0).unwrap().to_string();
            trade_data.buyer = input_accounts.get(0).unwrap().to_string();
            trade_data.seller = input_accounts.get(11).unwrap().to_string();

            trade_data.category = "buy".to_string();
            trade_data.amm_fee = 0.0;
            trade_data.royalty = 0.0;

            enrich_with_logs_data(&mut trade_data, log_messages);
            enrich_with_inner_instructions_data(&mut trade_data, accounts, inner_instructions);

            result = Some(trade_data);
        }
        _ => {}
    }

    return result;
}
