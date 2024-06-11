use base64::{engine::general_purpose, Engine as _};
use borsh::{BorshDeserialize, BorshSerialize};

use crate::pb::sf::solana::nft::trades::v1::TradeData;

const BUY_SELL_EVENT_DISCRIMINATOR: u64 = 12975750536203128930;

const BUY_SINGLE_LISTING_DISCRIMINATOR: u64 = 10182184063413640437;
const SELL_NFT_TOKEN_POOL_DISCRIMINATOR: u64 = 3488891489721789497;
const BUY_NFT_DISCRIMINATOR: u64 = 16020266160874061920;
const SELL_NFT_TRADE_POOL_DISCRIMINATOR: u64 = 6495489243035292291;
const BUY_SINGLE_LISTING_T22_DISCRIMINATOR: u64 = 15588158998506002790;
const SELL_NFT_TOKEN_POOL_T22_DISCRIMINATOR: u64 = 3577586649810332309;
const SELL_NFT_TRADE_POOL_T22_DISCRIMINATOR: u64 = 672568274287300988;
const BUY_NFT_T22_DISCRIMINATOR: u64 = 5707124689885649819;
const WNS_BUY_SINGLE_LISTING_DISCRIMINATOR: u64 = 14373793278627941916;
const WNS_BUY_NFT_DISCRIMINATOR: u64 = 5620760298461527512;
const WNS_SELL_NFT_TOKEN_POOL_DISCRIMINATOR: u64 = 10317446357617561128;
const WNS_SELL_NFT_TRADE_POOL_DISCRIMINATOR: u64 = 541012503203524521;

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct Event {
    current_price: u64,
    tswap_fee: u64,
    mm_fee: u64,
    creators_fee: u64,
}

pub fn parse_logs(log_messages: &Vec<String>) -> Option<Event> {
    let mut result: Option<Event> = None;

    for log_message in log_messages {
        if log_message.starts_with("Program data: ") {
            let b64_str = log_message.replace("Program data: ", "").trim().to_string();
            let bytes_stream = general_purpose::STANDARD.decode(b64_str);
            if !bytes_stream.is_err() {
                let bytes_stream_unwraped = bytes_stream.unwrap();
                let (event_discriminator, rest) = bytes_stream_unwraped.split_at(8);

                let disc_bytes_arr: [u8; 8] = event_discriminator.to_vec().try_into().unwrap();
                let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

                match discriminator {
                    BUY_SELL_EVENT_DISCRIMINATOR => {
                        result = Some(Event::try_from_slice(rest).unwrap());
                    }
                    _ => {}
                }
            }
        }
    }

    return result;
}

pub fn enrich_with_events_data(trade_data: &mut TradeData, log_messages: &Vec<String>) -> () {
    let event_data = parse_logs(log_messages);
    if event_data.is_some() {
        let event_data_unwraped = event_data.unwrap();
        trade_data.amount = event_data_unwraped.current_price as f64;
        trade_data.taker_fee = event_data_unwraped.tswap_fee as f64;
        trade_data.maker_fee = -0.004 * event_data_unwraped.current_price as f64;
        trade_data.amm_fee = event_data_unwraped.mm_fee as f64;
        trade_data.royalty = event_data_unwraped.creators_fee as f64;
    }
}

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    input_accounts: Vec<String>,
    log_messages: &Vec<String>,
) -> Option<TradeData> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut result = None;
    let mut trade_data: TradeData;

    match discriminator {
        SELL_NFT_TOKEN_POOL_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "SellNftTokenPool".to_string();
            trade_data.platform = "tensorswap".to_string();
            trade_data.category = "sell".to_string();
            trade_data.currency_mint = "So11111111111111111111111111111111111111112".to_string();

            trade_data.mint = input_accounts.get(6).unwrap().to_string();
            trade_data.buyer = input_accounts.get(9).unwrap().to_string();
            trade_data.seller = input_accounts.get(10).unwrap().to_string();

            enrich_with_events_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        BUY_NFT_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "BuyNft".to_string();
            trade_data.platform = "tensorswap".to_string();

            trade_data.category = "buy".to_string();
            trade_data.currency_mint = "So11111111111111111111111111111111111111112".to_string();

            trade_data.mint = input_accounts.get(5).unwrap().to_string();
            trade_data.buyer = input_accounts.get(11).unwrap().to_string();
            trade_data.seller = input_accounts.get(10).unwrap().to_string();

            enrich_with_events_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        BUY_SINGLE_LISTING_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "BuySingleListing".to_string();
            trade_data.platform = "tensorswap".to_string();

            trade_data.category = "buy".to_string();
            trade_data.currency_mint = "So11111111111111111111111111111111111111112".to_string();

            trade_data.mint = input_accounts.get(4).unwrap().to_string();
            trade_data.buyer = input_accounts.get(8).unwrap().to_string();
            trade_data.seller = input_accounts.get(7).unwrap().to_string();

            enrich_with_events_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        SELL_NFT_TRADE_POOL_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "SellNftTradePool".to_string();
            trade_data.platform = "tensorswap".to_string();

            trade_data.category = "sell".to_string();
            trade_data.currency_mint = "So11111111111111111111111111111111111111112".to_string();

            trade_data.mint = input_accounts.get(6).unwrap().to_string();
            trade_data.buyer = input_accounts.get(8).unwrap().to_string();
            trade_data.seller = input_accounts.get(10).unwrap().to_string();

            enrich_with_events_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        BUY_SINGLE_LISTING_T22_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "BuySingleListingT22".to_string();
            trade_data.platform = "tensorswap".to_string();

            trade_data.category = "buy".to_string();
            trade_data.currency_mint = "So11111111111111111111111111111111111111112".to_string();

            trade_data.mint = input_accounts.get(4).unwrap().to_string();
            trade_data.buyer = input_accounts.get(7).unwrap().to_string();
            trade_data.seller = input_accounts.get(6).unwrap().to_string();

            enrich_with_events_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        SELL_NFT_TOKEN_POOL_T22_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "SellNftTokenPoolT22".to_string();
            trade_data.platform = "tensorswap".to_string();

            trade_data.category = "sell".to_string();
            trade_data.currency_mint = "So11111111111111111111111111111111111111112".to_string();

            trade_data.mint = input_accounts.get(6).unwrap().to_string();
            trade_data.buyer = input_accounts.get(8).unwrap().to_string();
            trade_data.seller = input_accounts.get(9).unwrap().to_string();

            enrich_with_events_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        SELL_NFT_TRADE_POOL_T22_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "SellNftTradePoolT22".to_string();
            trade_data.platform = "tensorswap".to_string();

            trade_data.category = "sell".to_string();
            trade_data.currency_mint = "So11111111111111111111111111111111111111112".to_string();

            trade_data.mint = input_accounts.get(6).unwrap().to_string();
            trade_data.buyer = input_accounts.get(8).unwrap().to_string();
            trade_data.seller = input_accounts.get(9).unwrap().to_string();

            enrich_with_events_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        BUY_NFT_T22_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "BuyNftT22".to_string();
            trade_data.platform = "tensorswap".to_string();

            trade_data.category = "buy".to_string();
            trade_data.currency_mint = "So11111111111111111111111111111111111111112".to_string();

            trade_data.mint = input_accounts.get(5).unwrap().to_string();
            trade_data.buyer = input_accounts.get(10).unwrap().to_string();
            trade_data.seller = input_accounts.get(9).unwrap().to_string();

            enrich_with_events_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        WNS_BUY_SINGLE_LISTING_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "WnsBuySingleListing".to_string();
            trade_data.platform = "tensorswap".to_string();

            trade_data.category = "buy".to_string();
            trade_data.currency_mint = "So11111111111111111111111111111111111111112".to_string();

            trade_data.mint = input_accounts.get(4).unwrap().to_string();
            trade_data.buyer = input_accounts.get(7).unwrap().to_string();
            trade_data.seller = input_accounts.get(6).unwrap().to_string();

            enrich_with_events_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        WNS_BUY_NFT_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "WnsBuyNft".to_string();
            trade_data.platform = "tensorswap".to_string();

            trade_data.category = "buy".to_string();
            trade_data.currency_mint = "So11111111111111111111111111111111111111112".to_string();

            trade_data.mint = input_accounts.get(5).unwrap().to_string();
            trade_data.buyer = input_accounts.get(10).unwrap().to_string();
            trade_data.seller = input_accounts.get(9).unwrap().to_string();

            enrich_with_events_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        WNS_SELL_NFT_TOKEN_POOL_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "WnsSellNftTokenPool".to_string();
            trade_data.platform = "tensorswap".to_string();

            trade_data.category = "sell".to_string();
            trade_data.currency_mint = "So11111111111111111111111111111111111111112".to_string();

            trade_data.mint = input_accounts.get(6).unwrap().to_string();
            trade_data.buyer = input_accounts.get(8).unwrap().to_string();
            trade_data.seller = input_accounts.get(9).unwrap().to_string();

            enrich_with_events_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        WNS_SELL_NFT_TRADE_POOL_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "WnsSellNftTradePool".to_string();
            trade_data.platform = "tensorswap".to_string();

            trade_data.category = "sell".to_string();
            trade_data.currency_mint = "So11111111111111111111111111111111111111112".to_string();

            trade_data.mint = input_accounts.get(6).unwrap().to_string();
            trade_data.buyer = input_accounts.get(8).unwrap().to_string();
            trade_data.seller = input_accounts.get(9).unwrap().to_string();

            enrich_with_events_data(&mut trade_data, log_messages);

            result = Some(trade_data);
        }
        _ => {}
    }

    return result;
}
