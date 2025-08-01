use borsh::{BorshDeserialize, BorshSerialize};
use core::str;

use substreams_solana::pb::sf::solana::r#type::v1::{InnerInstructions, TokenBalance};

use crate::{
    pb::sf::solana::liquidity::providers::v1::{TradeData, DecodedLog},
    utils::{get_mint_address_for, get_token_transfer},
};

const AddLiquidityByWeight: u64 = u64::from_le_bytes([28, 140, 238, 99, 231, 162, 21, 149]);
const RemoveLiquidity: u64 = u64::from_le_bytes([80, 85, 209, 72, 24, 206, 177, 108]);
const AddLiquidityOneSide: u64 = u64::from_le_bytes([94, 155, 103, 151, 70, 95, 220, 165]);
const AddLiquidity: u64 = u64::from_le_bytes([181, 157, 89, 67, 143, 182, 52, 72]);
const AddLiquidityByStrategy: u64 = u64::from_le_bytes([7, 3, 150, 127, 148, 40, 61, 200]);
const AddLiquidityByStrategyOneSide: u64 = u64::from_le_bytes([41, 5, 238, 175, 100, 225, 6, 205]);
const RemoveAllLiquidity: u64 = u64::from_le_bytes([10, 51, 61, 35, 112, 105, 24, 85]);
const RemoveLiquidityByRange: u64 = u64::from_le_bytes([26, 82, 102, 152, 240, 74, 105, 26]);
const ClaimFee: u64 = u64::from_le_bytes([169, 32, 79, 137, 136, 232, 70, 137]);
const ClaimReward: u64 = u64::from_le_bytes([149, 95, 181, 242, 94, 90, 158, 162]);
const InitializePosition: u64 = u64::from_le_bytes([219, 192, 234, 71, 190, 191, 102, 80]);
const InitializePositionPda: u64 = u64::from_le_bytes([46, 82, 125, 146, 85, 141, 228, 153]);
const InitializePositionByOperator: u64 =
    u64::from_le_bytes([251, 189, 190, 244, 117, 254, 35, 148]);
const ClosePosition: u64 = u64::from_le_bytes([123, 134, 81, 0, 49, 68, 98, 98]);
const AddLiquidityOneSidePrecise: u64 = u64::from_le_bytes([161, 194, 103, 84, 171, 71, 250, 154]);
const AddLiquidityByStrategy2: u64 = u64::from_le_bytes([3, 221, 149, 218, 111, 141, 118, 213]);
const ClaimFee2: u64 = u64::from_le_bytes([112, 191, 101, 171, 28, 144, 127, 187]);
const ClaimReward2: u64 = u64::from_le_bytes([190, 3, 127, 119, 178, 87, 157, 183]);
const AddLiquidity2: u64 = u64::from_le_bytes([228, 162, 78, 28, 70, 219, 116, 115]);
const AddLiquidityOneSidePrecise2: u64 = u64::from_le_bytes([33, 51, 163, 201, 117, 98, 125, 231]);
const RemoveLiquidity2: u64 = u64::from_le_bytes([230, 215, 82, 127, 241, 101, 227, 146]);
const RemoveLiquidityByRange2: u64 = u64::from_le_bytes([204, 2, 195, 145, 53, 145, 145, 205]);
const ClosePositionIfEmpty: u64 = u64::from_le_bytes([59, 124, 212, 118, 91, 152, 110, 157]);
const ClosePosition2: u64 = u64::from_le_bytes([174, 90, 35, 115, 186, 40, 147, 226]);

// Event detection constants
const EVENT_LOG_DISCRIMINATOR: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29];
const REBALANCING_EVENT_DISCRIMINATOR: [u8; 8] = [0, 109, 117, 179, 61, 91, 199, 200];

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
#[repr(u8)]
enum StrategyTypeLayout {
    #[default]
    Spot,
    Curve,
    BidAsk,
    SpotBalanced,
    CurveBalanced,
    BidAskBalanced,
    SpotImBalanced,
    CurveImBalanced,
    BidAskImBalanced,
}

impl StrategyTypeLayout {
    pub fn to_proto_struct(&self) -> String {
        let mut result = "Spot".to_string();

        match self {
            StrategyTypeLayout::Spot => {
                result = "Spot".to_string();
            }
            StrategyTypeLayout::Curve => {
                result = "Curve".to_string();
            }
            StrategyTypeLayout::BidAsk => {
                result = "BidAsk".to_string();
            }
            StrategyTypeLayout::SpotBalanced => {
                result = "SpotBalanced".to_string();
            }
            StrategyTypeLayout::CurveBalanced => {
                result = "CurveBalanced".to_string();
            }
            StrategyTypeLayout::BidAskBalanced => {
                result = "BidAskBalanced".to_string();
            }
            StrategyTypeLayout::SpotImBalanced => {
                result = "SpotImBalanced".to_string();
            }
            StrategyTypeLayout::CurveImBalanced => {
                result = "CurveImBalanced".to_string();
            }
            StrategyTypeLayout::BidAskImBalanced => {
                result = "BidAskImBalanced".to_string();
            }
        }

        return result;
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct StrategyParametersLayout {
    minBinId: i32,
    maxBinId: i32,
    strategyType: StrategyTypeLayout,
    // parameteres: [u8; 64],
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct AddLiquidityByStrategyLayout {
    amountX: u64,
    amountY: u64,
    activeId: i32,
    maxActiveBinSlippage: i32,
    strategyParameters: StrategyParametersLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct AddLiquidityByStrategyOneSideLayout {
    amount: u64,
    activeId: i32,
    maxActiveBinSlippage: i32,
    strategyParameters: StrategyParametersLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct InitializePositionLayout {
    lowerBinId: i32,
    width: i32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct InitializePositionPdaLayout {
    lowerBinId: i32,
    width: i32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct InitializePositionByOperatorLayout {
    lowerBinId: i32,
    width: i32,
    owner: [u8; 32],
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct RemoveLiquidityByRangeLayout {
    fromBinId: i32,
    toBinId: i32,
    bpsToRemove: u16,
}

#[derive(BorshDeserialize, Debug)]
struct RebalancingEvent {
    lb_pair: [u8; 32],           // pubkey
    position: [u8; 32],          // pubkey  
    owner: [u8; 32],             // pubkey
    active_bin_id: i32,
    x_withdrawn_amount: u64,
    x_added_amount: u64,
    y_withdrawn_amount: u64,
    y_added_amount: u64,
    x_fee_amount: u64,
    y_fee_amount: u64,
    old_min_id: i32,
    old_max_id: i32,
    new_min_id: i32,
    new_max_id: i32,
    rewards: [u64; 2],
}

// Helper functions for event detection and parsing

fn is_anchor_event(data: &[u8]) -> bool {
    data.len() >= 8 && &data[0..8] == EVENT_LOG_DISCRIMINATOR
}

fn get_event_discriminator(data: &[u8]) -> Option<[u8; 8]> {
    if data.len() >= 16 {
        let mut disc = [0u8; 8];
        disc.copy_from_slice(&data[8..16]);
        Some(disc)
    } else {
        None
    }
}

fn try_base58_decode(data_str: &str) -> Option<Vec<u8>> {
    bs58::decode(data_str).into_vec().ok()
}

fn parse_rebalancing_event(
    event_data: &[u8],
    signer: &String,
    accounts: &Vec<String>,
) -> Option<TradeData> {
    if let Ok(rebalancing) = RebalancingEvent::deserialize(&mut event_data.clone()) {
        let mut td = TradeData::default();
        td.instruction_type = "RebalancingLog".to_string();
        td.lp_wallet = signer.to_string();
        
        // Create decoded logs
        let mut fields = std::collections::HashMap::new();
        fields.insert("lb_pair".to_string(), bs58::encode(&rebalancing.lb_pair).into_string());
        fields.insert("position".to_string(), bs58::encode(&rebalancing.position).into_string());
        fields.insert("owner".to_string(), bs58::encode(&rebalancing.owner).into_string());
        fields.insert("active_bin_id".to_string(), rebalancing.active_bin_id.to_string());
        fields.insert("x_withdrawn_amount".to_string(), rebalancing.x_withdrawn_amount.to_string());
        fields.insert("x_added_amount".to_string(), rebalancing.x_added_amount.to_string());
        fields.insert("y_withdrawn_amount".to_string(), rebalancing.y_withdrawn_amount.to_string());
        fields.insert("y_added_amount".to_string(), rebalancing.y_added_amount.to_string());
        fields.insert("x_fee_amount".to_string(), rebalancing.x_fee_amount.to_string());
        fields.insert("y_fee_amount".to_string(), rebalancing.y_fee_amount.to_string());
        fields.insert("old_min_id".to_string(), rebalancing.old_min_id.to_string());
        fields.insert("old_max_id".to_string(), rebalancing.old_max_id.to_string());
        fields.insert("new_min_id".to_string(), rebalancing.new_min_id.to_string());
        fields.insert("new_max_id".to_string(), rebalancing.new_max_id.to_string());
        fields.insert("rewards_0".to_string(), rebalancing.rewards[0].to_string());
        fields.insert("rewards_1".to_string(), rebalancing.rewards[1].to_string());
        
        td.decoded_logs = Some(DecodedLog {
            event_name: "Rebalancing".to_string(),
            fields,
        });
        
        // Set pool from lb_pair
        td.pool = bs58::encode(&rebalancing.lb_pair).into_string();
        td.position = bs58::encode(&rebalancing.position).into_string();
        
        Some(td)
    } else {
        None
    }
}

fn parse_anchor_event(
    event_discriminator: [u8; 8],
    event_data: &[u8],
    signer: &String,
    accounts: &Vec<String>,
) -> Option<TradeData> {
    match event_discriminator {
        REBALANCING_EVENT_DISCRIMINATOR => {
            parse_rebalancing_event(event_data, signer, accounts)
        }
        _ => None
    }
}

// Public utility function to parse base58 encoded event data
pub fn parse_base58_event_data(
    base58_data: &str,
    signer: &String,
    accounts: &Vec<String>,
) -> Option<TradeData> {
    if let Some(decoded_data) = try_base58_decode(base58_data) {
        if is_anchor_event(&decoded_data) {
            if let Some(event_discriminator) = get_event_discriminator(&decoded_data) {
                return parse_anchor_event(
                    event_discriminator,
                    &decoded_data[16..],
                    signer,
                    accounts,
                );
            }
        }
    }
    None
}

pub fn parse_trade_instruction(
    signer: &String,
    bytes_stream: Vec<u8>,
    accounts: &Vec<String>,
    input_accounts: Vec<String>,
    pre_token_balances: &Vec<TokenBalance>,
    post_token_balances: &Vec<TokenBalance>,
    inner_idx: u32,
    inner_instructions: &Vec<InnerInstructions>,
) -> Option<TradeData> {
    // Check if this is an Anchor CPI event first
    if is_anchor_event(&bytes_stream) {
        if let Some(event_discriminator) = get_event_discriminator(&bytes_stream) {
            if let Some(event_result) = parse_anchor_event(
                event_discriminator,
                &bytes_stream[16..],
                signer,
                accounts,
            ) {
                return Some(event_result);
            }
        }
    }

    // If not an event or event parsing failed, continue with regular instruction parsing
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut td = TradeData::default();
    let mut result = None;

    match discriminator {
        AddLiquidityByWeight => {
            td.instruction_type = "AddLiquidityByWeight".to_string();
            td.pool = input_accounts.get(1).unwrap().to_string();
            td.account_a = input_accounts.get(5).unwrap().to_string();
            td.account_b = input_accounts.get(6).unwrap().to_string();

            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.position = input_accounts.get(0).unwrap().to_string();

            result = Some(td);
        }
        RemoveLiquidity => {
            td.instruction_type = "RemoveLiquidity".to_string();
            td.pool = input_accounts.get(1).unwrap().to_string();
            td.account_a = input_accounts.get(5).unwrap().to_string();
            td.account_b = input_accounts.get(6).unwrap().to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.position = input_accounts.get(0).unwrap().to_string();

            result = Some(td);
        }
        AddLiquidityOneSide => {
            td.instruction_type = "AddLiquidityOneSide".to_string();
            td.pool = input_accounts.get(1).unwrap().to_string();
            td.account_a = input_accounts.get(4).unwrap().to_string();
            td.account_b = "".to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.position = input_accounts.get(0).unwrap().to_string();

            result = Some(td);
        }
        AddLiquidity => {
            td.instruction_type = "AddLiquidity".to_string();
            td.pool = input_accounts.get(1).unwrap().to_string();
            td.account_a = input_accounts.get(5).unwrap().to_string();
            td.account_b = input_accounts.get(6).unwrap().to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.position = input_accounts.get(0).unwrap().to_string();

            result = Some(td);
        }
        AddLiquidityByStrategy => {
            td.instruction_type = "AddLiquidityByStrategy".to_string();
            td.pool = input_accounts.get(1).unwrap().to_string();
            td.account_a = input_accounts.get(5).unwrap().to_string();
            td.account_b = input_accounts.get(6).unwrap().to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.position = input_accounts.get(0).unwrap().to_string();

            let data: AddLiquidityByStrategyLayout =
                AddLiquidityByStrategyLayout::deserialize(&mut rest.clone()).unwrap();
            td.strategy_type = data.strategyParameters.strategyType.to_proto_struct();
            td.tick_lower_index = data.strategyParameters.minBinId;
            td.tick_upper_index = data.strategyParameters.maxBinId;

            result = Some(td);
        }
        AddLiquidityByStrategyOneSide => {
            td.instruction_type = "AddLiquidityByStrategyOneSide".to_string();
            td.pool = input_accounts.get(1).unwrap().to_string();
            td.account_a = input_accounts.get(4).unwrap().to_string();
            td.account_b = "".to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.position = input_accounts.get(0).unwrap().to_string();

            let data: AddLiquidityByStrategyOneSideLayout =
                AddLiquidityByStrategyOneSideLayout::deserialize(&mut rest.clone()).unwrap();
            td.strategy_type = data.strategyParameters.strategyType.to_proto_struct();
            td.tick_lower_index = data.strategyParameters.minBinId;
            td.tick_upper_index = data.strategyParameters.maxBinId;

            result = Some(td);
        }
        RemoveAllLiquidity => {
            td.instruction_type = "RemoveAllLiquidity".to_string();
            td.pool = input_accounts.get(1).unwrap().to_string();
            td.account_a = input_accounts.get(5).unwrap().to_string();
            td.account_b = input_accounts.get(6).unwrap().to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.position = input_accounts.get(0).unwrap().to_string();

            result = Some(td);
        }
        RemoveLiquidityByRange => {
            td.instruction_type = "RemoveLiquidityByRange".to_string();
            td.pool = input_accounts.get(1).unwrap().to_string();
            td.account_a = input_accounts.get(5).unwrap().to_string();
            td.account_b = input_accounts.get(6).unwrap().to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.position = input_accounts.get(0).unwrap().to_string();
            let data: RemoveLiquidityByRangeLayout =
                RemoveLiquidityByRangeLayout::deserialize(&mut rest.clone()).unwrap();
            td.tick_lower_index = data.fromBinId;
            td.tick_upper_index = data.toBinId;
            td.bps_to_remove = Some(data.bpsToRemove as u32);

            result = Some(td);
        }
        ClaimFee => {
            td.instruction_type = "ClaimFee".to_string();
            td.pool = input_accounts.get(0).unwrap().to_string();
            td.account_a = input_accounts.get(5).unwrap().to_string();
            td.account_b = input_accounts.get(6).unwrap().to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.position = input_accounts.get(1).unwrap().to_string();

            result = Some(td);
        }
        ClaimReward => {
            td.instruction_type = "ClaimReward".to_string();
            td.pool = input_accounts.get(0).unwrap().to_string();
            td.account_a = input_accounts.get(5).unwrap().to_string();
            td.account_b = "".to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.position = input_accounts.get(1).unwrap().to_string();

            result = Some(td);
        }
        InitializePosition => {
            td.instruction_type = "InitializePosition".to_string();
            td.pool = input_accounts.get(2).unwrap().to_string();
            td.lp_wallet = signer.to_string();
            td.position = input_accounts.get(1).unwrap().to_string();

            let data = InitializePositionLayout::deserialize(&mut rest.clone()).unwrap();
            td.tick_lower_index = data.lowerBinId;
            td.tick_upper_index = data.lowerBinId + data.width;

            result = Some(td);
        }
        InitializePositionPda => {
            td.instruction_type = "InitializePositionPda".to_string();
            td.pool = input_accounts.get(3).unwrap().to_string();
            td.lp_wallet = signer.to_string();
            td.position = input_accounts.get(2).unwrap().to_string();

            let data: InitializePositionPdaLayout =
                InitializePositionPdaLayout::deserialize(&mut rest.clone()).unwrap();
            td.tick_lower_index = data.lowerBinId;
            td.tick_upper_index = data.lowerBinId + data.width;

            result = Some(td);
        }
        InitializePositionByOperator => {
            td.instruction_type = "InitializePositionByOperator".to_string();
            td.pool = input_accounts.get(3).unwrap().to_string();
            td.lp_wallet = signer.to_string();
            td.position = input_accounts.get(2).unwrap().to_string();

            let data: InitializePositionByOperatorLayout =
                InitializePositionByOperatorLayout::deserialize(&mut rest.clone()).unwrap();
            td.tick_lower_index = data.lowerBinId;
            td.tick_upper_index = data.lowerBinId + data.width;

            result = Some(td);
        }
        ClosePosition => {
            td.instruction_type = "ClosePosition".to_string();
            td.pool = input_accounts.get(1).unwrap().to_string();
            td.lp_wallet = signer.to_string();
            td.position = input_accounts.get(0).unwrap().to_string();

            result = Some(td);
        }

        ClosePosition2 => {
            td.instruction_type = "ClosePosition2".to_string();
            td.lp_wallet = signer.to_string();
            td.position = input_accounts.get(0).unwrap().to_string();

            result = Some(td);
        }

        AddLiquidityOneSidePrecise => {
            td.instruction_type = "AddLiquidityOneSidePrecise".to_string();
            td.pool = input_accounts.get(1).unwrap().to_string();
            td.account_a = input_accounts.get(4).unwrap().to_string();
            td.account_b = "".to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.position = input_accounts.get(0).unwrap().to_string();

            result = Some(td);
        }
        AddLiquidityOneSidePrecise2 => {
            td.instruction_type = "AddLiquidityOneSidePrecise2".to_string();
            td.pool = input_accounts.get(1).unwrap().to_string();
            td.account_a = input_accounts.get(4).unwrap().to_string();
            td.account_b = "".to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.position = input_accounts.get(0).unwrap().to_string();

            result = Some(td);
        }
        AddLiquidityByStrategy2 => {
            td.instruction_type = "AddLiquidityByStrategy2".to_string();
            td.pool = input_accounts.get(1).unwrap().to_string();
            td.account_a = input_accounts.get(5).unwrap().to_string();
            td.account_b = input_accounts.get(6).unwrap().to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.position = input_accounts.get(0).unwrap().to_string();
            let data: AddLiquidityByStrategyLayout =
                AddLiquidityByStrategyLayout::deserialize(&mut rest.clone()).unwrap();
            td.strategy_type = data.strategyParameters.strategyType.to_proto_struct();
            td.tick_lower_index = data.strategyParameters.minBinId;
            td.tick_upper_index = data.strategyParameters.maxBinId;

            result = Some(td);
        }
        ClaimFee2 => {
            td.instruction_type = "ClaimFee2".to_string();
            td.pool = input_accounts.get(0).unwrap().to_string();
            td.account_a = input_accounts.get(3).unwrap().to_string();
            td.account_b = input_accounts.get(4).unwrap().to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.position = input_accounts.get(1).unwrap().to_string();

            result = Some(td);
        }
        ClaimReward2 => {
            td.instruction_type = "ClaimReward2".to_string();
            td.pool = input_accounts.get(0).unwrap().to_string();
            td.account_a = input_accounts.get(3).unwrap().to_string();
            td.account_b = "".to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.position = input_accounts.get(1).unwrap().to_string();

            result = Some(td);
        }
        AddLiquidity2 => {
            td.instruction_type = "AddLiquidity2".to_string();
            td.pool = input_accounts.get(1).unwrap().to_string();
            td.account_a = input_accounts.get(5).unwrap().to_string();
            td.account_b = input_accounts.get(6).unwrap().to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.position = input_accounts.get(0).unwrap().to_string();
            result = Some(td);
        }
        AddLiquidityByStrategy2 => {
            td.instruction_type = "AddLiquidityByStrategy2".to_string();
            td.pool = input_accounts.get(1).unwrap().to_string();
            td.account_a = input_accounts.get(5).unwrap().to_string();
            td.account_b = input_accounts.get(6).unwrap().to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "destination".to_string(),
            );
            td.position = input_accounts.get(0).unwrap().to_string();
            let data: AddLiquidityByStrategyLayout =
                AddLiquidityByStrategyLayout::deserialize(&mut rest.clone()).unwrap();
            td.strategy_type = data.strategyParameters.strategyType.to_proto_struct();
            td.tick_lower_index = data.strategyParameters.minBinId;
            td.tick_upper_index = data.strategyParameters.maxBinId;

            result = Some(td);
        }
        RemoveLiquidity2 => {
            td.instruction_type = "RemoveLiquidity2".to_string();
            td.pool = input_accounts.get(1).unwrap().to_string();
            td.account_a = input_accounts.get(5).unwrap().to_string();
            td.account_b = input_accounts.get(6).unwrap().to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.position = input_accounts.get(0).unwrap().to_string();

            result = Some(td);
        }
        RemoveLiquidityByRange2 => {
            td.instruction_type = "RemoveLiquidityByRange2".to_string();

            td.pool = input_accounts.get(1).unwrap().to_string();
            td.account_a = input_accounts.get(5).unwrap().to_string();
            td.account_b = input_accounts.get(6).unwrap().to_string();
            td.lp_wallet = signer.to_string();

            td.mint_a = get_mint_address_for(&td.account_a, post_token_balances, accounts);
            td.mint_b = get_mint_address_for(&td.account_b, post_token_balances, accounts);

            td.token_a_amount = get_token_transfer(
                &td.account_a,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.token_b_amount = get_token_transfer(
                &td.account_b,
                inner_idx,
                inner_instructions,
                accounts,
                "source".to_string(),
            );
            td.position = input_accounts.get(0).unwrap().to_string();
            let data: RemoveLiquidityByRangeLayout =
                RemoveLiquidityByRangeLayout::deserialize(&mut rest.clone()).unwrap();
            td.tick_lower_index = data.fromBinId;
            td.tick_upper_index = data.toBinId;
            td.bps_to_remove = Some(data.bpsToRemove as u32);

            result = Some(td);
        },
        ClosePositionIfEmpty => {
            td.instruction_type = "ClosePositionIfEmpty".to_string();
            td.pool = "".to_string();
            td.lp_wallet = signer.to_string();
            td.position = input_accounts.get(0).unwrap().to_string();

            result = Some(td);
        }

        _ => {}
    }

    return result;
}
