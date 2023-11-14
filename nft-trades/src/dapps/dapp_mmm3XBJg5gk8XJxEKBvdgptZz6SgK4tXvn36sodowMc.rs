use borsh::{BorshDeserialize, BorshSerialize};
use serde_json::Value;

use crate::pb::sf::solana::nft::trades::v1::TradeData;

const SOL_FULFILL_BUY_DISCRIMINATOR: u64 = 8517980486539284572;
const SOL_FULFILL_SELL_DISCRIMINATOR: u64 = 16747164525079344292;
const SOL_MIP1_FULFILL_BUY_DISCRIMINATOR: u64 = 10497635681119916780;
const SOL_MIP1_FULFILL_SELL_DISCRIMINATOR: u64 = 15150224768793905979;
const SOL_OCP_FULFILL_BUY_DISCRIMINATOR: u64 = 2380949227974615409;

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct SolFulfillBuyLayout {
    assetAmount: u64,
    minPaymentAmount: u64,
    allowlistAux: Option<String>,
    makerFeeBp: i16,
    takerFeeBp: i16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct SolFulfillSellLayout {
    assetAmount: u64,
    maxPaymentAmount: u64,
    buysideCreatorRoyaltyBp: u16,
    allowlistAux: Option<String>,
    makerFeeBp: i16,
    takerFeeBp: i16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct SolMip1FulfillBuyLayout {
    assetAmount: u64,
    minPaymentAmount: u64,
    allowlistAux: Option<String>,
    makerFeeBp: i16,
    takerFeeBp: i16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct SolMip1FulfillSellLayout {
    assetAmount: u64,
    maxPaymentAmount: u64,
    allowlistAux: Option<String>,
    makerFeeBp: i16,
    takerFeeBp: i16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct SolOcpFulfillBuyLayout {
    assetAmount: u64,
    minPaymentAmount: u64,
    allowlistAux: Option<String>,
    makerFeeBp: i16,
    takerFeeBp: i16,
}

pub fn parse_logs(log_messages: &Vec<String>) -> Option<(f64, f64, f64)> {
    let mut result: Option<(f64, f64, f64)> = None;

    for log_message in log_messages {
        if log_message.starts_with("Program log: ") & log_message.contains("royalty") {
            let json_str = log_message.replace("Program log: ", "").trim().to_string();
            let mut json_obj: Value = serde_json::from_str(&json_str).unwrap();

            let amm_fee_value = json_obj.get_mut("lp_fee").unwrap();
            let amm_fee_numeric: f64 = amm_fee_value.to_string().parse().unwrap();

            let royalty_value = json_obj.get_mut("royalty_paid").unwrap();
            let royalty_numeric: f64 = royalty_value.to_string().parse().unwrap();

            let amount_value = json_obj.get_mut("total_price").unwrap();
            let amount_numeric: f64 = amount_value.to_string().parse().unwrap();
            result = Some((amm_fee_numeric, royalty_numeric, amount_numeric));
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
        trade_data.amount = log_data_unwraped.2;
    }
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

    let mut result = None;
    let mut trade_data: TradeData;

    match discriminator {
        SOL_FULFILL_BUY_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "SolFulfillBuy".to_string();
            trade_data.platform = "coralcube_me_amm".to_string();
            trade_data.category = "buy".to_string();
            trade_data.currency = "SOL".to_string();

            let buyer = input_accounts.get(1).unwrap();
            trade_data.mint = input_accounts.get(8).unwrap().to_string();
            trade_data.buyer = buyer.to_string();
            trade_data.seller = input_accounts.get(0).unwrap().to_string();

            let instruction_data = SolFulfillBuyLayout::try_from_slice(rest).unwrap();
            trade_data.taker_fee = instruction_data.takerFeeBp as f64 * trade_data.amount / 10000.0;
            trade_data.maker_fee = instruction_data.makerFeeBp as f64 * trade_data.amount / 10000.0;

            enrich_with_logs_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        SOL_FULFILL_SELL_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "SolFulfillSell".to_string();
            trade_data.platform = "coralcube_me_amm".to_string();
            trade_data.category = "sell".to_string();
            trade_data.currency = "SOL".to_string();

            let buyer = input_accounts.get(0).unwrap();
            trade_data.mint = input_accounts.get(8).unwrap().to_string();
            trade_data.buyer = buyer.to_string();
            trade_data.seller = input_accounts.get(4).unwrap().to_string();

            let instruction_data = SolFulfillSellLayout::try_from_slice(rest).unwrap();
            trade_data.taker_fee = instruction_data.takerFeeBp as f64 * trade_data.amount / 10000.0;
            trade_data.maker_fee = instruction_data.makerFeeBp as f64 * trade_data.amount / 10000.0;

            enrich_with_logs_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        SOL_MIP1_FULFILL_BUY_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "SolMip1FulfillBuy".to_string();
            trade_data.platform = "coralcube_me_amm".to_string();
            trade_data.category = "buy".to_string();
            trade_data.currency = "SOL".to_string();

            let buyer = input_accounts.get(1).unwrap();
            trade_data.mint = input_accounts.get(7).unwrap().to_string();
            trade_data.buyer = buyer.to_string();
            trade_data.seller = input_accounts.get(0).unwrap().to_string();

            let instruction_data = SolMip1FulfillBuyLayout::try_from_slice(rest).unwrap();
            trade_data.taker_fee = instruction_data.takerFeeBp as f64 * trade_data.amount / 10000.0;
            trade_data.maker_fee = instruction_data.makerFeeBp as f64 * trade_data.amount / 10000.0;

            enrich_with_logs_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        SOL_MIP1_FULFILL_SELL_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "SolMip1FulfillSell".to_string();
            trade_data.platform = "coralcube_me_amm".to_string();
            trade_data.category = "sell".to_string();
            trade_data.currency = "SOL".to_string();

            let buyer = input_accounts.get(0).unwrap();
            trade_data.mint = input_accounts.get(7).unwrap().to_string();
            trade_data.buyer = buyer.to_string();
            trade_data.seller = input_accounts.get(4).unwrap().to_string();

            let instruction_data = SolMip1FulfillSellLayout::try_from_slice(rest).unwrap();
            trade_data.taker_fee = instruction_data.takerFeeBp as f64 * trade_data.amount / 10000.0;
            trade_data.maker_fee = instruction_data.makerFeeBp as f64 * trade_data.amount / 10000.0;

            enrich_with_logs_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        SOL_OCP_FULFILL_BUY_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "SolOcpFulfillBuy".to_string();
            trade_data.platform = "coralcube_me_amm".to_string();
            trade_data.category = "buy".to_string();
            trade_data.currency = "SOL".to_string();

            let buyer = input_accounts.get(1).unwrap();
            trade_data.mint = input_accounts.get(7).unwrap().to_string();
            trade_data.buyer = buyer.to_string();
            trade_data.seller = input_accounts.get(0).unwrap().to_string();

            let instruction_data = SolOcpFulfillBuyLayout::try_from_slice(rest).unwrap();
            trade_data.taker_fee = instruction_data.takerFeeBp as f64 * trade_data.amount / 10000.0;
            trade_data.maker_fee = instruction_data.makerFeeBp as f64 * trade_data.amount / 10000.0;

            enrich_with_logs_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        _ => {}
    }

    return result;
}
