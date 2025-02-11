use borsh::{BorshDeserialize, BorshSerialize};
use core::str;

use substreams_solana::pb::sf::solana::r#type::v1::{InnerInstructions, TokenBalance};

use crate::{
    pb::sf::solana::liquidity::providers::v1::TradeData,
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
        _ => {}
    }

    return result;
}
