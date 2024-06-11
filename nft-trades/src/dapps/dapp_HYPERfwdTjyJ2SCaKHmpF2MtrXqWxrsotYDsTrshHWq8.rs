use borsh::{BorshDeserialize, BorshSerialize};
use substreams_solana::pb::sf::solana::r#type::v1::TokenBalance;

use crate::pb::sf::solana::nft::trades::v1::TradeData;

const EXECUTE_SALE_DISCRIMINATOR: u64 = 442251406432881189;

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct ExecuteSaleLayout {
    escrowPaymentBump: u8,
    programAsSignerBump: u8,
    buyerTradeStateBump: u8,
    sellerTradeStateBump: u8,
    buyerPrice: u64,
    buyerBrokerBasisPoints: u16,
    sellerBrokerBasisPoints: u16,
    tokenSize: u64
}

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    input_accounts: Vec<String>,
    accounts: &Vec<String>,
    log_messages: &Vec<String>,
    post_token_balances: &Vec<TokenBalance>,
) -> Option<TradeData> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let signer = accounts.get(0).unwrap().to_string();

    let mut result = None;
    let mut trade_data: TradeData;

    match discriminator {
        EXECUTE_SALE_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "ExecuteSale".to_string();
            trade_data.platform = "hyperspace".to_string();

            trade_data.mint = input_accounts.get(5).unwrap().to_string();
            trade_data.buyer = input_accounts.get(0).unwrap().to_string();
            trade_data.seller = input_accounts.get(2).unwrap().to_string();

            if signer.eq(&trade_data.buyer.to_string()) {
                trade_data.category = "buy".to_string();
            } else {
                trade_data.category = "sell".to_string();
            }

            trade_data.currency_mint = "So11111111111111111111111111111111111111112".to_string();

            let instruction_data: ExecuteSaleLayout;
            instruction_data = ExecuteSaleLayout::deserialize(&mut rest.clone()).unwrap();
            
            trade_data.taker_fee = ((instruction_data.buyerBrokerBasisPoints as u64 * instruction_data.buyerPrice) / 10000) as f64;
            trade_data.maker_fee = ((instruction_data.sellerBrokerBasisPoints as f64 * instruction_data.buyerPrice as f64) / 10000.0) as f64;
            trade_data.amount = instruction_data.buyerPrice as f64;
            trade_data.amm_fee = 0.0;

            result = Some(trade_data);
        }
        _ => {}
    }

    return result;
}
