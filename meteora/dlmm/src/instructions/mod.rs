use substreams_solana::pb::sf::solana::r#type::v1::CompiledInstruction;
use chrono::{NaiveDateTime, Timelike};
use crate::pb::sf::solana::meteora_dlmm::v1::{
    PbCreatePoolLayout, PbCreatePositionLayout, PbClosePositionLayout, PbInitializeRewardLayout,
    PbInitializeTickArrayLayout, PbRemoveLiquidityLayout, PbSetRewardEmissionsLayout,
    PbSwapRouterBaseInLayout, PbTransferPositionOwnerLayout, PbUpdateFeesLayout,
    PbInitializeConfigLayout, InstructionArgs, Meta,
    
    PbAddLiquidityLayout, PbSwapLayout, PbInitializeLbPairLayout, PbInitializePermissionLbPairLayout,
    PbInitializePositionLayout, PbInitializePositionPdaLayout, PbSwapWithPriceImpactLayout,
    PbSwapExactOutLayout, PbCollectFeesLayout, PbRemoveAllLiquidityLayout,
    PbRemoveLiquidityByRangeLayout, PbRemoveLiquiditySingleSideLayout, PbGoToABinLayout,
    PbInitializePositionByOperatorLayout, PbUpdatePositionOperatorLayout, PbClaimLiquidityLayout,
    PbWithdrawProtocolFeeLayout, PbUpdateFeeParametersLayout, PbUpdateFeeOwnerLayout,
    
    PbCollectRewardLayout, PbClosePresetParameterLayout, PbInitializePresetParameterLayout,
    PbInitializePresetParameterV2Layout, PbTogglePairStatusLayout, PbUpdateWhitelistedWalletLayout,
    PbIncreaseOracleLengthLayout, PbMigratePositionLayout, PbMigrateBinArrayLayout,
    PbUpdateFeesAndRewardsLayout, PbFundRewardLayout, PbUpdateRewardFunderLayout, PbUpdateRewardDurationLayout,
    PbWithdrawIneligibleRewardLayout, PbCloseLbPairLayout, PbInitializeBinArrayLayout,
    PbInitializeBinArrayBitmapExtensionLayout, PbSetLockReleaseSlotLayout, PbSetActivationSlotLayout,
    PbSetMaxSwappedAmountLayout, PbSetPreActivationDurationLayout, PbSetPreActivationSwapAddressLayout,
    PbIdlWriteLayout,
    
    PbLiquidityParameterLayout, /* Removed PbLiquidityOneSideParameterLayout, PbFeeParameterLayout, */
    
    PbAddLiquidityByWeightLayout, PbAddLiquidityByStrategyLayout, PbAddLiquidityOneSideLayout,
    PbAddLiquidityByStrategyOneSideLayout, PbAddLiquidityOneSidePreciseLayout,
    
    // Removed PbInitializeCustomizablePermissionlessLbPairParams
    PbInitializeCustomizablePermissionlessLbPairLayout,
    
    PbEventLogWrapper, pb_event_log_wrapper,
    
    PbSwapLogFields, PbRemoveLiquidityLogFields, PbAddLiquidityLogFields, PbFundRewardLogFields,
    PbInitializeRewardLogFields, PbUpdateRewardDurationLogFields, PbUpdateRewardFunderLogFields,
    PbPositionCloseLogFields, PbClaimFeeLogFields, PbLbPairCreateLogFields, PbPositionCreateLogFields,
    PbFeeParameterUpdateLogFields, PbIncreaseObservationLogFields, PbWithdrawIneligibleRewardLogFields,
    PbUpdatePositionOperatorLogFields, PbUpdatePositionLockReleaseSlotLogFields, PbGoToABinLogFields,
    PbUpdatePositionLockReleasePointLogFields, PbUnknownEvent1LogFields, PbCompositionFeeLogFields,
    PbClaimRewardLogFields,
    
    // Removed PbInitializePresetParameterIxLayout, PbInitializePresetParameterV2IxLayout
    PbBinLiquidityDistributionByWeightLayout,
    PbCompressedBinDepositAmountLayout
};
use crate::pb::sf::solana::meteora_dlmm::v1::instruction_args::InstructionArgs as IArgs;
use crate::prepare_input_accounts;
use sha2::{Digest, Sha256};
use hex;
use chrono::DateTime;
use substreams::log;
use bs58;
use std::convert::TryInto;
use i16;

// Meteora DLMM Program ID
const METEORA_DLMM_PROGRAM_ID: &str = "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo";

// Define common instruction types for Meteora DLMM
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionType {
    // Core pool operations
    InitializeLbPair,
    InitializePermissionLbPair,
    InitializeCustomizablePermissionlessLbPair,
    CloseLbPair,
    InitializeBinArray,
    InitializeBinArrayBitmapExtension,
    
    // Liquidity operations
    AddLiquidity,
    AddLiquidityByWeight,
    AddLiquidityByStrategy,
    AddLiquidityOneSide,
    AddLiquidityByStrategyOneSide,
    AddLiquidityOneSidePrecise,
    RemoveLiquidity,
    RemoveAllLiquidity,
    RemoveLiquidityByRange,
    RemoveLiquiditySingleSide,
    
    // Trading operations
    Swap,
    SwapWithPriceImpact,
    SwapExactOut,
    GoToABin,
    
    // Position management
    InitializePosition,
    InitializePositionPda,
    InitializePositionByOperator,
    ClosePosition,
    UpdatePositionOperator,
    ClaimLiquidity,
    
    // Fee and reward operations
    ClaimFee,
    ClaimReward,
    WithdrawProtocolFee,
    UpdateFeeParameters,
    UpdateFeeOwner,
    InitializeReward,
    UpdateRewardFunder,
    UpdateRewardDuration,
    WithdrawIneligibleReward,
    
    // Parameter management
    ClosePresetParameter,
    InitializePresetParameter,
    InitializePresetParameterV2,
    TogglePairStatus,
    UpdateWhitelistedWallet,
    
    // Migration and updates
    MigratePosition,
    MigrateBinArray,
    UpdateFeesAndRewards,
    SetLockReleaseSlot,
    SetActivationSlot,
    SetActivationPoint,
    
    // Event log wrapper
    EventLog,
    
    // Added from Python
    FundReward,
    IncreaseOracleLength,
    SetMaxSwappedAmount,
    SetPreActivationDuration,
    SetPreActivationSwapAddress,
    IdlWrite, // Might be internal, keeping for completeness

    // Unknown instruction
    Unknown,
}

// Event discriminators
const EVENT_COMPOSITION_FEE_DISCRIMINATOR: &[u8] = &[128, 151, 123, 106, 17, 102, 113, 142];
const EVENT_ADD_LIQUIDITY_DISCRIMINATOR: &[u8] = &[31, 94, 125, 90, 227, 52, 61, 186];
const EVENT_REMOVE_LIQUIDITY_DISCRIMINATOR: &[u8] = &[116, 244, 97, 232, 103, 31, 152, 58];
const EVENT_SWAP_DISCRIMINATOR: &[u8] = &[81, 108, 227, 190, 205, 208, 10, 196];
const EVENT_CLAIM_REWARD_DISCRIMINATOR: &[u8] = &[148, 116, 134, 204, 22, 171, 85, 95];
const EVENT_FUND_REWARD_DISCRIMINATOR: &[u8] = &[246, 228, 58, 130, 145, 170, 79, 204];
const EVENT_INITIALIZE_REWARD_DISCRIMINATOR: &[u8] = &[211, 153, 88, 62, 149, 60, 177, 70];
const EVENT_UPDATE_REWARD_DURATION_DISCRIMINATOR: &[u8] = &[223, 245, 224, 153, 49, 29, 163, 172];
const EVENT_UPDATE_REWARD_FUNDER_DISCRIMINATOR: &[u8] = &[224, 178, 174, 74, 252, 165, 85, 180];
const EVENT_POSITION_CLOSE_DISCRIMINATOR: &[u8] = &[255, 196, 16, 107, 28, 202, 53, 128];
const EVENT_CLAIM_FEE_DISCRIMINATOR: &[u8] = &[75, 122, 154, 48, 140, 74, 123, 163];
const EVENT_LB_PAIR_CREATE_DISCRIMINATOR: &[u8] = &[185, 74, 252, 125, 27, 215, 188, 111];
const EVENT_POSITION_CREATE_DISCRIMINATOR: &[u8] = &[144, 142, 252, 84, 157, 53, 37, 121];
const EVENT_FEE_PARAMETER_UPDATE_DISCRIMINATOR: &[u8] = &[48, 76, 241, 117, 144, 215, 242, 44];
const EVENT_INCREASE_OBSERVATION_DISCRIMINATOR: &[u8] = &[99, 249, 17, 121, 166, 156, 207, 215];
const EVENT_WITHDRAW_INELIGIBLE_REWARD_DISCRIMINATOR: &[u8] = &[231, 189, 65, 149, 102, 215, 154, 244];
const EVENT_UPDATE_POSITION_OPERATOR_DISCRIMINATOR: &[u8] = &[39, 115, 48, 204, 246, 47, 66, 57];
const EVENT_UPDATE_POSITION_LOCK_RELEASE_SLOT_DISCRIMINATOR: &[u8] = &[176, 165, 93, 114, 250, 229, 146, 255];
const EVENT_GO_TO_A_BIN_DISCRIMINATOR: &[u8] = &[59, 138, 76, 68, 138, 131, 176, 67];
const EVENT_UPDATE_POSITION_LOCK_RELEASE_POINT_DISCRIMINATOR: &[u8] = &[133, 214, 66, 224, 64, 12, 7, 191];
const EVENT_UNKNOWN_EVENT1_DISCRIMINATOR: &[u8] = &[179, 72, 71, 30, 59, 19, 170, 3];

// Meteora DLMM instruction types, based on the program's instruction discriminators
pub const INSTRUCTION_TYPES: [(&[u8], &str); 53] = [
    // Core functionality
    (&[45, 154, 237, 210, 221, 15, 166, 92], "InitializeLbPair"),
    (&[108, 102, 213, 85, 251, 3, 53, 21], "InitializePermissionLbPair"),
    (&[46, 39, 41, 135, 111, 183, 200, 64], "InitializeCustomizablePermissionlessLbPair"),
    (&[103, 211, 56, 22, 205, 102, 233, 116], "CloseLbPair"),
    (&[35, 86, 19, 185, 78, 212, 75, 211], "InitializeBinArray"),
    (&[47, 157, 226, 180, 12, 240, 33, 71], "InitializeBinArrayBitmapExtension"),
    (&[181, 157, 89, 67, 143, 182, 52, 72], "AddLiquidity"),
    (&[28, 140, 238, 99, 231, 162, 21, 149], "AddLiquidityByWeight"),
    (&[7, 3, 150, 127, 148, 40, 61, 200], "AddLiquidityByStrategy"),
    (&[94, 155, 103, 151, 70, 95, 220, 165], "AddLiquidityOneSide"),
    (&[41, 5, 238, 175, 100, 225, 6, 205], "AddLiquidityByStrategyOneSide"),
    (&[161, 194, 103, 84, 171, 71, 250, 154], "AddLiquidityOneSidePrecise"),
    (&[80, 85, 209, 72, 24, 206, 177, 108], "RemoveLiquidity"),
    (&[10, 51, 61, 35, 112, 105, 24, 85], "RemoveAllLiquidity"),
    (&[26, 82, 102, 152, 240, 74, 105, 26], "RemoveLiquidityByRange"),
    (&[84, 84, 177, 66, 254, 185, 10, 251], "RemoveLiquiditySingleSide"),
    (&[248, 198, 158, 145, 225, 117, 135, 200], "Swap"),
    (&[56, 173, 230, 208, 173, 228, 156, 205], "SwapWithPriceImpact"),
    (&[250, 73, 101, 33, 38, 207, 75, 184], "SwapExactOut"),
    (&[146, 72, 174, 224, 40, 253, 84, 174], "GoToABin"),
    (&[219, 192, 234, 71, 190, 191, 102, 80], "InitializePosition"),
    (&[46, 82, 125, 146, 85, 141, 228, 153], "InitializePositionPda"),
    (&[251, 189, 190, 244, 117, 254, 35, 148], "InitializePositionByOperator"),
    (&[123, 134, 81, 0, 49, 68, 98, 98], "ClosePosition"),
    (&[202, 184, 103, 143, 180, 191, 116, 217], "UpdatePositionOperator"),
    (&[222, 151, 70, 189, 137, 37, 157, 213], "ClaimLiquidity"),
    (&[169, 32, 79, 137, 136, 232, 70, 137], "ClaimFee"),
    (&[149, 95, 181, 242, 94, 90, 158, 162], "ClaimReward"),
    (&[158, 201, 158, 189, 33, 93, 162, 103], "WithdrawProtocolFee"),
    (&[128, 128, 208, 91, 246, 53, 31, 176], "UpdateFeeParameters"),
    (&[60, 63, 17, 64, 13, 196, 166, 243], "UpdateFeeOwner"),
    (&[95, 135, 192, 196, 242, 129, 230, 68], "InitializeReward"),
    (&[188, 50, 249, 165, 93, 151, 38, 63], "FundReward"),
    (&[211, 28, 48, 32, 215, 160, 35, 23], "UpdateRewardFunder"),
    (&[138, 174, 196, 169, 213, 235, 254, 107], "UpdateRewardDuration"),
    (&[148, 206, 42, 195, 247, 49, 103, 8], "WithdrawIneligibleReward"),
    (&[4, 148, 145, 100, 134, 26, 181, 61], "ClosePresetParameter"),
    (&[66, 188, 71, 211, 98, 109, 14, 186], "InitializePresetParameter"),
    (&[117, 199, 62, 103, 6, 142, 31, 203], "InitializePresetParameterV2"),
    (&[61, 115, 52, 23, 46, 13, 31, 144], "TogglePairStatus"),
    (&[4, 105, 92, 167, 132, 28, 9, 90], "UpdateWhitelistedWallet"),
    (&[190, 61, 125, 87, 103, 79, 158, 173], "IncreaseOracleLength"),
    (&[15, 132, 59, 50, 199, 6, 251, 46], "MigratePosition"),
    (&[17, 23, 159, 211, 101, 184, 41, 241], "MigrateBinArray"),
    (&[154, 230, 250, 13, 236, 209, 75, 223], "UpdateFeesAndRewards"),
    (&[207, 224, 170, 143, 189, 159, 46, 150], "SetLockReleaseSlot"),
    (&[200, 227, 90, 83, 27, 79, 191, 88], "SetActivationSlot"),
    (&[91, 249, 15, 165, 26, 129, 254, 125], "SetActivationPoint"),
    (&[181, 76, 219, 75, 16, 232, 212, 213], "SetMaxSwappedAmount"),
    (&[165, 61, 201, 244, 130, 159, 22, 100], "SetPreActivationDuration"),
    (&[57, 139, 47, 123, 216, 80, 223, 10], "SetPreActivationSwapAddress"),
    (&[228, 69, 165, 46, 81, 203, 154, 29], "EventLog"),
    (&[64, 244, 188, 120, 167, 233, 105, 10], "IdlWrite"),
];

/// Helper function to convert camelCase to snake_case
fn camel_to_snake(name: &str) -> String {
    let mut snake_case = String::new();
    for (i, ch) in name.chars().enumerate() {
        if ch.is_uppercase() && i != 0 {
            snake_case.push('_');
        }
        snake_case.push(ch.to_ascii_lowercase());
    }
    snake_case
}

/// Computes the discriminator bytes for an instruction name
pub fn compute_discriminator(instruction_name: &str) -> [u8; 8] {
    let formatted_name = format!("global:{}", camel_to_snake(instruction_name));
    let mut hasher = Sha256::new();
    hasher.update(formatted_name.as_bytes());
    let hash_result = hasher.finalize();
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&hash_result[0..8]);
    discriminator
}

/// Get instruction type from discriminator bytes
pub fn get_instruction_type(discriminator: &[u8]) -> Option<InstructionType> {
    if discriminator.len() != 8 {
        return None;
    }
    
    for (disc, name) in INSTRUCTION_TYPES.iter() {
        if *disc == discriminator {
            return match *name {
                "InitializeLbPair" => Some(InstructionType::InitializeLbPair),
                "InitializePermissionLbPair" => Some(InstructionType::InitializePermissionLbPair),
                "InitializeCustomizablePermissionlessLbPair" => Some(InstructionType::InitializeCustomizablePermissionlessLbPair),
                "CloseLbPair" => Some(InstructionType::CloseLbPair),
                "InitializeBinArray" => Some(InstructionType::InitializeBinArray),
                "InitializeBinArrayBitmapExtension" => Some(InstructionType::InitializeBinArrayBitmapExtension),
                "AddLiquidity" => Some(InstructionType::AddLiquidity),
                "AddLiquidityByWeight" => Some(InstructionType::AddLiquidityByWeight),
                "AddLiquidityByStrategy" => Some(InstructionType::AddLiquidityByStrategy),
                "AddLiquidityOneSide" => Some(InstructionType::AddLiquidityOneSide),
                "AddLiquidityByStrategyOneSide" => Some(InstructionType::AddLiquidityByStrategyOneSide),
                "AddLiquidityOneSidePrecise" => Some(InstructionType::AddLiquidityOneSidePrecise),
                "RemoveLiquidity" => Some(InstructionType::RemoveLiquidity),
                "RemoveAllLiquidity" => Some(InstructionType::RemoveAllLiquidity),
                "RemoveLiquidityByRange" => Some(InstructionType::RemoveLiquidityByRange),
                "RemoveLiquiditySingleSide" => Some(InstructionType::RemoveLiquiditySingleSide),
                "Swap" => Some(InstructionType::Swap),
                "SwapWithPriceImpact" => Some(InstructionType::SwapWithPriceImpact),
                "SwapExactOut" => Some(InstructionType::SwapExactOut),
                "GoToABin" => Some(InstructionType::GoToABin),
                "InitializePosition" => Some(InstructionType::InitializePosition),
                "InitializePositionPda" => Some(InstructionType::InitializePositionPda),
                "InitializePositionByOperator" => Some(InstructionType::InitializePositionByOperator),
                "ClosePosition" => Some(InstructionType::ClosePosition),
                "UpdatePositionOperator" => Some(InstructionType::UpdatePositionOperator),
                "ClaimLiquidity" => Some(InstructionType::ClaimLiquidity),
                "ClaimFee" => Some(InstructionType::ClaimFee),
                "ClaimReward" => Some(InstructionType::ClaimReward),
                "WithdrawProtocolFee" => Some(InstructionType::WithdrawProtocolFee),
                "UpdateFeeParameters" => Some(InstructionType::UpdateFeeParameters),
                "UpdateFeeOwner" => Some(InstructionType::UpdateFeeOwner),
                "InitializeReward" => Some(InstructionType::InitializeReward),
                "FundReward" => Some(InstructionType::FundReward),
                "UpdateRewardFunder" => Some(InstructionType::UpdateRewardFunder),
                "UpdateRewardDuration" => Some(InstructionType::UpdateRewardDuration),
                "WithdrawIneligibleReward" => Some(InstructionType::WithdrawIneligibleReward),
                "ClosePresetParameter" => Some(InstructionType::ClosePresetParameter),
                "InitializePresetParameter" => Some(InstructionType::InitializePresetParameter),
                "InitializePresetParameterV2" => Some(InstructionType::InitializePresetParameterV2),
                "TogglePairStatus" => Some(InstructionType::TogglePairStatus),
                "UpdateWhitelistedWallet" => Some(InstructionType::UpdateWhitelistedWallet),
                "IncreaseOracleLength" => Some(InstructionType::IncreaseOracleLength),
                "MigratePosition" => Some(InstructionType::MigratePosition),
                "MigrateBinArray" => Some(InstructionType::MigrateBinArray),
                "UpdateFeesAndRewards" => Some(InstructionType::UpdateFeesAndRewards),
                "SetLockReleaseSlot" => Some(InstructionType::SetLockReleaseSlot),
                "SetActivationSlot" => Some(InstructionType::SetActivationSlot),
                "SetActivationPoint" => Some(InstructionType::SetActivationPoint),
                "SetMaxSwappedAmount" => Some(InstructionType::SetMaxSwappedAmount),
                "SetPreActivationDuration" => Some(InstructionType::SetPreActivationDuration),
                "SetPreActivationSwapAddress" => Some(InstructionType::SetPreActivationSwapAddress),
                "EventLog" => Some(InstructionType::EventLog),
                "IdlWrite" => Some(InstructionType::IdlWrite),
                _ => Some(InstructionType::Unknown),
            };
        }
    }
    
    None
}

/// Convert an instruction type to string
pub fn instruction_type_to_str(inst_type: InstructionType) -> &'static str {
    match inst_type {
        InstructionType::InitializeLbPair => "InitializeLbPair",
        InstructionType::InitializePermissionLbPair => "InitializePermissionLbPair",
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
        InstructionType::Swap => "Swap",
        InstructionType::SwapWithPriceImpact => "SwapWithPriceImpact",
        InstructionType::SwapExactOut => "SwapExactOut",
        InstructionType::GoToABin => "GoToABin",
        InstructionType::InitializePosition => "InitializePosition",
        InstructionType::InitializePositionPda => "InitializePositionPda",
        InstructionType::InitializePositionByOperator => "InitializePositionByOperator",
        InstructionType::ClosePosition => "ClosePosition",
        InstructionType::UpdatePositionOperator => "UpdatePositionOperator",
        InstructionType::ClaimLiquidity => "ClaimLiquidity",
        InstructionType::ClaimFee => "ClaimFee",
        InstructionType::ClaimReward => "ClaimReward",
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
        InstructionType::InitializePresetParameterV2 => "InitializePresetParameterV2",
        InstructionType::TogglePairStatus => "TogglePairStatus",
        InstructionType::UpdateWhitelistedWallet => "UpdateWhitelistedWallet",
        InstructionType::IncreaseOracleLength => "IncreaseOracleLength",
        InstructionType::MigratePosition => "MigratePosition",
        InstructionType::MigrateBinArray => "MigrateBinArray",
        InstructionType::UpdateFeesAndRewards => "UpdateFeesAndRewards",
        InstructionType::SetLockReleaseSlot => "SetLockReleaseSlot",
        InstructionType::SetActivationSlot => "SetActivationSlot",
        InstructionType::SetActivationPoint => "SetActivationPoint",
        InstructionType::SetMaxSwappedAmount => "SetMaxSwappedAmount",
        InstructionType::SetPreActivationDuration => "SetPreActivationDuration",
        InstructionType::SetPreActivationSwapAddress => "SetPreActivationSwapAddress",
        InstructionType::EventLog => "EventLog",
        InstructionType::IdlWrite => "IdlWrite",
        InstructionType::Unknown => "Unknown",
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
    let instruction_type = get_instruction_type(discriminator).unwrap_or(InstructionType::Unknown);
    
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
    let meta = Meta {
        tx_id: tx_id.to_string(),
        block_slot,
        block_time,
        block_date,
        instruction_index,
        is_inner_instruction,
        inner_instruction_index,
        signer: signer_pubkey.map(String::from),
        outer_program: outer_program.map(String::from),
        instruction_type: instruction_type_str.to_string(),
        input_accounts,
        args,
    };

    Some(meta)
}

/// Get instruction type string from discriminator
fn get_instruction_type_from_discriminator(discriminator: &[u8]) -> Option<&'static str> {
    for (disc, name) in INSTRUCTION_TYPES.iter() {
        if *disc == discriminator {
            return Some(name);
        }
    }
    None
}

/// Process instruction data to extract arguments
pub fn process_instruction_data(data: &[u8], discriminator: &[u8]) -> Option<InstructionArgs> {
    let inst_type = get_instruction_type_from_discriminator(discriminator)?;
    
    // Create a basic InstructionArgs object
    let mut args = InstructionArgs {
        instruction_args: None,
    };
    
    // Process based on instruction type
    match inst_type {
        "Swap" => {
            let amount_in = u64::from_le_bytes(data[8..16].try_into().unwrap_or_default()).to_string();
            let min_amount_out = u64::from_le_bytes(data[16..24].try_into().unwrap_or_default()).to_string();
            log::info!("Parsed Swap: amountIn={}, minAmountOut={}", amount_in, min_amount_out);
            let layout = PbSwapLayout {
                amount_in: Some(amount_in),
                min_amount_out: Some(min_amount_out),
            };
            args.instruction_args = Some(IArgs::Swap(layout));
        },
        
        "SwapWithPriceImpact" => {
            let amount_in = parse_u64(data, 8).ok();
            let active_id = parse_i32(data, 16).ok();
            let max_price_impact_bps = parse_i16(data, 20).ok().map(|v| v as i32);
             log::info!("Parsed SwapWithPriceImpact: amountIn={:?}, activeId={:?}, maxPriceImpactBps={:?}",
                       amount_in, active_id, max_price_impact_bps);
            let layout = PbSwapWithPriceImpactLayout {
                amount_in,
                active_id,
                max_price_impact_bps,
            };
            args.instruction_args = Some(IArgs::SwapWithPriceImpact(layout));
        },
        
        "SwapExactOut" => {
            let max_in_amount = parse_u64(data, 8).ok();
            let out_amount = parse_u64(data, 16).ok();
            log::info!("Parsed SwapExactOut: maxInAmount={:?}, outAmount={:?}",
                      max_in_amount, out_amount);
            let layout = PbSwapExactOutLayout {
                max_in_amount,
                out_amount,
            };
            args.instruction_args = Some(IArgs::SwapExactOut(layout));
        },
        
        "InitializeLbPair" | "InitializePermissionLbPair" => {
            let active_id = parse_i32(data, 8).ok();
            let bin_step = parse_i16(data, 12).ok().map(|v| v as i32);
            log::info!("Parsed Initialize(Permission)LbPair: activeId={:?}, binStep={:?}",
                      active_id, bin_step);
            if inst_type == "InitializeLbPair" {
                let layout = PbInitializeLbPairLayout {
                    active_id,
                    bin_step,
                };
                args.instruction_args = Some(IArgs::InitializeLbPair(layout));
            } else {
                let layout = PbInitializePermissionLbPairLayout {
                    active_id,
                    bin_step,
                };
                args.instruction_args = Some(IArgs::InitializePermissionLbPair(layout));
            }
        },
        
        "InitializePosition" | "InitializePositionPda" => {
             let lower_bin_id = parse_i32(data, 8).ok();
             let width = parse_i32(data, 12).ok();
            log::info!("Parsed InitializePosition(Pda): lowerBinId={:?}, width={:?}",
                      lower_bin_id, width);
            if inst_type == "InitializePosition" {
                let layout = PbInitializePositionLayout {
                    lower_bin_id,
                    width,
                };
                args.instruction_args = Some(IArgs::InitializePosition(layout));
            } else {
                let layout = PbInitializePositionPdaLayout {
                    lower_bin_id,
                    width,
                };
                args.instruction_args = Some(IArgs::InitializePositionPda(layout));
            }
        },
        
        "ClosePosition" => {
            let layout = PbClosePositionLayout {};
            args.instruction_args = Some(IArgs::ClosePosition(layout));
        },
        
        "ClaimFee" => {
            let layout = PbCollectFeesLayout {};
            args.instruction_args = Some(IArgs::CollectFees(layout));
        },
        
        "ClaimReward" => {
            let reward_index = parse_i64(data, 8).ok();
            log::info!("Parsed ClaimReward: rewardIndex={:?}", reward_index);
            let layout = PbCollectRewardLayout {
                reward_index: reward_index.map(|i| i as u32).unwrap_or(0),
            };
            args.instruction_args = Some(IArgs::CollectReward(layout));
        },
        
        "InitializeCustomizablePermissionlessLbPair" => {
             log::info!("Parsing InitializeCustomizablePermissionlessLbPair");
             // Parse directly into the flattened layout
             let layout = PbInitializeCustomizablePermissionlessLbPairLayout {
                 active_id: parse_i32(data, 8).ok(),
                 bin_step: parse_i16(data, 12).ok().map(|v| v as i32),
                 base_factor: parse_i16(data, 14).ok().map(|v| v as i32),
                 activation_type: if data.len() > 16 { Some(data[16] as u32) } else { None },
                 has_alpha_vault: if data.len() > 17 { Some(data[17] != 0) } else { None },
                 activation_point: parse_i64(data, 18).ok(),
             };
             args.instruction_args = Some(IArgs::InitializeCustomizablePermissionlessLbPair(layout));
        },
        "CloseLbPair" => {
            let layout = PbCloseLbPairLayout {};
            args.instruction_args = Some(IArgs::CloseLbPair(layout));
        },
        "InitializeBinArray" => {
            let index = parse_i64(data, 8).ok();
            log::info!("Parsed InitializeBinArray: index={:?}", index);
            let layout = PbInitializeBinArrayLayout { index };
            args.instruction_args = Some(IArgs::InitializeBinArray(layout));
        },
        "InitializeBinArrayBitmapExtension" => {
             let layout = PbInitializeBinArrayBitmapExtensionLayout {};
             args.instruction_args = Some(IArgs::InitializeBinArrayBitmapExtension(layout));
        },
        "AddLiquidity" => {
            log::info!("Parsing AddLiquidity (stub)");
            // Initialize with default/placeholder values for required fields
            let layout = PbAddLiquidityLayout {
                tick_lower_index: 0,
                tick_upper_index: 0,
                liquidity_amount: "0".to_string(),
                token_max_a: 0,
                token_max_b: 0,
                liquidity_parameter: None,
            };
            args.instruction_args = Some(IArgs::AddLiquidity(layout));
        },
        "AddLiquidityByWeight" => {
            log::info!("Parsing AddLiquidityByWeight (stub)");
            let layout = PbAddLiquidityByWeightLayout { liquidity_parameter: None };
            args.instruction_args = Some(IArgs::AddLiquidityByWeight(layout));
        },
        "AddLiquidityByStrategy" => {
            log::info!("Parsing AddLiquidityByStrategy (stub)");
            let layout = PbAddLiquidityByStrategyLayout { liquidity_parameter: None };
            args.instruction_args = Some(IArgs::AddLiquidityByStrategy(layout));
        },
        "AddLiquidityOneSide" => {
            log::info!("Parsing AddLiquidityOneSide");
            let amount = parse_u64(data, 8).ok();
            let active_id = parse_i32(data, 16).ok();
            let max_active_bin_slippage = parse_i32(data, 20).ok();

            let mut current_offset = 24;
            let bin_dist_len = parse_u32(data, current_offset).unwrap_or(0);
            current_offset += 4;
            
            let mut bin_liquidity_dist = Vec::with_capacity(bin_dist_len as usize);
            for _ in 0..bin_dist_len {
                if current_offset + 6 > data.len() { // Check for sufficient data (i32 + u16)
                    log::info!("AddLiquidityOneSide: Data too short for bin distribution element");
                    break; 
                }
                let bin_id = parse_i32(data, current_offset).ok();
                current_offset += 4;
                let weight = parse_i16(data, current_offset).ok().map(|v| v as i32); // Read u16 as i16, then cast
                current_offset += 2;
                bin_liquidity_dist.push(PbBinLiquidityDistributionByWeightLayout {
                    bin_id,
                    weight,
                });
            }
            
            log::info!("Parsed AddLiquidityOneSide: amount={:?}, active_id={:?}, max_active_bin_slippage={:?}, dist_len={}", 
                      amount, active_id, max_active_bin_slippage, bin_dist_len);

            // Create the flattened layout directly
            let layout = PbAddLiquidityOneSideLayout {
                amount,
                active_id,
                max_active_bin_slippage,
                bin_liquidity_dist,
            };
            args.instruction_args = Some(IArgs::AddLiquidityOneSide(layout));
        },
        "AddLiquidityByStrategyOneSide" => {
            log::info!("Parsing AddLiquidityByStrategyOneSide (stub)");
            let layout = PbAddLiquidityByStrategyOneSideLayout { liquidity_parameter: None };
            args.instruction_args = Some(IArgs::AddLiquidityByStrategyOneSide(layout));
        },
        "AddLiquidityOneSidePrecise" => {
            log::info!("Parsing AddLiquidityOneSidePrecise");
            // Parse AddLiquiditySingleSidePreciseParameterLayout fields
            let mut current_offset = 8;
            let bins_len = parse_u32(data, current_offset).unwrap_or(0);
            current_offset += 4;
            
            let mut bins = Vec::with_capacity(bins_len as usize);
            for _ in 0..bins_len {
                if current_offset + 8 > data.len() { // Check for sufficient data (i32 + u32)
                    log::info!("AddLiquidityOneSidePrecise: Data too short for bin element");
                    break; 
                }
                let bin_id = parse_i32(data, current_offset).ok();
                current_offset += 4;
                let amount = parse_u32(data, current_offset).ok().map(|v| v as i32);
                current_offset += 4;
                bins.push(PbCompressedBinDepositAmountLayout {
                    bin_id,
                    amount,
                });
            }

            let decompress_multiplier = parse_u64(data, current_offset).ok().map(|v| v as i64);
            
            log::info!("Parsed AddLiquidityOneSidePrecise: bins_len={}, decompress_multiplier={:?}", bins_len, decompress_multiplier);

            // Create flattened layout
            let layout = PbAddLiquidityOneSidePreciseLayout {
                bins,
                decompress_multiplier,
            };
            args.instruction_args = Some(IArgs::AddLiquidityOneSidePrecise(layout));
        },
        "RemoveLiquidity" => {
            log::info!("Parsing RemoveLiquidity (stub)");
            // Use default/placeholder values for required fields
            let layout = PbRemoveLiquidityLayout {
                tick_lower_index: 0,
                tick_upper_index: 0,
                liquidity_amount: "0".to_string(),
                token_min_a: 0,
                token_min_b: 0,
                bin_liquidity_removal: vec![],
            };
            args.instruction_args = Some(IArgs::RemoveLiquidity(layout));
        },
        "RemoveAllLiquidity" => {
            let layout = PbRemoveAllLiquidityLayout {};
            args.instruction_args = Some(IArgs::RemoveAllLiquidity(layout));
        },
        "RemoveLiquidityByRange" => {
            let from_bin_id = parse_i32(data, 8).ok();
            let to_bin_id = parse_i32(data, 12).ok();
            let bps_to_remove = parse_i16(data, 16).ok().map(|v| v as i32);
            log::info!("Parsed RemoveLiquidityByRange: from={:?}, to={:?}, bps={:?}", from_bin_id, to_bin_id, bps_to_remove);
            let layout = PbRemoveLiquidityByRangeLayout { from_bin_id, to_bin_id, bps_to_remove };
            args.instruction_args = Some(IArgs::RemoveLiquidityByRange(layout));
        },
        "RemoveLiquiditySingleSide" => {
            let layout = PbRemoveLiquiditySingleSideLayout {};
            args.instruction_args = Some(IArgs::RemoveLiquiditySingleSide(layout));
        },
        "GoToABin" => {
            let bin_id = parse_i32(data, 8).ok();
            log::info!("Parsed GoToABin: bin_id={:?}", bin_id);
            let layout = PbGoToABinLayout { bin_id };
            args.instruction_args = Some(IArgs::GoToABin(layout));
        },
         "InitializePositionByOperator" => {
            let lower_bin_id = parse_i32(data, 8).ok();
            let width = parse_i32(data, 12).ok();
            let owner = bytes_to_pubkey_str(data, 16).ok();
            log::info!("Parsed InitializePositionByOperator: lower={:?}, width={:?}, owner={:?}", lower_bin_id, width, owner.as_deref().unwrap_or("None"));
            let layout = PbInitializePositionByOperatorLayout { lower_bin_id, width, owner };
             args.instruction_args = Some(IArgs::InitializePositionByOperator(layout));
        },
        "UpdatePositionOperator" => {
            let operator = bytes_to_pubkey_str(data, 8).ok();
            log::info!("Parsed UpdatePositionOperator: operator={:?}", operator.as_deref().unwrap_or("None"));
            let layout = PbUpdatePositionOperatorLayout { operator };
            args.instruction_args = Some(IArgs::UpdatePositionOperator(layout));
        },
        "ClaimLiquidity" => {
            log::info!("Parsing ClaimLiquidity (no args defined)");
            let layout = PbClaimLiquidityLayout {};
            args.instruction_args = Some(IArgs::ClaimLiquidity(layout));
        },
        "WithdrawProtocolFee" => {
            let amount_x = parse_u64(data, 8).ok();
            let amount_y = parse_u64(data, 16).ok();
             log::info!("Parsed WithdrawProtocolFee: x={:?}, y={:?}", amount_x, amount_y);
            let layout = PbWithdrawProtocolFeeLayout { amount_x, amount_y };
            args.instruction_args = Some(IArgs::WithdrawProtocolFee(layout));
        },
        "UpdateFeeParameters" => {
            log::info!("Parsing UpdateFeeParameters");
            // Parse FeeParameterLayout fields directly
            let protocol_share = parse_i16(data, 8).ok().map(|v| v as i32);
            let base_factor = parse_i16(data, 10).ok().map(|v| v as i32);
            
            log::info!("Parsed UpdateFeeParameters: protocol_share={:?}, base_factor={:?}", protocol_share, base_factor);
            
            let layout = PbUpdateFeeParametersLayout { 
                protocol_share,
                base_factor,
            }; 
            args.instruction_args = Some(IArgs::UpdateFeeParameters(layout));
        },
        "UpdateFeeOwner" => {
            let layout = PbUpdateFeeOwnerLayout {};
            args.instruction_args = Some(IArgs::UpdateFeeOwner(layout));
        },
         "InitializeReward" => {
            let reward_index = parse_i64(data, 8).ok();
            let reward_duration = parse_i64(data, 16).ok();
            let funder = bytes_to_pubkey_str(data, 24).ok();
            log::info!("Parsed InitializeReward: index={:?}, duration={:?}, funder={:?}", reward_index, reward_duration, funder.as_deref().unwrap_or("None"));
            
            // Use default values for required fields of PbInitializeRewardLayout
            let layout = PbInitializeRewardLayout {
                emissions_per_second_x64: "0".to_string(), // Default value
                open_time: 0, // Default value
                end_time: reward_duration.unwrap_or(0) as u64,
            };
            args.instruction_args = Some(IArgs::InitializeReward(layout));
        },
        "FundReward" => {
            let reward_index = parse_i64(data, 8).ok();
            let amount = parse_i64(data, 16).ok();
            let carry_forward = if data.len() > 24 { Some(data[24] != 0) } else { None };
            log::info!("Parsed FundReward: index={:?}, amount={:?}, carry={:?}", reward_index, amount, carry_forward);
            let layout = PbFundRewardLayout { reward_index, amount, carry_forward };
            args.instruction_args = Some(IArgs::FundReward(layout));
        },
        "UpdateRewardFunder" => {
             let reward_index = parse_i64(data, 8).ok();
             let new_funder = bytes_to_pubkey_str(data, 16).ok();
             log::info!("Parsed UpdateRewardFunder: index={:?}, new_funder={:?}", reward_index, new_funder.as_deref().unwrap_or("None"));
            let layout = PbUpdateRewardFunderLayout { reward_index, new_funder };
            args.instruction_args = Some(IArgs::UpdateRewardFunder(layout));
        },
        "UpdateRewardDuration" => {
             let reward_index = parse_i64(data, 8).ok();
             let new_duration = parse_i64(data, 16).ok();
             log::info!("Parsed UpdateRewardDuration: index={:?}, new_duration={:?}", reward_index, new_duration);
            let layout = PbUpdateRewardDurationLayout { reward_index, new_duration };
            args.instruction_args = Some(IArgs::UpdateRewardDuration(layout));
        },
        "WithdrawIneligibleReward" => {
            let reward_index = parse_i64(data, 8).ok();
            log::info!("Parsed WithdrawIneligibleReward: reward_index={:?}", reward_index);
            let layout = PbWithdrawIneligibleRewardLayout { reward_index };
            args.instruction_args = Some(IArgs::WithdrawIneligibleReward(layout));
        },
        "ClosePresetParameter" => {
             let layout = PbClosePresetParameterLayout {};
             args.instruction_args = Some(IArgs::ClosePresetParameter(layout));
        },
        "InitializePresetParameter" | "InitializePresetParameterV2" => {
            log::info!("Parsing InitializePresetParameter(V2)");
            let bin_step = parse_i16(data, 8).ok().map(|v| v as i32);
            let base_factor = parse_i16(data, 10).ok().map(|v| v as i32);
            let filter_period = parse_i16(data, 12).ok().map(|v| v as i32);
            let decay_period = parse_i16(data, 14).ok().map(|v| v as i32);
            let reduction_factor = parse_i16(data, 16).ok().map(|v| v as i32);
            let variable_fee_control = parse_i32(data, 18).ok();
            let max_volatility_accumulator = parse_i32(data, 22).ok();
            let min_bin_id = parse_i32(data, 26).ok();
            let max_bin_id = parse_i32(data, 30).ok();
            let protocol_share = parse_i16(data, 34).ok().map(|v| v as i32);
                
            if inst_type == "InitializePresetParameter" {
                log::info!("Parsed InitializePresetParameter: bin_step={:?}, base_factor={:?}, filter_period={:?}, decay_period={:?}, reduction_factor={:?}, variable_fee_control={:?}, max_volatility_accumulator={:?}, min_bin_id={:?}, max_bin_id={:?}, protocol_share={:?}", 
                          bin_step, base_factor, filter_period, decay_period, reduction_factor, variable_fee_control, max_volatility_accumulator, min_bin_id, max_bin_id, protocol_share);
                
                // Create flattened layout directly
                let layout = PbInitializePresetParameterLayout {
                    bin_step, 
                    base_factor, 
                    filter_period, 
                    decay_period,
                    reduction_factor, 
                    variable_fee_control, 
                    max_volatility_accumulator,
                    min_bin_id, 
                    max_bin_id, 
                    protocol_share,
                };
                args.instruction_args = Some(IArgs::InitializePresetParameter(layout));
            } else { // InitializePresetParameterV2
                let host_fee = parse_i16(data, 36).ok().map(|v| v as i32);
                
                log::info!("Parsed InitializePresetParameterV2: bin_step={:?}, base_factor={:?}, filter_period={:?}, decay_period={:?}, reduction_factor={:?}, variable_fee_control={:?}, max_volatility_accumulator={:?}, min_bin_id={:?}, max_bin_id={:?}, protocol_share={:?}, host_fee={:?}", 
                          bin_step, base_factor, filter_period, decay_period, reduction_factor, variable_fee_control, max_volatility_accumulator, min_bin_id, max_bin_id, protocol_share, host_fee);
                
                // Create flattened layout directly
                let layout = PbInitializePresetParameterV2Layout {
                    bin_step, 
                    base_factor, 
                    filter_period, 
                    decay_period,
                    reduction_factor, 
                    variable_fee_control, 
                    max_volatility_accumulator,
                    min_bin_id, 
                    max_bin_id, 
                    protocol_share,
                    host_fee,
                };
                args.instruction_args = Some(IArgs::InitializePresetParameterV2(layout));
            }
        },
        "TogglePairStatus" => {
            let layout = PbTogglePairStatusLayout{};
            args.instruction_args = Some(IArgs::TogglePairStatus(layout));
        },
        "UpdateWhitelistedWallet" => {
            let idx = parse_u32(data, 8).ok();
            let wallet = bytes_to_pubkey_str(data, 12).ok();
            log::info!("Parsed UpdateWhitelistedWallet: idx={:?}, wallet={:?}", idx, wallet.as_deref().unwrap_or("None"));
            let layout = PbUpdateWhitelistedWalletLayout { 
                idx: idx.map(|v| v as i32), 
                wallet 
            };
            args.instruction_args = Some(IArgs::UpdateWhitelistedWallet(layout));
        },
        "IncreaseOracleLength" => {
            let length_to_add = parse_i64(data, 8).ok();
            log::info!("Parsed IncreaseOracleLength: length_to_add={:?}", length_to_add);
             let layout = PbIncreaseOracleLengthLayout { length_to_add };
             args.instruction_args = Some(IArgs::IncreaseOracleLength(layout));
        },
        "MigratePosition" => {
             let layout = PbMigratePositionLayout {};
             args.instruction_args = Some(IArgs::MigratePosition(layout));
        },
        "MigrateBinArray" => {
             let layout = PbMigrateBinArrayLayout {};
             args.instruction_args = Some(IArgs::MigrateBinArray(layout));
        },
        "UpdateFeesAndRewards" => {
             let layout = PbUpdateFeesAndRewardsLayout {};
             args.instruction_args = Some(IArgs::UpdateFeesAndRewards(layout));
        },
        "SetLockReleaseSlot" => {
            let new_lock_release_slot = parse_i64(data, 8).ok();
            log::info!("Parsed SetLockReleaseSlot: slot={:?}", new_lock_release_slot);
            let layout = PbSetLockReleaseSlotLayout { new_lock_release_slot };
            args.instruction_args = Some(IArgs::SetLockReleaseSlot(layout));
        },
        "SetActivationSlot" => {
             let activation_slot = parse_i64(data, 8).ok();
             log::info!("Parsed SetActivationSlot: slot={:?}", activation_slot);
            let layout = PbSetActivationSlotLayout { activation_slot };
            args.instruction_args = Some(IArgs::SetActivationSlot(layout));
        },
        "SetMaxSwappedAmount" => {
            let swap_cap_deactivate_slot = parse_u64(data, 8).ok();
            let max_swapped_amount = parse_u64(data, 16).ok();
            log::info!("Parsed SetMaxSwappedAmount: slot={:?}, amount={:?}", swap_cap_deactivate_slot, max_swapped_amount);
            let layout = PbSetMaxSwappedAmountLayout { 
                swap_cap_deactivate_slot: swap_cap_deactivate_slot.map(|v| v as i64), 
                max_swapped_amount: max_swapped_amount.map(|v| v.to_string()),
            };
            args.instruction_args = Some(IArgs::SetMaxSwappedAmount(layout));
        },
        "SetPreActivationDuration" => {
            let pre_activation_duration = parse_i64(data, 8).ok();
             log::info!("Parsed SetPreActivationDuration: duration={:?}", pre_activation_duration);
            let layout = PbSetPreActivationDurationLayout { pre_activation_duration };
            args.instruction_args = Some(IArgs::SetPreActivationDuration(layout));
        },
        "SetPreActivationSwapAddress" => {
            let pre_activation_swap_address = bytes_to_pubkey_str(data, 8).ok();
            log::info!("Parsed SetPreActivationSwapAddress: address={:?}", pre_activation_swap_address.as_deref().unwrap_or("None"));
            let layout = PbSetPreActivationSwapAddressLayout { pre_activation_swap_address };
            args.instruction_args = Some(IArgs::SetPreActivationSwapAddress(layout));
        },
         "IdlWrite" => {
            let layout = PbIdlWriteLayout {};
            args.instruction_args = Some(IArgs::IdlWrite(layout));
        },
        "EventLog" => {
            if data.len() < 16 {
                log::info!("EventLog data too short to contain event discriminator");
                return None;
            }
            let event_discriminator = &data[8..16];
            let event_data = &data[16..];

            let event_args_option: Option<InstructionArgs> =
                if event_discriminator == EVENT_SWAP_DISCRIMINATOR {
                    parse_event_wrapper(event_data, "SwapLog", |d| {
                        if d.len() < 113 { return Err("SwapLog data too short"); }
                        Ok(PbSwapLogFields {
                            lb_pair: bytes_to_pubkey_str(d, 0)?,
                            from: bytes_to_pubkey_str(d, 32)?,
                            start_bin_id: Some(parse_i32(d, 64)?),
                            end_bin_id: Some(parse_i32(d, 68)?),
                            amount_in: Some(parse_u64(d, 72)?),
                            amount_out: Some(parse_u64(d, 80)?),
                            swap_for_y: Some(d[88] != 0),
                            fee: Some(parse_u64(d, 89)?),
                            protocol_fee: Some(parse_u64(d, 97)?),
                            fee_bps: parse_u128(d, 105)?.to_string(),
                            host_fee: Some(parse_u64(d, 121)?),
                        })
                    }, pb_event_log_wrapper::EventFields::SwapLogFields)
                } else if event_discriminator == EVENT_COMPOSITION_FEE_DISCRIMINATOR {
                    parse_event_wrapper(event_data, "CompositionFeeLog", |d| {
                        if d.len() < 66 { return Err("CompositionFeeLog data too short"); }
                        Ok(PbCompositionFeeLogFields {
                           from: bytes_to_pubkey_str(d, 0)?,
                           bin_id: Some(parse_i16(d, 32)? as i32),
                           token_x_fee_amount: Some(parse_u64(d, 34)?),
                           token_y_fee_amount: Some(parse_u64(d, 42)?),
                           protocol_token_x_fee_amount: Some(parse_u64(d, 50)?),
                           protocol_token_y_fee_amount: Some(parse_u64(d, 58)?),
                        })
                    }, pb_event_log_wrapper::EventFields::CompositionFeeLogFields)
                } else if event_discriminator == EVENT_ADD_LIQUIDITY_DISCRIMINATOR {
                    parse_event_wrapper(event_data, "AddLiquidityLog", |d| {
                         if d.len() < 116 { return Err("AddLiquidityLog data too short"); }
                         let amounts = vec![parse_u64(d, 96)?, parse_u64(d, 104)?];
                         Ok(PbAddLiquidityLogFields {
                            lb_pair: bytes_to_pubkey_str(d, 0)?,
                            from: bytes_to_pubkey_str(d, 32)?,
                            position: bytes_to_pubkey_str(d, 64)?,
                            amounts,
                            active_bin_id: Some(parse_i32(d, 112)?),
                         })
                    }, pb_event_log_wrapper::EventFields::AddLiquidityLogFields)
                } else if event_discriminator == EVENT_REMOVE_LIQUIDITY_DISCRIMINATOR {
                    parse_event_wrapper(event_data, "RemoveLiquidityLog", |d| {
                        if d.len() < 116 { return Err("RemoveLiquidityLog data too short"); }
                        let amounts = vec![parse_u64(d, 96)?, parse_u64(d, 104)?];
                        Ok(PbRemoveLiquidityLogFields {
                            lb_pair: bytes_to_pubkey_str(d, 0)?,
                            from: bytes_to_pubkey_str(d, 32)?,
                            position: bytes_to_pubkey_str(d, 64)?,
                            amounts,
                            active_bin_id: Some(parse_i32(d, 112)?),
                        })
                    }, pb_event_log_wrapper::EventFields::RemoveLiquidityLogFields)
                 } else if event_discriminator == EVENT_CLAIM_REWARD_DISCRIMINATOR {
                    parse_event_wrapper(event_data, "ClaimRewardLog", |d| {
                        if d.len() < 112 { return Err("ClaimRewardLog data too short"); }
                        Ok(PbClaimRewardLogFields {
                            lb_pair: bytes_to_pubkey_str(d, 0)?,
                            position: bytes_to_pubkey_str(d, 32)?,
                            owner: bytes_to_pubkey_str(d, 64)?,
                            reward_index: Some(parse_i64(d, 96)?),
                            total_reward: Some(parse_i64(d, 104)?),
                        })
                    }, pb_event_log_wrapper::EventFields::ClaimRewardLogFields)
                 } else if event_discriminator == EVENT_FUND_REWARD_DISCRIMINATOR {
                     parse_event_wrapper(event_data, "FundRewardLog", |d| {
                        if d.len() < 80 { return Err("FundRewardLog data too short"); }
                         Ok(PbFundRewardLogFields {
                             lb_pair: bytes_to_pubkey_str(d, 0)?,
                             funder: bytes_to_pubkey_str(d, 32)?,
                             reward_index: Some(parse_i64(d, 64)?),
                             amount: Some(parse_i64(d, 72)?),
                         })
                    }, pb_event_log_wrapper::EventFields::FundRewardLogFields)
                 } else if event_discriminator == EVENT_INITIALIZE_REWARD_DISCRIMINATOR {
                    parse_event_wrapper(event_data, "InitializeRewardLog", |d| {
                        if d.len() < 112 { return Err("InitializeRewardLog data too short"); }
                        Ok(PbInitializeRewardLogFields {
                            lb_pair: bytes_to_pubkey_str(d, 0)?,
                            reward_mint: bytes_to_pubkey_str(d, 32)?,
                            funder: bytes_to_pubkey_str(d, 64)?,
                            reward_index: Some(parse_i64(d, 96)?),
                            reward_duration: Some(parse_i64(d, 104)?),
                        })
                    }, pb_event_log_wrapper::EventFields::InitializeRewardLogFields)
                 } else if event_discriminator == EVENT_UPDATE_REWARD_DURATION_DISCRIMINATOR {
                     parse_event_wrapper(event_data, "UpdateRewardDurationLog", |d| {
                        if d.len() < 56 { return Err("UpdateRewardDurationLog data too short"); }
                         Ok(PbUpdateRewardDurationLogFields {
                             lb_pair: bytes_to_pubkey_str(d, 0)?,
                             reward_index: Some(parse_i64(d, 32)?),
                             old_reward_duration: Some(parse_i64(d, 40)?),
                             new_reward_duration: Some(parse_i64(d, 48)?),
                         })
                    }, pb_event_log_wrapper::EventFields::UpdateRewardDurationLogFields)
                 } else if event_discriminator == EVENT_UPDATE_REWARD_FUNDER_DISCRIMINATOR {
                     parse_event_wrapper(event_data, "UpdateRewardFunderLog", |d| {
                        if d.len() < 104 { return Err("UpdateRewardFunderLog data too short"); }
                         Ok(PbUpdateRewardFunderLogFields {
                             lb_pair: bytes_to_pubkey_str(d, 0)?,
                             reward_index: Some(parse_i64(d, 32)?),
                             old_funder: bytes_to_pubkey_str(d, 40)?,
                             new_funder: bytes_to_pubkey_str(d, 72)?,
                         })
                    }, pb_event_log_wrapper::EventFields::UpdateRewardFunderLogFields)
                 } else if event_discriminator == EVENT_POSITION_CLOSE_DISCRIMINATOR {
                     parse_event_wrapper(event_data, "PositionCloseLog", |d| {
                        if d.len() < 64 { return Err("PositionCloseLog data too short"); }
                         Ok(PbPositionCloseLogFields {
                             position: bytes_to_pubkey_str(d, 0)?,
                             owner: bytes_to_pubkey_str(d, 32)?,
                         })
                    }, pb_event_log_wrapper::EventFields::PositionCloseLogFields)
                 } else if event_discriminator == EVENT_CLAIM_FEE_DISCRIMINATOR {
                     parse_event_wrapper(event_data, "ClaimFeeLog", |d| {
                        if d.len() < 112 { return Err("ClaimFeeLog data too short"); }
                         Ok(PbClaimFeeLogFields {
                             lb_pair: bytes_to_pubkey_str(d, 0)?,
                             position: bytes_to_pubkey_str(d, 32)?,
                             owner: bytes_to_pubkey_str(d, 64)?,
                             fee_x: Some(parse_i64(d, 96)?),
                             fee_y: Some(parse_i64(d, 104)?),
                         })
                    }, pb_event_log_wrapper::EventFields::ClaimFeeLogFields)
                 } else if event_discriminator == EVENT_LB_PAIR_CREATE_DISCRIMINATOR {
                     parse_event_wrapper(event_data, "LbPairCreateLog", |d| {
                        if d.len() < 100 { return Err("LbPairCreateLog data too short"); }
                        Ok(PbLbPairCreateLogFields {
                            lb_pair: bytes_to_pubkey_str(d, 0)?,
                            bin_step: Some(parse_i16(d, 32)? as i32),
                            token_x: bytes_to_pubkey_str(d, 34)?,
                            token_y: bytes_to_pubkey_str(d, 66)?,
                        })
                    }, pb_event_log_wrapper::EventFields::LbPairCreateLogFields)
                 } else if event_discriminator == EVENT_POSITION_CREATE_DISCRIMINATOR {
                     parse_event_wrapper(event_data, "PositionCreateLog", |d| {
                        if d.len() < 96 { return Err("PositionCreateLog data too short"); }
                         Ok(PbPositionCreateLogFields {
                             lb_pair: bytes_to_pubkey_str(d, 0)?,
                             position: bytes_to_pubkey_str(d, 32)?,
                             owner: bytes_to_pubkey_str(d, 64)?,
                         })
                     }, pb_event_log_wrapper::EventFields::PositionCreateLogFields)
                 } else if event_discriminator == EVENT_FEE_PARAMETER_UPDATE_DISCRIMINATOR {
                     parse_event_wrapper(event_data, "FeeParameterUpdateLog", |d| {
                         if d.len() < 36 { return Err("FeeParameterUpdateLog data too short"); }
                         Ok(PbFeeParameterUpdateLogFields {
                             lb_pair: bytes_to_pubkey_str(d, 0)?,
                             protocol_share: Some(parse_i16(d, 32)? as i32), 
                             base_factor: Some(parse_i16(d, 34)? as i32),
                         })
                     }, pb_event_log_wrapper::EventFields::FeeParameterUpdateLogFields)
                 } else if event_discriminator == EVENT_INCREASE_OBSERVATION_DISCRIMINATOR {
                     parse_event_wrapper(event_data, "IncreaseObservationLog", |d| {
                         if d.len() < 40 { return Err("IncreaseObservationLog data too short"); }
                         Ok(PbIncreaseObservationLogFields {
                             oracle: bytes_to_pubkey_str(d, 0)?,
                             new_observation_length: Some(parse_i64(d, 32)?),
                         })
                     }, pb_event_log_wrapper::EventFields::IncreaseObservationLogFields)
                 } else if event_discriminator == EVENT_WITHDRAW_INELIGIBLE_REWARD_DISCRIMINATOR {
                     parse_event_wrapper(event_data, "WithdrawIneligibleRewardLog", |d| {
                         if d.len() < 72 { return Err("WithdrawIneligibleRewardLog data too short"); }
                         Ok(PbWithdrawIneligibleRewardLogFields {
                             lb_pair: bytes_to_pubkey_str(d, 0)?,
                             reward_mint: bytes_to_pubkey_str(d, 32)?,
                             amount: Some(parse_i64(d, 64)?),
                         })
                     }, pb_event_log_wrapper::EventFields::WithdrawIneligibleRewardLogFields)
                 } else if event_discriminator == EVENT_UPDATE_POSITION_OPERATOR_DISCRIMINATOR {
                     parse_event_wrapper(event_data, "UpdatePositionOperatorLog", |d| {
                         if d.len() < 96 { return Err("UpdatePositionOperatorLog data too short"); }
                         Ok(PbUpdatePositionOperatorLogFields {
                             position: bytes_to_pubkey_str(d, 0)?,
                             old_operator: bytes_to_pubkey_str(d, 32)?,
                             new_operator: bytes_to_pubkey_str(d, 64)?,
                         })
                     }, pb_event_log_wrapper::EventFields::UpdatePositionOperatorLogFields)
                 } else if event_discriminator == EVENT_UPDATE_POSITION_LOCK_RELEASE_SLOT_DISCRIMINATOR {
                     parse_event_wrapper(event_data, "UpdatePositionLockReleaseSlotLog", |d| {
                         if d.len() < 88 { return Err("UpdatePositionLockReleaseSlotLog data too short"); }
                         Ok(PbUpdatePositionLockReleaseSlotLogFields {
                             position: bytes_to_pubkey_str(d, 0)?,
                             current_slot: Some(parse_i64(d, 32)?),
                             new_lock_release_slot: Some(parse_i64(d, 40)?),
                             old_lock_release_slot: Some(parse_i64(d, 48)?),
                             sender: bytes_to_pubkey_str(d, 56)?,
                         })
                     }, pb_event_log_wrapper::EventFields::UpdatePositionLockReleaseSlotLogFields)
                 } else if event_discriminator == EVENT_GO_TO_A_BIN_DISCRIMINATOR {
                     parse_event_wrapper(event_data, "GoToABinLog", |d| {
                         if d.len() < 40 { return Err("GoToABinLog data too short"); }
                         Ok(PbGoToABinLogFields {
                             lb_pair: bytes_to_pubkey_str(d, 0)?,
                             from_bin_id: Some(parse_i32(d, 32)?),
                             to_bin_id: Some(parse_i32(d, 36)?),
                         })
                     }, pb_event_log_wrapper::EventFields::GoToABinLogFields)
                 } else if event_discriminator == EVENT_UPDATE_POSITION_LOCK_RELEASE_POINT_DISCRIMINATOR {
                    parse_event_wrapper(event_data, "UpdatePositionLockReleasePointLog", |d| {
                        if d.len() < 88 { return Err("UpdatePositionLockReleasePointLog data too short"); }
                        Ok(PbUpdatePositionLockReleasePointLogFields {
                            position: bytes_to_pubkey_str(d, 0)?,
                            current_point: Some(parse_i64(d, 32)?),
                            new_lock_release_point: Some(parse_i64(d, 40)?),
                            old_lock_release_point: Some(parse_i64(d, 48)?),
                            sender: bytes_to_pubkey_str(d, 56)?,
                        })
                    }, pb_event_log_wrapper::EventFields::UpdatePositionLockReleasePointLogFields)
                } else if event_discriminator == EVENT_UNKNOWN_EVENT1_DISCRIMINATOR {
                    parse_event_wrapper(event_data, "UnknownEvent1Log", |d| {
                         if d.len() < 112 { return Err("UnknownEvent1Log data too short"); }
                         Ok(PbUnknownEvent1LogFields {
                             vault: bytes_to_pubkey_str(d, 0)?,
                             escrow: bytes_to_pubkey_str(d, 32)?,
                             owner: bytes_to_pubkey_str(d, 64)?,
                             amount: Some(parse_i64(d, 96)?),
                             vault_total_claimed_token: Some(parse_i64(d, 104)?),
                         })
                    }, pb_event_log_wrapper::EventFields::UnknownEvent1LogFields)
                } else {
                     log::info!("Unknown event discriminator: {}", hex::encode(event_discriminator));
                     None
                 };

            return event_args_option;
        },
        _ => {
            log::info!("No specific parsing implemented for instruction type: {}", inst_type);
            args.instruction_args = None;
        }
    }
    
    if args.instruction_args.is_some() {
        Some(args)
    } else {
        None
    }
}

// Helper function to parse a fixed-size byte slice into a PubKey string
fn bytes_to_pubkey_str(data: &[u8], offset: usize) -> Result<String, &'static str> {
    if offset + 32 > data.len() {
        return Err("Data too short for PubKey");
    }
    Ok(bs58::encode(&data[offset..offset + 32]).into_string())
}

// Helper function to parse primitive types from byte slice
fn parse_u64(data: &[u8], offset: usize) -> Result<u64, &'static str> {
    if offset + 8 > data.len() { return Err("Data too short for u64"); }
    data[offset..offset+8].try_into().map(u64::from_le_bytes).map_err(|_| "Slice len mismatch for u64")
}
fn parse_i64(data: &[u8], offset: usize) -> Result<i64, &'static str> {
    if offset + 8 > data.len() { return Err("Data too short for i64"); }
    data[offset..offset+8].try_into().map(i64::from_le_bytes).map_err(|_| "Slice len mismatch for i64")
}
fn parse_i32(data: &[u8], offset: usize) -> Result<i32, &'static str> {
    if offset + 4 > data.len() { return Err("Data too short for i32"); }
    data[offset..offset+4].try_into().map(i32::from_le_bytes).map_err(|_| "Slice len mismatch for i32")
}
fn parse_u32(data: &[u8], offset: usize) -> Result<u32, &'static str> {
    if offset + 4 > data.len() { return Err("Data too short for u32"); }
    data[offset..offset+4].try_into().map(u32::from_le_bytes).map_err(|_| "Slice len mismatch for u32")
}
fn parse_i16(data: &[u8], offset: usize) -> Result<i16, &'static str> {
    if offset + 2 > data.len() { return Err("Data too short for i16"); }
    data[offset..offset+2].try_into().map(i16::from_le_bytes).map_err(|_| "Slice len mismatch for i16")
}
fn parse_u128(data: &[u8], offset: usize) -> Result<u128, &'static str> {
    if offset + 16 > data.len() { return Err("Data too short for u128"); }
    data[offset..offset+16].try_into().map(u128::from_le_bytes).map_err(|_| "Slice len mismatch for u128")
}

// Generic function to parse event data and create the wrapper
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
            log::info!("Failed to parse {} event data: {}. Data len: {}", event_name, e, event_data.len()); 
            None
        }
    }
} 