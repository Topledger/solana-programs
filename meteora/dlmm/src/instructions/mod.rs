#[allow(unused_imports)]
use std::convert::TryInto;
use std::vec::Vec;
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
    PbClaimRewardLayout, PbClaimFeesLayout, PbClosePositionLayout, PbRemoveAllLiquidityLayout,
    PbTransferPositionOwnerLayout, PbRemoveLiquidityByRangeLayout, PbAddLiquidityOneSidePreciseLayout,
    PbGoToABinLayout, PbWithdrawIneligibleRewardLayout, PbUpdateFeesAndRewardsLayout, PbEventLogWrapper,
    pb_event_log_wrapper, PbLiquidityParameterLayout, PbInitializeLbPairLayout, PbInitializePermissionLbPairLayout,
    PbInitializeBinArrayLayout, PbInitializePresetParameterLayout, PbClosePresetParameterLayout, PbClosePresetParameter2Layout,
    PbCloseLbPairLayout, PbUpdateFeeParametersLayout, PbUpdateFeeOwnerLayout, PbTogglePairStatusLayout,
    PbUpdateWhitelistedWalletLayout, PbIncreaseOracleLengthLayout, PbInitializeBinArrayBitmapExtensionLayout,
    PbMigrateBinArrayLayout, PbSetActivationSlotLayout, PbSetMaxSwappedAmountLayout, PbSetPreActivationDurationLayout,
    PbSetPreActivationSwapAddressLayout, PbSetLockReleaseSlotLayout, PbInitializeCustomizablePermissionlessLbPairLayout,
    PbAddLiquidityByWeightLayout, PbAddLiquidityByStrategyLayout, PbAddLiquidityOneSideLayout,
    PbAddLiquidityByStrategyOneSideLayout, PbRemoveLiquiditySingleSideLayout, PbClaimLiquidityLayout,
    PbInitializePositionByOperatorLayout, PbMigratePositionLayout, PbIdlWriteLayout,
    PbBinLiquidityDistributionByWeightLayout, PbCompressedBinDepositAmountLayout,
    PbInitializeLbPair2Layout, PbClaimFee2Layout,
    PbRemainingAccountsInfo, PbRemainingAccountsSlice, PbAccountsType,
    PbInitializeTokenBadgeLayout, PbCreateClaimProtocolFeeOperatorLayout, 
    PbCloseClaimProtocolFeeOperatorLayout,
    PbInitializeCustomizablePermissionlessLbPair2Layout, PbAddLiquidity2Layout, 
    PbAddLiquidityByStrategy2Layout, 
    PbCustomizableParams, PbLiquidityParameter, PbBinLiquidityDistribution, 
    PbLiquidityParameterByStrategy, PbStrategyParameters, PbStrategyType,
    PbBinLiquidityReduction,
    PbAddLiquidityOneSidePrecise2Layout, PbRemoveLiquidity2Layout, 
    PbRemoveLiquidityByRange2Layout, PbAddLiquiditySingleSidePreciseParameter2,
    PbSwap2Layout, PbSwapExactOut2Layout, PbSwapWithPriceImpact2Layout,
    PbClosePosition2Layout, PbUpdateFeesAndReward2Layout, 
    PbClosePositionIfEmptyLayout, PbInitializePresetParameterV2Layout,
    PbInitPermissionPairIx,
    PbLiquidityParameterByStrategyOneSide, PbClaimReward2Layout,
    PbSetActivationPointLayout, PbLiquidityParameterByWeight
};

// For convenience, alias the instruction args enum
use crate::pb::sf::solana::meteora_dlmm::v1::instruction_args::InstructionArgs as IArgs;

// Meteora DLMM Program ID
const METEORA_DLMM_PROGRAM_ID: &str = "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo";

// Use a proper event log discriminator 
const EVENT_LOG_DISCRIMINATOR: &[u8] = &[228, 69, 165, 46, 81, 203, 154, 29];

// Enum representing different instruction types
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InstructionType {
    // Core Pool Operations
    InitializeLbPair, // IDL: initializeLbPair
    InitializePermissionLbPair, // IDL: initializePermissionLbPair
    InitializeBinArray, // IDL: initializeBinArray
    InitializePresetParameter, // IDL: initializePresetParameter
    InitializePresetParameter2, // IDL: initializePresetParameter2
    ClosePresetParameter, // IDL: closePresetParameter
    ClosePresetParameter2, // IDL: closePresetParameter2
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
    AddLiquidityByStrategyOneSide2, // NEW from IDL
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

// TODO: This array needs to be updated to match the InstructionType enum and IDL names exactly.
//       The order also matters for discriminator matching.
const INSTRUCTION_TYPES: &[(&str, InstructionType)] = &[
    ("initializeLbPair", InstructionType::InitializeLbPair),
    ("initializePermissionLbPair", InstructionType::InitializePermissionLbPair),
    ("initializePosition", InstructionType::InitializePosition),
    ("initializePositionPda", InstructionType::InitializePositionPda),
    ("closePosition", InstructionType::ClosePosition),
    ("claimFee", InstructionType::ClaimFee),
    ("claimReward", InstructionType::ClaimReward),
    ("swap", InstructionType::Swap),
    ("swapWithPriceImpact", InstructionType::SwapWithPriceImpact),
    ("swapExactOut", InstructionType::SwapExactOut),
    ("initializeBinArray", InstructionType::InitializeBinArray),
    ("initializePresetParameter", InstructionType::InitializePresetParameter),
    ("initializePresetParameter2", InstructionType::InitializePresetParameter2),
    ("closePresetParameter", InstructionType::ClosePresetParameter),
    ("closePresetParameter2", InstructionType::ClosePresetParameter2),
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
    ("addLiquidityByStrategyOneSide2", InstructionType::AddLiquidityByStrategyOneSide2),
    ("removeLiquidity2", InstructionType::RemoveLiquidity2),
    ("removeLiquidityByRange2", InstructionType::RemoveLiquidityByRange2),
    ("swap2", InstructionType::Swap2),
    ("swapExactOut2", InstructionType::SwapExactOut2),
    ("swapWithPriceImpact2", InstructionType::SwapWithPriceImpact2),
    ("closePosition2", InstructionType::ClosePosition2),
    ("updateFeesAndReward2", InstructionType::UpdateFeesAndRewards),
    ("closePositionIfEmpty", InstructionType::ClosePositionIfEmpty),

    // Special case
    ("eventLog", InstructionType::EventLog),
];

/// Compute an 8-byte discriminator from a string by hashing its bytes and taking the first 8 bytes
pub fn compute_discriminator(name: &str) -> [u8; 8] {
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
        InstructionType::ClosePresetParameter2 => "ClosePresetParameter2",
        InstructionType::InitializePresetParameter => "InitializePresetParameter",
        InstructionType::InitializePresetParameter2 => "InitializePresetParameter2",
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
        InstructionType::AddLiquidityByStrategyOneSide2 => "addLiquidityByStrategyOneSide2",
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
    outer_instruction_index: u32, // Renamed: Represents the top-level instruction index
    is_inner_instruction: bool,
    actual_inner_index: Option<u32>, // Renamed: Represents the index within the inner block, if applicable
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
            // Check if it's an event log instruction
            if data.len() >= 8 && &data[0..8] == EVENT_LOG_DISCRIMINATOR {
                log::info!("Found EventLog instruction in process_instruction");
                "EventLog" // Use "EventLog" as instruction type for event logs
            } else {
                log::info!("Unknown instruction discriminator: {}", hex::encode(discriminator));
                return None;
            }
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
        instruction_index: Some(outer_instruction_index), // This now correctly gets the outer index
        is_inner_instruction: Some(is_inner_instruction),     // Wrap in Some
        inner_instruction_index: Some(actual_inner_index.unwrap_or(0)), // Explicitly wrap, using 0 if None
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
            // Using the full data here - process_event_log will handle the second discriminator
            log::info!("Found EventLog instruction with discriminator: {}", hex::encode(&data[0..8]));
            return process_event_log(data, InstructionArgs {
                instruction_args: Some(instruction_args::InstructionArgs::EventLog(PbEventLogWrapper {
                    event_name: "EventLog".to_string(),
                    event_fields: None,
                }))
            });
        }
        return None;
    }

    let inst_type = inst_type_opt.unwrap();
    let inst_name = get_instruction_type_str(inst_type);


    // Parse based on instruction type
    match inst_type {
        // Core Pool Operations
        InstructionType::InitializeLbPair => {
            // Check length: 8 bytes (discriminator) + 4 bytes (activeId) + 2 bytes (binStep) = 14 bytes
            if data.len() < 14 {
                log::warn!("Data too short for InitializeLbPair: {} bytes, expected at least 14", data.len());
                return None;
            }
            log::info!("Processing InitializeLbPair. Data (len {}): {}", data.len(), hex::encode(data));

            let active_id_res = parse_i32(data, 8); // Read 4 bytes from offset 8
            let bin_step_res = parse_u16(data, 12); // Read 2 bytes from offset 12 (8 + 4)

            log::info!("Parsed InitializeLbPair: active_id={:?}, bin_step={:?}", active_id_res, bin_step_res);

            // Use direct fields in layout
            args.instruction_args = Some(instruction_args::InstructionArgs::InitializeLbPair(PbInitializeLbPairLayout {
                active_id: active_id_res.ok(), // Assign Option<i32>
                bin_step: bin_step_res.ok().map(|v| v as u32), // Assign Option<u16> -> Option<u32>
            }));
        },
        InstructionType::InitializePermissionLbPair => {
            if data.len() >= 29 { // Adjusted size: 8(discriminator) + 4(i32) + 2(u16) + 2(u16) + 4(i32) + 4(i32) + 8(u64) + 1(u8) = 33
                let active_id = parse_i32(data, 8).unwrap_or_default();
                
                // Parse u16 fields (2 bytes each)
                let bin_step = parse_u16(data, 12).unwrap_or_default();
                let base_factor = parse_u16(data, 14).unwrap_or_default();
                
                // Parse i32 fields
                let min_bin_id = parse_i32(data, 16).unwrap_or_default();
                let max_bin_id = parse_i32(data, 20).unwrap_or_default();
                
                // Parse u64 field
                let lock_duration = parse_u64(data, 24).unwrap_or_default();
                
                // Parse u8 field (1 byte)
                let activation_type = if data.len() > 32 { data[32] } else { 0 };

                // Create nested struct to match proto structure
                let ix_data = crate::pb::sf::solana::meteora_dlmm::v1::PbInitPermissionPairIx {
                    active_id: Some(active_id),
                    bin_step: Some(bin_step as u32), // Convert u16 to u32
                    base_factor: Some(base_factor as u32), // Convert u16 to u32
                    min_bin_id: Some(min_bin_id),
                    max_bin_id: Some(max_bin_id),
                    lock_duration: Some(lock_duration),
                    activation_type: Some(activation_type as u32), // Convert u8 to u32
                };

                // Create parent struct and assign nested struct
                let inst_args = crate::pb::sf::solana::meteora_dlmm::v1::PbInitializePermissionLbPairLayout {
                    ix_data: Some(ix_data),
                };
                
                args.instruction_args = Some(
                    crate::pb::sf::solana::meteora_dlmm::v1::instruction_args::InstructionArgs::InitializePermissionLbPair(
                        inst_args
                    ),
                );
            }
        },
        InstructionType::InitializeBinArray => {
            if data.len() < 16 { return None; }
            args.instruction_args = Some(instruction_args::InstructionArgs::InitializeBinArray(PbInitializeBinArrayLayout {
                index: Some(parse_i64(data, 8).unwrap_or(0)),
            }));
        },
        InstructionType::InitializePresetParameter => {
            if data.len() < 36 { return None; }
            args.instruction_args = Some(instruction_args::InstructionArgs::InitializePresetParameter(PbInitializePresetParameterLayout {
                // 2-byte fields (Int16ul in Python)
                bin_step: Some(parse_u16(data, 8).unwrap_or(0) as u32),
                base_factor: Some(parse_u16(data, 10).unwrap_or(0) as u32),
                filter_period: Some(parse_u16(data, 12).unwrap_or(0) as u32),
                decay_period: Some(parse_u16(data, 14).unwrap_or(0) as u32),
                reduction_factor: Some(parse_u16(data, 16).unwrap_or(0) as u32),
                // 4-byte fields (Int32ul in Python)
                variable_fee_control: Some(parse_u32(data, 18).unwrap_or(0)),
                max_volatility_accumulator: Some(parse_u32(data, 22).unwrap_or(0)),
                // 4-byte signed fields (Int32sl in Python)
                min_bin_id: Some(parse_i32(data, 26).unwrap_or(0)),
                max_bin_id: Some(parse_i32(data, 30).unwrap_or(0)),
                // 2-byte field (Int16ul in Python)
                protocol_share: Some(parse_u16(data, 34).unwrap_or(0) as u32),
            }));
        },
        InstructionType::InitializePresetParameter2 => {
            if data.len() < 31 { return None; }
            args.instruction_args = Some(instruction_args::InstructionArgs::InitializePresetParameterV2(PbInitializePresetParameterV2Layout {
                // First field is index (2-byte)
                index: Some(parse_u16(data, 8).unwrap_or(0) as u32),
                // 2-byte fields
                bin_step: Some(parse_u16(data, 10).unwrap_or(0) as u32),
                base_factor: Some(parse_u16(data, 12).unwrap_or(0) as u32),
                filter_period: Some(parse_u16(data, 14).unwrap_or(0) as u32),
                decay_period: Some(parse_u16(data, 16).unwrap_or(0) as u32),
                reduction_factor: Some(parse_u16(data, 18).unwrap_or(0) as u32),
                // 4-byte fields
                variable_fee_control: Some(parse_u32(data, 20).unwrap_or(0)),
                max_volatility_accumulator: Some(parse_u32(data, 24).unwrap_or(0)),
                // 2-byte field
                protocol_share: Some(parse_u16(data, 28).unwrap_or(0) as u32),
                // 1-byte field
                base_fee_power_factor: Some(if data.len() > 30 { data[30] as u32 } else { 0 }),
            }));
        },
        InstructionType::ClosePresetParameter => {
            // No arguments needed
            args.instruction_args = Some(instruction_args::InstructionArgs::ClosePresetParameter(PbClosePresetParameterLayout {}));
        },
        InstructionType::ClosePresetParameter2 => {
            // No arguments needed
            args.instruction_args = Some(instruction_args::InstructionArgs::ClosePresetParameter(PbClosePresetParameterLayout {}));
        },
        InstructionType::CloseLbPair => {
            // No arguments needed
            args.instruction_args = Some(instruction_args::InstructionArgs::CloseLbPair(PbCloseLbPairLayout {}));
        },
        InstructionType::UpdateFeeParameters => {
            // Args: feeParameter { protocolShare (u16), baseFactor (u16) }
            // Check length: 8 (disc) + 2 (protocolShare) + 2 (baseFactor) = 12 bytes
            if data.len() < 12 { return None; } 
            
            // Parse nested fields
            let protocol_share_res = parse_u16(data, 8); // Int16ul -> u16
            let base_factor_res = parse_u16(data, 10); // Int16ul -> u16

            // Create nested struct
            let fee_parameter = crate::pb::sf::solana::meteora_dlmm::v1::PbFeeParameterLayout {
                protocol_share: protocol_share_res.ok().map(|v| v as u32), // Cast u16 to u32
                base_factor: base_factor_res.ok().map(|v| v as u32), // Cast u16 to u32
            };

            args.instruction_args = Some(instruction_args::InstructionArgs::UpdateFeeParameters(PbUpdateFeeParametersLayout {
                fee_parameter: Some(fee_parameter), // Assign nested struct
            }));
        },
        InstructionType::UpdateFeeOwner => {
            // No arguments needed
            args.instruction_args = Some(instruction_args::InstructionArgs::UpdateFeeOwner(PbUpdateFeeOwnerLayout {}));
        },
        InstructionType::TogglePairStatus => {
            // No arguments for this instruction
            args.instruction_args = Some(instruction_args::InstructionArgs::TogglePairStatus(PbTogglePairStatusLayout {}));
        },
        InstructionType::UpdateWhitelistedWallet => {
            // Args: idx (u8), wallet (PubKey)
            // Check length: 8 (disc) + 1 (idx) + 32 (wallet) = 41 bytes
            if data.len() < 41 { return None; } 
            
            let idx_res = parse_u8(data, 8); // Int8ul -> u8
            let wallet_res = bytes_to_pubkey_str(data, 9); // Pubkey starts at offset 9 (8 + 1)

            args.instruction_args = Some(instruction_args::InstructionArgs::UpdateWhitelistedWallet(PbUpdateWhitelistedWalletLayout {
                // Proto field is i32, cast from u8
                idx: idx_res.ok().map(|v| v as i32), 
                wallet: wallet_res.ok(), // Assign Option<String>
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
            // Args: activationSlot (u64)
            if data.len() < 16 { return None; } // 8 disc + 8 u64
            let activation_slot_opt = parse_u64(data, 8).ok(); // Use parse_u64
            args.instruction_args = Some(instruction_args::InstructionArgs::SetActivationSlot(PbSetActivationSlotLayout {
                activation_slot: activation_slot_opt, // Assign Option<u64>
            }));
        },
        InstructionType::SetMaxSwappedAmount => {
            // Args: swapCapDeactivateSlot (u64), maxSwappedAmount (u64)
            if data.len() < 24 { return None; } // 8 bytes disc + 8 bytes u64 + 8 bytes u64
            let swap_cap_deactivate_slot_opt = parse_u64(data, 8).ok();
            let max_swapped_amount_opt = parse_u64(data, 16).ok();

            args.instruction_args = Some(instruction_args::InstructionArgs::SetMaxSwappedAmount(PbSetMaxSwappedAmountLayout {
                swap_cap_deactivate_slot: swap_cap_deactivate_slot_opt,
                max_swapped_amount: max_swapped_amount_opt,
            }));
        },
        InstructionType::SetPreActivationDuration => {
            // Args: preActivationDuration (u64)
            if data.len() < 16 { return None; } // 8 bytes disc + 8 bytes u64
            let pre_activation_duration_opt = parse_u64(data, 8).ok();

            args.instruction_args = Some(instruction_args::InstructionArgs::SetPreActivationDuration(PbSetPreActivationDurationLayout {
                pre_activation_duration: pre_activation_duration_opt,
            }));
        },
        InstructionType::SetPreActivationSwapAddress => {
            if data.len() < 40 { return None; } // 8 disc + 32 pubkey
            args.instruction_args = Some(instruction_args::InstructionArgs::SetPreActivationSwapAddress(PbSetPreActivationSwapAddressLayout {
                pre_activation_swap_address: Some(bytes_to_pubkey_str(data, 8).unwrap_or_default()),
            }));
        },
        InstructionType::SetLockReleaseSlot => {
            // Args: newLockReleaseSlot (u64)
            if data.len() < 16 { return None; } // 8 bytes disc + 8 bytes u64
            let new_lock_release_slot_opt = parse_u64(data, 8).ok();

            args.instruction_args = Some(instruction_args::InstructionArgs::SetLockReleaseSlot(PbSetLockReleaseSlotLayout {
                new_lock_release_slot: new_lock_release_slot_opt,
            }));
        },
        InstructionType::WithdrawProtocolFee => {
            // Args: amountX (u64), amountY (u64), remainingAccountsInfo (Optional<...>) 
            let mut current_offset = 8;
            // Check length for base args: 8 disc + 8 amountX + 8 amountY = 24
            if data.len() < 24 { return None; } 
            
            let amount_x_opt = parse_u64(data, current_offset).ok();
            current_offset += 8;
            let amount_y_opt = parse_u64(data, current_offset).ok();
            current_offset += 8;

            // Parse the optional RemainingAccountsInfo
            let remaining_accounts = parse_remaining_accounts_info(data, current_offset);
            
            args.instruction_args = Some(instruction_args::InstructionArgs::WithdrawProtocolFee(PbWithdrawProtocolFeeLayout {
                amount_x: amount_x_opt,
                amount_y: amount_y_opt,
                remaining_accounts_info: remaining_accounts, // Assign parsed info
            }));
        },
        InstructionType::InitializeCustomizablePermissionlessLbPair => {
            // Args: params: CustomizableParams
            // Total size = 8 (disc) + 83 (params) = 91 bytes minimum
            let mut current_offset = 8;
            let params_size = 83; // Calculated size of CustomizableParams
            if data.len() < current_offset + params_size { 
                log::warn!("Data too short for InitializeCustomizablePermissionlessLbPair params: {} bytes, expected at least {}", data.len() - current_offset, params_size);
                return None; 
            }
            
            // Parse CustomizableParams fields sequentially
            let active_id_opt = parse_i32(data, current_offset).ok();
            current_offset += 4;
            let bin_step_opt = parse_u16(data, current_offset).ok().map(|v| v as u32); // Map u16 to u32
            current_offset += 2;
            let base_factor_opt = parse_u16(data, current_offset).ok().map(|v| v as u32); // Map u16 to u32
            current_offset += 2;

            let activation_type = data[current_offset]; // u8
            current_offset += 1;
            let has_alpha_vault = data[current_offset] != 0; // bool
            current_offset += 1;

            // Parse Option<u64> activation_point
            let activation_point_present = data[current_offset] != 0;
            let activation_point = if activation_point_present { parse_u64(data, current_offset + 1).ok() } else { None };
            current_offset += 9; // 1 byte discriminant + 8 bytes value

            let creator_pool_on_off_control = data[current_offset] != 0; // bool
            current_offset += 1;
            let base_fee_power_factor = data[current_offset]; // u8
            current_offset += 1;

            // Extract padding bytes but don't store them explicitly unless needed
            let padding_bytes = data[current_offset..(current_offset + 62)].to_vec();
            // current_offset += 62; // Update offset if needed

            let padding_numeric = padding_bytes.iter().map(|&b| b as u32).collect::<Vec<u32>>();
            // current_offset += 62; // Update offset if needed

            // Construct the PbCustomizableParams struct
            let params = PbCustomizableParams {
                active_id: active_id_opt,
                bin_step: bin_step_opt,
                base_factor: base_factor_opt,
                activation_type: Some(activation_type as u32), // Cast u8 to u32
                has_alpha_vault: Some(has_alpha_vault),
                activation_point: activation_point, 
                creator_pool_on_off_control: Some(creator_pool_on_off_control),
                base_fee_power_factor: Some(base_fee_power_factor as u32), // Cast u8 to u32
                padding: padding_numeric.clone(), // Assign numeric Vec<u32> to the final padding field
            };
            
            // Assign the parsed params to the correct layout
            args.instruction_args = Some(instruction_args::InstructionArgs::InitializeCustomizablePermissionlessLbPair(
                PbInitializeCustomizablePermissionlessLbPairLayout {
                    params: Some(params), // Use the nested params struct
                }
            ));
        },

        // Liquidity Operations
        InstructionType::AddLiquidity => {
            // Args: liquidityParameter: LiquidityParameterLayout
            // LiquidityParameterLayout: amountX(u64), amountY(u64), binLiquidityDist(Vec<BinLiquidityDistributionLayout>)
            // Offsets: disc(8), amountX(8), amountY(8), vecLen(4) -> 8+8+8+4 = 28 bytes minimum before vector data
            let mut current_offset = 8;
            if data.len() < 28 { 
                log::warn!("Data too short for AddLiquidity base args: {} bytes, expected 28", data.len());
                return None; 
            }
            
            let amount_x_opt = parse_u64(data, current_offset).ok();
            current_offset += 8;
            let amount_y_opt = parse_u64(data, current_offset).ok();
            current_offset += 8;

            // Parse the vector
            let (bin_dist_vec, _next_offset) = parse_bin_liquidity_distribution_vec(data, current_offset);

            // Create the parameter struct
            let liquidity_parameter = PbLiquidityParameter {
                amount_x: amount_x_opt,
                amount_y: amount_y_opt,
                bin_liquidity_dist: bin_dist_vec,
            };
            
            args.instruction_args = Some(instruction_args::InstructionArgs::AddLiquidity(PbAddLiquidityLayout {
                liquidity_parameter: Some(liquidity_parameter),
            }));
        },
        InstructionType::AddLiquidityByWeight => {
            // Args: liquidityParameter: LiquidityParameterByWeight
            // LiquidityParameterByWeight: amountX(u64), amountY(u64), activeId(i32), maxActiveBinSlippage(i32), binLiquidityDist(Vec<BinLiquidityDistributionByWeight>)
            // Offsets: disc(8), amountX(8), amountY(8), activeId(4), maxSlip(4), vecLen(4) -> 8+8+8+4+4+4 = 36 bytes minimum before vector data
            let mut current_offset = 8;
            if data.len() < 36 {
                log::warn!("Data too short for AddLiquidityByWeight base args: {} bytes, expected 36", data.len());
                return None;
            }

            let amount_x_opt = parse_u64(data, current_offset).ok();
            current_offset += 8;
            let amount_y_opt = parse_u64(data, current_offset).ok();
            current_offset += 8;
            let active_id_opt = parse_i32(data, current_offset).ok();
            current_offset += 4;
            let max_active_bin_slippage_opt = parse_i32(data, current_offset).ok();
            current_offset += 4;

            // Parse the vector
            let (bin_dist_vec, _next_offset) = parse_bin_liquidity_dist_by_weight_vec(data, current_offset);

            // Create the parameter struct
            let liquidity_parameter = PbLiquidityParameterByWeight {
                amount_x: amount_x_opt,
                amount_y: amount_y_opt,
                active_id: active_id_opt,
                max_active_bin_slippage: max_active_bin_slippage_opt,
                bin_liquidity_dist: bin_dist_vec,
            };

            args.instruction_args = Some(instruction_args::InstructionArgs::AddLiquidityByWeight(PbAddLiquidityByWeightLayout {
                 liquidity_parameter: Some(liquidity_parameter),
            }));
        },
        InstructionType::AddLiquidityByStrategy => {
            // Args: liquidityParameter: LiquidityParameterByStrategy, remainingAccountsInfo: Optional<RemainingAccountsInfo>
            let mut current_offset = 8;
            // Placeholder check - Needs correct length based on actual LiquidityParameterByStrategy structure if fully parsed
            // Check for base + strategy + params = 8 + 8 + 4 + 4 + 4 + 4 + 1 + 64 = 97 bytes minimum after discriminator
            if data.len() < 8 + 97 { 
                log::warn!("Data potentially too short for full AddLiquidityByStrategy parsing: {} bytes", data.len());
                return None; 
            }

            // Assuming PbLiquidityParameterByStrategy structure for parsing
            let amount_x_res = parse_u64(data, current_offset);
            current_offset += 8;
            let amount_y_res = parse_u64(data, current_offset);
            current_offset += 8;
            let active_id_res = parse_i32(data, current_offset);
            current_offset += 4;
            let max_active_bin_slippage_res = parse_i32(data, current_offset);
            current_offset += 4;

            // Parse StrategyParameters
            // Length check already done above
            let min_bin_id_res = parse_i32(data, current_offset);
            let max_bin_id_res = parse_i32(data, current_offset + 4);
            let strategy_type_byte = data[current_offset + 8];
            let parameters_bytes = data[(current_offset + 9)..(current_offset + 9 + 64)].to_vec();
            let parameters_numeric = parameters_bytes.iter().map(|&b| b as u32).collect::<Vec<u32>>(); // Create numeric vec

            let strategy_type = match strategy_type_byte {
                 0 => Some(PbStrategyType::SpotOneSide as i32),
                 1 => Some(PbStrategyType::CurveOneSide as i32),
                 2 => Some(PbStrategyType::BidAskOneSide as i32),
                 3 => Some(PbStrategyType::SpotBalanced as i32),
                 4 => Some(PbStrategyType::CurveBalanced as i32),
                 5 => Some(PbStrategyType::BidAskBalanced as i32),
                 6 => Some(PbStrategyType::SpotImbalanced as i32),
                 7 => Some(PbStrategyType::CurveImbalanced as i32),
                 8 => Some(PbStrategyType::BidAskImbalanced as i32),
                _ => { log::warn!("Unknown strategy type byte: {}", strategy_type_byte); None },
            };

            let strategy_parameters = PbStrategyParameters {
                min_bin_id: min_bin_id_res.ok(),
                max_bin_id: max_bin_id_res.ok(),
                strategy_type: strategy_type,
                parameters: parameters_numeric, // Assign Vec<u32> to the 'parameters' field
            };

            // Create the correct liquidity parameter struct now
            let liquidity_parameter = PbLiquidityParameterByStrategy {
                amount_x: amount_x_res.ok(),
                amount_y: amount_y_res.ok(),
                active_id: active_id_res.ok(),
                max_active_bin_slippage: max_active_bin_slippage_res.ok(),
                strategy_parameters: Some(strategy_parameters),
            };

            // Assign the correctly parsed liquidity parameter
            args.instruction_args = Some(instruction_args::InstructionArgs::AddLiquidityByStrategy(PbAddLiquidityByStrategyLayout {
                 liquidity_parameter: Some(liquidity_parameter),
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
                                bin_id: if bin_id == 0 { None } else { Some(bin_id) },
                                weight: if weight == 0 { None } else { Some(weight as u32) }, // Cast u16 to u32
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
            // Args: parameter: LiquidityParameterByStrategyOneSide
            // LiquidityParameterByStrategyOneSide: { amount: u64, activeId: i32, maxActiveBinSlippage: i32, strategyParameters: StrategyParameters }
            // StrategyParameters: { minBinId: i32, maxBinId: i32, strategyType: StrategyType, parameters: [u8; 64] }
            // Updated length check: 8(disc) + 8(amount) + 4(activeId) + 4(maxSlip) + 4(minBin) + 4(maxBin) + 1(type) + 64(params) = 97
            if data.len() < 97 { return None; }

            let amount_res = parse_u64(data, 8);
            let active_id = parse_i32(data, 16).unwrap_or(0);
            let max_active_bin_slippage = parse_i32(data, 20).unwrap_or(0);
            let min_bin_id = parse_i32(data, 24).unwrap_or(0);
            let max_bin_id = parse_i32(data, 28).unwrap_or(0);
            let strategy_type_byte = data[32]; // u8
            let parameters_bytes = data[33..97].to_vec(); // Extract the 64 bytes for parameters
            let parameters_numeric = parameters_bytes.iter().map(|&b| b as u32).collect::<Vec<u32>>(); // Create numeric vec


            // The following code parses the correct parameters, now that the proto should be fixed.
            let strategy_parameters = PbStrategyParameters {
                min_bin_id: Some(min_bin_id),
                max_bin_id: Some(max_bin_id),
                strategy_type: Some(strategy_type_byte as i32), // Cast u8 to i32 for enum
                parameters: parameters_numeric, // Assign Vec<u32> to the 'parameters' field
            };

            let correct_liquidity_parameter = PbLiquidityParameterByStrategyOneSide {
                amount: amount_res.ok(), // Assign Option<u64>
                active_id: Some(active_id),
                max_active_bin_slippage: Some(max_active_bin_slippage),
                strategy_parameters: Some(strategy_parameters),
            };

            args.instruction_args = Some(instruction_args::InstructionArgs::AddLiquidityByStrategyOneSide(PbAddLiquidityByStrategyOneSideLayout {
                liquidity_parameter: Some(correct_liquidity_parameter), // Assign the correct parameter type
            }));
        },
        InstructionType::AddLiquidityOneSidePrecise => {
            // Args: bins: Vec<CompressedBinDepositAmount>, decompressMultiplier: u64
            let mut current_offset = 8;
            let (bins, next_offset) = parse_compressed_bin_deposit_vec(data, current_offset);
            current_offset = next_offset;

            if data.len() < current_offset + 8 { return None; } // decompress_multiplier (u64)
            let decompress_multiplier_opt = parse_u64(data, current_offset).ok();

            args.instruction_args = Some(IArgs::AddLiquidityOneSidePrecise(PbAddLiquidityOneSidePreciseLayout {
                bins,
                decompress_multiplier: decompress_multiplier_opt,
            }));
        },
        InstructionType::RemoveLiquidity => {
            // Args: binLiquidityRemoval: Vec<BinLiquidityReduction>
            let mut current_offset = 8; // Start after discriminator
            
            // Check for vector length (at least 4 bytes)
            if data.len() < current_offset + 4 { 
                log::warn!("Data too short for RemoveLiquidity vector length: {} bytes", data.len());
                return None; 
            }

            // Parse the vector of BinLiquidityReduction directly
            let (reductions, _next_offset) = parse_bin_liquidity_reduction_vec(data, current_offset);

            args.instruction_args = Some(IArgs::RemoveLiquidity(PbRemoveLiquidityLayout {
                // Only include the parsed vector
                bin_liquidity_removal: reductions,
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
            args.instruction_args = Some(instruction_args::InstructionArgs::ClaimLiquidity(PbClaimLiquidityLayout {}));
        },
        InstructionType::ClaimFee => {
            // No arguments needed for V1 ClaimFee, but assign the empty struct to ensure "args" key appears.
            args.instruction_args = Some(instruction_args::InstructionArgs::ClaimFees(PbClaimFeesLayout {}));
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
            // Args: rewardIndex (u64), rewardDuration (u64), funder (publicKey)
            // Offsets: disc(8) + idx(8) + duration(8) + funder(32) = 56 bytes total needed
            if data.len() < 56 { 
                log::warn!("Data too short for InitializeReward: {} bytes", data.len());
                return None; 
            }
            
            // Use .ok() to map Ok -> Some, Err -> None
            let reward_index_opt = parse_u64(data, 8).ok();
            let reward_duration_opt = parse_u64(data, 16).ok();
            let funder_opt = bytes_to_pubkey_str(data, 24).ok(); // .ok() converts Result to Option

            args.instruction_args = Some(instruction_args::InstructionArgs::InitializeReward(PbInitializeRewardLayout {
                // Update fields to match proto and parsed values
                reward_index: reward_index_opt, // Assuming proto has reward_index: Option<u64>
                reward_duration: reward_duration_opt, // Assuming proto has reward_duration: Option<u64>
                funder: funder_opt, // Assuming proto has funder: Option<String>
                // Remove fields not present in JSON args
                // emissions_per_second_x64: None, 
                // open_time: None, 
                // end_time: None,
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
            if data.len() < 16 { return None; } // 8 bytes disc + 8 bytes u64
            
            args.instruction_args = Some(instruction_args::InstructionArgs::ClaimReward(PbClaimRewardLayout {
                reward_index: Some(parse_u64(data, 8).unwrap_or(0)), // Use u64 parser
            }));
        },

        InstructionType::ClaimReward2 => {
            // Args: rewardIndex: u64, minBinId: i32, maxBinId: i32, remainingAccountsInfo: RemainingAccountsInfo
            let mut current_offset = 8;
            if data.len() < current_offset + 16 { // Need 8 for u64, 4 for i32, 4 for i32
                log::warn!("Data too short for ClaimReward2 base args: {} bytes", data.len());
                return None;
            }

            let reward_index_opt = parse_u64(data, current_offset).ok();
            current_offset += 8;
            let min_bin_id_opt = parse_i32(data, current_offset).ok();
            current_offset += 4;
            let max_bin_id_opt = parse_i32(data, current_offset).ok();
            current_offset += 4;

            let remaining_accounts = parse_remaining_accounts_info(data, current_offset);

            args.instruction_args = Some(IArgs::ClaimReward2(PbClaimReward2Layout {
                reward_index: reward_index_opt, // Assign the Option<u64> directly
                min_bin_id: min_bin_id_opt,
                max_bin_id: max_bin_id_opt,
                remaining_accounts_info: remaining_accounts,
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
            // Args: activationPoint (u64)
            if data.len() < 16 { return None; } // 8 bytes disc + 8 bytes u64
            let activation_point_opt = parse_u64(data, 8).ok();

            args.instruction_args = Some(IArgs::SetActivationPoint(PbSetActivationPointLayout {
                activation_point: activation_point_opt,
            }));
        },
        InstructionType::UpdateFeesAndRewards => {
            // No arguments needed
            args.instruction_args = Some(instruction_args::InstructionArgs::UpdateFeesAndRewards(PbUpdateFeesAndRewardsLayout {}));
        },

        // V2 Instructions (require more details from IDL - stubbed for now with empty args)
        InstructionType::InitializeLbPair2 => {
            const PADDING_LEN: usize = 96;
            const MIN_LEN: usize = 8 + 4 + PADDING_LEN; // discriminator + active_id + padding
            if data.len() < MIN_LEN {
                log::warn!("Data too short for InitializeLbPair2 (needs {} bytes): {} bytes found", MIN_LEN, data.len());
                return None;
            }
            
            let active_id_opt = parse_i32(data, 8).ok();
            
            let padding_start_offset = 12;
            let padding_bytes = &data[padding_start_offset..(padding_start_offset + PADDING_LEN)];
            let padding_numeric = padding_bytes.iter().map(|&b| b as u32).collect::<Vec<u32>>();
            
            args.instruction_args = Some(IArgs::InitializeLbPair2(PbInitializeLbPair2Layout {
                active_id: active_id_opt,
                padding: padding_numeric,
            }));
        },

        InstructionType::ClaimFee2 => {
            if data.len() < 20 { // 8 disc + 4 min_bin + 4 max_bin + 4 vec_len
                 log::warn!("Data too short for ClaimFee2 base args: {} bytes", data.len());
                 return None;
            }
            // Use .ok() for optional fields
            let min_bin_id_opt = parse_i32(data, 8).ok();
            let max_bin_id_opt = parse_i32(data, 12).ok();

            let mut parsed_slices = Vec::new();
            let slices_len_result = parse_u32(data, 16);
            let mut current_offset = 20;

            if let Ok(slices_len) = slices_len_result {
                for _ in 0..slices_len {
                    if data.len() < current_offset + 2 { // Need 1 byte for type, 1 byte for len
                        log::warn!("Data too short for RemainingAccountsSlice at offset {}: {} bytes", current_offset, data.len());
                        break;
                    }
                    let accounts_type_byte = data[current_offset];
                    let length_byte = data[current_offset + 1];

                    let accounts_type = match accounts_type_byte {
                        0 => Some(PbAccountsType::TransferHookX as i32),
                        1 => Some(PbAccountsType::TransferHookY as i32),
                        2 => Some(PbAccountsType::TransferHookReward as i32),
                        _ => {
                            log::warn!("Unknown PbAccountsType byte: {}", accounts_type_byte);
                            None
                        }
                    };

                    parsed_slices.push(PbRemainingAccountsSlice {
                        accounts_type: accounts_type,
                        length: if length_byte == 0 { None } else { Some(length_byte as u32) },
                    });

                    current_offset += 2; // Move to next slice (assuming slice itself is just 2 bytes)
                                       // Check IDL definition of RemainingAccountsSlice size if this is wrong
                }
            } else {
                 log::warn!("Failed to parse RemainingAccountsInfo length for ClaimFee2");
            }

            let remaining_accounts_info = if parsed_slices.is_empty() {
                None
            } else {
                Some(PbRemainingAccountsInfo { slices: parsed_slices })
            };

            args.instruction_args = Some(IArgs::ClaimFee2(PbClaimFee2Layout {
                min_bin_id: min_bin_id_opt,
                max_bin_id: max_bin_id_opt,
                remaining_accounts_info,
            }));
        },

        // --- Add handlers for the current batch --- 
        InstructionType::InitializeTokenBadge => {
             args.instruction_args = Some(IArgs::InitializeTokenBadge(PbInitializeTokenBadgeLayout {}));
        },
        InstructionType::CreateClaimProtocolFeeOperator => {
             args.instruction_args = Some(IArgs::CreateClaimProtocolFeeOperator(PbCreateClaimProtocolFeeOperatorLayout {}));
        },
        InstructionType::CloseClaimProtocolFeeOperator => {
             args.instruction_args = Some(IArgs::CloseClaimProtocolFeeOperator(PbCloseClaimProtocolFeeOperatorLayout {}));
        },

        InstructionType::InitializeCustomizablePermissionlessLbPair2 => {
            // Args: params: CustomizableParams
            let mut current_offset = 8;
            // Parse CustomizableParams
            if data.len() < current_offset + 8 { return None; } // active_id, bin_step, base_factor, activation_type
            // Use .ok() for optional fields
            let active_id_opt = parse_i32(data, current_offset).ok();
            let bin_step_opt = parse_u16(data, current_offset + 4).ok().map(|v| v as u32); // Map u16 to u32
            let base_factor_opt = parse_u16(data, current_offset + 6).ok().map(|v| v as u32); // Map u16 to u32
            current_offset += 8;

            if data.len() < current_offset + 1 { return None; } // activation_type (u8)
            let activation_type = data[current_offset];
            current_offset += 1;

            if data.len() < current_offset + 1 { return None; } // has_alpha_vault (bool)
            let has_alpha_vault = data[current_offset] != 0;
            current_offset += 1;

            if data.len() < current_offset + 9 { return None; } // Option<u64> activation_point (1 byte disc + 8 bytes value)
            let activation_point_present = data[current_offset] != 0;
            // Keep existing Option<u64> parsing logic
            let activation_point = if activation_point_present { parse_u64(data, current_offset + 1).ok() } else { None };
            current_offset += 9;

            if data.len() < current_offset + 1 { return None; } // creator_pool_on_off_control (bool)
            let creator_pool_on_off_control = data[current_offset] != 0;
            current_offset += 1;

            if data.len() < current_offset + 1 { return None; } // base_fee_power_factor (u8)
            let base_fee_power_factor = data[current_offset];
            // Ignore padding (62 bytes)

            // Extract padding bytes but don't store them explicitly unless needed
            let padding_bytes_v2 = if data.len() >= current_offset + 62 {
                data[current_offset..(current_offset + 62)].to_vec()
            } else {
                log::warn!("Data too short for padding in InitializeCustomizablePermissionlessLbPair2");
                vec![] // Return empty vec if data is short
            };
            // current_offset += 62; // Update offset if needed

            let padding_numeric_v2 = padding_bytes_v2.iter().map(|&b| b as u32).collect::<Vec<u32>>();
            // current_offset += 62; // Update offset if needed

            let params = PbCustomizableParams {
                 // Apply .ok() results
                active_id: active_id_opt,
                bin_step: bin_step_opt,
                base_factor: base_factor_opt,
                // Bools and enums mapped directly (assuming non-zero means Some)
                activation_type: Some(activation_type as u32), // Assuming u8 maps to enum/u32
                has_alpha_vault: Some(has_alpha_vault),
                activation_point: activation_point, // Already Option<u64>
                creator_pool_on_off_control: Some(creator_pool_on_off_control),
                base_fee_power_factor: Some(base_fee_power_factor as u32), // Assuming u8 maps to u32
                padding: padding_numeric_v2.clone(), // Assign numeric Vec<u32> to the final padding field
            };

            args.instruction_args = Some(IArgs::InitializeCustomizablePermissionlessLbPair2(
                 PbInitializeCustomizablePermissionlessLbPair2Layout { params: Some(params) }
            ));
        },

        InstructionType::AddLiquidity2 => {
             // Args: liquidityParameter: LiquidityParameter, remainingAccountsInfo: RemainingAccountsInfo
            let mut current_offset = 8;
            // Parse LiquidityParameter
            if data.len() < current_offset + 16 { return None; } // amount_x, amount_y
            // Use .ok() for optional fields
            let amount_x_opt = parse_u64(data, current_offset).ok();
            let amount_y_opt = parse_u64(data, current_offset + 8).ok();
            current_offset += 16;
            let (bin_dist, next_offset) = parse_bin_liquidity_distribution_vec(data, current_offset);
            current_offset = next_offset;

            let liq_param = PbLiquidityParameter {
                 // Apply .ok() results
                amount_x: amount_x_opt,
                amount_y: amount_y_opt,
                bin_liquidity_dist: bin_dist,
            };

            // Parse RemainingAccountsInfo
            let remaining_accounts = parse_remaining_accounts_info(data, current_offset);

            args.instruction_args = Some(IArgs::AddLiquidity2(PbAddLiquidity2Layout {
                liquidity_parameter: Some(liq_param),
                remaining_accounts_info: remaining_accounts,
            }));
        },

        InstructionType::AddLiquidityByStrategy2 => {
            // Args: liquidityParameter: LiquidityParameterByStrategy, remainingAccountsInfo: RemainingAccountsInfo
            let mut current_offset = 8;
            // Updated length check: 8(disc) + 8(amountX) + 8(amountY) + 4(activeId) + 4(maxSlip) + 4(minBin) + 4(maxBin) + 1(type) + 64(params) = 105
            // Need additional bytes for remaining_accounts_info length (at least 4)
            let base_len = 105;
            if data.len() < base_len + 4 { // Check for base + strategy + params + remaining_accounts vec length
                log::warn!("Data too short for AddLiquidityByStrategy2: {} bytes", data.len());
                return None;
            }

            // Use .ok() for optional fields
            let amount_x_opt = parse_u64(data, current_offset).ok();
            current_offset += 8;
            let amount_y_opt = parse_u64(data, current_offset).ok();
            current_offset += 8;
            let active_id_opt = parse_i32(data, current_offset).ok();
            current_offset += 4;
            let max_active_bin_slippage_opt = parse_i32(data, current_offset).ok();
            current_offset += 4;

            // Parse StrategyParameters (as struct)
            if data.len() < current_offset + 9 + 64 { // Check for strategy base + parameters bytes
                 log::warn!("Data too short for StrategyParameters in AddLiquidityByStrategy2: {} bytes needed from offset {}", 9 + 64, current_offset);
                 return None; // Cannot parse StrategyParameters fully
            }
            let min_bin_id_opt = parse_i32(data, current_offset).ok();
            let max_bin_id_opt = parse_i32(data, current_offset + 4).ok();
            let strategy_type_byte = data[current_offset + 8];
            current_offset += 9;
            let parameters_bytes = data[current_offset..(current_offset + 64)].to_vec(); // Extract parameters
            let parameters_numeric = parameters_bytes.iter().map(|&b| b as u32).collect::<Vec<u32>>(); // Create numeric vec
            current_offset += 64;

             let strategy_type = match strategy_type_byte {
                 0 => Some(PbStrategyType::SpotOneSide as i32),
                 1 => Some(PbStrategyType::CurveOneSide as i32),
                 2 => Some(PbStrategyType::BidAskOneSide as i32),
                 3 => Some(PbStrategyType::SpotBalanced as i32),
                 4 => Some(PbStrategyType::CurveBalanced as i32),
                 5 => Some(PbStrategyType::BidAskBalanced as i32),
                 6 => Some(PbStrategyType::SpotImbalanced as i32),
                 7 => Some(PbStrategyType::CurveImbalanced as i32),
                 8 => Some(PbStrategyType::BidAskImbalanced as i32),
                _ => { log::warn!("Unknown strategy type byte: {}", strategy_type_byte); None },
             };

            let strat_params = PbStrategyParameters {
                 min_bin_id: min_bin_id_opt,
                 max_bin_id: max_bin_id_opt,
                 strategy_type,
                 parameters: parameters_numeric, // Assign Vec<u32> to the 'parameters' field
            };

            let liq_param = PbLiquidityParameterByStrategy {
                amount_x: amount_x_opt,
                amount_y: amount_y_opt,
                active_id: active_id_opt,
                max_active_bin_slippage: max_active_bin_slippage_opt,
                strategy_parameters: Some(strat_params),
            };

            // Parse RemainingAccountsInfo
            let remaining_accounts = parse_remaining_accounts_info(data, current_offset);

             args.instruction_args = Some(IArgs::AddLiquidityByStrategy2(PbAddLiquidityByStrategy2Layout {
                 liquidity_parameter: Some(liq_param),
                 remaining_accounts_info: remaining_accounts,
             }));
        },

        InstructionType::AddLiquidityByStrategyOneSide2 => {
            // Args: liquidityParameter: LiquidityParameterByStrategyOneSide, remainingAccountsInfo: RemainingAccountsInfo
            let mut current_offset = 8;
            // Updated length check: 8(disc) + 8(amount) + 4(activeId) + 4(maxSlip) + 4(minBin) + 4(maxBin) + 1(type) + 64(params) = 97
            // Need additional bytes for remaining_accounts_info length (at least 4)
             let base_len = 97;
             if data.len() < base_len + 4 { // Check for base + strategy + params + remaining_accounts vec length
                 log::warn!("Data too short for AddLiquidityByStrategyOneSide2: {} bytes", data.len());
                 return None;
            }

            // Use .ok() for optional fields
            let amount_opt = parse_u64(data, current_offset).ok();
            current_offset += 8;
            let active_id_opt = parse_i32(data, current_offset).ok();
            current_offset += 4;
            let max_active_bin_slippage_opt = parse_i32(data, current_offset).ok();
            current_offset += 4;

            // Parse StrategyParameters (as struct)
            if data.len() < current_offset + 9 + 64 { // Check for strategy base + parameters bytes
                 log::warn!("Data too short for StrategyParameters in AddLiquidityByStrategyOneSide2: {} bytes needed from offset {}", 9 + 64, current_offset);
                 return None; // Cannot parse StrategyParameters fully
            }
            let min_bin_id_opt = parse_i32(data, current_offset).ok();
            let max_bin_id_opt = parse_i32(data, current_offset + 4).ok();
            let strategy_type_byte = data[current_offset + 8];
            current_offset += 9;
            let parameters_bytes = data[current_offset..(current_offset + 64)].to_vec(); // Extract parameters
            let parameters_numeric = parameters_bytes.iter().map(|&b| b as u32).collect::<Vec<u32>>(); // Create numeric vec
            current_offset += 64;

            let strategy_type = match strategy_type_byte {
                0 => Some(PbStrategyType::SpotOneSide as i32),
                1 => Some(PbStrategyType::CurveOneSide as i32),
                2 => Some(PbStrategyType::BidAskOneSide as i32),
                3 => Some(PbStrategyType::SpotBalanced as i32),
                4 => Some(PbStrategyType::CurveBalanced as i32),
                5 => Some(PbStrategyType::BidAskBalanced as i32),
                6 => Some(PbStrategyType::SpotImbalanced as i32),
                7 => Some(PbStrategyType::CurveImbalanced as i32),
                8 => Some(PbStrategyType::BidAskImbalanced as i32),
                _ => { log::warn!("Unknown strategy type byte: {}", strategy_type_byte); None },
            };

            let strat_params = PbStrategyParameters {
                min_bin_id: min_bin_id_opt,
                max_bin_id: max_bin_id_opt,
                strategy_type,
                parameters: parameters_numeric, // Assign Vec<u32> to the 'parameters' field
            };

            // Import this directly from the main crate path
            let liq_param = crate::pb::sf::solana::meteora_dlmm::v1::PbLiquidityParameterByStrategyOneSide {
                amount: amount_opt,
                active_id: active_id_opt,
                max_active_bin_slippage: max_active_bin_slippage_opt,
                strategy_parameters: Some(strat_params),
            };

            // Parse RemainingAccountsInfo
            let remaining_accounts = parse_remaining_accounts_info(data, current_offset);

            // Qualify the struct name fully
            args.instruction_args = Some(IArgs::AddLiquidityByStrategyOneSide2(crate::pb::sf::solana::meteora_dlmm::v1::PbAddLiquidityByStrategyOneSide2Layout {
                liquidity_parameter: Some(liq_param),
                remaining_accounts_info: remaining_accounts,
            }));
        },

        InstructionType::AddLiquidityOneSidePrecise2 => {
            // Args: liquidityParameter: AddLiquiditySingleSidePreciseParameter2, remainingAccountsInfo: RemainingAccountsInfo
            let mut current_offset = 8;
            log::debug!("Parsing AddLiquidityOneSidePrecise2 starting at offset {}", current_offset);
            // Parse AddLiquiditySingleSidePreciseParameter2
            let (bins, next_offset_bins) = parse_compressed_bin_deposit_vec(data, current_offset);
            current_offset = next_offset_bins;
            log::debug!("Parsed {} bins. Offset now: {}", bins.len(), current_offset);

            if data.len() < current_offset + 16 {
                log::warn!("Data too short for decompress_multiplier and max_amount. Len: {}, Expected at least: {}", data.len(), current_offset + 16);
                return None;
            } // decompress_multiplier (u64), max_amount (u64)
             // Use .ok() for optional fields
            let decompress_multiplier_opt = parse_u64(data, current_offset).ok();
            let max_amount_opt = parse_u64(data, current_offset + 8).ok();
            current_offset += 16;
            log::debug!("Parsed decompress_multiplier={:?}, max_amount={:?}. Offset now: {}", decompress_multiplier_opt, max_amount_opt, current_offset);


            let liq_param = PbAddLiquiditySingleSidePreciseParameter2 {
                bins: bins,
                 // Apply .ok() results
                decompress_multiplier: decompress_multiplier_opt,
                max_amount: max_amount_opt,
            };

            // Parse RemainingAccountsInfo
            let remaining_accounts = parse_remaining_accounts_info(data, current_offset);
            log::debug!("Parsed remaining_accounts_info: {:?}", remaining_accounts);


            args.instruction_args = Some(IArgs::AddLiquidityOneSidePrecise2(PbAddLiquidityOneSidePrecise2Layout {
                liquidity_parameter: Some(liq_param),
                remaining_accounts_info: remaining_accounts,
            }));
            log::debug!("Finished parsing AddLiquidityOneSidePrecise2");
        },

        InstructionType::RemoveLiquidity2 => {
            // Args: binLiquidityRemoval: Vec<BinLiquidityReduction>, remainingAccountsInfo: RemainingAccountsInfo
            let mut current_offset = 8;
            // bin_liquidity_removal parsing seems fine (returns Vec)
            let (reductions, next_offset) = parse_bin_liquidity_reduction_vec(data, current_offset);
            current_offset = next_offset;
            let remaining_accounts = parse_remaining_accounts_info(data, current_offset);

            args.instruction_args = Some(IArgs::RemoveLiquidity2(PbRemoveLiquidity2Layout {
                bin_liquidity_removal: reductions,
                remaining_accounts_info: remaining_accounts,
            }));
        },

        InstructionType::RemoveLiquidityByRange2 => {
            // Args: fromBinId: i32, toBinId: i32, bpsToRemove: u16, remainingAccountsInfo: RemainingAccountsInfo
            let mut current_offset = 8;
             if data.len() < current_offset + 10 { return None; } // from (i32), to (i32), bps (u16)
             // Use .ok() for optional fields
            let from_bin_id_opt = parse_i32(data, current_offset).ok();
            let to_bin_id_opt = parse_i32(data, current_offset + 4).ok();
            let bps_to_remove_opt = parse_u16(data, current_offset + 8).ok().map(|v| v as u32); // Map u16 to u32
            current_offset += 10;
            let remaining_accounts = parse_remaining_accounts_info(data, current_offset);

            args.instruction_args = Some(IArgs::RemoveLiquidityByRange2(PbRemoveLiquidityByRange2Layout {
                 // Apply .ok() results
                from_bin_id: from_bin_id_opt,
                to_bin_id: to_bin_id_opt,
                bps_to_remove: bps_to_remove_opt,
                remaining_accounts_info: remaining_accounts,
            }));
        },
        
        // --- Add handlers for the current batch --- 
        InstructionType::Swap2 => {
            // Args: amountIn: u64, minAmountOut: u64, remainingAccountsInfo: RemainingAccountsInfo
            let mut current_offset = 8;
             if data.len() < current_offset + 16 { return None; } // amount_in, min_amount_out
             // Use .ok() for optional fields
            let amount_in_opt = parse_u64(data, current_offset).ok();
            let min_amount_out_opt = parse_u64(data, current_offset + 8).ok();
            current_offset += 16;
            let remaining_accounts = parse_remaining_accounts_info(data, current_offset);

             args.instruction_args = Some(IArgs::Swap2(PbSwap2Layout {
                  // Apply .ok() results
                 amount_in: amount_in_opt,
                 min_amount_out: min_amount_out_opt,
                 remaining_accounts_info: remaining_accounts,
             }));
        },

        InstructionType::SwapExactOut2 => {
            // Args: maxInAmount: u64, outAmount: u64, remainingAccountsInfo: RemainingAccountsInfo
            let mut current_offset = 8;
             if data.len() < current_offset + 16 { return None; } // max_in_amount, out_amount
             // Use .ok() for optional fields
            let max_in_amount_opt = parse_u64(data, current_offset).ok();
            let out_amount_opt = parse_u64(data, current_offset + 8).ok();
            current_offset += 16;
            let remaining_accounts = parse_remaining_accounts_info(data, current_offset);

             args.instruction_args = Some(IArgs::SwapExactOut2(PbSwapExactOut2Layout {
                  // Apply .ok() results
                 max_in_amount: max_in_amount_opt,
                 out_amount: out_amount_opt,
                 remaining_accounts_info: remaining_accounts,
             }));
        },

        InstructionType::SwapWithPriceImpact2 => {
             // Args: amountIn: u64, activeId: Option<i32>, maxPriceImpactBps: u16, remainingAccountsInfo: RemainingAccountsInfo
            let mut current_offset = 8;
             if data.len() < current_offset + 8 { return None; } // amount_in
             // Use .ok() for optional amount_in
            let amount_in_opt = parse_u64(data, current_offset).ok();
            current_offset += 8;

            // Parse Option<i32> activeId - existing logic is correct
             if data.len() < current_offset + 1 { return None; } // Option discriminator byte
            let active_id_present = data[current_offset] != 0;
            let mut active_id = None; // Keep this as Option<i32>
            current_offset += 1;
            if active_id_present {
                 if data.len() < current_offset + 4 { return None; }
                 // Use .ok() here as well, although the outer check handles None already
                 active_id = parse_i32(data, current_offset).ok();
                 current_offset += 4;
            }

             if data.len() < current_offset + 2 { return None; } // max_price_impact_bps (u16)
             // Use .ok() for optional max_price_impact_bps
            let max_price_impact_bps_opt = parse_u16(data, current_offset).ok().map(|v| v as u32); // Map u16 to u32
            current_offset += 2;

            let remaining_accounts = parse_remaining_accounts_info(data, current_offset);

             args.instruction_args = Some(IArgs::SwapWithPriceImpact2(PbSwapWithPriceImpact2Layout {
                  // Apply .ok() results
                 amount_in: amount_in_opt,
                 active_id: active_id, // Keep as Option<i32>
                 max_price_impact_bps: max_price_impact_bps_opt,
                 remaining_accounts_info: remaining_accounts,
             }));
        },

        // --- Add handlers for the current batch --- 
        InstructionType::ClosePosition2 => {
            // No arguments
            args.instruction_args = Some(IArgs::ClosePosition2(PbClosePosition2Layout {}));
        },

        InstructionType::UpdateFeesAndRewards => { // Handles both UpdateFeesAndRewards and updateFeesAndReward2
            let mut current_offset = 8; // Remove mut later if cargo fix doesn't
            if data.len() >= current_offset + 8 { // Check if data length matches V2 args
                 // Use .ok() for optional fields
                let min_bin_id_opt = parse_i32(data, current_offset).ok();
                let max_bin_id_opt = parse_i32(data, current_offset + 4).ok();
                 args.instruction_args = Some(IArgs::UpdateFeesAndReward2(PbUpdateFeesAndReward2Layout {
                     // Apply .ok() results
                     min_bin_id: min_bin_id_opt,
                     max_bin_id: max_bin_id_opt,
                 }));
            } else {
                // Handle as V1 (UpdateFeesAndRewards) which has no args in IDL
                log::debug!("Processing UpdateFeesAndRewards with data length {}, treating as V1 (no args)", data.len());
                 args.instruction_args = Some(IArgs::UpdateFeesAndRewards(PbUpdateFeesAndRewardsLayout {}));
            }
        },

        InstructionType::ClosePositionIfEmpty => {
            // No arguments
            args.instruction_args = Some(IArgs::ClosePositionIfEmpty(PbClosePositionIfEmptyLayout {}));
        },

        // --- End V2 Handlers ---

        // Special case
        InstructionType::EventLog => {
            return process_event_log(data, args);
        },

        // Catch-all for unimplemented V2 instructions (for now)
        // Remove this once all V2 instructions are handled
        _ => {
            log::debug!("Instruction type {:?} not yet fully parsed.", inst_type);
            // Return None or keep args as default/empty
            // For now, returning None to be explicit that parsing is incomplete
            return None;
        }
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
fn parse_u8(data: &[u8], offset: usize) -> Result<u8, &'static str> {
    if offset + 1 > data.len() { return Err(r#"Data too short for u8"#); }
    Ok(data[offset])
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
    if data.len() < 16 { // Need at least 8 bytes for EVENT_LOG_DISCRIMINATOR + 8 bytes for event discriminator
        log::info!("Event log data too short to contain both discriminators");
        return None;
    }

    // First check if this is really an event log by checking the first 8 bytes
    if &data[0..8] != EVENT_LOG_DISCRIMINATOR {
        log::info!("Data doesn't start with EVENT_LOG_DISCRIMINATOR: {} vs {}", 
                 hex::encode(&data[0..8]), hex::encode(EVENT_LOG_DISCRIMINATOR));
        return None;
    }

    // Skip the first 8 bytes (EVENT_LOG_DISCRIMINATOR) to get the event-specific discriminator
    let discriminator = &data[8..16];
    // Skip both discriminators to get the actual event data
    let event_data = &data[16..];
    
    // Create wrapper with default empty event name
    let mut event_wrapper = PbEventLogWrapper {
        event_name: "Unknown".to_string(),
        event_fields: None,
    };

    // Add enhanced debugging for discriminator matching
    log::info!("Trying to match event discriminator: {}", hex::encode(discriminator));
    
    // Debug log for some common event types to see what their computed discriminators are
    let debug_event_types = [
        "Swap", "FundReward", "AddLiquidity", "RemoveLiquidity", 
        "Swap2", "ClaimFee2", "ClaimReward2", "AddLiquidity2", "RemoveLiquidity2"
    ];
    
    for event_type in debug_event_types.iter() {
        let computed = compute_event_discriminator(event_type);
        if computed[..] == discriminator[..] {
            log::info!("MATCH FOUND: {} event discriminator matches current data", event_type);
        } else {
            log::debug!("Event: {}, Computed: {}, Actual: {}", 
                     event_type, hex::encode(computed), hex::encode(discriminator));
        }
    }

    // Match event discriminator against computed types
    let event_name = match discriminator {
        // Use correct event discriminators (not instruction discriminators)
        &[81, 108, 227, 190, 205, 208, 10, 196] => "Swap".to_string(),
        &[31, 94, 125, 90, 227, 52, 61, 186] => "AddLiquidity".to_string(),
        &[116, 244, 97, 232, 103, 31, 152, 58] => "RemoveLiquidity".to_string(),
        &[148, 116, 134, 204, 22, 171, 85, 95] => "ClaimReward".to_string(),
        &[246, 228, 58, 130, 145, 170, 79, 204] => "FundReward".to_string(),
        &[211, 153, 88, 62, 149, 60, 177, 70] => "InitializeReward".to_string(),
        &[223, 245, 224, 153, 49, 29, 163, 172] => "UpdateRewardDuration".to_string(),
        &[224, 178, 174, 74, 252, 165, 85, 180] => "UpdateRewardFunder".to_string(),
        &[255, 196, 16, 107, 28, 202, 53, 128] => "PositionClose".to_string(),
        &[75, 122, 154, 48, 140, 74, 123, 163] => "ClaimFee".to_string(),
        &[185, 74, 252, 125, 27, 215, 188, 111] => "LbPairCreate".to_string(),
        &[144, 142, 252, 84, 157, 53, 37, 121] => "PositionCreate".to_string(),
        &[48, 76, 241, 117, 144, 215, 242, 44] => "FeeParameterUpdate".to_string(),
        &[99, 249, 17, 121, 166, 156, 207, 215] => "IncreaseObservation".to_string(),
        &[231, 189, 65, 149, 102, 215, 154, 244] => "WithdrawIneligibleReward".to_string(),
        &[39, 115, 48, 204, 246, 47, 66, 57] => "UpdatePositionOperator".to_string(),
        &[176, 165, 93, 114, 250, 229, 146, 255] => "UpdatePositionLockReleaseSlot".to_string(),
        &[59, 138, 76, 68, 138, 131, 176, 67] => "GoToABin".to_string(),
        &[133, 214, 66, 224, 64, 12, 7, 191] => "UpdatePositionLockReleasePoint".to_string(),
        &[128, 151, 123, 106, 17, 102, 113, 142] => "CompositionFee".to_string(),
        &[157, 239, 42, 204, 30, 56, 223, 46] => "IncreasePositionLength".to_string(),
        &[52, 118, 235, 85, 172, 169, 15, 128] => "DecreasePositionLength".to_string(),
        &[88, 88, 178, 135, 194, 146, 91, 243] => "DynamicFeeParameterUpdate".to_string(),
        // Keep special case for unknown event with hardcoded discriminator
        d if d == &[179, 72, 71, 30, 59, 19, 170, 3] => "UnknownEvent1".to_string(),
        _ => {
            // Just log the unknown discriminator and use a generic name
            log::info!("Unknown event discriminator: {}", hex::encode(discriminator));
            format!("Unknown_{}", hex::encode(discriminator))
        }
    };
    
    event_wrapper.event_name = event_name.clone();

    // Use a match against the event name instead of comparing to hardcoded discriminators
    match event_name.as_str() {
        "Swap" => {
            // Always create the struct, using defaults if data is short
            let fields = pb_event_log_wrapper::EventFields::SwapLogFields(
                crate::pb::sf::solana::meteora_dlmm::v1::PbSwapLogFields {
                    lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                    from: if event_data.len() >= 64 { bytes_to_pubkey_str(event_data, 32).unwrap_or_default() } else { "".to_string() },
                    start_bin_id: Some(if event_data.len() >= 68 { parse_i32(event_data, 64).unwrap_or(0) } else { 0 }),
                    end_bin_id: Some(if event_data.len() >= 72 { parse_i32(event_data, 68).unwrap_or(0) } else { 0 }),
                    amount_in: Some(if event_data.len() >= 80 { parse_u64(event_data, 72).unwrap_or(0) } else { 0 }),
                    amount_out: Some(if event_data.len() >= 88 { parse_u64(event_data, 80).unwrap_or(0) } else { 0 }),
                    swap_for_y: Some(if event_data.len() >= 89 { event_data[88] != 0 } else { false }),
                    fee: Some(if event_data.len() >= 97 { parse_u64(event_data, 89).unwrap_or(0) } else { 0 }),
                    protocol_fee: Some(if event_data.len() >= 105 { parse_u64(event_data, 97).unwrap_or(0) } else { 0 }),
                    fee_bps: if event_data.len() >= 121 { parse_u128(event_data, 105).unwrap_or(0).to_string() } else { "0".to_string() },
                    host_fee: Some(if event_data.len() >= 129 { parse_u64(event_data, 121).unwrap_or(0) } else { 0 }),
                }
            );
            event_wrapper.event_fields = Some(fields);
        },
        
        "AddLiquidity" => {
            // Calculate the offset for amounts (after lb_pair, from, position: 3*32 = 96)
            let amounts_offset = 96;
            let mut amounts = Vec::with_capacity(2);
            if event_data.len() >= amounts_offset + 16 { // Check if data is long enough for two u64
                if let Ok(amount0) = parse_u64(event_data, amounts_offset) {
                    amounts.push(amount0);
                } else {
                    log::warn!("Failed to parse amount[0] for AddLiquidity event");
                    amounts.push(0); // Default value
                }
                if let Ok(amount1) = parse_u64(event_data, amounts_offset + 8) {
                    amounts.push(amount1);
                } else {
                    log::warn!("Failed to parse amount[1] for AddLiquidity event");
                    amounts.push(0); // Default value
                }
            } else {
                log::warn!("Event data too short for AddLiquidity amounts: len={}", event_data.len());
                amounts.push(0); // Default values if data is too short
                amounts.push(0);
            }
            
            // Calculate offset for active_bin_id (after amounts: 96 + 16 = 112)
            let active_bin_id_offset = 112;

            let fields = pb_event_log_wrapper::EventFields::AddLiquidityLogFields(
                crate::pb::sf::solana::meteora_dlmm::v1::PbAddLiquidityLogFields {
                    lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                    from: if event_data.len() >= 64 { bytes_to_pubkey_str(event_data, 32).unwrap_or_default() } else { "".to_string() },
                    position: if event_data.len() >= 96 { bytes_to_pubkey_str(event_data, 64).unwrap_or_default() } else { "".to_string() },
                    amounts: amounts, // Use the parsed amounts
                    active_bin_id: Some(if event_data.len() >= active_bin_id_offset + 4 { parse_i32(event_data, active_bin_id_offset).unwrap_or(0) } else { 0 }),
                }
            );
            event_wrapper.event_fields = Some(fields);
        },
        
        "RemoveLiquidity" => {
            // Calculate the offset for amounts (after lb_pair, from, position: 3*32 = 96)
            let amounts_offset = 96;
            let mut amounts = Vec::with_capacity(2);
            if event_data.len() >= amounts_offset + 16 { // Check if data is long enough for two u64
                if let Ok(amount0) = parse_u64(event_data, amounts_offset) {
                    amounts.push(amount0);
                } else {
                    log::warn!("Failed to parse amount[0] for RemoveLiquidity event");
                    amounts.push(0); // Default value
                }
                if let Ok(amount1) = parse_u64(event_data, amounts_offset + 8) {
                    amounts.push(amount1);
                } else {
                    log::warn!("Failed to parse amount[1] for RemoveLiquidity event");
                    amounts.push(0); // Default value
                }
            } else {
                log::warn!("Event data too short for RemoveLiquidity amounts: len={}", event_data.len());
                amounts.push(0); // Default values if data is too short
                amounts.push(0);
            }
            
             // Calculate offset for active_bin_id (after amounts: 96 + 16 = 112)
            let active_bin_id_offset = 112;

            let fields = pb_event_log_wrapper::EventFields::RemoveLiquidityLogFields(
                crate::pb::sf::solana::meteora_dlmm::v1::PbRemoveLiquidityLogFields {
                    lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                    from: if event_data.len() >= 64 { bytes_to_pubkey_str(event_data, 32).unwrap_or_default() } else { "".to_string() },
                    position: if event_data.len() >= 96 { bytes_to_pubkey_str(event_data, 64).unwrap_or_default() } else { "".to_string() },
                    amounts: amounts, // Use the parsed amounts
                    active_bin_id: Some(if event_data.len() >= active_bin_id_offset + 4 { parse_i32(event_data, active_bin_id_offset).unwrap_or(0) } else { 0 }),
                }
            );
            event_wrapper.event_fields = Some(fields);
        },
        
        "ClaimReward" => {
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
        },
        
        "FundReward" => {
            let fields = pb_event_log_wrapper::EventFields::FundRewardLogFields(
                crate::pb::sf::solana::meteora_dlmm::v1::PbFundRewardLogFields {
                    lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                    funder: if event_data.len() >= 64 { bytes_to_pubkey_str(event_data, 32).unwrap_or_default() } else { "".to_string() },
                    reward_index: Some(if event_data.len() >= 72 { parse_i64(event_data, 64).unwrap_or(0) } else { 0 }),
                    amount: Some(if event_data.len() >= 80 { parse_i64(event_data, 72).unwrap_or(0) } else { 0 }),
                }
            );
            event_wrapper.event_fields = Some(fields);
        },
        
        "InitializeReward" => {
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
        },
        
        "UpdateRewardDuration" => {
            let fields = pb_event_log_wrapper::EventFields::UpdateRewardDurationLogFields(
                crate::pb::sf::solana::meteora_dlmm::v1::PbUpdateRewardDurationLogFields {
                    lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                    reward_index: Some(if event_data.len() >= 40 { parse_i64(event_data, 32).unwrap_or(0) } else { 0 }),
                    old_reward_duration: Some(if event_data.len() >= 48 { parse_i64(event_data, 40).unwrap_or(0) } else { 0 }),
                    new_reward_duration: Some(if event_data.len() >= 56 { parse_i64(event_data, 48).unwrap_or(0) } else { 0 }),
                }
            );
            event_wrapper.event_fields = Some(fields);
        },
        
        "UpdateRewardFunder" => {
            let fields = pb_event_log_wrapper::EventFields::UpdateRewardFunderLogFields(
                crate::pb::sf::solana::meteora_dlmm::v1::PbUpdateRewardFunderLogFields {
                    lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                    reward_index: Some(if event_data.len() >= 40 { parse_i64(event_data, 32).unwrap_or(0) } else { 0 }),
                    old_funder: if event_data.len() >= 72 { bytes_to_pubkey_str(event_data, 40).unwrap_or_default() } else { "".to_string() },
                    new_funder: if event_data.len() >= 104 { bytes_to_pubkey_str(event_data, 72).unwrap_or_default() } else { "".to_string() },
                }
            );
            event_wrapper.event_fields = Some(fields);
        },
        
        "PositionClose" => {
            let fields = pb_event_log_wrapper::EventFields::PositionCloseLogFields(
                crate::pb::sf::solana::meteora_dlmm::v1::PbPositionCloseLogFields {
                    position: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                    owner: if event_data.len() >= 64 { bytes_to_pubkey_str(event_data, 32).unwrap_or_default() } else { "".to_string() },
                }
            );
            event_wrapper.event_fields = Some(fields);
        },
        
        "ClaimFee" => {
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
        },
        
        "LbPairCreate" => {
            log::info!("Raw event data for LbPairCreate: {:?}", event_data);
            
            // First get the lb_pair address (bytes 0-32)
            let lb_pair = if event_data.len() >= 32 { 
                bytes_to_pubkey_str(event_data, 0).unwrap_or_default() 
            } else { 
                "".to_string() 
            };
            
            // Next field is bin_step (likely u16 or i16, not i32)
            let bin_step = if event_data.len() >= 34 { 
                parse_u16(event_data, 32).unwrap_or(0) as i32 
            } else { 
                0 
            };
            
            // token_x should start immediately after bin_step
            let token_x = if event_data.len() >= 66 { 
                bytes_to_pubkey_str(event_data, 34).unwrap_or_default() 
            } else { 
                "".to_string() 
            };
            
            // token_y should start immediately after token_x
            let token_y = if event_data.len() >= 98 { 
                bytes_to_pubkey_str(event_data, 66).unwrap_or_default() 
            } else { 
                "".to_string() 
            };
            
            let fields = pb_event_log_wrapper::EventFields::LbPairCreateLogFields(
                crate::pb::sf::solana::meteora_dlmm::v1::PbLbPairCreateLogFields {
                    lb_pair: lb_pair.clone(),
                    bin_step: Some(bin_step),
                    token_x: token_x.clone(),
                    token_y: token_y.clone(),
                }
            );
            event_wrapper.event_fields = Some(fields);
            
            log::info!("Processing LbPairCreate event: lb_pair={}, bin_step={}, token_x={}, token_y={}",
                      lb_pair, bin_step, token_x, token_y);
        },
        
        "PositionCreate" => {
            let fields = pb_event_log_wrapper::EventFields::PositionCreateLogFields(
                crate::pb::sf::solana::meteora_dlmm::v1::PbPositionCreateLogFields {
                    lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                    position: if event_data.len() >= 64 { bytes_to_pubkey_str(event_data, 32).unwrap_or_default() } else { "".to_string() },
                    owner: if event_data.len() >= 96 { bytes_to_pubkey_str(event_data, 64).unwrap_or_default() } else { "".to_string() },
                }
            );
            event_wrapper.event_fields = Some(fields);
        },
        
        "FeeParameterUpdate" => {
            // Layout: lbPair(32), protocolShare(u16), baseFactor(u16)
            // Total size: 32 + 2 + 2 = 36 bytes
            let protocol_share_offset = 32;
            let base_factor_offset = 34;
            const MIN_LEN: usize = 36;

            let fields = pb_event_log_wrapper::EventFields::FeeParameterUpdateLogFields(
                crate::pb::sf::solana::meteora_dlmm::v1::PbFeeParameterUpdateLogFields {
                    lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                    // Parse as u16 (Int16ul), map to Option<u32> for proto
                    protocol_share: if event_data.len() >= protocol_share_offset + 2 { parse_u16(event_data, protocol_share_offset).ok().map(|v| v as u32) } else { None },
                    base_factor: if event_data.len() >= base_factor_offset + 2 { parse_u16(event_data, base_factor_offset).ok().map(|v| v as u32) } else { None },
                }
            );
            event_wrapper.event_fields = Some(fields);
        },
        
        "IncreaseObservation" => {
            let fields = pb_event_log_wrapper::EventFields::IncreaseObservationLogFields(
                crate::pb::sf::solana::meteora_dlmm::v1::PbIncreaseObservationLogFields {
                    oracle: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                    new_observation_length: Some(if event_data.len() >= 40 { parse_i64(event_data, 32).unwrap_or(0) } else { 0 }),
                }
            );
            event_wrapper.event_fields = Some(fields);
        },
        
        "WithdrawIneligibleReward" => {
            let fields = pb_event_log_wrapper::EventFields::WithdrawIneligibleRewardLogFields(
                crate::pb::sf::solana::meteora_dlmm::v1::PbWithdrawIneligibleRewardLogFields {
                    lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                    reward_mint: if event_data.len() >= 64 { bytes_to_pubkey_str(event_data, 32).unwrap_or_default() } else { "".to_string() },
                    amount: Some(if event_data.len() >= 72 { parse_i64(event_data, 64).unwrap_or(0) } else { 0 }),
                }
            );
            event_wrapper.event_fields = Some(fields);
        },
        
        "UpdatePositionOperator" => {
            let fields = pb_event_log_wrapper::EventFields::UpdatePositionOperatorLogFields(
                crate::pb::sf::solana::meteora_dlmm::v1::PbUpdatePositionOperatorLogFields {
                    position: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                    old_operator: if event_data.len() >= 64 { bytes_to_pubkey_str(event_data, 32).unwrap_or_default() } else { "".to_string() },
                    new_operator: if event_data.len() >= 96 { bytes_to_pubkey_str(event_data, 64).unwrap_or_default() } else { "".to_string() },
                }
            );
            event_wrapper.event_fields = Some(fields);
        },
        
        "UpdatePositionLockReleaseSlot" => {
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
        },
        
        "GoToABin" => {
            let fields = pb_event_log_wrapper::EventFields::GoToABinLogFields(
                crate::pb::sf::solana::meteora_dlmm::v1::PbGoToABinLogFields {
                    lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                    from_bin_id: Some(if event_data.len() >= 36 { parse_i32(event_data, 32).unwrap_or(0) } else { 0 }),
                    to_bin_id: Some(if event_data.len() >= 40 { parse_i32(event_data, 36).unwrap_or(0) } else { 0 }),
                }
            );
            event_wrapper.event_fields = Some(fields);
        },
        
        "UpdatePositionLockReleasePoint" => {
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
        },
        
        "UnknownEvent1" => {
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
        },
        
        "CompositionFee" => {
            // Layout: from(32), binId(i16), tokenXFee(u64), tokenYFee(u64), protocolXFee(u64), protocolYFee(u64)
            // Total size: 32 + 2 + 8 + 8 + 8 + 8 = 66 bytes
            const MIN_LEN: usize = 66; 
            let bin_id_offset = 32;
            let token_x_offset = 34;
            let token_y_offset = 42;
            let protocol_x_offset = 50;
            let protocol_y_offset = 58;

            let fields = pb_event_log_wrapper::EventFields::CompositionFeeLogFields(
                crate::pb::sf::solana::meteora_dlmm::v1::PbCompositionFeeLogFields {
                    from: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                    // Parse as i16 (2 bytes) and cast to Option<i32> for proto
                    bin_id: if event_data.len() >= bin_id_offset + 2 { parse_i16(event_data, bin_id_offset).ok().map(|v| v as i32) } else { None },
                    token_x_fee_amount: if event_data.len() >= token_x_offset + 8 { parse_u64(event_data, token_x_offset).ok() } else { None },
                    token_y_fee_amount: if event_data.len() >= token_y_offset + 8 { parse_u64(event_data, token_y_offset).ok() } else { None },
                    protocol_token_x_fee_amount: if event_data.len() >= protocol_x_offset + 8 { parse_u64(event_data, protocol_x_offset).ok() } else { None },
                    protocol_token_y_fee_amount: if event_data.len() >= protocol_y_offset + 8 { parse_u64(event_data, protocol_y_offset).ok() } else { None },
                }
            );
            event_wrapper.event_fields = Some(fields);
        },
        
        "IncreasePositionLength" => {
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
        },
        
        "DecreasePositionLength" => {
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
        },
        
        "DynamicFeeParameterUpdate" => {
            let fields = pb_event_log_wrapper::EventFields::DynamicFeeParameterUpdateLogFields(
                crate::pb::sf::solana::meteora_dlmm::v1::PbDynamicFeeParameterUpdateLogFields {
                    lb_pair: if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() },
                    filter_period: Some(if event_data.len() >= 36 { parse_u32(event_data, 32).unwrap_or(0) } else { 0 }),
                    decay_period: Some(if event_data.len() >= 40 { parse_u32(event_data, 36).unwrap_or(0) } else { 0 }),
                    reduction_factor: Some(if event_data.len() >= 44 { parse_u32(event_data, 40).unwrap_or(0) } else { 0 }),
                    variable_fee_control: Some(if event_data.len() >= 48 { parse_u32(event_data, 44).unwrap_or(0) } else { 0 }),
                    max_volatility_accumulator: Some(if event_data.len() >= 52 { parse_u32(event_data, 48).unwrap_or(0) } else { 0 }),
                }
            );
            event_wrapper.event_fields = Some(fields);

            log::info!("Processing DynamicFeeParameterUpdate event: lb_pair={}",
                      if event_data.len() >= 32 { bytes_to_pubkey_str(event_data, 0).unwrap_or_default() } else { "".to_string() });
        },
        
        _ => {
            // Unknown event - fields remain None
        }
    }


    // Log that we identified an event only if fields were set
    if event_wrapper.event_fields.is_some() {
        log::info!("Identified event: {}", event_wrapper.event_name);
    }

    // Set the event wrapper as the instruction args
    args.instruction_args = Some(instruction_args::InstructionArgs::EventLog(event_wrapper));
    
    log::info!("Successfully processed event log: {}", event_name);
    
    // Return the updated args
    Some(args)
} 

// --- Add Helper Functions Back ---

// Helper function to parse RemainingAccountsInfo
fn parse_remaining_accounts_info(data: &[u8], start_offset: usize) -> Option<PbRemainingAccountsInfo> {
    if data.len() < start_offset + 4 { // Need 4 bytes for vec length
        log::warn!("Data too short for RemainingAccountsInfo length: offset={}, len={}", start_offset, data.len());
        return None;
    }
    let slices_len_res = parse_u32(data, start_offset);
    if slices_len_res.is_err() {
        log::warn!("Failed to parse RemainingAccountsInfo length at offset {}", start_offset);
        return None;
    }
    let slices_len = slices_len_res.unwrap() as usize;
    let mut current_offset = start_offset + 4;

    // If the length is zero, it means the optional structure is not present.
    if slices_len == 0 {
        log::debug!("RemainingAccountsInfo vector length is 0 at offset {}. Returning None.", start_offset);
        return None;
    }

    let mut parsed_slices = Vec::with_capacity(slices_len);

    for i in 0..slices_len {
        if data.len() < current_offset + 2 { // Need 1 byte for type, 1 byte for length
            log::warn!("Data too short for RemainingAccountsSlice #{} at offset {}: len={}", i, current_offset, data.len());
            break; // Stop parsing if data is insufficient
        }
        let accounts_type_byte = data[current_offset];
        let length_byte = data[current_offset + 1];

        let accounts_type = match accounts_type_byte {
            0 => Some(PbAccountsType::TransferHookX as i32),
            1 => Some(PbAccountsType::TransferHookY as i32),
            2 => Some(PbAccountsType::TransferHookReward as i32),
            _ => {
                log::warn!("Unknown PbAccountsType byte: {}", accounts_type_byte);
                None
            }
        };

        parsed_slices.push(PbRemainingAccountsSlice {
            accounts_type: accounts_type,
            length: if length_byte == 0 { None } else { Some(length_byte as u32) },
        });
        current_offset += 2; // Size of RemainingAccountsSlice based on IDL (u8 + u8)
    }

    if parsed_slices.is_empty() {
        None
    } else {
        Some(PbRemainingAccountsInfo { slices: parsed_slices })
    }
}

// Placeholder for parsing Vec<PbBinLiquidityReduction>
fn parse_bin_liquidity_reduction_vec(data: &[u8], start_offset: usize) -> (Vec<PbBinLiquidityReduction>, usize) {
    let mut results = Vec::new();
    let mut current_offset = start_offset;
    if data.len() < current_offset + 4 { return (results, current_offset); } // Check for len

    if let Ok(vec_len) = parse_u32(data, current_offset) {
        current_offset += 4;
        for _ in 0..vec_len {
            if data.len() < current_offset + 6 { break; } // 4 bytes bin_id + 2 bytes bps_to_remove
            let bin_id_res = parse_i32(data, current_offset);
            let bps_res = parse_u16(data, current_offset + 4);
            if let (Ok(bin_id), Ok(bps)) = (bin_id_res, bps_res) {
                 results.push(PbBinLiquidityReduction {
                     bin_id: bin_id, // Assign directly, proto field is int32
                     bps_to_remove: bps as u32, // Assign directly and cast, proto field is uint32
                 });
            } else {
                 log::warn!("Failed to parse BinLiquidityReduction element");
            }
            current_offset += 6; // Size of BinLiquidityReduction (i32 + u16)
        }
    } else {
         log::warn!("Failed to parse Vec<BinLiquidityReduction> length");
    }
    (results, current_offset)
}

// Placeholder for parsing Vec<PbCompressedBinDepositAmountLayout>
fn parse_compressed_bin_deposit_vec(data: &[u8], start_offset: usize) -> (Vec<PbCompressedBinDepositAmountLayout>, usize) { // Correct return type
    let mut results = Vec::new();
    let mut current_offset = start_offset;
    if data.len() < current_offset + 4 { return (results, current_offset); } // Check for len

    if let Ok(vec_len) = parse_u32(data, current_offset) {
        current_offset += 4;
        for _ in 0..vec_len {
            // Size of CompressedBinDepositAmount is i32 (4) + u32 (4) = 8 bytes
            if data.len() < current_offset + 8 { break; } 
            let bin_id_res = parse_i32(data, current_offset);
            let amount_res = parse_u32(data, current_offset + 4); // Parse u32 based on IDL/Python layout
             if let (Ok(bin_id), Ok(amount)) = (bin_id_res, amount_res) {
                 results.push(PbCompressedBinDepositAmountLayout { // Correct struct name
                     bin_id: bin_id, // Assign directly, proto field is int32
                     amount_total: amount.to_string(), // Assign amount_total as string (using parsed u32)
                 });
            } else {
                 log::warn!("Failed to parse CompressedBinDepositAmount element");
            }
            current_offset += 8; // Update offset by correct size (4 + 4)
        }
    } else {
         // log::warn!("Failed to parse Vec<CompressedBinDepositAmount> length"); // Removed log
    }
    (results, current_offset)
}

// Placeholder for parsing Vec<PbBinLiquidityDistribution>
fn parse_bin_liquidity_distribution_vec(data: &[u8], start_offset: usize) -> (Vec<PbBinLiquidityDistribution>, usize) {
    let mut results = Vec::new();
    let mut current_offset = start_offset;
    // Need 4 bytes for vector length
    if data.len() < current_offset + 4 {
        log::warn!("Data too short for Vec<BinLiquidityDistribution> length at offset {}", current_offset);
        return (results, current_offset);
    }

    if let Ok(vec_len) = parse_u32(data, current_offset) {
        current_offset += 4;
        log::debug!("Parsing Vec<BinLiquidityDistribution> with length: {}", vec_len);
        for i in 0..vec_len {
            // Each element: binId(i32, 4) + distX(u16, 2) + distY(u16, 2) = 8 bytes
            if data.len() < current_offset + 8 {
                log::warn!("Data too short for BinLiquidityDistribution element #{} at offset {}", i, current_offset);
                break;
            }
            let bin_id_res = parse_i32(data, current_offset);
            let dist_x_res = parse_u16(data, current_offset + 4);
            let dist_y_res = parse_u16(data, current_offset + 6);

            if let (Ok(bin_id), Ok(dist_x), Ok(dist_y)) = (bin_id_res, dist_x_res, dist_y_res) {
                results.push(PbBinLiquidityDistribution {
                    bin_id: Some(bin_id),
                    distribution_x: Some(dist_x as u32), // Cast u16 to u32
                    distribution_y: Some(dist_y as u32), // Cast u16 to u32
                });
                 log::trace!("Parsed BinDist[{}]: bin_id={}, distX={}, distY={}", i, bin_id, dist_x, dist_y);
            } else {
                log::warn!("Failed to parse BinLiquidityDistribution element #{}", i);
            }
            current_offset += 8; // Move offset forward by 8 bytes
        }
    } else {
        log::warn!("Failed to parse Vec<BinLiquidityDistribution> length at offset {}", start_offset);
    }
    (results, current_offset)
}

// --- End Helper Functions ---

/// Compute an 8-byte event discriminator using the format "event:{name}"
pub fn compute_event_discriminator(name: &str) -> [u8; 8] {
    // Precomputed lookup table for known events, based on hardcoded discriminators in process_event_log
    match name {
        "Swap" => [81, 108, 227, 190, 205, 208, 10, 196],
        "AddLiquidity" => [31, 94, 125, 90, 227, 52, 61, 186],
        "RemoveLiquidity" => [116, 244, 97, 232, 103, 31, 152, 58],
        "ClaimReward" => [148, 116, 134, 204, 22, 171, 85, 95],
        "FundReward" => [246, 228, 58, 130, 145, 170, 79, 204],
        "InitializeReward" => [211, 153, 88, 62, 149, 60, 177, 70],
        "UpdateRewardDuration" => [223, 245, 224, 153, 49, 29, 163, 172],
        "UpdateRewardFunder" => [224, 178, 174, 74, 252, 165, 85, 180],
        "PositionClose" => [255, 196, 16, 107, 28, 202, 53, 128],
        "ClaimFee" => [75, 122, 154, 48, 140, 74, 123, 163],
        "LbPairCreate" => [185, 74, 252, 125, 27, 215, 188, 111],
        "PositionCreate" => [144, 142, 252, 84, 157, 53, 37, 121],
        "FeeParameterUpdate" => [48, 76, 241, 117, 144, 215, 242, 44],
        "IncreaseObservation" => [99, 249, 17, 121, 166, 156, 207, 215],
        "WithdrawIneligibleReward" => [231, 189, 65, 149, 102, 215, 154, 244],
        "UpdatePositionOperator" => [39, 115, 48, 204, 246, 47, 66, 57],
        "UpdatePositionLockReleaseSlot" => [176, 165, 93, 114, 250, 229, 146, 255],
        "GoToABin" => [59, 138, 76, 68, 138, 131, 176, 67],
        "UpdatePositionLockReleasePoint" => [133, 214, 66, 224, 64, 12, 7, 191],
        "CompositionFee" => [128, 151, 123, 106, 17, 102, 113, 142],
        "IncreasePositionLength" => [157, 239, 42, 204, 30, 56, 223, 46],
        "DecreasePositionLength" => [52, 118, 235, 85, 172, 169, 15, 128],
        "DynamicFeeParameterUpdate" => [88, 88, 178, 135, 194, 146, 91, 243],
        // For any new events, use the standard calculation method
        _ => {
            // Standard computation for events not in our lookup table
            // For Meteora/Anchor event logs, the discriminator is calculated by:
            // - Taking the first 8 bytes of the SHA256 hash of "event:" + event_name (not snake_cased)
            let prefixed_name = format!("event:{}", name);
            
            let mut hasher = Sha256::new();
            hasher.update(prefixed_name.as_bytes());
            let result = hasher.finalize();
            let mut discriminator = [0u8; 8];
            discriminator.copy_from_slice(&result[..8]);
            
            log::debug!("Computed discriminator for {}: {}", name, hex::encode(&discriminator));
            discriminator
        }
    }
}

// Helper function to parse Vec<PbBinLiquidityDistributionByWeightLayout>
// Returns the parsed vector and the offset after parsing
fn parse_bin_liquidity_dist_by_weight_vec(data: &[u8], start_offset: usize) -> (Vec<PbBinLiquidityDistributionByWeightLayout>, usize) {
    let mut results = Vec::new();
    let mut current_offset = start_offset;
    // Need 4 bytes for vector length
    if data.len() < current_offset + 4 {
        log::warn!("Data too short for Vec<BinLiquidityDistributionByWeight> length at offset {}", current_offset);
        return (results, current_offset);
    }

    if let Ok(vec_len) = parse_u32(data, current_offset) {
        current_offset += 4;
        log::debug!("Parsing Vec<BinLiquidityDistributionByWeight> with length: {}", vec_len);
        for i in 0..vec_len {
            // Each element is binId (i32, 4 bytes) + weight (u16, 2 bytes) = 6 bytes
            if data.len() < current_offset + 6 {
                log::warn!("Data too short for BinLiquidityDistributionByWeight element #{} at offset {}", i, current_offset);
                break; // Stop parsing if data is insufficient
            }
            let bin_id_res = parse_i32(data, current_offset);
            let weight_res = parse_u16(data, current_offset + 4);

            if let (Ok(bin_id), Ok(weight)) = (bin_id_res, weight_res) {
                results.push(PbBinLiquidityDistributionByWeightLayout {
                    bin_id: Some(bin_id),
                    weight: Some(weight as u32), // Cast u16 to u32
                });
                log::trace!("Parsed BinDistWeight[{}]: bin_id={}, weight={}", i, bin_id, weight);
            } else {
                log::warn!("Failed to parse BinLiquidityDistributionByWeight element #{}", i);
                // Optionally push default or skip
            }
            current_offset += 6; // Move offset forward by 6 bytes
        }
    } else {
        log::warn!("Failed to parse Vec<BinLiquidityDistributionByWeight> length at offset {}", start_offset);
    }
    (results, current_offset)
}