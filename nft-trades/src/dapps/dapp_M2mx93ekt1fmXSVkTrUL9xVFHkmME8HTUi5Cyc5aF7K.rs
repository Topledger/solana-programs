use std::collections::HashSet;

use borsh::{BorshDeserialize, BorshSerialize};
use serde_json::Value;
use substreams_solana::pb::sf::solana::r#type::v1::TokenBalance;

use crate::pb::sf::solana::nft::trades::v1::TradeData;

const EXECUTE_SALE_V2_DISCRIMINATOR: u64 = 13922176540003654747;
const OCP_EXECUTE_SALE_V2_DISCRIMINATOR: u64 = 6995388316419838920;
const MIP1_EXECUTE_SALE_V2_DISCRIMINATOR: u64 = 8569101353535448044;

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct ExecuteSaleV2Layout {
    tempA: [u8; 2],
    price: u64,
    tempB: [u8; 24],
    makerFeeBp: i16,
    takerFeeBp: u16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct OCPExecuteSaleV2Layout {
    price: u64,
    makerFeeBp: i16,
    takerFeeBp: u16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct MIP1ExecuteSaleV2Layout {
    price: u64,
    makerFeeBp: i16,
    takerFeeBp: u16,
}

pub fn parse_logs(log_messages: &Vec<String>) -> Option<f64> {
    let mut result: Option<f64> = None;

    for log_message in log_messages {
        if log_message.starts_with("Program log: ") & log_message.contains("royalty") {
            let json_str = log_message.replace("Program log: ", "").trim().to_string();
            let mut json_obj: Value = serde_json::from_str(&json_str).unwrap();
            let royalty_value = json_obj.get_mut("royalty").unwrap();
            let royalty_numeric: f64 = royalty_value.to_string().parse().unwrap();
            result = Some(royalty_numeric);
        }
    }

    return result;
}

pub fn enrich_with_logs_data(trade_data: &mut TradeData, log_messages: &Vec<String>) -> () {
    let log_data = parse_logs(log_messages);
    if log_data.is_some() {
        let log_data_unwraped = log_data.unwrap();
        trade_data.royalty = log_data_unwraped;
    }
}

pub fn get_currency_mint(post_token_balances: &Vec<TokenBalance>, nft_mint: &String) -> String {
    let mut mints: Vec<String> = vec![];
    for x in post_token_balances.iter() {
        mints.push(x.mint.to_string());
    }

    let distinct_mints: HashSet<String> = mints.into_iter().collect();
    match distinct_mints.len() {
        1 => {
            return "So11111111111111111111111111111111111111112".to_string();
        }
        2 => {
            let nft_mints: HashSet<String> = vec![nft_mint.to_string()].into_iter().collect();
            let result = distinct_mints
                .difference(&nft_mints)
                .collect::<Vec<&String>>();
            return result.get(0).unwrap().to_string();
        }
        _ => {
            panic!("Found 3 distinct mints");
        }
    }
}

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    input_accounts: Vec<String>,
    log_messages: &Vec<String>,
    post_token_balances: &Vec<TokenBalance>,
) -> Option<TradeData> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut result = None;
    let mut trade_data: TradeData;

    match discriminator {
        EXECUTE_SALE_V2_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "ExecuteSaleV2".to_string();
            trade_data.platform = "magiceden".to_string();
            trade_data.category = "".to_string();

            trade_data.mint = input_accounts.get(4).unwrap().to_string();
            trade_data.buyer = input_accounts.get(0).unwrap().to_string();
            trade_data.seller = input_accounts.get(1).unwrap().to_string();

            trade_data.currency_mint = get_currency_mint(post_token_balances, &trade_data.mint);

            let instruction_data: ExecuteSaleV2Layout;
            if rest.len() > 38 {
                let (rest_split, _) = rest.split_at(38);
                instruction_data = ExecuteSaleV2Layout::try_from_slice(rest_split).unwrap();
            } else {
                instruction_data = ExecuteSaleV2Layout::try_from_slice(rest).unwrap();
            }

            trade_data.taker_fee =
                ((instruction_data.takerFeeBp as u64 * instruction_data.price) / 10000) as f64;
            trade_data.maker_fee = ((instruction_data.makerFeeBp as f64
                * instruction_data.price as f64)
                / 10000.0) as f64;
            trade_data.amount = instruction_data.price as f64
                + trade_data.taker_fee as f64
                + trade_data.maker_fee as f64;
            trade_data.amm_fee = 0.0;
            enrich_with_logs_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        OCP_EXECUTE_SALE_V2_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "OcpExecuteSaleV2".to_string();
            trade_data.platform = "magiceden".to_string();
            trade_data.category = "".to_string();

            trade_data.mint = input_accounts.get(7).unwrap().to_string();
            trade_data.buyer = input_accounts.get(1).unwrap().to_string();
            trade_data.seller = input_accounts.get(2).unwrap().to_string();

            trade_data.currency_mint = get_currency_mint(post_token_balances, &trade_data.mint);

            let instruction_data = OCPExecuteSaleV2Layout::try_from_slice(rest).unwrap();
            trade_data.taker_fee = ((instruction_data.takerFeeBp as f64
                * instruction_data.price as f64)
                / 10000.0) as f64;
            trade_data.maker_fee = ((instruction_data.makerFeeBp as f64
                * instruction_data.price as f64)
                / 10000.0) as f64;
            trade_data.amount = instruction_data.price as f64
                + trade_data.maker_fee as f64
                + trade_data.taker_fee as f64;
            trade_data.amm_fee = 0.0;

            enrich_with_logs_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        MIP1_EXECUTE_SALE_V2_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "Mip1ExecuteSaleV2".to_string();
            trade_data.platform = "magiceden".to_string();
            trade_data.category = "".to_string();

            trade_data.mint = input_accounts.get(7).unwrap().to_string();
            trade_data.buyer = input_accounts.get(1).unwrap().to_string();
            trade_data.seller = input_accounts.get(2).unwrap().to_string();

            trade_data.currency_mint = get_currency_mint(post_token_balances, &trade_data.mint);

            let instruction_data: MIP1ExecuteSaleV2Layout;
            if rest.len() > 12 {
                let (rest_split, _) = rest.split_at(12);
                instruction_data = MIP1ExecuteSaleV2Layout::try_from_slice(rest_split).unwrap();
            } else {
                instruction_data = MIP1ExecuteSaleV2Layout::try_from_slice(rest).unwrap();
            }

            trade_data.taker_fee = ((instruction_data.takerFeeBp as f64
                * instruction_data.price as f64)
                / 10000.0) as f64;
            trade_data.maker_fee = ((instruction_data.makerFeeBp as f64
                * instruction_data.price as f64)
                / 10000.0) as f64;
            trade_data.amount = instruction_data.price as f64
                + trade_data.taker_fee as f64
                + trade_data.maker_fee as f64;
            trade_data.amm_fee = 0.0;
            enrich_with_logs_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        _ => {}
    }

    return result;
}
