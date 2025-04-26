use std::convert::TryInto;
use std::fmt;
use std::str::FromStr;
use std::vec::Vec;
use std::ops::Index;
use std::collections::HashMap;
use sha2::{Sha256, Digest};
use log;
use hex;
use bs58;
use chrono::DateTime;
use substreams_solana::pb::sf::solana::r#type::v1::CompiledInstruction;

// Import all the necessary protobuf message types
use crate::pb::sf::solana::meteora_dlmm::v1::{
    Meta, InstructionArgs, instruction_args, 
    PbSwapLayout, PbAddLiquidityLayout, PbRemoveLiquidityLayout,
    PbInitializePositionLayout, PbInitializePositionPdaLayout, PbUpdatePositionOperatorLayout, 
    PbSwapWithPriceImpactLayout, PbSwapExactOutLayout, PbWithdrawProtocolFeeLayout, PbInitializeRewardLayout,
    PbSetRewardEmissionsLayout, PbFundRewardLayout, PbUpdateRewardFunderLayout, PbUpdateRewardDurationLayout,
    PbCollectRewardLayout, PbCollectFeesLayout, PbClosePositionLayout, PbRemoveAllLiquidityLayout,
    PbTransferPositionOwnerLayout, PbRemoveLiquidityByRangeLayout, PbAddLiquidityOneSidePreciseLayout,
    PbGoToABinLayout, PbWithdrawIneligibleRewardLayout, PbUpdateFeesAndRewardsLayout, PbEventLogWrapper,
    pb_event_log_wrapper, PbLiquidityParameterLayout, PbInitializeLbPairLayout, PbInitializePermissionLbPairLayout,
    PbInitializeBinArrayLayout, PbInitializePresetParameterLayout, PbClosePresetParameterLayout,
    PbCloseLbPairLayout, PbUpdateFeeParametersLayout, PbUpdateFeeOwnerLayout, PbTogglePairStatusLayout,
    PbUpdateWhitelistedWalletLayout, PbIncreaseOracleLengthLayout, PbInitializeBinArrayBitmapExtensionLayout,
    PbMigrateBinArrayLayout, PbSetActivationSlotLayout, PbSetMaxSwappedAmountLayout, PbSetPreActivationDurationLayout,
    PbSetPreActivationSwapAddressLayout, PbSetLockReleaseSlotLayout, PbInitializeCustomizablePermissionlessLbPairLayout,
    PbAddLiquidityByWeightLayout, PbAddLiquidityByStrategyLayout, PbAddLiquidityOneSideLayout,
    PbAddLiquidityByStrategyOneSideLayout, PbRemoveLiquiditySingleSideLayout, PbClaimLiquidityLayout,
    PbInitializePositionByOperatorLayout, PbMigratePositionLayout, PbIdlWriteLayout,
    PbBinLiquidityDistributionByWeightLayout, PbCompressedBinDepositAmountLayout
};

// For convenience, alias the instruction args enum
use crate::pb::sf::solana::meteora_dlmm::v1::instruction_args::InstructionArgs as IArgs;

// Meteora DLMM Program ID
const METEORA_DLMM_PROGRAM_ID: &str = "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo";

// Enum representing different instruction types
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InstructionType {
    // Core Pool Operations
    InitializeLbPair, // IDL: initializeLbPair
    InitializePermissionLbPair, // IDL: initializePermissionLbPair
    InitializeBinArray, // IDL: initializeBinArray
    InitializePresetParameter, // IDL: initializePresetParameter
    ClosePresetParameter, // IDL: closePresetParameter
    CloseLbPair, // IDL: closeLbPair
    UpdateFeeParameters, // IDL: updateFeeParameters
    UpdateFeeOwner, // IDL: updateFeeOwner
    TogglePairStatus, // IDL: togglePairStatus
    UpdateWhitelistedWallet, // IDL: updateWhitelistedWallet
    IncreaseOracleLength, // IDL: increaseOracleLength
    InitializeBinArrayBitmapExtension, // IDL: initializeBinArrayBitmapExtension
    MigrateBinArray, // IDL: migrateBinArray
    SetActivationSlot, // IDL: setActivationSlot
    SetMaxSwappedAmount, // IDL: setMaxSwappedAmount
    SetPreActivationDuration, // IDL: setPreActivationDuration
    SetPreActivationSwapAddress, // IDL: setPreActivationSwapAddress
    SetLockReleaseSlot, // IDL: setLockReleaseSlot
    WithdrawProtocolFee, // IDL: withdrawProtocolFee
    SetActivationPoint, // IDL: setActivationPoint
    InitializeCustomizablePermissionlessLbPair, // IDL: initializeCustomizablePermissionlessLbPair

    // Liquidity Operations
    AddLiquidity, // IDL: addLiquidity
    AddLiquidityByWeight, // IDL: addLiquidityByWeight
    AddLiquidityByStrategy, // IDL: addLiquidityByStrategy
    AddLiquidityOneSide, // IDL: addLiquidityOneSide
    AddLiquidityByStrategyOneSide, // IDL: addLiquidityByStrategyOneSide
    AddLiquidityOneSidePrecise, // IDL: addLiquidityOneSidePrecise
    RemoveLiquidity, // IDL: removeLiquidity
    RemoveAllLiquidity, // IDL: removeAllLiquidity
    RemoveLiquidityByRange, // IDL: removeLiquidityByRange
    RemoveLiquiditySingleSide, // IDL: removeLiquiditySingleSide
    ClaimLiquidity, // IDL: claimLiquidity
    ClaimFee, // IDL: claimFee

    // Trading Operations
    Swap, // IDL: swap
    SwapWithPriceImpact, // IDL: swapWithPriceImpact
    SwapExactOut, // IDL: swapExactOut
    GoToABin, // IDL: goToABin

    // Position Management
    InitializePosition, // IDL: initializePosition
    InitializePositionPda, // IDL: initializePositionPda
    InitializePositionByOperator, // IDL: initializePositionByOperator
    ClosePosition, // IDL: closePosition
    UpdatePositionOperator, // IDL: updatePositionOperator
    MigratePosition, // IDL: migratePosition
    TransferPositionOwner, // NEW from IDL

    // Rewards Management
    InitializeReward, // IDL: initializeReward
    FundReward, // IDL: fundReward
    ClaimReward, // IDL: claimReward
    UpdateRewardFunder, // IDL: updateRewardFunder
    UpdateRewardDuration, // IDL: updateRewardDuration
    WithdrawIneligibleReward, // IDL: withdrawIneligibleReward
    SetRewardEmissions, // NEW from IDL

    // Admin/Utility
    IdlWrite, // IDL: idlWrite
    InitializeTokenBadge, // NEW from IDL
    CreateClaimProtocolFeeOperator, // NEW from IDL
    CloseClaimProtocolFeeOperator, // NEW from IDL

    // V2 Instructions (Require Proto Updates/Parsing Logic)
    InitializeLbPair2, // NEW from IDL
    InitializeCustomizablePermissionlessLbPair2, // NEW from IDL
    ClaimFee2, // NEW from IDL
    ClaimReward2, // NEW from IDL
    AddLiquidity2, // NEW from IDL
    AddLiquidityByStrategy2, // NEW from IDL
    AddLiquidityOneSidePrecise2, // NEW from IDL
    RemoveLiquidity2, // NEW from IDL
    RemoveLiquidityByRange2, // NEW from IDL
    Swap2, // NEW from IDL
    SwapExactOut2, // NEW from IDL
    SwapWithPriceImpact2, // NEW from IDL
    ClosePosition2, // NEW from IDL
    UpdateFeesAndRewards, // IDL: updateFeesAndRewards / updateFeesAndReward2 (Combined)
    ClosePositionIfEmpty, // NEW from IDL

    // Event Log (Special case)
    EventLog,
}

// Event discriminators
const EVENT_COMPOSITION_FEE_DISCRIMINATOR: &[u8] = &[220, 173, 171, 46, 117, 16, 250, 22];
const EVENT_ADD_LIQUIDITY_DISCRIMINATOR: &[u8] = &[75, 16, 143, 85, 158, 142, 79, 209];
const EVENT_REMOVE_LIQUIDITY_DISCRIMINATOR: &[u8] = &[133, 94, 200, 100, 59, 148, 76, 203];
const EVENT_SWAP_DISCRIMINATOR: &[u8] = &[148, 13, 55, 222, 120, 220, 22, 65];
const EVENT_CLAIM_REWARD_DISCRIMINATOR: &[u8] = &[173, 22, 221, 116, 213, 176, 188, 175];
const EVENT_FUND_REWARD_DISCRIMINATOR: &[u8] = &[61, 13, 255, 176, 106, 247, 203, 24];
const EVENT_INITIALIZE_REWARD_DISCRIMINATOR: &[u8] = &[37, 216, 20, 211, 181, 115, 146, 2];
const EVENT_UPDATE_REWARD_DURATION_DISCRIMINATOR: &[u8] = &[202, 150, 52, 51, 130, 149, 22, 34];
const EVENT_UPDATE_REWARD_FUNDER_DISCRIMINATOR: &[u8] = &[73, 169, 123, 25, 146, 210, 236, 121];
const EVENT_POSITION_CLOSE_DISCRIMINATOR: &[u8] = &[77, 239, 165, 5, 182, 6, 24, 140];
const EVENT_CLAIM_FEE_DISCRIMINATOR: &[u8] = &[67, 28, 252, 254, 139, 191, 42, 197];
const EVENT_LB_PAIR_CREATE_DISCRIMINATOR: &[u8] = &[60, 164, 14, 54, 231, 17, 162, 255];
const EVENT_POSITION_CREATE_DISCRIMINATOR: &[u8] = &[210, 192, 164, 185, 43, 131, 106, 66];
const EVENT_FEE_PARAMETER_UPDATE_DISCRIMINATOR: &[u8] = &[3, 89, 137, 250, 156, 109, 156, 131];
const EVENT_INCREASE_OBSERVATION_DISCRIMINATOR: &[u8] = &[56, 122, 125, 134, 96, 152, 207, 57];
const EVENT_WITHDRAW_INELIGIBLE_REWARD_DISCRIMINATOR: &[u8] = &[226, 62, 82, 13, 174, 30, 6, 132];
const EVENT_UPDATE_POSITION_OPERATOR_DISCRIMINATOR: &[u8] = &[87, 252, 133, 141, 135, 217, 104, 132];
const EVENT_UPDATE_POSITION_LOCK_RELEASE_SLOT_DISCRIMINATOR: &[u8] = &[148, 113, 235, 97, 116, 147, 13, 98];
const EVENT_GO_TO_A_BIN_DISCRIMINATOR: &[u8] = &[44, 173, 250, 85, 11, 159, 32, 35];
const EVENT_UPDATE_POSITION_LOCK_RELEASE_POINT_DISCRIMINATOR: &[u8] = &[183, 213, 111, 83, 40, 239, 41, 187];
const EVENT_INCREASE_POSITION_LENGTH_DISCRIMINATOR: &[u8] = &[227, 85, 84, 147, 8, 105, 191, 24];
const EVENT_DECREASE_POSITION_LENGTH_DISCRIMINATOR: &[u8] = &[8, 202, 160, 141, 192, 197, 21, 247];
const EVENT_DYNAMIC_FEE_PARAMETER_UPDATE_DISCRIMINATOR: &[u8] = &[69, 95, 192, 251, 144, 196, 179, 221];
const EVENT_UNKNOWN_EVENT1_DISCRIMINATOR: &[u8] = &[179, 72, 71, 30, 59, 19, 170, 3];

// Use a proper event log discriminator 
const EVENT_LOG_DISCRIMINATOR: &[u8] = &[31, 236, 14, 41, 98, 139, 236, 72];

// TODO: This array needs to be updated to match the InstructionType enum and IDL names exactly.
//       The order also matters for discriminator matching.
const INSTRUCTION_TYPES: &[(&str, InstructionType)] = &[
    // Use camelCase instruction names from IDL
    ("initializeLbPair", InstructionType::InitializeLbPair),
    ("initializePermissionLbPair", InstructionType::InitializePermissionLbPair),
    ("initializeBinArray", InstructionType::InitializeBinArray),
    ("initializePresetParameter", InstructionType::InitializePresetParameter),
    ("closePresetParameter", InstructionType::ClosePresetParameter),
    ("closeLbPair", InstructionType::CloseLbPair),
    ("updateFeeParameters", InstructionType::UpdateFeeParameters),
    ("updateFeeOwner", InstructionType::UpdateFeeOwner),
    ("togglePairStatus", InstructionType::TogglePairStatus),
    ("updateWhitelistedWallet", InstructionType::UpdateWhitelistedWallet),
    ("increaseOracleLength", InstructionType::IncreaseOracleLength),
    ("initializeBinArrayBitmapExtension", InstructionType::InitializeBinArrayBitmapExtension),
    ("migrateBinArray", InstructionType::MigrateBinArray),
    ("setActivationSlot", InstructionType::SetActivationSlot),
    ("setMaxSwappedAmount", InstructionType::SetMaxSwappedAmount),
    ("setPreActivationDuration", InstructionType::SetPreActivationDuration),
    ("setPreActivationSwapAddress", InstructionType::SetPreActivationSwapAddress),
    ("setLockReleaseSlot", InstructionType::SetLockReleaseSlot),
    ("withdrawProtocolFee", InstructionType::WithdrawProtocolFee),
    ("setActivationPoint", InstructionType::SetActivationPoint),
    ("initializeCustomizablePermissionlessLbPair", InstructionType::InitializeCustomizablePermissionlessLbPair),
    ("addLiquidity", InstructionType::AddLiquidity),
    ("addLiquidityByWeight", InstructionType::AddLiquidityByWeight),
    ("addLiquidityByStrategy", InstructionType::AddLiquidityByStrategy),
    ("addLiquidityOneSide", InstructionType::AddLiquidityOneSide),
    ("addLiquidityByStrategyOneSide", InstructionType::AddLiquidityByStrategyOneSide),
    ("addLiquidityOneSidePrecise", InstructionType::AddLiquidityOneSidePrecise),
    ("removeLiquidity", InstructionType::RemoveLiquidity),
    ("removeAllLiquidity", InstructionType::RemoveAllLiquidity),
    ("removeLiquidityByRange", InstructionType::RemoveLiquidityByRange),
    ("removeLiquiditySingleSide", InstructionType::RemoveLiquiditySingleSide),
    ("swap", InstructionType::Swap),
    ("swapWithPriceImpact", InstructionType::SwapWithPriceImpact),
    ("swapExactOut", InstructionType::SwapExactOut),
    ("goToABin", InstructionType::GoToABin),
    ("initializePosition", InstructionType::InitializePosition),
    ("initializePositionPda", InstructionType::InitializePositionPda),
    ("initializePositionByOperator", InstructionType::InitializePositionByOperator),
    ("closePosition", InstructionType::ClosePosition),
    ("updatePositionOperator", InstructionType::UpdatePositionOperator),
    ("migratePosition", InstructionType::MigratePosition),
    ("claimFee", InstructionType::ClaimFee),
    ("claimReward", InstructionType::ClaimReward),
    ("claimLiquidity", InstructionType::ClaimLiquidity),
    ("initializeReward", InstructionType::InitializeReward),
    ("fundReward", InstructionType::FundReward),
    ("updateRewardFunder", InstructionType::UpdateRewardFunder),
    ("updateRewardDuration", InstructionType::UpdateRewardDuration),
    ("withdrawIneligibleReward", InstructionType::WithdrawIneligibleReward),
    ("idlWrite", InstructionType::IdlWrite),
    ("updateFeesAndRewards", InstructionType::UpdateFeesAndRewards),
    // V2 and additional variants
    ("setRewardEmissions", InstructionType::SetRewardEmissions),
    ("transferPositionOwner", InstructionType::TransferPositionOwner),
    ("initializeTokenBadge", InstructionType::InitializeTokenBadge),
    ("createClaimProtocolFeeOperator", InstructionType::CreateClaimProtocolFeeOperator),
    ("closeClaimProtocolFeeOperator", InstructionType::CloseClaimProtocolFeeOperator),
    ("initializeLbPair2", InstructionType::InitializeLbPair2),
    ("initializeCustomizablePermissionlessLbPair2", InstructionType::InitializeCustomizablePermissionlessLbPair2),
    ("claimFee2", InstructionType::ClaimFee2),
    ("claimReward2", InstructionType::ClaimReward2),
    ("addLiquidity2", InstructionType::AddLiquidity2),
    ("addLiquidityByStrategy2", InstructionType::AddLiquidityByStrategy2),
    ("addLiquidityOneSidePrecise2", InstructionType::AddLiquidityOneSidePrecise2),
    ("removeLiquidity2", InstructionType::RemoveLiquidity2),
    ("removeLiquidityByRange2", InstructionType::RemoveLiquidityByRange2),
    ("swap2", InstructionType::Swap2),
    ("swapExactOut2", InstructionType::SwapExactOut2),
    ("swapWithPriceImpact2", InstructionType::SwapWithPriceImpact2),
    ("closePosition2", InstructionType::ClosePosition2),
    ("updateFeesAndReward2", InstructionType::UpdateFeesAndRewards),
    ("closePositionIfEmpty", InstructionType::ClosePositionIfEmpty),
];

/// Compute an 8-byte discriminator from a string by hashing its bytes and taking the first 8 bytes
fn compute_discriminator(name: &str) -> [u8; 8] {
    // For Meteora/Anchor programs, the instruction discriminator is calculated by:
    // - Taking the first 8 bytes of the SHA256 hash of "global:" + instruction_name in snake_case
    let prefixed_name = format!("global:{}", camel_to_snake(name));
    
    let mut hasher = Sha256::new();
    hasher.update(prefixed_name.as_bytes());
    let result = hasher.finalize();
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&result[..8]);
    discriminator
}

/// Convert camelCase to snake_case
fn camel_to_snake(camel: &str) -> String {
    let mut snake = String::with_capacity(camel.len() + 5); // Approximation
    let mut chars = camel.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c.is_uppercase() {
            if !snake.is_empty() && snake.chars().last().unwrap() != '_' {
                snake.push('_');
            }
            snake.push(c.to_lowercase().next().unwrap());
        } else {
            snake.push(c);
        }
    }
    
    snake
}

/// Get instruction type from discriminator
fn get_instruction_type(discriminator: &[u8]) -> Option<InstructionType> {
    get_instruction_type_from_discriminator(discriminator).map(|name| {
        for (inst_name, inst_type) in INSTRUCTION_TYPES {
            if *inst_name == name {  // Compare &str with &str
                return *inst_type;
            }
        }
        // If not found, default to EventLog as fallback
        InstructionType::EventLog
    })
}

/// Get instruction type from discriminator bytes
fn get_instruction_type_from_discriminator(discriminator: &[u8]) -> Option<&'static str> {
    if discriminator.len() != 8 {
        return None; // Discriminator must be 8 bytes
    }
    // Iterate through INSTRUCTION_TYPES and compare discriminators
    for (name, _) in INSTRUCTION_TYPES {
        let expected_discriminator = compute_discriminator(name);
        if discriminator == expected_discriminator {
            return Some(name);
        }
    }
    None
}

/// Get instruction type string from InstructionType enum (used for matching in process_instruction_data)
fn get_instruction_type_str(inst_type: InstructionType) -> &'static str {
    match inst_type {
        // Map enum variants back to the string names used in INSTRUCTION_TYPES/IDL
        InstructionType::Swap => "Swap",
        InstructionType::SwapWithPriceImpact => "SwapWithPriceImpact",
        InstructionType::SwapExactOut => "SwapExactOut",
        InstructionType::InitializeLbPair => "InitializeLbPair",
        InstructionType::InitializePermissionLbPair => "InitializePermissionLbPair",
        InstructionType::InitializePosition => "InitializePosition",
        InstructionType::InitializePositionPda => "InitializePositionPda",
        InstructionType::ClosePosition => "ClosePosition",
        InstructionType::ClaimFee => "ClaimFee",
        InstructionType::ClaimReward => "ClaimReward",
        InstructionType::InitializeCustomizablePermissionlessLbPair => "InitializeCustomizablePermissionlessLbPair",
        InstructionType::CloseLbPair => "CloseLbPair",
        InstructionType::InitializeBinArray => "InitializeBinArray",
        InstructionType::InitializeBinArrayBitmapExtension => "InitializeBinArrayBitmapExtension",
        InstructionType::AddLiquidity => "AddLiquidity",
        InstructionType::AddLiquidityByWeight => "AddLiquidityByWeight",
        InstructionType::AddLiquidityByStrategy => "AddLiquidityByStrategy",
        InstructionType::AddLiquidityOneSide => "AddLiquidityOneSide",
        InstructionType::AddLiquidityByStrategyOneSide => "AddLiquidityByStrategyOneSide",
        InstructionType::AddLiquidityOneSidePrecise => "AddLiquidityOneSidePrecise",
        InstructionType::RemoveLiquidity => "RemoveLiquidity",
        InstructionType::RemoveAllLiquidity => "RemoveAllLiquidity",
        InstructionType::RemoveLiquidityByRange => "RemoveLiquidityByRange",
        InstructionType::RemoveLiquiditySingleSide => "RemoveLiquiditySingleSide",
        InstructionType::GoToABin => "GoToABin",
        InstructionType::InitializePositionByOperator => "InitializePositionByOperator",
        InstructionType::UpdatePositionOperator => "UpdatePositionOperator",
        InstructionType::ClaimLiquidity => "ClaimLiquidity",
        InstructionType::WithdrawProtocolFee => "WithdrawProtocolFee",
        InstructionType::UpdateFeeParameters => "UpdateFeeParameters",
        InstructionType::UpdateFeeOwner => "UpdateFeeOwner",
        InstructionType::InitializeReward => "InitializeReward",
        InstructionType::FundReward => "FundReward",
        InstructionType::UpdateRewardFunder => "UpdateRewardFunder",
        InstructionType::UpdateRewardDuration => "UpdateRewardDuration",
        InstructionType::WithdrawIneligibleReward => "WithdrawIneligibleReward",
        InstructionType::ClosePresetParameter => "ClosePresetParameter",
        InstructionType::InitializePresetParameter => "InitializePresetParameter",
        InstructionType::TogglePairStatus => "TogglePairStatus",
        InstructionType::UpdateWhitelistedWallet => "UpdateWhitelistedWallet",
        InstructionType::IncreaseOracleLength => "IncreaseOracleLength",
        InstructionType::MigratePosition => "MigratePosition",
        InstructionType::MigrateBinArray => "MigrateBinArray",
        InstructionType::UpdateFeesAndRewards => "UpdateFeesAndRewards", // Handles both
        InstructionType::SetLockReleaseSlot => "SetLockReleaseSlot",
        InstructionType::SetActivationSlot => "SetActivationSlot",
        InstructionType::SetMaxSwappedAmount => "SetMaxSwappedAmount",
        InstructionType::SetPreActivationDuration => "SetPreActivationDuration",
        InstructionType::SetPreActivationSwapAddress => "SetPreActivationSwapAddress",
        InstructionType::IdlWrite => "IdlWrite",
        InstructionType::SetActivationPoint => "SetActivationPoint",

        // New IDL mappings
        InstructionType::SetRewardEmissions => "setRewardEmissions",
        InstructionType::TransferPositionOwner => "transferPositionOwner",
        InstructionType::InitializeTokenBadge => "initializeTokenBadge",
        InstructionType::CreateClaimProtocolFeeOperator => "createClaimProtocolFeeOperator",
        InstructionType::CloseClaimProtocolFeeOperator => "closeClaimProtocolFeeOperator",
        InstructionType::InitializeLbPair2 => "initializeLbPair2",
        InstructionType::InitializeCustomizablePermissionlessLbPair2 => "initializeCustomizablePermissionlessLbPair2",
        InstructionType::ClaimFee2 => "claimFee2",
        InstructionType::ClaimReward2 => "claimReward2",
        InstructionType::AddLiquidity2 => "addLiquidity2",
        InstructionType::AddLiquidityByStrategy2 => "addLiquidityByStrategy2",
        InstructionType::AddLiquidityOneSidePrecise2 => "addLiquidityOneSidePrecise2",
        InstructionType::RemoveLiquidity2 => "removeLiquidity2",
        InstructionType::RemoveLiquidityByRange2 => "removeLiquidityByRange2",
        InstructionType::Swap2 => "swap2",
        InstructionType::SwapExactOut2 => "swapExactOut2",
        InstructionType::SwapWithPriceImpact2 => "swapWithPriceImpact2",
        InstructionType::ClosePosition2 => "closePosition2",
        InstructionType::ClosePositionIfEmpty => "closePositionIfEmpty",

        // Special case
        InstructionType::EventLog => "EventLog",
    }
}

/// Process a single instruction into a Meta object
pub fn process_instruction(
    instruction: &CompiledInstruction,
    account_keys: &[String],
    block_slot: u64,
    block_time: i64,
    tx_id: &str,
    instruction_index: u32,
    is_inner_instruction: bool,
    inner_instruction_index: Option<u32>,
    signer_pubkey: Option<&str>,
    outer_program: Option<&str>,
) -> Option<Meta> {
    let program_id = match account_keys.get(instruction.program_id_index as usize) {
        Some(id) => id,
        None => return None,
    };

    // Only process instructions from the Meteora DLMM program
    if program_id != METEORA_DLMM_PROGRAM_ID {
        return None;
    }

    let data = &instruction.data;
    if data.len() < 8 {
        return None;
    }

    let discriminator = &data[0..8];
    
    // Find instruction type
    let instruction_type_str = match get_instruction_type_from_discriminator(discriminator) {
        Some(inst_type) => inst_type,
        None => {
            log::info!("Unknown instruction discriminator: {}", hex::encode(discriminator));
            return None;
        }
    };
    
    // Get instruction type enum for mapping accounts
    let instruction_type = get_instruction_type(discriminator).unwrap_or(InstructionType::EventLog);
    
    // Map accounts to roles using prepare_input_accounts module
    let account_mapper_result = crate::prepare_input_accounts::map_accounts(
        &instruction.accounts,
        instruction_type
    );
    
    // Create input_accounts mapping with ROLES AS KEYS and ACCOUNT ADDRESSES AS VALUES
    // Also skip generic account_X entries
    let mut input_accounts = std::collections::HashMap::new();
    for (idx, account_idx) in instruction.accounts.iter().enumerate() {
        if idx < account_mapper_result.accounts.len() {
            if let Some(address) = account_keys.get(*account_idx as usize) {
                let role = &account_mapper_result.accounts[idx];
                
                // Skip generic account_X entries
                if !role.starts_with("account_") {
                    input_accounts.insert(role.clone(), address.clone());
                }
            }
        }
    }
    
    // Parse instruction data to get arguments
    let args = process_instruction_data(data, discriminator);

    // Create date string from timestamp
    let dt = DateTime::from_timestamp(block_time, 0).unwrap_or_else(|| DateTime::from_timestamp(0, 0).unwrap());
    let block_date = dt.format("%Y-%m-%d").to_string();

    // Create a new Meta object with the parsed information
    // Wrap ONLY optional fields in Some()
    let meta = Meta {
        tx_id: tx_id.to_string(), // Not optional
        block_slot,               // Not optional
        block_time,               // Not optional
        block_date,               // Not optional
        instruction_index: Some(instruction_index),        // Wrap in Some
        is_inner_instruction: Some(is_inner_instruction),     // Wrap in Some
        inner_instruction_index: Some(inner_instruction_index.unwrap_or(0)), // Optional: Use default, wrap in Some()
        signer: Some(signer_pubkey.map_or(String::new(), String::from)), // Optional: Use default, wrap in Some()
        outer_program: Some(outer_program.map_or(String::new(), String::from)), // Optional: Use default, wrap in Some()
        instruction_type: instruction_type_str.to_string(), // Not optional
        input_accounts,           // Not optional (map)
        args,                     // Optional by definition in proto
    };

    Some(meta)
}

/// Process instruction data to extract arguments
pub fn process_instruction_data(data: &[u8], discriminator: &[u8]) -> Option<InstructionArgs> {
    let mut args = InstructionArgs::default();
    let inst_type_opt = get_instruction_type(discriminator);

    if inst_type_opt.is_none() {
        // Check if this is an event log instruction with "EventLog" discriminator
        if data.len() >= 8 && &data[0..8] == EVENT_LOG_DISCRIMINATOR {
            return process_event_log(&data[8..], InstructionArgs {
                instruction_args: Some(instruction_args::InstructionArgs::EventLog(PbEventLogWrapper {
                    event_name: "EventLog".to_string(),
                    event_fields: None,
                }))
            });
        }
        log::info!("Unknown instruction discriminator: {}", hex::encode(discriminator));
        return None;
    }

    let inst_type = inst_type_opt.unwrap();
    let inst_name = get_instruction_type_str(inst_type);

    // Common pattern: log the specific instruction type we're processing
    log::debug!("Processing {} instruction", inst_name);

    // Parse based on instruction type
    match inst_type {
        // Core Pool Operations
        InstructionType::InitializeLbPair => {
            if data.len() < 16 { return None; }
            args.instruction_args = Some(instruction_args::InstructionArgs::InitializeLbPair(PbInitializeLbPairLayout {
                active_id: Some(parse_i32(data, 8).unwrap_or(0)),
                bin_step: Some(parse_i32(data, 12).unwrap_or(0)),
            }));
        },
        InstructionType::InitializePermissionLbPair => {
            if data.len() < 16 { return None; }
            args.instruction_args = Some(instruction_args::InstructionArgs::InitializePermissionLbPair(PbInitializePermissionLbPairLayout {
                active_id: Some(parse_i32(data, 8).unwrap_or(0)),
                bin_step: Some(parse_i32(data, 12).unwrap_or(0)),
            }));
        },
        InstructionType::InitializeBinArray => {
            if data.len() < 16 { return None; }
            args.instruction_args = Some(instruction_args::InstructionArgs::InitializeBinArray(PbInitializeBinArrayLayout {
                index: Some(parse_i64(data, 8).unwrap_or(0)),
            }));
        },
        InstructionType::InitializePresetParameter => {
            // Parse all preset parameter fields
            if data.len() < 40 { return None; }
            args.instruction_args = Some(instruction_args::InstructionArgs::InitializePresetParameter(PbInitializePresetParameterLayout {
                bin_step: Some(parse_i32(data, 8).unwrap_or(0)),
                base_factor: Some(parse_i32(data, 12).unwrap_or(0)),
                filter_period: Some(parse_i32(data, 16).unwrap_or(0)),
                decay_period: Some(parse_i32(data, 20).unwrap_or(0)),
                reduction_factor: Some(parse_i32(data, 24).unwrap_or(0)),
                variable_fee_control: Some(parse_i32(data, 28).unwrap_or(0)),
                max_volatility_accumulator: Some(parse_i32(data, 32).unwrap_or(0)),
                min_bin_id: Some(parse_i32(data, 36).unwrap_or(0)),
                max_bin_id: Some(parse_i32(data, 40).unwrap_or(0)),
                protocol_share: Some(parse_i32(data, 44).unwrap_or(0)),
            }));
        },
        InstructionType::ClosePresetParameter => {
            // No arguments needed
            args.instruction_args = Some(instruction_args::InstructionArgs::ClosePresetParameter(PbClosePresetParameterLayout {}));
        },
        InstructionType::CloseLbPair => {
            // No arguments needed
            args.instruction_args = Some(instruction_args::InstructionArgs::CloseLbPair(PbCloseLbPairLayout {}));
        },
        InstructionType::UpdateFeeParameters => {
            if data.len() < 16 { return None; }
            args.instruction_args = Some(instruction_args::InstructionArgs::UpdateFeeParameters(PbUpdateFeeParametersLayout {
                protocol_share: Some(parse_i32(data, 8).unwrap_or(0)),
                base_factor: Some(parse_i32(data, 12).unwrap_or(0)),
            }));
        },
        InstructionType::UpdateFeeOwner => {
            // No arguments needed
            args.instruction_args = Some(instruction_args::InstructionArgs::UpdateFeeOwner(PbUpdateFeeOwnerLayout {}));
        },
        InstructionType::TogglePairStatus => {
            // No arguments needed
            args.instruction_args = Some(instruction_args::InstructionArgs::TogglePairStatus(PbTogglePairStatusLayout {}));
        },
        InstructionType::UpdateWhitelistedWallet => {
            if data.len() < 42 { return None; }
            args.instruction_args = Some(instruction_args::InstructionArgs::UpdateWhitelistedWallet(PbUpdateWhitelistedWalletLayout {
                idx: Some(parse_i16(data, 8).unwrap_or(0) as i32),
                wallet: Some(bytes_to_pubkey_str(data, 10).unwrap_or_default()),
            }));
        },
        InstructionType::IncreaseOracleLength => {
            if data.len() < 16 { return None; }
            args.instruction_args = Some(instruction_args::InstructionArgs::IncreaseOracleLength(PbIncreaseOracleLengthLayout {
                length_to_add: Some(parse_u32(data, 8).unwrap_or(0) as i64),
            }));
        },
        InstructionType::InitializeBinArrayBitmapExtension => {
            // No arguments needed
            args.instruction_args = Some(instruction_args::InstructionArgs::InitializeBinArrayBitmapExtension(PbInitializeBinArrayBitmapExtensionLayout {}));
        },
        InstructionType::MigrateBinArray => {
            // No arguments needed
            args.instruction_args = Some(instruction_args::InstructionArgs::MigrateBinArray(PbMigrateBinArrayLayout {}));
        },
        InstructionType::SetActivationSlot => {
            if data.len() < 16 { return None; }
            args.instruction_args = Some(instruction_args::InstructionArgs::SetActivationSlot(PbSetActivationSlotLayout {
                activation_slot: Some(parse_i64(data, 8).unwrap_or(0)),
            }));
        },
        InstructionType::SetMaxSwappedAmount => {
            if data.len() < 32 { return None; }
            args.instruction_args = Some(instruction_args::InstructionArgs::SetMaxSwappedAmount(PbSetMaxSwappedAmountLayout {
                swap_cap_deactivate_slot: Some(parse_i64(data, 8).unwrap_or(0)),
                max_swapped_amount: Some(parse_u128(data, 16).unwrap_or(0).to_string()),
            }));
        },
        InstructionType::SetPreActivationDuration => {
            if data.len() < 16 { return None; }
            args.instruction_args = Some(instruction_args::InstructionArgs::SetPreActivationDuration(PbSetPreActivationDurationLayout {
                pre_activation_duration: Some(parse_i64(data, 8).unwrap_or(0)),
            }));
        },
        InstructionType::SetPreActivationSwapAddress => {
            if data.len() < 40 { return None; }
            args.instruction_args = Some(instruction_args::InstructionArgs::SetPreActivationSwapAddress(PbSetPreActivationSwapAddressLayout {
                pre_activation_swap_address: Some(bytes_to_pubkey_str(data, 8).unwrap_or_default()),
            }));
        },
        InstructionType::SetLockReleaseSlot => {
            if data.len() < 16 { return None; }
            args.instruction_args = Some(instruction_args::InstructionArgs::SetLockReleaseSlot(PbSetLockReleaseSlotLayout {
                new_lock_release_slot: Some(parse_i64(data, 8).unwrap_or(0)),
            }));
        },
        InstructionType::WithdrawProtocolFee => {
            if data.len() < 24 { return None; }
            args.instruction_args = Some(instruction_args::InstructionArgs::WithdrawProtocolFee(PbWithdrawProtocolFeeLayout {
                amount_x: Some(parse_u64(data, 8).unwrap_or(0)),
                amount_y: Some(parse_u64(data, 16).unwrap_or(0)),
            }));
        },
        InstructionType::InitializeCustomizablePermissionlessLbPair => {
            if data.len() < 40 { return None; }
            
            let active_id = parse_i32(data, 8).unwrap_or(0);
            let bin_step = parse_i32(data, 12).unwrap_or(0);
            let base_factor = parse_i32(data, 16).unwrap_or(0);
            let activation_type = data[20] as u32; // Assuming u8 maps to uint32
            let has_alpha_vault = data[21] != 0;   // Assuming bool
            let activation_point = parse_i64(data, 24).unwrap_or(0);
            
            args.instruction_args = Some(instruction_args::InstructionArgs::InitializeCustomizablePermissionlessLbPair(
                PbInitializeCustomizablePermissionlessLbPairLayout {
                    active_id: Some(active_id),
                    bin_step: Some(bin_step),
                    base_factor: Some(base_factor),
                    activation_type: Some(activation_type),
                    has_alpha_vault: Some(has_alpha_vault),
                    activation_point: Some(activation_point),
                }
            ));
        },

        // Liquidity Operations
        InstructionType::AddLiquidity => {
            if data.len() < 32 { return None; }
            
            let tick_lower_index = parse_i32(data, 8).unwrap_or(0);
            let tick_upper_index = parse_i32(data, 12).unwrap_or(0);
            let liquidity_amount = parse_u128(data, 16).unwrap_or(0).to_string();
            let token_max_a = parse_u64(data, 32).unwrap_or(0);
            let token_max_b = parse_u64(data, 40).unwrap_or(0);
            
            // Liquidity parameter would be bytes slice, skip for now
            
            args.instruction_args = Some(instruction_args::InstructionArgs::AddLiquidity(PbAddLiquidityLayout {
                tick_lower_index,
                tick_upper_index,
                liquidity_amount,
                token_max_a,
                token_max_b,
                liquidity_parameter: None,
            }));
        },
        InstructionType::AddLiquidityByWeight => {
            // Complex structure, would need more detailed logic for the liquidity_parameter
            args.instruction_args = Some(instruction_args::InstructionArgs::AddLiquidityByWeight(PbAddLiquidityByWeightLayout {
                liquidity_parameter: Some(PbLiquidityParameterLayout {}),
            }));
        },
        InstructionType::AddLiquidityByStrategy => {
            // Complex structure, would need more detailed logic for the liquidity_parameter
            args.instruction_args = Some(instruction_args::InstructionArgs::AddLiquidityByStrategy(PbAddLiquidityByStrategyLayout {
                liquidity_parameter: Some(PbLiquidityParameterLayout {}),
            }));
        },
        InstructionType::AddLiquidityOneSide => {
            if data.len() < 24 { return None; }
            
            let amount = parse_u64(data, 8).unwrap_or(0);
            let active_id = parse_i32(data, 16).unwrap_or(0);
            let max_active_bin_slippage = parse_i32(data, 20).unwrap_or(0);
            
            // For bin_liquidity_dist, we need to parse the vector of BinLiquidityDistributionByWeight
            let mut bin_liquidity_dist = Vec::new();
            
            // Try to parse the bin distribution array if there's enough data
            // Assume length is u32 (4 bytes) starting at offset 24, data starts at 28
            if data.len() >= 28 { 
                // Parse length as u32
                let bin_dist_len = match parse_u32(data, 24) { 
                    Ok(len) => len as usize,
                    Err(_) => 0, // Default to 0 if length parsing fails
                };
                let mut offset = 28; // Start after the u32 length field
                
                for _ in 0..bin_dist_len {
                    if offset + 6 <= data.len() { // 4 bytes for bin_id + 2 bytes for weight
                        // Use parse_u16 for weight based on IDL
                        if let (Ok(bin_id), Ok(weight)) = (parse_i32(data, offset), parse_u16(data, offset + 4)) {
                            bin_liquidity_dist.push(PbBinLiquidityDistributionByWeightLayout {
                                bin_id: Some(bin_id),
                                weight: Some(weight as i32), // Convert u16 to i32 as per the proto definition
                            });
                        }
                        offset += 6; // Move to the next element (i32 + u16 = 4 + 2 = 6 bytes)
                    } else {
                        break; // Not enough data to parse the entry
                    }
                }
            }
            
            log::debug!("Parsed {} bin liquidity distributions for AddLiquidityOneSide", bin_liquidity_dist.len());
            
            args.instruction_args = Some(instruction_args::InstructionArgs::AddLiquidityOneSide(PbAddLiquidityOneSideLayout {
                amount: Some(amount),
                active_id: Some(active_id),
                max_active_bin_slippage: Some(max_active_bin_slippage),
                bin_liquidity_dist,
            }));
        },
        InstructionType::AddLiquidityByStrategyOneSide => {
            // Complex structure, needs detailed parsing
            args.instruction_args = Some(instruction_args::InstructionArgs::AddLiquidityByStrategyOneSide(PbAddLiquidityByStrategyOneSideLayout {
                liquidity_parameter: Some(PbLiquidityParameterLayout {}),
            }));
        },
        InstructionType::AddLiquidityOneSidePrecise => {
            if data.len() < 12 { return None; }
            
            // Parse decompress_multiplier from data
            let decompress_multiplier = parse_i64(data, 8).unwrap_or(0);
            
            // Parse the bins array
            let mut bins = Vec::new();
            
            // Try to parse the bin deposit amounts array
            if data.len() >= 12 {
                // First byte after the header + decompress_multiplier should indicate the number of bins
                let bins_len = data[16] as usize;
                let mut offset = 17; // Start after the length byte
                
                for _ in 0..bins_len {
                    if offset + 6 <= data.len() { // 4 bytes for bin_id + 2 bytes for amount
                        if let (Ok(bin_id), Ok(amount)) = (parse_i32(data, offset), parse_i16(data, offset + 4)) {
                            bins.push(PbCompressedBinDepositAmountLayout {
                                bin_id: Some(bin_id),
                                amount: Some(amount as i32), // Convert to i32 as per proto definition
                            });
                        }
                        offset += 6;
                    } else {
                        break; // Not enough data to parse the entry
                    }
                }
            }
            
            log::debug!("Parsed {} bin deposit amounts for AddLiquidityOneSidePrecise", bins.len());
            
            args.instruction_args = Some(instruction_args::InstructionArgs::AddLiquidityOneSidePrecise(PbAddLiquidityOneSidePreciseLayout {
                bins,
                decompress_multiplier: Some(decompress_multiplier),
            }));
        },
        InstructionType::RemoveLiquidity => {
            if data.len() < 48 { return None; }
            
            let tick_lower_index = parse_i32(data, 8).unwrap_or(0);
            let tick_upper_index = parse_i32(data, 12).unwrap_or(0);
            let liquidity_amount = parse_u128(data, 16).unwrap_or(0).to_string();
            let token_min_a = parse_u64(data, 32).unwrap_or(0);
            let token_min_b = parse_u64(data, 40).unwrap_or(0);
            
            // Parse bin_liquidity_removal array if available
            let mut bin_liquidity_removal = Vec::new();
            
            // Check if we have more data for bin_liquidity_removal array
            if data.len() > 48 {
                // First, there should be a byte indicating the length of the array
                let removal_array_len = data[48] as usize;
                let mut offset = 49; // Start from the next byte
                
                for _ in 0..removal_array_len {
                    // Each removal should be at least a bytes vector, assume it follows a length-prefixed format
                    if offset < data.len() {
                        let element_len = data[offset] as usize;
                        offset += 1;
                        
                        if offset + element_len <= data.len() {
                            // Extract the bytes element
                            let element_bytes = data[offset..(offset + element_len)].to_vec();
                            bin_liquidity_removal.push(element_bytes);
                            offset += element_len;
                        } else {
                            break; // Not enough data to parse this element
                        }
                    } else {
                        break; // Not enough data for the next length byte
                    }
                }
            }
            
            log::debug!("Parsed {} bin liquidity removal elements for RemoveLiquidity", bin_liquidity_removal.len());
            
            args.instruction_args = Some(instruction_args::InstructionArgs::RemoveLiquidity(PbRemoveLiquidityLayout {
                tick_lower_index,
                tick_upper_index,
                liquidity_amount,
                token_min_a,
                token_min_b,
                bin_liquidity_removal,
            }));
        },
        InstructionType::RemoveAllLiquidity => {
            // No arguments needed
            args.instruction_args = Some(instruction_args::InstructionArgs::RemoveAllLiquidity(PbRemoveAllLiquidityLayout {}));
        },
        InstructionType::RemoveLiquidityByRange => {
            if data.len() < 14 { return None; }
            
            args.instruction_args = Some(instruction_args::InstructionArgs::RemoveLiquidityByRange(PbRemoveLiquidityByRangeLayout {
                from_bin_id: Some(parse_i32(data, 8).unwrap_or(0)),
                to_bin_id: Some(parse_i32(data, 12).unwrap_or(0)),
                bps_to_remove: Some(parse_u16(data, 16).unwrap_or(0) as i32), // Convert u16 to i32 as per proto definition
            }));
        },
        InstructionType::RemoveLiquiditySingleSide => {
            // No simple arguments
            args.instruction_args = Some(instruction_args::InstructionArgs::RemoveLiquiditySingleSide(PbRemoveLiquiditySingleSideLayout {}));
        },
        InstructionType::ClaimLiquidity => {
            // No arguments needed
            args.instruction_args = Some(instruction_args::InstructionArgs::ClaimLiquidity(PbClaimLiquidityLayout {}));
        },
        InstructionType::ClaimFee => {
            // No arguments needed
            args.instruction_args = Some(instruction_args::InstructionArgs::CollectFees(PbCollectFeesLayout {}));
        },

        // Trading Operations
        InstructionType::Swap => {
            if data.len() < 16 { return None; }
            let amount_in = parse_u64(data, 8).unwrap_or(0).to_string();
            let min_amount_out = parse_u64(data, 16).unwrap_or(0).to_string();
            
            args.instruction_args = Some(instruction_args::InstructionArgs::Swap(PbSwapLayout {
                amount_in: Some(amount_in),
                min_amount_out: Some(min_amount_out),
            }));
        },
        InstructionType::SwapWithPriceImpact => {
            if data.len() < 20 { return None; }
            
            args.instruction_args = Some(instruction_args::InstructionArgs::SwapWithPriceImpact(PbSwapWithPriceImpactLayout {
                amount_in: Some(parse_u64(data, 8).unwrap_or(0)),
                active_id: Some(parse_i32(data, 16).unwrap_or(0)),
                max_price_impact_bps: Some(parse_i32(data, 20).unwrap_or(0)),
            }));
        },
        InstructionType::SwapExactOut => {
            if data.len() < 24 { return None; }
            
            args.instruction_args = Some(instruction_args::InstructionArgs::SwapExactOut(PbSwapExactOutLayout {
                max_in_amount: Some(parse_u64(data, 8).unwrap_or(0)),
                out_amount: Some(parse_u64(data, 16).unwrap_or(0)),
            }));
        },
        InstructionType::GoToABin => {
            if data.len() < 12 { return None; }
            
            args.instruction_args = Some(instruction_args::InstructionArgs::GoToABin(PbGoToABinLayout {
                bin_id: Some(parse_i32(data, 8).unwrap_or(0)),
            }));
        },

        // Position Management
        InstructionType::InitializePosition => {
            if data.len() < 16 { return None; }
            
            args.instruction_args = Some(instruction_args::InstructionArgs::InitializePosition(PbInitializePositionLayout {
                lower_bin_id: Some(parse_i32(data, 8).unwrap_or(0)),
                width: Some(parse_i32(data, 12).unwrap_or(0)),
            }));
        },
        InstructionType::InitializePositionPda => {
            if data.len() < 16 { return None; }
            
            args.instruction_args = Some(instruction_args::InstructionArgs::InitializePositionPda(PbInitializePositionPdaLayout {
                lower_bin_id: Some(parse_i32(data, 8).unwrap_or(0)),
                width: Some(parse_i32(data, 12).unwrap_or(0)),
            }));
        },
        InstructionType::InitializePositionByOperator => {
            if data.len() < 48 { return None; }
            
            args.instruction_args = Some(instruction_args::InstructionArgs::InitializePositionByOperator(PbInitializePositionByOperatorLayout {
                lower_bin_id: Some(parse_i32(data, 8).unwrap_or(0)),
                width: Some(parse_i32(data, 12).unwrap_or(0)),
                owner: Some(bytes_to_pubkey_str(data, 16).unwrap_or_default()),
            }));
        },
        InstructionType::ClosePosition => {
            // No arguments needed
            args.instruction_args = Some(instruction_args::InstructionArgs::ClosePosition(PbClosePositionLayout {}));
        },
        InstructionType::UpdatePositionOperator => {
            if data.len() < 40 { return None; }
            
            args.instruction_args = Some(instruction_args::InstructionArgs::UpdatePositionOperator(PbUpdatePositionOperatorLayout {
                operator: Some(bytes_to_pubkey_str(data, 8).unwrap_or_default()),
            }));
        },
        InstructionType::MigratePosition => {
            // No arguments needed
            args.instruction_args = Some(instruction_args::InstructionArgs::MigratePosition(PbMigratePositionLayout {}));
        },
        InstructionType::TransferPositionOwner => {
            if data.len() < 40 { return None; }
            
            args.instruction_args = Some(instruction_args::InstructionArgs::TransferPositionOwner(PbTransferPositionOwnerLayout {
                new_owner: bytes_to_pubkey_str(data, 8).unwrap_or_default(),
            }));
        },

        // Rewards Management
        InstructionType::InitializeReward => {
            if data.len() < 32 { return None; }
            
            args.instruction_args = Some(instruction_args::InstructionArgs::InitializeReward(PbInitializeRewardLayout {
                emissions_per_second_x64: parse_u128(data, 8).unwrap_or(0).to_string(),
                open_time: parse_u64(data, 24).unwrap_or(0),
                end_time: parse_u64(data, 32).unwrap_or(0),
            }));
        },
        InstructionType::FundReward => {
            if data.len() < 25 { return None; }
            
            args.instruction_args = Some(instruction_args::InstructionArgs::FundReward(PbFundRewardLayout {
                reward_index: Some(parse_i64(data, 8).unwrap_or(0)),
                amount: Some(parse_i64(data, 16).unwrap_or(0)),
                carry_forward: Some(data[24] != 0),
            }));
        },
        InstructionType::ClaimReward => {
            if data.len() < 12 { return None; }
            
            args.instruction_args = Some(instruction_args::InstructionArgs::CollectReward(PbCollectRewardLayout {
                reward_index: parse_u32(data, 8).unwrap_or(0),
            }));
        },
        InstructionType::UpdateRewardFunder => {
            if data.len() < 48 { return None; }
            
            args.instruction_args = Some(instruction_args::InstructionArgs::UpdateRewardFunder(PbUpdateRewardFunderLayout {
                reward_index: Some(parse_i64(data, 8).unwrap_or(0)),
                new_funder: Some(bytes_to_pubkey_str(data, 16).unwrap_or_default()),
            }));
        },
        InstructionType::UpdateRewardDuration => {
            if data.len() < 24 { return None; }
            
            args.instruction_args = Some(instruction_args::InstructionArgs::UpdateRewardDuration(PbUpdateRewardDurationLayout {
                reward_index: Some(parse_i64(data, 8).unwrap_or(0)),
                new_duration: Some(parse_i64(data, 16).unwrap_or(0)),
            }));
        },
        InstructionType::WithdrawIneligibleReward => {
            if data.len() < 16 { return None; }
            
            args.instruction_args = Some(instruction_args::InstructionArgs::WithdrawIneligibleReward(PbWithdrawIneligibleRewardLayout {
                reward_index: Some(parse_i64(data, 8).unwrap_or(0)),
            }));
        },
        InstructionType::SetRewardEmissions => {
            if data.len() < 40 { return None; }
            
            args.instruction_args = Some(instruction_args::InstructionArgs::SetRewardEmissions(PbSetRewardEmissionsLayout {
                reward_index: parse_u32(data, 8).unwrap_or(0),
                emissions_per_second_x64: parse_u128(data, 12).unwrap_or(0).to_string(),
                open_time: parse_u64(data, 28).unwrap_or(0),
                end_time: parse_u64(data, 36).unwrap_or(0),
            }));
        },

        // Admin/Utility
        InstructionType::IdlWrite => {
            // No arguments needed
            args.instruction_args = Some(instruction_args::InstructionArgs::IdlWrite(PbIdlWriteLayout {}));
        },
        InstructionType::SetActivationPoint => {
            if data.len() < 16 { return None; }
            // Since there's no specific layout defined for this instruction in the proto,
            // we'll log it and return None for now
            log::info!("Processing SetActivationPoint instruction (not fully implemented)");
            return None;
        },
        InstructionType::UpdateFeesAndRewards => {
            // No arguments needed
            args.instruction_args = Some(instruction_args::InstructionArgs::UpdateFeesAndRewards(PbUpdateFeesAndRewardsLayout {}));
        },

        // V2 Instructions (require more details from IDL - stubbed for now with empty args)
        InstructionType::InitializeLbPair2 |
        InstructionType::InitializeCustomizablePermissionlessLbPair2 |
        InstructionType::ClaimFee2 |
        InstructionType::ClaimReward2 |
        InstructionType::AddLiquidity2 |
        InstructionType::AddLiquidityByStrategy2 |
        InstructionType::AddLiquidityOneSidePrecise2 |
        InstructionType::RemoveLiquidity2 |
        InstructionType::RemoveLiquidityByRange2 |
        InstructionType::Swap2 |
        InstructionType::SwapExactOut2 |
        InstructionType::SwapWithPriceImpact2 |
        InstructionType::ClosePosition2 |
        InstructionType::ClosePositionIfEmpty |
        InstructionType::InitializeTokenBadge |
        InstructionType::CreateClaimProtocolFeeOperator |
        InstructionType::CloseClaimProtocolFeeOperator => {
            // Map V2 instructions to their V1 counterparts where possible
            match inst_type {
                InstructionType::ClaimFee2 => {
                    if data.len() < 12 { return None; }
                    
                    // ClaimFee2 has minBinId, maxBinId parameters but they don't change proto representation
                    log::debug!("Processing ClaimFee2 instruction with minBinId={}, maxBinId={}", 
                               parse_i32(data, 8).unwrap_or(0), parse_i32(data, 12).unwrap_or(0));
                    
                    args.instruction_args = Some(instruction_args::InstructionArgs::CollectFees(PbCollectFeesLayout {}));
                },
                
                InstructionType::ClaimReward2 => {
                    if data.len() < 16 { return None; }
                    
                    // ClaimReward2 has reward_index, minBinId, maxBinId parameters
                    let reward_index = parse_u64(data, 8).unwrap_or(0) as u32;
                    log::debug!("Processing ClaimReward2 instruction with reward_index={}, minBinId={}, maxBinId={}", 
                               reward_index, parse_i32(data, 16).unwrap_or(0), parse_i32(data, 20).unwrap_or(0));
                    
                    args.instruction_args = Some(instruction_args::InstructionArgs::CollectReward(PbCollectRewardLayout {
                        reward_index,
                    }));
                },
                
                InstructionType::ClosePosition2 | InstructionType::ClosePositionIfEmpty => {
                    // Both map to the same V1 instruction for now
                    args.instruction_args = Some(instruction_args::InstructionArgs::ClosePosition(PbClosePositionLayout {}));
                },
                
                InstructionType::InitializeLbPair2 => {
                    if data.len() < 16 { return None; }
                    
                    // Assuming the params are similar to V1 but with additional fields
                    let active_id = parse_i32(data, 8).unwrap_or(0);
                    let bin_step = parse_i32(data, 12).unwrap_or(0);
                    
                    args.instruction_args = Some(instruction_args::InstructionArgs::InitializeLbPair(PbInitializeLbPairLayout {
                        active_id: Some(active_id),
                        bin_step: Some(bin_step),
                    }));
                },
                
                InstructionType::InitializeCustomizablePermissionlessLbPair2 => {
                    if data.len() < 40 { return None; }
                    
                    // Similar parsing to V1 but potentially with custom params structure
                    let active_id = parse_i32(data, 8).unwrap_or(0);
                    let bin_step = parse_i32(data, 12).unwrap_or(0);
                    let base_factor = parse_i32(data, 16).unwrap_or(0);
                    let activation_type = data[20] as u32;
                    let has_alpha_vault = data[21] != 0;
                    let activation_point = parse_i64(data, 24).unwrap_or(0);
                    
                    args.instruction_args = Some(instruction_args::InstructionArgs::InitializeCustomizablePermissionlessLbPair(
                        PbInitializeCustomizablePermissionlessLbPairLayout {
                            active_id: Some(active_id),
                            bin_step: Some(bin_step),
                            base_factor: Some(base_factor),
                            activation_type: Some(activation_type),
                            has_alpha_vault: Some(has_alpha_vault),
                            activation_point: Some(activation_point),
                        }
                    ));
                },
                
                _ => {
                    // Handle unexpected instruction types gracefully
                    log::warn!("Unexpected V2 instruction type in match: {:?}", inst_type);
                    return None;
                }
            }
        },
        
        // Special case
        InstructionType::EventLog => {
            return process_event_log(data, args);
        },
        InstructionType::Swap2 | 
        InstructionType::SwapExactOut2 | 
        InstructionType::SwapWithPriceImpact2 => {
            if data.len() < 24 { return None; }
            
            // Parse the common arguments based on the instruction type
            match inst_type {
                InstructionType::Swap2 => {
                    // For Swap2, we have amountIn and minAmountOut
                    let amount_in = parse_u64(data, 8).unwrap_or(0);
                    let min_amount_out = parse_u64(data, 16).unwrap_or(0);
                    
                    // We also need to parse RemainingAccountsInfo but that's complex
                    // and would require deeper knowledge of the structure
                    log::debug!("Parsed Swap2 instruction with amount_in={}, min_amount_out={}", 
                               amount_in, min_amount_out);
                    
                    // Using PbSwapLayout as a close match for now
                    args.instruction_args = Some(instruction_args::InstructionArgs::Swap(PbSwapLayout {
                        amount_in: Some(amount_in.to_string()),
                        min_amount_out: Some(min_amount_out.to_string()),
                    }));
                },
                InstructionType::SwapExactOut2 => {
                    // For SwapExactOut2, we have maxInAmount and outAmount
                    let max_in_amount = parse_u64(data, 8).unwrap_or(0);
                    let out_amount = parse_u64(data, 16).unwrap_or(0);
                    
                    log::debug!("Parsed SwapExactOut2 instruction with max_in_amount={}, out_amount={}", 
                               max_in_amount, out_amount);
                    
                    args.instruction_args = Some(instruction_args::InstructionArgs::SwapExactOut(PbSwapExactOutLayout {
                        max_in_amount: Some(max_in_amount),
                        out_amount: Some(out_amount),
                    }));
                },
                InstructionType::SwapWithPriceImpact2 => {
                    // For SwapWithPriceImpact2, we have amountIn, activeId, and maxPriceImpactBps
                    let amount_in = parse_u64(data, 8).unwrap_or(0);
                    
                    // activeId is optional, so we need to check if it's present
                    let active_id_present = data[16] != 0; // Check if the option is Some
                    let active_id = if active_id_present && data.len() >= 21 {
                        Some(parse_i32(data, 17).unwrap_or(0))
                    } else {
                        None
                    };
                    
                    // Parse maxPriceImpactBps as u16
                    let offset = if active_id_present { 21 } else { 17 };
                    let max_price_impact_bps = if data.len() >= offset + 2 {
                        parse_u16(data, offset).unwrap_or(0)
                    } else {
                        0
                    };
                    
                    log::debug!("Parsed SwapWithPriceImpact2 instruction with amount_in={}, active_id={:?}, max_price_impact_bps={}", 
                               amount_in, active_id, max_price_impact_bps);
                    
                    args.instruction_args = Some(instruction_args::InstructionArgs::SwapWithPriceImpact(PbSwapWithPriceImpactLayout {
                        amount_in: Some(amount_in),
                        active_id,
                        max_price_impact_bps: Some(max_price_impact_bps as i32), // Convert u16 to i32
                    }));
                },
                _ => {
                    // Handle unexpected instruction types gracefully
                    log::warn!("Unexpected V2 instruction type in match: {:?}", inst_type);
                    return None;
                }
            }
        },
        InstructionType::AddLiquidity2 |
        InstructionType::AddLiquidityByStrategy2 |
        InstructionType::AddLiquidityOneSidePrecise2 |
        InstructionType::RemoveLiquidity2 |
        InstructionType::RemoveLiquidityByRange2 => {
            // For now, map these V2 instructions to their V1 counterparts
            // These will need more detailed implementations but this gives basic support
            
            match inst_type {
                InstructionType::AddLiquidity2 => {
                    if data.len() < 32 { return None; }
                    
                    // Similar structure to V1 AddLiquidity, but with additional RemainingAccountsInfo
                    let liquidity_parameter = if data.len() > 48 {
                        Some(data[48..].to_vec())
                    } else {
                        None
                    };
                    
                    args.instruction_args = Some(instruction_args::InstructionArgs::AddLiquidity(PbAddLiquidityLayout {
                        tick_lower_index: parse_i32(data, 8).unwrap_or(0),
                        tick_upper_index: parse_i32(data, 12).unwrap_or(0),
                        liquidity_amount: parse_u128(data, 16).unwrap_or(0).to_string(),
                        token_max_a: parse_u64(data, 32).unwrap_or(0),
                        token_max_b: parse_u64(data, 40).unwrap_or(0),
                        liquidity_parameter,
                    }));
                },
                
                InstructionType::AddLiquidityByStrategy2 => {
                    // Map to V1 with basic structure, details on LiquidityParameterByStrategy would need more parsing
                    args.instruction_args = Some(instruction_args::InstructionArgs::AddLiquidityByStrategy(PbAddLiquidityByStrategyLayout {
                        liquidity_parameter: Some(PbLiquidityParameterLayout {}),
                    }));
                },
                
                InstructionType::AddLiquidityOneSidePrecise2 => {
                    if data.len() < 12 { return None; }
                    
                    // Similar to V1, parse decompress_multiplier and bins array
                    let decompress_multiplier = parse_i64(data, 8).unwrap_or(0);
                    
                    // Parse bins array with extra handling for V2's format
                    let mut bins = Vec::new();
                    
                    if data.len() >= 16 {
                        let bins_len = data[16] as usize;
                        let mut offset = 17;
                        
                        for _ in 0..bins_len {
                            if offset + 6 <= data.len() {
                                if let (Ok(bin_id), Ok(amount)) = (parse_i32(data, offset), parse_i16(data, offset + 4)) {
                                    bins.push(PbCompressedBinDepositAmountLayout {
                                        bin_id: Some(bin_id),
                                        amount: Some(amount as i32),
                                    });
                                }
                                offset += 6;
                            } else {
                                break;
                            }
                        }
                    }
                    
                    args.instruction_args = Some(instruction_args::InstructionArgs::AddLiquidityOneSidePrecise(PbAddLiquidityOneSidePreciseLayout {
                        bins,
                        decompress_multiplier: Some(decompress_multiplier),
                    }));
                },
                
                InstructionType::RemoveLiquidity2 => {
                    if data.len() < 32 { return None; }
                    
                    // Parse binLiquidityRemoval array for V2 format
                    let mut bin_liquidity_removal = Vec::new();
                    
                    if data.len() > 48 {
                        let bins_len = data[48] as usize;
                        let mut offset = 49;
                        
                        for _ in 0..bins_len {
                            if offset < data.len() {
                                let element_len = data[offset] as usize;
                                offset += 1;
                                
                                if offset + element_len <= data.len() {
                                    bin_liquidity_removal.push(data[offset..(offset + element_len)].to_vec());
                                    offset += element_len;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    
                    args.instruction_args = Some(instruction_args::InstructionArgs::RemoveLiquidity(PbRemoveLiquidityLayout {
                        tick_lower_index: parse_i32(data, 8).unwrap_or(0),
                        tick_upper_index: parse_i32(data, 12).unwrap_or(0),
                        liquidity_amount: parse_u128(data, 16).unwrap_or(0).to_string(),
                        token_min_a: parse_u64(data, 32).unwrap_or(0),
                        token_min_b: parse_u64(data, 40).unwrap_or(0),
                        bin_liquidity_removal,
                    }));
                },
                
                InstructionType::RemoveLiquidityByRange2 => {
                    if data.len() < 14 { return None; }
                    
                    args.instruction_args = Some(instruction_args::InstructionArgs::RemoveLiquidityByRange(PbRemoveLiquidityByRangeLayout {
                        from_bin_id: Some(parse_i32(data, 8).unwrap_or(0)),
                        to_bin_id: Some(parse_i32(data, 12).unwrap_or(0)),
                        bps_to_remove: Some(if data.len() >= 14 { parse_i16(data, 14).unwrap_or(0) as i32 } else { 0 }),
                    }));
                },
                
                _ => {
                    // Handle unexpected instruction types gracefully
                    log::warn!("Unexpected V2 instruction type in match: {:?}", inst_type);
                    return None;
                }
            }
        },
    }

    // Return Some(args) only if instruction_args is Some, otherwise None
    if args.instruction_args.is_some() {
        Some(args)
    } else {
        None
    }
}

// Helper function to parse a fixed-size byte slice into a PubKey string
fn bytes_to_pubkey_str(data: &[u8], offset: usize) -> Result<String, &'static str> {
    if offset + 32 > data.len() {
        return Err(r#"Data too short for PubKey"#);
    }
    Ok(bs58::encode(&data[offset..offset + 32]).into_string())
}

// Helper function to parse primitive types from byte slice
fn parse_u64(data: &[u8], offset: usize) -> Result<u64, &'static str> {
    if offset + 8 > data.len() { return Err(r#"Data too short for u64"#); }
    data[offset..offset+8].try_into().map(u64::from_le_bytes).map_err(|_| r#"Slice len mismatch for u64"#)
}
fn parse_i64(data: &[u8], offset: usize) -> Result<i64, &'static str> {
    if offset + 8 > data.len() { return Err(r#"Data too short for i64"#); }
    data[offset..offset+8].try_into().map(i64::from_le_bytes).map_err(|_| r#"Slice len mismatch for i64"#)
}
fn parse_i32(data: &[u8], offset: usize) -> Result<i32, &'static str> {
    if offset + 4 > data.len() { return Err(r#"Data too short for i32"#); }
    data[offset..offset+4].try_into().map(i32::from_le_bytes).map_err(|_| r#"Slice len mismatch for i32"#)
}
fn parse_u32(data: &[u8], offset: usize) -> Result<u32, &'static str> {
    if offset + 4 > data.len() { return Err(r#"Data too short for u32"#); }
    data[offset..offset+4].try_into().map(u32::from_le_bytes).map_err(|_| r#"Slice len mismatch for u32"#)
}
fn parse_i16(data: &[u8], offset: usize) -> Result<i16, &'static str> {
    if offset + 2 > data.len() { return Err(r#"Data too short for i16"#); }
    data[offset..offset+2].try_into().map(i16::from_le_bytes).map_err(|_| r#"Slice len mismatch for i16"#)
}
fn parse_u128(data: &[u8], offset: usize) -> Result<u128, &'static str> {
    if offset + 16 > data.len() { return Err(r#"Data too short for u128"#); }
    data[offset..offset+16].try_into().map(u128::from_le_bytes).map_err(|_| r#"Slice len mismatch for u128"#)
}
fn parse_u16(data: &[u8], offset: usize) -> Result<u16, &'static str> {
    if offset + 2 > data.len() { return Err(r#"Data too short for u16"#); }
    data[offset..offset+2].try_into().map(u16::from_le_bytes).map_err(|_| r#"Slice len mismatch for u16"#)
}

fn parse_event_wrapper<F, T>(
    event_data: &[u8],
    event_name: &'static str,
    parser: F,
    wrapper_constructor: fn(T) -> pb_event_log_wrapper::EventFields,
) -> Option<InstructionArgs>
where
    F: Fn(&[u8]) -> Result<T, &'static str>,
    T: std::fmt::Debug
{
    match parser(event_data) {
        Ok(fields) => {
            let wrapper = PbEventLogWrapper {
                event_name: event_name.to_string(),
                event_fields: Some(wrapper_constructor(fields)),
            };
            Some(InstructionArgs { instruction_args: Some(IArgs::EventLog(wrapper)) })
        }
        Err(e) => {
            log::info!(r#"Failed to parse {} event data: {}. Data len: {}"#, event_name, e, event_data.len());
            None
        }
    }
}

// Process event log function with proper implementation
fn process_event_log(data: &[u8], mut args: InstructionArgs) -> Option<InstructionArgs> {
    if data.len() < 8 {
        log::info!("Event log data too short to contain discriminator");
        return None;
    }

    let discriminator = &data[0..8];
    let event_data = &data[8..];
    
    // Create wrapper with default empty event name
    let mut event_wrapper = PbEventLogWrapper {
        event_name: "Unknown".to_string(),
        event_fields: None,
    };

    // Match event discriminator against known types
    if discriminator == EVENT_SWAP_DISCRIMINATOR {
        event_wrapper.event_name = "Swap".to_string();
        // Always create the struct, using defaults if data is short
        let fields = pb_event_log_wrapper::EventFields::SwapLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbSwapLogFields {
                lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                from: if event_data.len() >= 64 { bytes_to_pubkey_str(event_data, 32).unwrap_or_default() } else { "".to_string() },
                start_bin_id: Some(if event_data.len() >= 68 { parse_i32(event_data, 64).unwrap_or(0) } else { 0 }),
                end_bin_id: Some(if event_data.len() >= 72 { parse_i32(event_data, 68).unwrap_or(0) } else { 0 }),
                amount_in: Some(if event_data.len() >= 80 { parse_u64(event_data, 72).unwrap_or(0) } else { 0 }),
                amount_out: Some(if event_data.len() >= 84 { parse_u64(event_data, 80).unwrap_or(0) } else { 0 }),
                swap_for_y: Some(if event_data.len() >= 85 { event_data[84] != 0 } else { false }),
                fee: Some(if event_data.len() >= 93 { parse_u64(event_data, 85).unwrap_or(0) } else { 0 }),
                protocol_fee: Some(if event_data.len() >= 101 { parse_u64(event_data, 93).unwrap_or(0) } else { 0 }),
                fee_bps: if event_data.len() >= 105 { parse_u32(event_data, 101).unwrap_or(0).to_string() } else { "0".to_string() },
                host_fee: Some(if event_data.len() >= 113 { parse_u64(event_data, 105).unwrap_or(0) } else { 0 }),
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_ADD_LIQUIDITY_DISCRIMINATOR {
        event_wrapper.event_name = "AddLiquidity".to_string();
        let amounts = Vec::new(); // Add logic later if needed
        let fields = pb_event_log_wrapper::EventFields::AddLiquidityLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbAddLiquidityLogFields {
                lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                from: if event_data.len() >= 64 { bytes_to_pubkey_str(event_data, 32).unwrap_or_default() } else { "".to_string() },
                position: if event_data.len() >= 96 { bytes_to_pubkey_str(event_data, 64).unwrap_or_default() } else { "".to_string() },
                amounts: amounts, // Keep as potentially empty vec
                active_bin_id: Some(if event_data.len() >= 100 { parse_i32(event_data, 96).unwrap_or(0) } else { 0 }),
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_REMOVE_LIQUIDITY_DISCRIMINATOR {
        event_wrapper.event_name = "RemoveLiquidity".to_string();
        let amounts = Vec::new(); // Add logic later if needed
        let fields = pb_event_log_wrapper::EventFields::RemoveLiquidityLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbRemoveLiquidityLogFields {
                lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                from: if event_data.len() >= 64 { bytes_to_pubkey_str(event_data, 32).unwrap_or_default() } else { "".to_string() },
                position: if event_data.len() >= 96 { bytes_to_pubkey_str(event_data, 64).unwrap_or_default() } else { "".to_string() },
                amounts: amounts, // Keep as potentially empty vec
                active_bin_id: Some(if event_data.len() >= 100 { parse_i32(event_data, 96).unwrap_or(0) } else { 0 }),
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_CLAIM_REWARD_DISCRIMINATOR {
        event_wrapper.event_name = "ClaimReward".to_string();
        let fields = pb_event_log_wrapper::EventFields::ClaimRewardLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbClaimRewardLogFields {
                lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                position: if event_data.len() >= 64 { bytes_to_pubkey_str(event_data, 32).unwrap_or_default() } else { "".to_string() },
                owner: if event_data.len() >= 96 { bytes_to_pubkey_str(event_data, 64).unwrap_or_default() } else { "".to_string() },
                reward_index: Some(if event_data.len() >= 104 { parse_i64(event_data, 96).unwrap_or(0) } else { 0 }),
                total_reward: Some(if event_data.len() >= 112 { parse_i64(event_data, 104).unwrap_or(0) } else { 0 }),
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_FUND_REWARD_DISCRIMINATOR {
        event_wrapper.event_name = "FundReward".to_string();
        let fields = pb_event_log_wrapper::EventFields::FundRewardLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbFundRewardLogFields {
                lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                funder: if event_data.len() >= 64 { bytes_to_pubkey_str(event_data, 32).unwrap_or_default() } else { "".to_string() },
                reward_index: Some(if event_data.len() >= 72 { parse_i64(event_data, 64).unwrap_or(0) } else { 0 }),
                amount: Some(if event_data.len() >= 80 { parse_i64(event_data, 72).unwrap_or(0) } else { 0 }),
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_INITIALIZE_REWARD_DISCRIMINATOR {
        event_wrapper.event_name = "InitializeReward".to_string();
        let fields = pb_event_log_wrapper::EventFields::InitializeRewardLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbInitializeRewardLogFields {
                lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                reward_mint: if event_data.len() >= 64 { bytes_to_pubkey_str(event_data, 32).unwrap_or_default() } else { "".to_string() },
                funder: if event_data.len() >= 96 { bytes_to_pubkey_str(event_data, 64).unwrap_or_default() } else { "".to_string() },
                reward_index: Some(if event_data.len() >= 104 { parse_i64(event_data, 96).unwrap_or(0) } else { 0 }),
                reward_duration: Some(if event_data.len() >= 112 { parse_i64(event_data, 104).unwrap_or(0) } else { 0 }),
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_UPDATE_REWARD_DURATION_DISCRIMINATOR {
        event_wrapper.event_name = "UpdateRewardDuration".to_string();
        let fields = pb_event_log_wrapper::EventFields::UpdateRewardDurationLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbUpdateRewardDurationLogFields {
                lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                reward_index: Some(if event_data.len() >= 40 { parse_i64(event_data, 32).unwrap_or(0) } else { 0 }),
                old_reward_duration: Some(if event_data.len() >= 48 { parse_i64(event_data, 40).unwrap_or(0) } else { 0 }),
                new_reward_duration: Some(if event_data.len() >= 56 { parse_i64(event_data, 48).unwrap_or(0) } else { 0 }),
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_UPDATE_REWARD_FUNDER_DISCRIMINATOR {
        event_wrapper.event_name = "UpdateRewardFunder".to_string();
        let fields = pb_event_log_wrapper::EventFields::UpdateRewardFunderLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbUpdateRewardFunderLogFields {
                lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                reward_index: Some(if event_data.len() >= 40 { parse_i64(event_data, 32).unwrap_or(0) } else { 0 }),
                old_funder: if event_data.len() >= 72 { bytes_to_pubkey_str(event_data, 40).unwrap_or_default() } else { "".to_string() },
                new_funder: if event_data.len() >= 104 { bytes_to_pubkey_str(event_data, 72).unwrap_or_default() } else { "".to_string() },
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_POSITION_CLOSE_DISCRIMINATOR {
        event_wrapper.event_name = "PositionClose".to_string();
        let fields = pb_event_log_wrapper::EventFields::PositionCloseLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbPositionCloseLogFields {
                position: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                owner: if event_data.len() >= 64 { bytes_to_pubkey_str(event_data, 32).unwrap_or_default() } else { "".to_string() },
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_CLAIM_FEE_DISCRIMINATOR {
        event_wrapper.event_name = "ClaimFee".to_string();
        let fields = pb_event_log_wrapper::EventFields::ClaimFeeLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbClaimFeeLogFields {
                lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                position: if event_data.len() >= 64 { bytes_to_pubkey_str(event_data, 32).unwrap_or_default() } else { "".to_string() },
                owner: if event_data.len() >= 96 { bytes_to_pubkey_str(event_data, 64).unwrap_or_default() } else { "".to_string() },
                fee_x: Some(if event_data.len() >= 104 { parse_i64(event_data, 96).unwrap_or(0) } else { 0 }),
                fee_y: Some(if event_data.len() >= 112 { parse_i64(event_data, 104).unwrap_or(0) } else { 0 }),
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_LB_PAIR_CREATE_DISCRIMINATOR {
        event_wrapper.event_name = "LbPairCreate".to_string();
        let fields = pb_event_log_wrapper::EventFields::LbPairCreateLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbLbPairCreateLogFields {
                lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                bin_step: Some(if event_data.len() >= 36 { parse_i32(event_data, 32).unwrap_or(0) } else { 0 }),
                token_x: if event_data.len() >= 68 { bytes_to_pubkey_str(event_data, 36).unwrap_or_default() } else { "".to_string() },
                token_y: if event_data.len() >= 100 { bytes_to_pubkey_str(event_data, 68).unwrap_or_default() } else { "".to_string() }, // Corrected offset check
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_POSITION_CREATE_DISCRIMINATOR {
        event_wrapper.event_name = "PositionCreate".to_string();
        let fields = pb_event_log_wrapper::EventFields::PositionCreateLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbPositionCreateLogFields {
                lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                position: if event_data.len() >= 64 { bytes_to_pubkey_str(event_data, 32).unwrap_or_default() } else { "".to_string() },
                owner: if event_data.len() >= 96 { bytes_to_pubkey_str(event_data, 64).unwrap_or_default() } else { "".to_string() },
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_FEE_PARAMETER_UPDATE_DISCRIMINATOR {
        event_wrapper.event_name = "FeeParameterUpdate".to_string();
        let fields = pb_event_log_wrapper::EventFields::FeeParameterUpdateLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbFeeParameterUpdateLogFields {
                lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                protocol_share: Some(if event_data.len() >= 36 { parse_i32(event_data, 32).unwrap_or(0) } else { 0 }),
                base_factor: Some(if event_data.len() >= 40 { parse_i32(event_data, 36).unwrap_or(0) } else { 0 }),
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_INCREASE_OBSERVATION_DISCRIMINATOR {
        event_wrapper.event_name = "IncreaseObservation".to_string();
        let fields = pb_event_log_wrapper::EventFields::IncreaseObservationLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbIncreaseObservationLogFields {
                oracle: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                new_observation_length: Some(if event_data.len() >= 40 { parse_i64(event_data, 32).unwrap_or(0) } else { 0 }),
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_WITHDRAW_INELIGIBLE_REWARD_DISCRIMINATOR {
        event_wrapper.event_name = "WithdrawIneligibleReward".to_string();
        let fields = pb_event_log_wrapper::EventFields::WithdrawIneligibleRewardLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbWithdrawIneligibleRewardLogFields {
                lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                reward_mint: if event_data.len() >= 64 { bytes_to_pubkey_str(event_data, 32).unwrap_or_default() } else { "".to_string() },
                amount: Some(if event_data.len() >= 72 { parse_i64(event_data, 64).unwrap_or(0) } else { 0 }),
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_UPDATE_POSITION_OPERATOR_DISCRIMINATOR {
        event_wrapper.event_name = "UpdatePositionOperator".to_string();
        let fields = pb_event_log_wrapper::EventFields::UpdatePositionOperatorLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbUpdatePositionOperatorLogFields {
                position: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                old_operator: if event_data.len() >= 64 { bytes_to_pubkey_str(event_data, 32).unwrap_or_default() } else { "".to_string() },
                new_operator: if event_data.len() >= 96 { bytes_to_pubkey_str(event_data, 64).unwrap_or_default() } else { "".to_string() },
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_UPDATE_POSITION_LOCK_RELEASE_SLOT_DISCRIMINATOR {
        event_wrapper.event_name = "UpdatePositionLockReleaseSlot".to_string();
        let fields = pb_event_log_wrapper::EventFields::UpdatePositionLockReleaseSlotLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbUpdatePositionLockReleaseSlotLogFields {
                position: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                current_slot: Some(if event_data.len() >= 40 { parse_i64(event_data, 32).unwrap_or(0) } else { 0 }),
                new_lock_release_slot: Some(if event_data.len() >= 48 { parse_i64(event_data, 40).unwrap_or(0) } else { 0 }),
                old_lock_release_slot: Some(if event_data.len() >= 56 { parse_i64(event_data, 48).unwrap_or(0) } else { 0 }),
                sender: if event_data.len() >= 88 { bytes_to_pubkey_str(event_data, 56).unwrap_or_default() } else { "".to_string() },
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_GO_TO_A_BIN_DISCRIMINATOR {
        event_wrapper.event_name = "GoToABin".to_string();
        let fields = pb_event_log_wrapper::EventFields::GoToABinLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbGoToABinLogFields {
                lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                from_bin_id: Some(if event_data.len() >= 36 { parse_i32(event_data, 32).unwrap_or(0) } else { 0 }),
                to_bin_id: Some(if event_data.len() >= 40 { parse_i32(event_data, 36).unwrap_or(0) } else { 0 }),
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_UPDATE_POSITION_LOCK_RELEASE_POINT_DISCRIMINATOR {
        event_wrapper.event_name = "UpdatePositionLockReleasePoint".to_string();
        let fields = pb_event_log_wrapper::EventFields::UpdatePositionLockReleasePointLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbUpdatePositionLockReleasePointLogFields {
                position: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                current_point: Some(if event_data.len() >= 40 { parse_i64(event_data, 32).unwrap_or(0) } else { 0 }),
                new_lock_release_point: Some(if event_data.len() >= 48 { parse_i64(event_data, 40).unwrap_or(0) } else { 0 }),
                old_lock_release_point: Some(if event_data.len() >= 56 { parse_i64(event_data, 48).unwrap_or(0) } else { 0 }),
                sender: if event_data.len() >= 88 { bytes_to_pubkey_str(event_data, 56).unwrap_or_default() } else { "".to_string() },
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_UNKNOWN_EVENT1_DISCRIMINATOR {
        event_wrapper.event_name = "UnknownEvent1".to_string();
        let fields = pb_event_log_wrapper::EventFields::UnknownEvent1LogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbUnknownEvent1LogFields {
                vault: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                escrow: if event_data.len() >= 64 { bytes_to_pubkey_str(event_data, 32).unwrap_or_default() } else { "".to_string() },
                owner: if event_data.len() >= 96 { bytes_to_pubkey_str(event_data, 64).unwrap_or_default() } else { "".to_string() },
                amount: Some(if event_data.len() >= 104 { parse_i64(event_data, 96).unwrap_or(0) } else { 0 }),
                vault_total_claimed_token: Some(if event_data.len() >= 112 { parse_i64(event_data, 104).unwrap_or(0) } else { 0 }),
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_COMPOSITION_FEE_DISCRIMINATOR {
        event_wrapper.event_name = "CompositionFee".to_string();
        let fields = pb_event_log_wrapper::EventFields::CompositionFeeLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbCompositionFeeLogFields {
                from: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                bin_id: Some(if event_data.len() >= 36 { parse_i32(event_data, 32).unwrap_or(0) } else { 0 }),
                token_x_fee_amount: Some(if event_data.len() >= 44 { parse_u64(event_data, 36).unwrap_or(0) } else { 0 }),
                token_y_fee_amount: Some(if event_data.len() >= 52 { parse_u64(event_data, 44).unwrap_or(0) } else { 0 }),
                protocol_token_x_fee_amount: Some(if event_data.len() >= 60 { parse_u64(event_data, 52).unwrap_or(0) } else { 0 }),
                protocol_token_y_fee_amount: Some(if event_data.len() >= 68 { parse_u64(event_data, 60).unwrap_or(0) } else { 0 }),
            }
        );
        event_wrapper.event_fields = Some(fields);

    } else if discriminator == EVENT_INCREASE_POSITION_LENGTH_DISCRIMINATOR {
        event_wrapper.event_name = "IncreasePositionLength".to_string();
        let fields = pb_event_log_wrapper::EventFields::IncreasePositionLengthLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbIncreasePositionLengthLogFields {
                position: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                new_length: Some(if event_data.len() >= 40 { parse_u64(event_data, 32).unwrap_or(0) } else { 0 }),
            }
        );
        event_wrapper.event_fields = Some(fields);

        log::info!("Processing IncreasePositionLength event: position={}, new_length={}",
                   if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                   if event_data.len() >= 40 { parse_u64(event_data, 32).unwrap_or(0) } else { 0 });

    } else if discriminator == EVENT_DECREASE_POSITION_LENGTH_DISCRIMINATOR {
        event_wrapper.event_name = "DecreasePositionLength".to_string();
        let fields = pb_event_log_wrapper::EventFields::DecreasePositionLengthLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbDecreasePositionLengthLogFields {
                position: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                new_length: Some(if event_data.len() >= 40 { parse_u64(event_data, 32).unwrap_or(0) } else { 0 }),
            }
        );
        event_wrapper.event_fields = Some(fields);

        log::info!("Processing DecreasePositionLength event: position={}, new_length={}",
                   if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                   if event_data.len() >= 40 { parse_u64(event_data, 32).unwrap_or(0) } else { 0 });

    } else if discriminator == EVENT_DYNAMIC_FEE_PARAMETER_UPDATE_DISCRIMINATOR {
        event_wrapper.event_name = "DynamicFeeParameterUpdate".to_string();
        let fields = pb_event_log_wrapper::EventFields::DynamicFeeParameterUpdateLogFields(
            crate::pb::sf::solana::meteora_dlmm::v1::PbDynamicFeeParameterUpdateLogFields {
                lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                volatility_accumulator: Some(if event_data.len() >= 36 { parse_u32(event_data, 32).unwrap_or(0) } else { 0 }),
                volatility_reference: Some(if event_data.len() >= 40 { parse_u32(event_data, 36).unwrap_or(0) } else { 0 }),
                index_reference: Some(if event_data.len() >= 44 { parse_u32(event_data, 40).unwrap_or(0) } else { 0 }),
            }
        );
        event_wrapper.event_fields = Some(fields);

        log::info!("Processing DynamicFeeParameterUpdate event: lb_pair={}, volatility_accumulator={}",
                   if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                   if event_data.len() >= 36 { parse_u32(event_data, 32).unwrap_or(0) } else { 0 });

    } else {
        log::info!("Unknown event discriminator: {}", hex::encode(discriminator));
        event_wrapper.event_name = format!("Unknown_{}", hex::encode(discriminator));
        // Keep event_fields as None for unknown events
    }

    // Log that we identified an event only if fields were set
    if event_wrapper.event_fields.is_some() {
        log::info!("Identified event: {}", event_wrapper.event_name);
    }

    // Set the event wrapper as the instruction args
    args.instruction_args = Some(instruction_args::InstructionArgs::EventLog(event_wrapper));

    Some(args)
} 