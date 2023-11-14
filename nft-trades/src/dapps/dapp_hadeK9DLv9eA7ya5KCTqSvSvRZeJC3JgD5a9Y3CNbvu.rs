use crate::pb::sf::solana::nft::trades::v1::TradeData;

const BUY_NFT_FROM_PAIR_DISCRIMINATOR: u64 = 13882606562806114661;
const SELL_NFT_TO_LIQUIDITY_PAIR_DISCRIMINATOR: u64 = 1784504931237825610;
const SELL_NFT_TO_TOKEN_TO_NFT_PAIR_DISCRIMINATOR: u64 = 13658316238801071011;

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    input_accounts: Vec<String>,
    accounts: &Vec<String>,
    pre_balances: &Vec<u64>,
    post_balances: &Vec<u64>,
) -> Option<TradeData> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut result = None;
    let mut trade_data: TradeData;

    match discriminator {
        BUY_NFT_FROM_PAIR_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "BuyNftFromPair".to_string();
            trade_data.platform = "hadeswap".to_string();
            trade_data.category = "buy".to_string();
            trade_data.currency = "SOL".to_string();
            trade_data.taker_fee = 0.0;
            trade_data.amm_fee = 0.0;
            trade_data.royalty = 0.0;

            trade_data.mint = input_accounts.get(6).unwrap().to_string();
            trade_data.buyer = input_accounts.get(2).unwrap().to_string();
            trade_data.seller = input_accounts.get(4).unwrap().to_string();

            trade_data.maker_fee = -1.0
                * get_sol_balance_change(
                    &input_accounts.get(5).unwrap().to_string(),
                    accounts,
                    pre_balances,
                    post_balances,
                );
            trade_data.amount = get_sol_balance_change(
                &input_accounts.get(2).unwrap().to_string(),
                accounts,
                pre_balances,
                post_balances,
            );

            result = Some(trade_data);
        }
        SELL_NFT_TO_LIQUIDITY_PAIR_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "SellNftToLiquidityPair".to_string();
            trade_data.platform = "hadeswap".to_string();
            trade_data.category = "sell".to_string();
            trade_data.currency = "SOL".to_string();
            trade_data.taker_fee = 0.0;
            trade_data.amm_fee = 0.0;
            trade_data.royalty = 0.0;

            trade_data.mint = input_accounts.get(4).unwrap().to_string();
            trade_data.buyer = input_accounts.get(5).unwrap().to_string();
            trade_data.seller = input_accounts.get(3).unwrap().to_string();

            trade_data.maker_fee = -1.0
                * get_sol_balance_change(
                    &input_accounts.get(6).unwrap().to_string(),
                    accounts,
                    pre_balances,
                    post_balances,
                );
            trade_data.amount = get_sol_balance_change(
                &input_accounts.get(9).unwrap().to_string(),
                accounts,
                pre_balances,
                post_balances,
            );

            result = Some(trade_data);
        }
        SELL_NFT_TO_TOKEN_TO_NFT_PAIR_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "SellNftToTokenToNftPair".to_string();
            trade_data.platform = "hadeswap".to_string();
            trade_data.category = "sell".to_string();
            trade_data.currency = "SOL".to_string();
            trade_data.taker_fee = 0.0;
            trade_data.amm_fee = 0.0;
            trade_data.royalty = 0.0;

            trade_data.mint = input_accounts.get(3).unwrap().to_string();
            trade_data.buyer = input_accounts.get(6).unwrap().to_string();
            trade_data.seller = input_accounts.get(2).unwrap().to_string();

            trade_data.maker_fee = -1.0
                * get_sol_balance_change(
                    &input_accounts.get(8).unwrap().to_string(),
                    accounts,
                    pre_balances,
                    post_balances,
                );
            trade_data.amount = get_sol_balance_change(
                &input_accounts.get(6).unwrap().to_string(),
                accounts,
                pre_balances,
                post_balances,
            );

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
    return pre_balance as f64 - post_balance as f64;
}
