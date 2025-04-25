use chrono::{NaiveDateTime, Timelike};
use crate::pb::sf::solana::meteora_dlmm::v1::{
    Meta, 
    PbAddLiquidityLayout, PbSwapLayout, PbCollectFeesLayout, PbCollectRewardLayout,
    PbCreatePoolLayout, PbCreatePositionLayout, PbClosePositionLayout, PbInitializeRewardLayout,
    PbInitializeTickArrayLayout, PbRemoveLiquidityLayout, PbSetRewardEmissionsLayout,
    PbSwapRouterBaseInLayout, PbTransferPositionOwnerLayout, PbUpdateFeesLayout,
    PbInitializeConfigLayout, InstructionArgs,
    PbSwapWithPriceImpactLayout, PbSwapExactOutLayout,
    PbInitializeLbPairLayout, PbInitializePermissionLbPairLayout,
    PbInitializePositionLayout, PbInitializePositionPdaLayout
};
use crate::pb::sf::solana::meteora_dlmm::v1::instruction_args::InstructionArgs as IArgs;
use crate::prepare_input_accounts;
use sha2::{Digest, Sha256};
use substreams_solana::pb::sf::solana::r#type::v1::CompiledInstruction;
use hex;
use chrono::DateTime;
use substreams::log;

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
    
    // Unknown instruction
    Unknown,
}

// Meteora DLMM instruction types, based on the program's instruction discriminators
pub const INSTRUCTION_TYPES: [(&[u8], &str); 46] = [
    // Core functionality
    (&[45, 154, 237, 210, 221, 15, 166, 92], "InitializeLbPair"),
    (&[108, 102, 213, 85, 251, 3, 53, 21], "InitializePermissionLbPair"),
    (&[183, 18, 70, 156, 148, 109, 161, 34], "InitializeCustomizablePermissionlessLbPair"),
    (&[103, 211, 56, 22, 205, 102, 233, 116], "CloseLbPair"),
    (&[153, 218, 17, 177, 184, 150, 90, 88], "InitializeBinArray"),
    (&[142, 249, 220, 201, 189, 205, 114, 77], "InitializeBinArrayBitmapExtension"),
    (&[101, 161, 221, 122, 163, 141, 233, 193], "AddLiquidity"),
    (&[118, 208, 192, 90, 207, 177, 106, 61], "AddLiquidityByWeight"),
    (&[61, 70, 79, 38, 11, 64, 64, 76], "AddLiquidityByStrategy"),
    (&[48, 193, 176, 139, 204, 216, 41, 183], "AddLiquidityOneSide"),
    (&[35, 40, 116, 191, 13, 79, 9, 91], "AddLiquidityByStrategyOneSide"),
    (&[31, 109, 63, 244, 14, 60, 39, 246], "AddLiquidityOneSidePrecise"),
    (&[12, 172, 30, 129, 203, 41, 82, 110], "RemoveLiquidity"),
    (&[103, 80, 42, 165, 216, 14, 247, 136], "RemoveAllLiquidity"),
    (&[155, 202, 139, 165, 28, 89, 241, 52], "RemoveLiquidityByRange"),
    (&[19, 118, 19, 58, 239, 219, 130, 240], "RemoveLiquiditySingleSide"),
    (&[248, 198, 158, 145, 225, 117, 135, 200], "Swap"),
    (&[44, 243, 97, 123, 228, 38, 252, 92], "SwapWithPriceImpact"),
    (&[86, 8, 91, 216, 46, 174, 121, 116], "SwapExactOut"),
    (&[214, 179, 221, 226, 248, 200, 129, 83], "GoToABin"),
    (&[123, 226, 137, 211, 169, 83, 233, 70], "InitializePosition"),
    (&[213, 157, 193, 142, 226, 248, 243, 145], "InitializePositionPda"),
    (&[126, 164, 120, 132, 200, 149, 50, 194], "InitializePositionByOperator"),
    (&[74, 86, 76, 199, 95, 230, 27, 7], "ClosePosition"),
    (&[2, 46, 53, 68, 103, 231, 186, 171], "UpdatePositionOperator"),
    (&[222, 151, 70, 189, 137, 37, 157, 213], "ClaimLiquidity"),
    (&[87, 159, 69, 143, 212, 112, 184, 248], "ClaimFee"),
    (&[166, 54, 139, 52, 91, 23, 203, 231], "ClaimReward"),
    (&[108, 68, 244, 81, 174, 90, 23, 35], "WithdrawProtocolFee"),
    (&[86, 196, 123, 44, 181, 235, 246, 118], "UpdateFeeParameters"),
    (&[4, 15, 101, 41, 210, 136, 252, 39], "UpdateFeeOwner"),
    (&[86, 66, 196, 120, 208, 117, 132, 131], "InitializeReward"),
    (&[174, 228, 37, 143, 226, 150, 134, 141], "UpdateRewardFunder"),
    (&[62, 198, 204, 120, 183, 6, 62, 86], "UpdateRewardDuration"),
    (&[54, 176, 223, 185, 77, 70, 171, 102], "WithdrawIneligibleReward"),
    (&[143, 203, 207, 100, 193, 217, 187, 17], "ClosePresetParameter"),
    (&[19, 127, 246, 223, 137, 221, 112, 94], "InitializePresetParameter"),
    (&[225, 245, 143, 245, 25, 122, 110, 210], "InitializePresetParameterV2"),
    (&[138, 189, 124, 87, 198, 175, 108, 18], "TogglePairStatus"),
    (&[242, 35, 198, 137, 82, 225, 242, 182], "UpdateWhitelistedWallet"),
    (&[57, 86, 31, 11, 39, 88, 131, 93], "MigratePosition"),
    (&[103, 165, 202, 139, 121, 149, 118, 149], "MigrateBinArray"),
    (&[144, 100, 244, 169, 166, 151, 97, 223], "UpdateFeesAndRewards"),
    (&[66, 101, 149, 74, 56, 132, 91, 35], "SetLockReleaseSlot"),
    (&[47, 101, 22, 237, 221, 164, 185, 232], "SetActivationSlot"),
    (&[91, 249, 15, 165, 26, 129, 254, 125], "SetActivationPoint"),
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
                "UpdateRewardFunder" => Some(InstructionType::UpdateRewardFunder),
                "UpdateRewardDuration" => Some(InstructionType::UpdateRewardDuration),
                "WithdrawIneligibleReward" => Some(InstructionType::WithdrawIneligibleReward),
                "ClosePresetParameter" => Some(InstructionType::ClosePresetParameter),
                "InitializePresetParameter" => Some(InstructionType::InitializePresetParameter),
                "InitializePresetParameterV2" => Some(InstructionType::InitializePresetParameterV2),
                "TogglePairStatus" => Some(InstructionType::TogglePairStatus),
                "UpdateWhitelistedWallet" => Some(InstructionType::UpdateWhitelistedWallet),
                "MigratePosition" => Some(InstructionType::MigratePosition),
                "MigrateBinArray" => Some(InstructionType::MigrateBinArray),
                "UpdateFeesAndRewards" => Some(InstructionType::UpdateFeesAndRewards),
                "SetLockReleaseSlot" => Some(InstructionType::SetLockReleaseSlot),
                "SetActivationSlot" => Some(InstructionType::SetActivationSlot),
                "SetActivationPoint" => Some(InstructionType::SetActivationPoint),
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
        InstructionType::UpdateRewardFunder => "UpdateRewardFunder",
        InstructionType::UpdateRewardDuration => "UpdateRewardDuration",
        InstructionType::WithdrawIneligibleReward => "WithdrawIneligibleReward",
        InstructionType::ClosePresetParameter => "ClosePresetParameter",
        InstructionType::InitializePresetParameter => "InitializePresetParameter",
        InstructionType::InitializePresetParameterV2 => "InitializePresetParameterV2",
        InstructionType::TogglePairStatus => "TogglePairStatus",
        InstructionType::UpdateWhitelistedWallet => "UpdateWhitelistedWallet",
        InstructionType::MigratePosition => "MigratePosition",
        InstructionType::MigrateBinArray => "MigrateBinArray",
        InstructionType::UpdateFeesAndRewards => "UpdateFeesAndRewards",
        InstructionType::SetLockReleaseSlot => "SetLockReleaseSlot",
        InstructionType::SetActivationSlot => "SetActivationSlot",
        InstructionType::SetActivationPoint => "SetActivationPoint",
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
            // Parse amount_in (u64) from the first 8 bytes after discriminator
            let amount_in = if data.len() >= 16 {
                u64::from_le_bytes([
                    data[8], data[9], data[10], data[11], 
                    data[12], data[13], data[14], data[15]
                ])
            } else {
                log::info!("Swap instruction data too short for amountIn");
                0 // Default value but still include it
            };
            
            // Parse minAmountOut (u64) from the next 8 bytes
            let min_amount_out = if data.len() >= 24 {
                u64::from_le_bytes([
                    data[16], data[17], data[18], data[19], 
                    data[20], data[21], data[22], data[23]
                ])
            } else {
                log::info!("Swap instruction data too short for minAmountOut");
                0 // Default value but still include it
            };
            
            log::info!("Parsed Swap: amountIn={}, minAmountOut={}", amount_in, min_amount_out);
            
            // Convert to strings since we changed the proto definition to use strings
            let layout = PbSwapLayout {
                amount_in: amount_in.to_string(),
                min_amount_out: min_amount_out.to_string(),
            };
            
            args.instruction_args = Some(IArgs::Swap(layout));
        },
        
        "SwapWithPriceImpact" => {
            // Read amount_in and active_id and max_price_impact_bps
            let amount_in = if data.len() >= 16 {
                u64::from_le_bytes([
                    data[8], data[9], data[10], data[11], 
                    data[12], data[13], data[14], data[15]
                ])
            } else {
                0 // Default value but still include it
            };
            
            let active_id = if data.len() >= 20 {
                i32::from_le_bytes([
                    data[16], data[17], data[18], data[19]
                ])
            } else {
                0 // Default value but still include it
            };
            
            let max_price_impact_bps = if data.len() >= 24 {
                i32::from_le_bytes([
                    data[20], data[21], data[22], data[23]
                ])
            } else {
                0 // Default value but still include it
            };
            
            log::info!("Parsed SwapWithPriceImpact: amountIn={}, activeId={}, maxPriceImpactBps={}", 
                       amount_in, active_id, max_price_impact_bps);
            
            // Create layout with all fields
            let layout = PbSwapWithPriceImpactLayout {
                amount_in,
                active_id,
                max_price_impact_bps,
            };
            
            args.instruction_args = Some(IArgs::SwapWithPriceImpact(layout));
        },
        
        "SwapExactOut" => {
            // Read max_in_amount and out_amount
            let max_in_amount = if data.len() >= 16 {
                u64::from_le_bytes([
                    data[8], data[9], data[10], data[11], 
                    data[12], data[13], data[14], data[15]
                ])
            } else {
                0 // Default value but still include it
            };
            
            let out_amount = if data.len() >= 24 {
                u64::from_le_bytes([
                    data[16], data[17], data[18], data[19], 
                    data[20], data[21], data[22], data[23]
                ])
            } else {
                0 // Default value but still include it
            };
            
            log::info!("Parsed SwapExactOut: maxInAmount={}, outAmount={}", 
                      max_in_amount, out_amount);
            
            // Create layout
            let layout = PbSwapExactOutLayout {
                max_in_amount,
                out_amount,
            };
            
            args.instruction_args = Some(IArgs::SwapExactOut(layout));
        },
        
        "InitializeLbPair" | "InitializePermissionLbPair" => {
            // Read active_id and bin_step
            let active_id = if data.len() >= 12 {
                i32::from_le_bytes([
                    data[8], data[9], data[10], data[11]
                ])
            } else {
                0 // Default value but still include it
            };
            
            let bin_step = if data.len() >= 16 {
                i32::from_le_bytes([
                    data[12], data[13], data[14], data[15]
                ])
            } else {
                0 // Default value but still include it
            };
            
            log::info!("Parsed Initialize: activeId={}, binStep={}", 
                      active_id, bin_step);
            
            // Choose the appropriate layout based on instruction type
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
            // Read lower_bin_id and width
            let lower_bin_id = if data.len() >= 12 {
                i32::from_le_bytes([
                    data[8], data[9], data[10], data[11]
                ])
            } else {
                0 // Default value but still include it
            };
            
            let width = if data.len() >= 16 {
                i32::from_le_bytes([
                    data[12], data[13], data[14], data[15]
                ])
            } else {
                0 // Default value but still include it
            };
            
            log::info!("Parsed InitializePosition: lowerBinId={}, width={}", 
                      lower_bin_id, width);
            
            // Choose the appropriate layout
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
            // Empty layout but still create it
            let layout = PbClosePositionLayout {};
            args.instruction_args = Some(IArgs::ClosePosition(layout));
        },
        
        "ClaimFee" => {
            // Empty layout but still create it
            let layout = PbCollectFeesLayout {};
            args.instruction_args = Some(IArgs::CollectFees(layout));
        },
        
        "ClaimReward" => {
            let reward_index = if data.len() >= 12 {
                u32::from_le_bytes([
                    data[8], data[9], data[10], data[11]
                ])
            } else {
                0 // Default value but still include it
            };
            
            log::info!("Parsed ClaimReward: rewardIndex={}", reward_index);
            
            let layout = PbCollectRewardLayout {
                reward_index,
            };
            
            args.instruction_args = Some(IArgs::CollectReward(layout));
        },
        
        // Add more instruction types as needed
        _ => {
            // Default case for instructions without specific handling
            log::info!("No specific parsing for instruction type: {}", inst_type);
            // For now, we don't set any args for unknown instruction types
        }
    }
    
    Some(args)
} 