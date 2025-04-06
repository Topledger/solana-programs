use substreams_solana::pb::sf::solana::r#type::v1::CompiledInstruction;
use crate::pb::sf::solana::raydium_clmm::v1::{
    Meta, FlatArg, InstructionArgs, instruction_args, DefaultInstructionLayout,
    PbPubKey, PbInt128, PbInitializeRewardParam,
    PbCreateAmmConfigLayout, PbUpdateAmmConfigLayout, PbCreatePoolLayout,
    PbUpdatePoolStatusLayout, PbCreateOperationAccountLayout, PbUpdateOperationAccountLayout,
    PbTransferRewardOwnerLayout, PbInitializeRewardLayout, PbCollectRemainingRewardsLayout,
    PbUpdateRewardInfosLayout, PbSetRewardParamsLayout, PbCollectProtocolFeeLayout,
    PbCollectFundFeeLayout, PbOpenPositionLayout, PbOpenPositionV2Layout,
    PbOpenPositionWithToken22NftLayout, PbClosePositionLayout, PbIncreaseLiquidityLayout,
    PbIncreaseLiquidityV2Layout, PbDecreaseLiquidityLayout, PbDecreaseLiquidityV2Layout, PbSwapLayout,
    PbSwapV2Layout, PbSwapRouterBaseInLayout
};
use crate::prepare_input_accounts;
use sha2::{Digest, Sha256};
use hex; // Needed for logging unknown discriminators if re-enabled

const RAYDIUM_CLMM_PROGRAM_ID: &str = "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK";

// Placeholder for Raydium CLMM instruction discriminators
pub const INSTRUCTION_TYPES: [(&[u8], &str); 24] = [
    (&[137, 52, 237, 212, 215, 117, 108, 104], "CreateAmmConfig"),
    (&[49, 60, 174, 136, 154, 28, 116, 200], "UpdateAmmConfig"),
    (&[233, 146, 209, 142, 207, 104, 64, 188], "CreatePool"),
    (&[130, 87, 108, 6, 46, 224, 117, 123], "UpdatePoolStatus"),
    (&[63, 87, 148, 33, 109, 35, 8, 104], "CreateOperationAccount"),
    (&[127, 70, 119, 40, 188, 227, 61, 7], "UpdateOperationAccount"),
    (&[7, 22, 12, 83, 242, 43, 48, 121], "TransferRewardOwner"),
    (&[95, 135, 192, 196, 242, 129, 230, 68], "InitializeReward"),
    (&[18, 237, 166, 197, 34, 16, 213, 144], "CollectRemainingRewards"),
    (&[163, 172, 224, 52, 11, 154, 106, 223], "UpdateRewardInfos"),
    (&[112, 52, 167, 75, 32, 201, 211, 137], "SetRewardParams"),
    (&[136, 136, 252, 221, 194, 66, 126, 89], "CollectProtocolFee"),
    (&[167, 138, 78, 149, 223, 194, 6, 126], "CollectFundFee"),
    (&[135, 128, 47, 77, 15, 152, 240, 49], "OpenPosition"),
    (&[77, 184, 74, 214, 112, 86, 241, 199], "OpenPositionV2"),
    (&[77, 255, 174, 82, 125, 29, 201, 46], "OpenPositionWithToken22Nft"),
    (&[123, 134, 81, 0, 49, 68, 98, 98], "ClosePosition"),
    (&[46, 156, 243, 118, 13, 205, 251, 178], "IncreaseLiquidity"),
    (&[133, 29, 89, 223, 69, 238, 176, 10], "IncreaseLiquidityV2"),
    (&[160, 38, 208, 111, 104, 91, 44, 1], "DecreaseLiquidity"),
    (&[58, 127, 188, 62, 79, 82, 196, 96], "DecreaseLiquidityV2"),
    (&[248, 198, 158, 145, 225, 117, 135, 200], "Swap"),
    (&[43, 4, 237, 11, 26, 201, 30, 98], "SwapV2"),
    (&[69, 125, 115, 218, 245, 186, 242, 196], "SwapRouterBaseIn"),
]; // TODO: Populate this

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

/// Verifies that a computed discriminator matches the one in INSTRUCTION_TYPES
pub fn verify_discriminator(instruction_name: &str) -> bool {
    let computed = compute_discriminator(instruction_name);
    for (disc, name) in INSTRUCTION_TYPES.iter() {
        if name == &instruction_name {
            return disc == &computed;
        }
    }
    false
}

/// Generates discriminator code for a list of instruction names.
pub fn generate_discriminator_code(instruction_names: &[&str]) -> String {
    let mut code = String::new();
    for name in instruction_names {
        let disc = compute_discriminator(name);
        code.push_str(&format!("    (&[{}, {}, {}, {}, {}, {}, {}, {}], \"{}\"),\n",
            disc[0], disc[1], disc[2], disc[3],
            disc[4], disc[5], disc[6], disc[7],
            name));
    }
    code
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

    if program_id != RAYDIUM_CLMM_PROGRAM_ID {
        return None;
    }

    let data = &instruction.data;
    if data.len() < 8 {
        return None;
    }

    let discriminator = &data[0..8];
    let (inst_type, decoded_args) = match decode_instruction_data(&data) {
        Ok((t, a)) => (t, a),
        Err(_) => return None, // Skip if unknown discriminator
    };

    let dt = chrono::DateTime::from_timestamp(block_time, 0).unwrap_or_else(|| chrono::DateTime::from_timestamp(0, 0).unwrap());
    let block_date = dt.format("%Y-%m-%d").to_string();

    let mut flat_args = FlatArg {
        ..Default::default()
    };

    // Map decoded nested args to the flat structure
    // The InstructionArgs is now a struct with an optional oneof field 'instruction_args'
    if let Some(args) = decoded_args.instruction_args {
        use crate::pb::sf::solana::raydium_clmm::v1::instruction_args::InstructionArgs as IArgs;
        match args {
            IArgs::CreateAmmConfig(args) => {
                flat_args.index = Some(args.index);
                flat_args.tick_spacing = Some(args.tick_spacing);
                flat_args.trade_fee_rate = Some(args.trade_fee_rate);
                flat_args.protocol_fee_rate = Some(args.protocol_fee_rate);
                flat_args.fund_fee_rate = Some(args.fund_fee_rate);
            }
            IArgs::UpdateAmmConfig(args) => {
                flat_args.param = Some(args.param);
                flat_args.value = Some(args.value);
            }
            IArgs::CreatePool(args) => {
                flat_args.sqrt_price_x64 = args.sqrt_price_x64;
                flat_args.open_time = args.open_time;
            }
            IArgs::UpdatePoolStatus(args) => {
                flat_args.status = Some(args.status);
            }
            IArgs::CreateOperationAccount(_) => {}
            IArgs::UpdateOperationAccount(args) => {
                flat_args.param = Some(args.param);
                flat_args.keys = args.keys;
            }
            IArgs::TransferRewardOwner(args) => {
                flat_args.new_owner = args.new_owner;
            }
            IArgs::InitializeReward(args) => {
                flat_args.params = args.params;
            }
            IArgs::CollectRemainingRewards(args) => {
                flat_args.reward_index = Some(args.reward_index);
            }
            IArgs::UpdateRewardInfos(_) => {}
            IArgs::SetRewardParams(args) => {
                flat_args.reward_index = Some(args.reward_index);
                flat_args.emissions_per_second_x64 = args.emissions_per_second_x64;
                flat_args.open_time = Some(args.open_time);
                flat_args.end_time = Some(args.end_time);
            }
            IArgs::CollectProtocolFee(args) => {
                flat_args.amount0_requested = Some(args.amount0_requested.to_string());
                flat_args.amount1_requested = Some(args.amount1_requested.to_string());
            }
            IArgs::CollectFundFee(args) => {
                flat_args.amount0_requested = Some(args.amount0_requested.to_string());
                flat_args.amount1_requested = Some(args.amount1_requested.to_string());
            }
            IArgs::OpenPosition(args) => {
                flat_args.tick_lower_index = Some(args.tick_lower_index);
                flat_args.tick_upper_index = Some(args.tick_upper_index);
                flat_args.tick_array_lower_start_index = Some(args.tick_array_lower_start_index);
                flat_args.tick_array_upper_start_index = Some(args.tick_array_upper_start_index);
                flat_args.liquidity = args.liquidity;
                flat_args.amount0_max = Some(args.amount0_max);
                flat_args.amount1_max = Some(args.amount1_max);
            }
            IArgs::OpenPositionV2(args) => {
                flat_args.tick_lower_index = Some(args.tick_lower_index);
                flat_args.tick_upper_index = Some(args.tick_upper_index);
                flat_args.tick_array_lower_start_index = Some(args.tick_array_lower_start_index);
                flat_args.tick_array_upper_start_index = Some(args.tick_array_upper_start_index);
                flat_args.liquidity = args.liquidity;
                flat_args.amount0_max = Some(args.amount0_max);
                flat_args.amount1_max = Some(args.amount1_max);
                flat_args.with_metadata = Some(args.with_metadata);
                flat_args.base_flag = args.base_flag;
            }
            IArgs::OpenPositionWithToken22Nft(args) => {
                flat_args.tick_lower_index = Some(args.tick_lower_index);
                flat_args.tick_upper_index = Some(args.tick_upper_index);
                flat_args.tick_array_lower_start_index = Some(args.tick_array_lower_start_index);
                flat_args.tick_array_upper_start_index = Some(args.tick_array_upper_start_index);
                flat_args.liquidity = args.liquidity;
                flat_args.amount0_max = Some(args.amount0_max);
                flat_args.amount1_max = Some(args.amount1_max);
                flat_args.with_metadata = Some(args.with_metadata);
                flat_args.base_flag = args.base_flag;
            }
            IArgs::ClosePosition(_) => {}
            IArgs::IncreaseLiquidity(args) => {
                flat_args.liquidity = args.liquidity;
                flat_args.amount0_max = Some(args.amount0_max);
                flat_args.amount1_max = Some(args.amount1_max);
            }
            IArgs::IncreaseLiquidityV2(args) => {
                flat_args.liquidity = args.liquidity;
                flat_args.amount0_max = Some(args.amount0_max);
                flat_args.amount1_max = Some(args.amount1_max);
                flat_args.base_flag = args.base_flag;
            }
            IArgs::DecreaseLiquidity(args) => {
                flat_args.liquidity = args.liquidity;
                flat_args.amount0_min = Some(args.amount0_min);
                flat_args.amount1_min = Some(args.amount1_min);
            }
            IArgs::DecreaseLiquidityV2(args) => {
                flat_args.liquidity = args.liquidity;
                flat_args.amount0_min = Some(args.amount0_min);
                flat_args.amount1_min = Some(args.amount1_min);
            }
            IArgs::Swap(args) => {
                flat_args.amount = Some(args.amount);
                flat_args.other_amount_threshold = Some(args.other_amount_threshold);
                flat_args.sqrt_price_limit_x64 = args.sqrt_price_limit_x64;
                flat_args.is_base_input = Some(args.is_base_input);
            }
            IArgs::SwapV2(args) => {
                flat_args.amount = Some(args.amount);
                flat_args.other_amount_threshold = Some(args.other_amount_threshold);
                flat_args.sqrt_price_limit_x64 = args.sqrt_price_limit_x64;
                flat_args.is_base_input = Some(args.is_base_input);
            }
            IArgs::SwapRouterBaseIn(args) => {
                flat_args.amount_in = Some(args.amount_in);
                flat_args.amount_out_minimum = Some(args.amount_out_minimum);
            }
            IArgs::DefaultInstruction(_) => {} // Handle default case
        }
    }

    let input_accounts_map = prepare_input_accounts::prepare_input_accounts(
        instruction,
        account_keys,
        &inst_type,
        instruction_index,
        inner_instruction_index.unwrap_or(0),
        is_inner_instruction
    );

    // Create and populate the Meta struct with ALL fields
    Some(Meta {
        block_date,
        block_time,
        block_slot,
        tx_id: tx_id.to_string(),
        instruction_index: Some(instruction_index),
        is_inner_instruction: Some(is_inner_instruction),
        inner_instruction_index: Some(inner_instruction_index.unwrap_or(0)),
        dapp: program_id.to_string(),
        instruction_type: inst_type,
        args: Some(flat_args),
        input_accounts: input_accounts_map,
        signer: signer_pubkey.unwrap_or("").to_string(),
        outer_program: outer_program.unwrap_or("").to_string(),
    })
}

/// Decode instruction data based on discriminator
pub fn decode_instruction_data(data: &[u8]) -> Result<(String, InstructionArgs), String> { // Returns InstructionArgs directly
    if data.len() < 8 {
        return Err("Instruction data too short".to_string());
    }
    let discriminator = &data[0..8];
    let inst_type_str = match INSTRUCTION_TYPES.iter().find(|(disc, _)| disc == &discriminator) {
        Some((_, inst_type)) => inst_type.to_string(),
        None => return Err(format!("Unknown instruction discriminator: {}", hex::encode(discriminator))),
    };

    // decode_instruction_args now returns InstructionArgs directly
    let instruction_args = decode_instruction_args(&inst_type_str, &data[8..]);
    Ok((inst_type_str, instruction_args))
}

/// Helper function to decode instruction arguments based on the instruction type
/// Returns InstructionArgs, using DefaultInstruction on failure.
pub fn decode_instruction_args(inst_type: &str, data: &[u8]) -> InstructionArgs { // Returns InstructionArgs
    use crate::pb::sf::solana::raydium_clmm::v1::*;
    use std::convert::TryInto; 

    // Create a default InstructionArgs with DefaultInstruction
    let default_args = InstructionArgs {
        instruction_args: Some(instruction_args::InstructionArgs::DefaultInstruction(DefaultInstructionLayout {})),
    };

    // Helper closures to capture default_args
    let bytes_to_pbint128 = |bytes: &[u8]| -> Option<PbInt128> {
        if bytes.len() >= 16 {
            let array: [u8; 16] = bytes[0..16].try_into().ok()?;
            let val = u128::from_le_bytes(array);
            Some(PbInt128 { value: val.to_string() })
        } else {
            None
        }
    };

    let bytes_to_pbpubkey = |bytes: &[u8]| -> Option<PbPubKey> {
        if bytes.len() >= 32 {
             let array: [u8; 32] = bytes[0..32].try_into().ok()?;
            Some(PbPubKey { value: array.to_vec() })
        } else {
            None
        }
    };

    let bytes_to_bool = |byte: Option<&u8>| -> bool {
        byte.map_or(false, |&b| b != 0)
    };

    let bytes_to_optional_bool = |byte: Option<&u8>| -> Option<bool> {
         byte.map(|&b| b != 0)
    };

    let read_u16 = |offset: usize| -> Option<u16> {
        data.get(offset..offset+2)?.try_into().map(u16::from_le_bytes).ok()
    };
    let read_u32 = |offset: usize| -> Option<u32> {
        data.get(offset..offset+4)?.try_into().map(u32::from_le_bytes).ok()
    };
    let read_i32 = |offset: usize| -> Option<i32> {
        data.get(offset..offset+4)?.try_into().map(i32::from_le_bytes).ok()
    };
    let read_i64 = |offset: usize| -> Option<i64> {
        data.get(offset..offset+8)?.try_into().map(i64::from_le_bytes).ok()
    };
    let read_u64 = |offset: usize| -> Option<u64> {
        data.get(offset..offset+8)?.try_into().map(u64::from_le_bytes).ok()
    };

    // Macro to simplify decoding and return default on error
    macro_rules! decode_or_default {
        ($expr:expr) => {
            match $expr {
                Some(val) => val,
                None => return default_args,
            }
        };
    }

    // Create a specific variant of instruction_args based on inst_type
    let args = match inst_type {
        "CreateAmmConfig" => {
            if data.len() >= 16 { 
                instruction_args::InstructionArgs::CreateAmmConfig(PbCreateAmmConfigLayout {
                    index: decode_or_default!(read_u16(0)) as u32,
                    tick_spacing: decode_or_default!(read_u16(2)) as u32,
                    trade_fee_rate: decode_or_default!(read_u32(4)),
                    protocol_fee_rate: decode_or_default!(read_u32(8)),
                    fund_fee_rate: decode_or_default!(read_u32(12)),
                })
            } else { return default_args }
        }
        "UpdateAmmConfig" => {
            if data.len() >= 5 { 
                instruction_args::InstructionArgs::UpdateAmmConfig(PbUpdateAmmConfigLayout {
                    param: decode_or_default!(data.get(0)).clone() as u32,
                    value: decode_or_default!(read_u32(1)),
                })
            } else { return default_args }
        }
        "CreatePool" => {
             if data.len() >= 16 { 
                 let sqrt_price = decode_or_default!(bytes_to_pbint128(&data[0..16]));
                 let open_time = if data.len() >= 24 { read_i64(16) } else { None }; 
                 instruction_args::InstructionArgs::CreatePool(PbCreatePoolLayout {
                    sqrt_price_x64: Some(sqrt_price),
                    open_time: open_time,
                })
             } else { return default_args }
        }
        "UpdatePoolStatus" => {
             if data.len() >= 1 {
                 instruction_args::InstructionArgs::UpdatePoolStatus(PbUpdatePoolStatusLayout {
                     status: decode_or_default!(data.get(0)).clone() as u32,
                 })
             } else { return default_args }
        }
        "CreateOperationAccount" => {
             instruction_args::InstructionArgs::CreateOperationAccount(PbCreateOperationAccountLayout {})
        }
        "UpdateOperationAccount" => {
             if data.len() >= 5 { 
                 let param = decode_or_default!(data.get(0)).clone() as u32;
                 let keys_len = decode_or_default!(read_u32(1));
                 let expected_data_len = 5 + keys_len as usize * 32;
                 if data.len() >= expected_data_len {
                     let mut keys = Vec::new();
                     for i in 0..keys_len {
                         let start = 5 + i as usize * 32;
                         keys.push(decode_or_default!(bytes_to_pbpubkey(&data[start..start + 32])));
                     }
                     instruction_args::InstructionArgs::UpdateOperationAccount(PbUpdateOperationAccountLayout {
                         param,
                         keys,
                     })
                 } else { return default_args }
             } else { return default_args }
        }
        "TransferRewardOwner" => {
            if data.len() >= 32 {
                instruction_args::InstructionArgs::TransferRewardOwner(PbTransferRewardOwnerLayout {
                    new_owner: Some(decode_or_default!(bytes_to_pbpubkey(&data[0..32]))),
                })
            } else { return default_args }
        }
        "InitializeReward" => {
             if data.len() >= 32 { 
                 instruction_args::InstructionArgs::InitializeReward(PbInitializeRewardLayout {
                     params: Some(PbInitializeRewardParam {
                         open_time: decode_or_default!(read_i64(0)),
                         end_time: decode_or_default!(read_i64(8)),
                         emissions_per_second_x64: Some(decode_or_default!(bytes_to_pbint128(&data[16..32]))),
                     })
                 })
             } else { return default_args }
        }
        "CollectRemainingRewards" => {
             if data.len() >= 1 {
                 instruction_args::InstructionArgs::CollectRemainingRewards(PbCollectRemainingRewardsLayout {
                     reward_index: decode_or_default!(data.get(0)).clone() as u32,
                 })
             } else { return default_args }
        }
        "UpdateRewardInfos" => {
             instruction_args::InstructionArgs::UpdateRewardInfos(PbUpdateRewardInfosLayout {})
        }
        "SetRewardParams" => {
            if data.len() >= 33 { 
                instruction_args::InstructionArgs::SetRewardParams(PbSetRewardParamsLayout {
                    reward_index: decode_or_default!(data.get(0)).clone() as u32,
                    emissions_per_second_x64: Some(decode_or_default!(bytes_to_pbint128(&data[1..17]))),
                    open_time: decode_or_default!(read_i64(17)),
                    end_time: decode_or_default!(read_i64(25)),
                })
            } else { return default_args }
        }
        "CollectProtocolFee" => {
             if data.len() >= 16 { 
                 instruction_args::InstructionArgs::CollectProtocolFee(PbCollectProtocolFeeLayout {
                     amount0_requested: decode_or_default!(read_u64(0)),
                     amount1_requested: decode_or_default!(read_u64(8)),
                 })
             } else { return default_args }
        }
        "CollectFundFee" => {
             if data.len() >= 16 { 
                 instruction_args::InstructionArgs::CollectFundFee(PbCollectFundFeeLayout {
                     amount0_requested: decode_or_default!(read_u64(0)),
                     amount1_requested: decode_or_default!(read_u64(8)),
                 })
             } else { return default_args }
        }
         "OpenPosition" => {
             if data.len() >= 48 {
                 instruction_args::InstructionArgs::OpenPosition(PbOpenPositionLayout {
                     tick_lower_index: decode_or_default!(read_i32(0)),
                     tick_upper_index: decode_or_default!(read_i32(4)),
                     tick_array_lower_start_index: decode_or_default!(read_i32(8)),
                     tick_array_upper_start_index: decode_or_default!(read_i32(12)),
                     liquidity: Some(decode_or_default!(bytes_to_pbint128(&data[16..32]))),
                     amount0_max: decode_or_default!(read_i64(32)),
                     amount1_max: decode_or_default!(read_i64(40)),
                 })
             } else { return default_args }
         }
        "OpenPositionV2" => {
            if data.len() >= 40 {
                instruction_args::InstructionArgs::OpenPositionV2(PbOpenPositionV2Layout {
                    tick_lower_index: decode_or_default!(read_i32(0)),
                    tick_upper_index: decode_or_default!(read_i32(4)),
                    tick_array_lower_start_index: decode_or_default!(read_i32(8)),
                    tick_array_upper_start_index: decode_or_default!(read_i32(12)),
                    liquidity: Some(decode_or_default!(bytes_to_pbint128(&data[16..32]))),
                    amount0_max: decode_or_default!(read_i64(32)),
                    amount1_max: decode_or_default!(read_i64(40)),
                    with_metadata: bytes_to_bool(data.get(48)),
                    base_flag: bytes_to_optional_bool(data.get(49)),
                })
            } else {
                return default_args;
            }
        }
        "OpenPositionWithToken22Nft" => {
            if data.len() >= 40 {
                instruction_args::InstructionArgs::OpenPositionWithToken22Nft(PbOpenPositionWithToken22NftLayout {
                    tick_lower_index: decode_or_default!(read_i32(0)),
                    tick_upper_index: decode_or_default!(read_i32(4)),
                    tick_array_lower_start_index: decode_or_default!(read_i32(8)),
                    tick_array_upper_start_index: decode_or_default!(read_i32(12)),
                    liquidity: Some(decode_or_default!(bytes_to_pbint128(&data[16..32]))),
                    amount0_max: decode_or_default!(read_i64(32)),
                    amount1_max: decode_or_default!(read_i64(40)),
                    with_metadata: bytes_to_bool(data.get(48)),
                    base_flag: bytes_to_optional_bool(data.get(49)),
                })
            } else {
                return default_args;
            }
        }
        "ClosePosition" => {
            instruction_args::InstructionArgs::ClosePosition(PbClosePositionLayout {})
        }
        "IncreaseLiquidity" => {
            if data.len() >= 32 { 
                 instruction_args::InstructionArgs::IncreaseLiquidity(PbIncreaseLiquidityLayout {
                     liquidity: Some(decode_or_default!(bytes_to_pbint128(&data[0..16]))),
                     amount0_max: decode_or_default!(read_i64(16)),
                     amount1_max: decode_or_default!(read_i64(24)),
                 })
            } else { return default_args }
        }
        "IncreaseLiquidityV2" => {
            if data.len() >= 32 { 
                 instruction_args::InstructionArgs::IncreaseLiquidityV2(PbIncreaseLiquidityV2Layout {
                     liquidity: Some(decode_or_default!(bytes_to_pbint128(&data[0..16]))),
                     amount0_max: decode_or_default!(read_i64(16)),
                     amount1_max: decode_or_default!(read_i64(24)),
                     base_flag: bytes_to_optional_bool(data.get(32)),
                 })
            } else { return default_args }
        }
        "DecreaseLiquidity" => {
            if data.len() >= 32 { 
                instruction_args::InstructionArgs::DecreaseLiquidity(PbDecreaseLiquidityLayout {
                     liquidity: Some(decode_or_default!(bytes_to_pbint128(&data[0..16]))),
                     amount0_min: decode_or_default!(read_i64(16)),
                     amount1_min: decode_or_default!(read_i64(24)),
                 })
            } else { return default_args }
        }
        "DecreaseLiquidityV2" => {
            if data.len() >= 32 { 
                instruction_args::InstructionArgs::DecreaseLiquidityV2(PbDecreaseLiquidityV2Layout {
                     liquidity: Some(decode_or_default!(bytes_to_pbint128(&data[0..16]))),
                     amount0_min: decode_or_default!(read_i64(16)),
                     amount1_min: decode_or_default!(read_i64(24)),
                 })
            } else { return default_args }
        }
        "Swap" => {
            if data.len() >= 33 { 
                instruction_args::InstructionArgs::Swap(PbSwapLayout {
                    amount: decode_or_default!(read_i64(0)),
                    other_amount_threshold: decode_or_default!(read_i64(8)),
                    sqrt_price_limit_x64: Some(decode_or_default!(bytes_to_pbint128(&data[16..32]))),
                    is_base_input: bytes_to_bool(data.get(32)),
                })
            } else { return default_args }
        }
        "SwapV2" => {
            if data.len() >= 33 { 
                instruction_args::InstructionArgs::SwapV2(PbSwapV2Layout {
                    amount: decode_or_default!(read_i64(0)),
                    other_amount_threshold: decode_or_default!(read_i64(8)),
                    sqrt_price_limit_x64: Some(decode_or_default!(bytes_to_pbint128(&data[16..32]))),
                    is_base_input: bytes_to_bool(data.get(32)),
                })
            } else { return default_args }
        }
        "SwapRouterBaseIn" => {
            if data.len() >= 16 { 
                instruction_args::InstructionArgs::SwapRouterBaseIn(PbSwapRouterBaseInLayout {
                    amount_in: decode_or_default!(read_i64(0)),
                    amount_out_minimum: decode_or_default!(read_i64(8)),
                })
            } else { return default_args }
        }
        // Return DefaultInstruction for unknown instruction types
        _ => return default_args,
    };

    // Return the InstructionArgs struct with the variant we created
    InstructionArgs {
        instruction_args: Some(args),
    }
} 