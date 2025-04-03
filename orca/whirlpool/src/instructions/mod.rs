// use borsh::BorshDeserialize; // Removed unused import
use substreams_solana::pb::sf::solana::r#type::v1::CompiledInstruction;

use bs58;
use sha2::{Digest, Sha256};

use crate::pb::sf::solana::orca_whirlpool::v1::{
    Meta,
    FlatArg,
    arg::InstructionArgs,
    PbSwapLayout,
    PbInitializePoolV2Layout,
    PbInitializeRewardV2Layout,
    PbSetRewardEmissionsV2Layout,
    PbOpenBundledPositionLayout,
    PbCloseBundledPositionLayout,
    PbInitializeConfigLayout,
    PbInitializeTickArrayLayout,
    PbInitializeFeeTierLayout,
    PbOpenPositionLayout,
    PbOpenPositionBumps,
    PbOpenPositionWithMetadataLayout,
    PbOpenPositionWithMetadataBumps,
    PbSetDefaultFeeRateLayout,
    PbSetDefaultProtocolFeeRateLayout,
    PbSetFeeRateLayout,
    PbSetProtocolFeeRateLayout,
    PbSetRewardAuthorityLayout,
    PbSetRewardAuthorityBySuperAuthorityLayout,
};
use crate::prepare_input_accounts;

const ORCA_WHIRLPOOL_PROGRAM_ID: &str = "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc";

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
/// This follows the same logic as the Python function:
/// ```python
/// def compute_discriminator_bytes(instruction_name):
///     formatted_name = f"global:{camel_to_snake(instruction_name)}"
///     sha256_hash = hashlib.sha256(formatted_name.encode()).hexdigest()
///     byte_values = [int(sha256_hash[i:i + 2], 16) for i in range(0, 16, 2)]
///     return bytes(byte_values)
/// ```
pub fn compute_discriminator(instruction_name: &str) -> [u8; 8] {
    // Format the instruction name: "global:snake_case_name"
    let formatted_name = format!("global:{}", camel_to_snake(instruction_name));
    
    // Compute the SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(formatted_name.as_bytes());
    let hash_result = hasher.finalize();
    
    // Extract the first 8 bytes
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&hash_result[0..8]);
    
    discriminator
}

/// Verifies that a computed discriminator matches the one in INSTRUCTION_TYPES
/// Returns true if match is found, false otherwise
pub fn verify_discriminator(instruction_name: &str) -> bool {
    let computed = compute_discriminator(instruction_name);
    
    for (disc, name) in INSTRUCTION_TYPES.iter() {
        if name == &instruction_name {
            return disc == &computed;
        }
    }
    
    false // Instruction name not found in INSTRUCTION_TYPES
}

/// Generates discriminator code for a list of instruction names.
/// Returns a string containing the code for INSTRUCTION_TYPES array entries.
/// This is useful for updating or adding new discriminators.
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

// Orca Whirlpool instruction discriminators
// Add more as needed. These match the first byte of instruction data.
// Discriminator => Instruction Type
pub const INSTRUCTION_TYPES: [(&[u8], &str); 48] = [
    (&[208, 127, 21, 1, 194, 190, 196, 70], "InitializeConfig"),
    (&[95, 180, 10, 172, 84, 174, 232, 40], "InitializePool"),
    (&[11, 188, 193, 214, 141, 91, 149, 184], "InitializeTickArray"),
    (&[183, 74, 156, 160, 112, 2, 42, 30], "InitializeFeeTier"),
    (&[95, 135, 192, 196, 242, 129, 230, 68], "InitializeReward"),
    (&[13, 197, 86, 168, 109, 176, 27, 244], "SetRewardEmissions"),
    (&[135, 128, 47, 77, 15, 152, 240, 49], "OpenPosition"),
    (&[242, 29, 134, 48, 58, 110, 14, 60], "OpenPositionWithMetadata"),
    (&[46, 156, 243, 118, 13, 205, 251, 178], "IncreaseLiquidity"),
    (&[160, 38, 208, 111, 104, 91, 44, 1], "DecreaseLiquidity"),
    (&[154, 230, 250, 13, 236, 209, 75, 223], "UpdateFeesAndRewards"),
    (&[164, 152, 207, 99, 30, 186, 19, 182], "CollectFees"),
    (&[70, 5, 132, 87, 86, 235, 177, 34], "CollectReward"),
    (&[22, 67, 23, 98, 150, 178, 70, 220], "CollectProtocolFees"),
    (&[248, 198, 158, 145, 225, 117, 135, 200], "Swap"),
    (&[123, 134, 81, 0, 49, 68, 98, 98], "ClosePosition"),
    (&[118, 215, 214, 157, 182, 229, 208, 228], "SetDefaultFeeRate"),
    (&[107, 205, 249, 226, 151, 35, 86, 0], "SetDefaultProtocolFeeRate"),
    (&[53, 243, 137, 65, 8, 140, 158, 6], "SetFeeRate"),
    (&[95, 7, 4, 50, 154, 79, 156, 131], "SetProtocolFeeRate"),
    (&[31, 1, 50, 87, 237, 101, 97, 132], "SetFeeAuthority"),
    (&[34, 150, 93, 244, 139, 225, 233, 67], "SetCollectProtocolFeesAuthority"),
    (&[34, 39, 183, 252, 83, 28, 85, 127], "SetRewardAuthority"),
    (&[240, 154, 201, 198, 148, 93, 56, 25], "SetRewardAuthorityBySuperAuthority"),
    (&[207, 5, 200, 209, 122, 56, 82, 183], "SetRewardEmissionsSuperAuthority"),
    (&[195, 96, 237, 108, 68, 162, 219, 230], "TwoHopSwap"),
    (&[117, 45, 241, 149, 24, 18, 194, 65], "InitializePositionBundle"),
    (&[93, 124, 16, 179, 249, 131, 115, 245], "InitializePositionBundleWithMetadata"),
    (&[100, 25, 99, 2, 217, 239, 124, 173], "DeletePositionBundle"),
    (&[169, 113, 126, 171, 213, 172, 212, 49], "OpenBundledPosition"),
    (&[41, 36, 216, 245, 27, 85, 103, 67], "CloseBundledPosition"),
    (&[55, 9, 53, 9, 114, 57, 209, 52], "InitializeConfigExtension"),
    (&[44, 94, 241, 116, 24, 188, 60, 143], "SetConfigExtensionAuthority"),
    (&[207, 202, 4, 32, 205, 79, 13, 178], "SetTokenBadgeAuthority"),
    (&[253, 77, 205, 95, 27, 224, 89, 223], "InitializeTokenBadge"),
    (&[53, 146, 68, 8, 18, 117, 17, 185], "DeleteTokenBadge"),
    (&[212, 47, 95, 92, 114, 102, 131, 250], "OpenPositionWithTokenExtensions"),
    (&[74, 115, 99, 93, 197, 69, 103, 7], "InitializeAccount"),
    (&[136, 138, 108, 118, 46, 146, 6, 111], "IdlWrite"),
    (&[207, 117, 95, 191, 229, 180, 226, 15], "CollectFeesV2"),
    (&[103, 128, 222, 134, 114, 200, 22, 200], "CollectProtocolFeesV2"),
    (&[177, 107, 37, 180, 160, 19, 49, 209], "CollectRewardV2"),
    (&[58, 127, 188, 62, 79, 82, 196, 96], "DecreaseLiquidityV2"),
    (&[133, 29, 89, 223, 69, 238, 176, 10], "IncreaseLiquidityV2"),
    (&[207, 45, 87, 242, 27, 63, 204, 67], "InitializePoolV2"),
    (&[91, 1, 77, 50, 235, 229, 133, 49], "InitializeRewardV2"),
    (&[114, 228, 72, 32, 193, 48, 160, 102], "SetRewardEmissionsV2"),
    (&[43, 4, 237, 11, 26, 201, 30, 98], "SwapV2"),
];

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
) -> Option<Meta> {
    let program_id = match account_keys.get(instruction.program_id_index as usize) {
        Some(id) => id,
        None => {
            return None;
        }
    };

    // Verify this is actually the Orca Whirlpool program
    if program_id != ORCA_WHIRLPOOL_PROGRAM_ID {
        return None;
    }

    let data = &instruction.data;
    
    // Check if instruction data is long enough to contain a discriminator
    if data.len() < 8 {
        return None;
    }

    // Extract the instruction discriminator (first 8 bytes)
    let discriminator = &data[0..8];
    let _disc_bs58 = bs58::encode(discriminator).into_string();
    
    // Match the discriminator to a known instruction type
    let _inst_type = match INSTRUCTION_TYPES.iter().find(|(disc, _)| disc == &discriminator) {
        Some((_, inst_type)) => {
            let inst_type_str = inst_type.to_string();
            inst_type_str
        },
        None => {
            return None;
        }
    };

    // Decode instruction type and args using the modified helper
    let (inst_type, decoded_args) = match decode_instruction_data(&data) {
        Ok((t, a)) => (t, a),
        Err(_) => return None, // Skip if decoding fails
    };

    // Extract timestamp
    let dt = chrono::DateTime::from_timestamp(block_time, 0).unwrap_or_else(|| chrono::DateTime::from_timestamp(0, 0).unwrap());
    let block_date = dt.format("%Y-%m-%d").to_string();

    // Populate the FlatArg structure (as before)
    let mut flat_args = FlatArg {
        ..Default::default()
    };

    // Map decoded nested args to the flat structure (as before)
    match decoded_args {
        InstructionArgs::Swap(swap) => {
            if let Some(val) = swap.amount { flat_args.amount = Some(val.to_string()); }
            if let Some(val) = swap.other_amount_threshold { flat_args.other_amount_threshold = Some(val.to_string()); }
            if let Some(spl) = swap.sqrt_price_limit {
                flat_args.sqrt_price_limit = spl.value;
            }
            if let Some(val) = swap.amount_specified_is_input { flat_args.amount_specified_is_input = Some(val); }
            if let Some(val) = swap.a_to_b { flat_args.a_to_b = Some(val); }
        }
        InstructionArgs::SwapV2(swap) => {
             if let Some(val) = swap.amount { flat_args.amount = Some(val.to_string()); }
            if let Some(val) = swap.other_amount_threshold { flat_args.other_amount_threshold = Some(val.to_string()); }
            if let Some(spl) = swap.sqrt_price_limit {
                flat_args.sqrt_price_limit = spl.value;
            }
            if let Some(val) = swap.amount_specified_is_input { flat_args.amount_specified_is_input = Some(val); }
            if let Some(val) = swap.a_to_b { flat_args.a_to_b = Some(val); }
        }
        InstructionArgs::TwoHopSwap(two_hop) => {
            if let Some(val) = two_hop.amount { flat_args.amount = Some(val.to_string()); }
            if let Some(val) = two_hop.other_amount_threshold { flat_args.other_amount_threshold = Some(val.to_string()); }
            if let Some(spl) = two_hop.sqrt_price_limit_one {
                flat_args.sqrt_price_limit_one = spl.value;
            }
             if let Some(spl) = two_hop.sqrt_price_limit_two {
                flat_args.sqrt_price_limit_two = spl.value;
            }
            if let Some(val) = two_hop.amount_specified_is_input { flat_args.amount_specified_is_input = Some(val); }
            if let Some(val) = two_hop.a_to_b_one { flat_args.a_to_b_one = Some(val); }
            if let Some(val) = two_hop.a_to_b_two { flat_args.a_to_b_two = Some(val); }
        }
         InstructionArgs::TwoHopSwapV2(two_hop) => {
            if let Some(val) = two_hop.amount { flat_args.amount = Some(val.to_string()); }
            if let Some(val) = two_hop.other_amount_threshold { flat_args.other_amount_threshold = Some(val.to_string()); }
            if let Some(spl) = two_hop.sqrt_price_limit_one {
                flat_args.sqrt_price_limit_one = spl.value;
            }
             if let Some(spl) = two_hop.sqrt_price_limit_two {
                flat_args.sqrt_price_limit_two = spl.value;
            }
            if let Some(val) = two_hop.amount_specified_is_input { flat_args.amount_specified_is_input = Some(val); }
            if let Some(val) = two_hop.a_to_b_one { flat_args.a_to_b_one = Some(val); }
            if let Some(val) = two_hop.a_to_b_two { flat_args.a_to_b_two = Some(val); }
        }
        InstructionArgs::InitializePool(pool) => {
            if let Some(val) = pool.tick_spacing { flat_args.tick_spacing = Some(val); }
            if let Some(val) = pool.initial_sqrt_price { flat_args.sqrt_price_limit = val.value; }
        }
        InstructionArgs::SetRewardEmissions(reward) => {
            if let Some(val) = reward.reward_index { flat_args.reward_index = Some(val); }
            if let Some(val) = reward.emissions_per_second_x64 { flat_args.emissions_per_second_x64 = val.value; }
        }
        InstructionArgs::IncreaseLiquidity(liq) => {
            if let Some(val) = liq.liquidity_amount { flat_args.liquidity_amount = val.value; }
            if let Some(val) = liq.token_max_a { flat_args.token_max_a = Some(val.to_string()); }
            if let Some(val) = liq.token_max_b { flat_args.token_max_b = Some(val.to_string()); }
        }
        InstructionArgs::DecreaseLiquidity(liq) => {
            if let Some(val) = liq.liquidity_amount { flat_args.liquidity_amount = val.value; }
            if let Some(val) = liq.token_min_a { flat_args.token_min_a = Some(val.to_string()); }
            if let Some(val) = liq.token_min_b { flat_args.token_min_b = Some(val.to_string()); }
        }
        InstructionArgs::CollectReward(reward) => {
            if let Some(val) = reward.reward_index { flat_args.reward_index = Some(val); }
        }
        InstructionArgs::IncreaseLiquidityV2(liq) => {
            if let Some(val) = liq.liquidity_amount { flat_args.liquidity_amount = val.value; }
            if let Some(val) = liq.token_max_a { flat_args.token_max_a = Some(val.to_string()); }
            if let Some(val) = liq.token_max_b { flat_args.token_max_b = Some(val.to_string()); }
        }
        InstructionArgs::DecreaseLiquidityV2(liq) => {
            if let Some(val) = liq.liquidity_amount { flat_args.liquidity_amount = val.value; }
            if let Some(val) = liq.token_min_a { flat_args.token_min_a = Some(val.to_string()); }
            if let Some(val) = liq.token_min_b { flat_args.token_min_b = Some(val.to_string()); }
        }
        InstructionArgs::InitializeTickArray(tick) => {
            if let Some(val) = tick.start_tick_index { flat_args.start_tick_index = Some(val); }
        }
        InstructionArgs::InitializeFeeTier(fee) => {
             if let Some(val) = fee.tick_spacing { flat_args.tick_spacing = Some(val); }
             if let Some(val) = fee.default_fee_rate { flat_args.default_fee_rate = Some(val); }
        }
        InstructionArgs::OpenPosition(pos) => {
             if let Some(val) = pos.tick_lower_index { flat_args.tick_lower_index = Some(val); }
             if let Some(val) = pos.tick_upper_index { flat_args.tick_upper_index = Some(val); }
        }
        InstructionArgs::OpenPositionWithMetadata(pos) => {
             if let Some(val) = pos.tick_lower_index { flat_args.tick_lower_index = Some(val); }
             if let Some(val) = pos.tick_upper_index { flat_args.tick_upper_index = Some(val); }
        }
         InstructionArgs::SetDefaultFeeRate(fee) => {
             if let Some(val) = fee.default_fee_rate { flat_args.default_fee_rate = Some(val); }
        }
         InstructionArgs::SetDefaultProtocolFeeRate(fee) => {
             if let Some(val) = fee.default_protocol_fee_rate { flat_args.default_protocol_fee_rate = Some(val); }
        }
         InstructionArgs::SetFeeRate(fee) => {
             if let Some(val) = fee.fee_rate { flat_args.fee_rate = Some(val); }
        }
         InstructionArgs::SetProtocolFeeRate(fee) => {
             if let Some(val) = fee.protocol_fee_rate { flat_args.protocol_fee_rate = Some(val); }
        }
         InstructionArgs::SetRewardAuthority(reward) => {
             if let Some(val) = reward.reward_index { flat_args.reward_index = Some(val); }
        }
         InstructionArgs::SetRewardAuthorityBySuperAuthority(reward) => {
             if let Some(val) = reward.reward_index { flat_args.reward_index = Some(val); }
        }
         InstructionArgs::OpenBundledPosition(bundle) => {
             if let Some(val) = bundle.bundle_index { flat_args.bundle_index = Some(val); }
             if let Some(val) = bundle.tick_lower_index { flat_args.tick_lower_index = Some(val); }
             if let Some(val) = bundle.tick_upper_index { flat_args.tick_upper_index = Some(val); }
        }
         InstructionArgs::CloseBundledPosition(bundle) => {
             if let Some(val) = bundle.bundle_index { flat_args.bundle_index = Some(val); }
        }
         InstructionArgs::InitializeRewardV2(reward) => {
             if let Some(val) = reward.reward_index { flat_args.reward_index = Some(val); }
        }
         InstructionArgs::SetRewardEmissionsV2(reward) => {
             if let Some(val) = reward.reward_index { flat_args.reward_index = Some(val); }
             if let Some(emissions) = reward.emissions_per_second_x64 {
                 flat_args.emissions_per_second_x64 = emissions.value;
             }
        }
         InstructionArgs::InitializePoolV2(pool) => {
             if let Some(val) = pool.tick_spacing { flat_args.tick_spacing = Some(val); }
             if let Some(price) = pool.initial_sqrt_price {
                 flat_args.sqrt_price_limit = price.value;
             }
        }
        InstructionArgs::InitializeConfig(config) => {
            if let Some(val) = config.default_protocol_fee_rate { flat_args.default_protocol_fee_rate = Some(val); }
        }
        InstructionArgs::OpenPositionWithTokenExtensions(pos) => {
             if let Some(val) = pos.tick_lower_index { flat_args.tick_lower_index = Some(val); }
             if let Some(val) = pos.tick_upper_index { flat_args.tick_upper_index = Some(val); }
             if let Some(val) = pos.with_token_metadata_extension { flat_args.with_token_metadata_extension = Some(val); }
        }
        _ => {}
    }

    // Populate input_accounts
    let account_indices: Vec<u32> = instruction.accounts.iter().map(|&idx| idx as u32).collect();
    let input_accounts = prepare_input_accounts::prepare_input_accounts(
        inst_type.clone(),
        &account_indices,
        account_keys,
    );

    // Create the Meta object using the new FlatArg (as before)
    let meta_obj = Meta {
        block_date: Some(block_date),
        block_time: Some(block_time),
        tx_id: Some(tx_id.to_string()),
        dapp: Some(ORCA_WHIRLPOOL_PROGRAM_ID.to_string()),
        block_slot: Some(block_slot),
        instruction_index: Some(instruction_index),
        is_inner_instruction: Some(is_inner_instruction),
        inner_instruction_index: Some(inner_instruction_index.unwrap_or(0)),
        instruction_type: Some(inst_type),
        args: Some(flat_args),
        input_accounts: Some(input_accounts),
    };

    Some(meta_obj)
}

/// Decode instruction data into instruction type and decoded args enum
pub fn decode_instruction_data(data: &[u8]) -> Result<(String, InstructionArgs), String> {
    if data.len() < 8 {
        return Err("Instruction data too short".to_string());
    }
    let discriminator = &data[0..8];
    let inst_type = match INSTRUCTION_TYPES.iter().find(|(disc, _)| disc == &discriminator) {
        Some((_, inst_type)) => inst_type.to_string(),
        None => return Err(format!("Unknown instruction discriminator: {:?}", discriminator)),
    };

    let instruction_args = decode_instruction_args(&inst_type, &data[8..]);
    Ok((inst_type, instruction_args))
}

// Helper function to decode instruction arguments based on the instruction type
pub fn decode_instruction_args(inst_type: &str, data: &[u8]) -> InstructionArgs {
    use crate::pb::sf::solana::orca_whirlpool::v1::{arg, PbCollectFeesLayout, PbUint128, PbOpenPositionWithTokenExtensionsLayout, PbPubKey};
    use crate::pb::sf::solana::orca_whirlpool::v1::{PbInitializePoolLayout, PbSetRewardEmissionsLayout, PbIncreaseLiquidityLayout, PbDecreaseLiquidityLayout, PbTwoHopSwapLayout};
    use crate::pb::sf::solana::orca_whirlpool::v1::{PbInitializeRewardLayout, PbCollectRewardLayout, PbIncreaseLiquidityV2Layout, PbDecreaseLiquidityV2Layout, PbSwapV2Layout, PbTwoHopSwapV2Layout};
    
    // Helper function to convert u128 to string
    fn u128_to_string(value: u128) -> String {
        value.to_string()
    }
    
    match inst_type {
        "Swap" => {
            if data.len() >= 34 {
                // Enough data to decode all fields
                let amount = u64::from_le_bytes(data[0..8].try_into().unwrap());
                let other_amount_threshold = u64::from_le_bytes(data[8..16].try_into().unwrap());
                let sqrt_price_limit_bytes = data[16..32].try_into().unwrap();
                let sqrt_price_limit = u128::from_le_bytes(sqrt_price_limit_bytes);
                let amount_specified_is_input = data[32] != 0;
                let a_to_b = data[33] != 0;

                // Create a PbUint128 with just the string value
                let pb_sqrt_price_limit = PbUint128 {
                    value: Some(u128_to_string(sqrt_price_limit)),
                };

                arg::InstructionArgs::Swap(PbSwapLayout {
                    amount: Some(amount),
                    other_amount_threshold: Some(other_amount_threshold),
                    sqrt_price_limit: Some(pb_sqrt_price_limit),
                    amount_specified_is_input: Some(amount_specified_is_input),
                    a_to_b: Some(a_to_b),
                })
            } else {
                // Not enough data, use default values to ensure fields are present in JSON
                // Default PbUint128 value
                let pb_sqrt_price_limit = PbUint128 {
                    value: Some("0".to_string()),
                };

                arg::InstructionArgs::Swap(PbSwapLayout {
                    amount: Some(0),
                    other_amount_threshold: Some(0),
                    sqrt_price_limit: Some(pb_sqrt_price_limit),
                    amount_specified_is_input: Some(false),
                    a_to_b: Some(false),
                })
            }
        },
        "InitializePool" => {
            if data.len() >= 19 {
                // Extract bumps (1 byte)
                let whirlpool_bump = data[0];
                
                // Extract tick spacing (2 bytes)
                let tick_spacing = u16::from_le_bytes(data[1..3].try_into().unwrap());
                
                // Extract initial sqrt price (16 bytes for u128)
                let initial_sqrt_price_bytes = data[3..19].try_into().unwrap();
                let initial_sqrt_price = u128::from_le_bytes(initial_sqrt_price_bytes);
                
                // Create a PbBumps with the whirlpool_bump
                let bumps = crate::pb::sf::solana::orca_whirlpool::v1::PbBumps {
                    whirlpool_bump: Some(whirlpool_bump as u32),
                };
                
                // Create a PbUint128 with just the string value
                let pb_initial_sqrt_price = PbUint128 {
                    value: Some(u128_to_string(initial_sqrt_price)),
                };

                arg::InstructionArgs::InitializePool(PbInitializePoolLayout {
                    bumps: Some(bumps),
                    tick_spacing: Some(tick_spacing as u32),
                    initial_sqrt_price: Some(pb_initial_sqrt_price),
                })
            } else {
                arg::InstructionArgs::CollectFees(PbCollectFeesLayout {})
            }
        },
        "SetRewardEmissions" => {
            if data.len() >= 17 {
                // Extract reward index (1 byte)
                let reward_index = data[0];
                
                // Extract emissions per second x64 (16 bytes for u128)
                let emissions_per_second_bytes = data[1..17].try_into().unwrap();
                let emissions_per_second = u128::from_le_bytes(emissions_per_second_bytes);
                
                // Create a PbUint128 with just the string value
                let pb_emissions_per_second = PbUint128 {
                    value: Some(u128_to_string(emissions_per_second)),
                };

                arg::InstructionArgs::SetRewardEmissions(PbSetRewardEmissionsLayout {
                    reward_index: Some(reward_index as u32),
                    emissions_per_second_x64: Some(pb_emissions_per_second),
                })
            } else {
                arg::InstructionArgs::CollectFees(PbCollectFeesLayout {})
            }
        },
        "IncreaseLiquidity" => {
            if data.len() >= 32 {
                // Extract liquidity amount (16 bytes for u128)
                let liquidity_amount_bytes = data[0..16].try_into().unwrap();
                let liquidity_amount = u128::from_le_bytes(liquidity_amount_bytes);
                
                // Extract token max A (8 bytes for u64)
                let token_max_a = u64::from_le_bytes(data[16..24].try_into().unwrap());
                
                // Extract token max B (8 bytes for u64)
                let token_max_b = u64::from_le_bytes(data[24..32].try_into().unwrap());
                
                // Create a PbUint128 with just the string value
                let pb_liquidity_amount = PbUint128 {
                    value: Some(u128_to_string(liquidity_amount)),
                };

                arg::InstructionArgs::IncreaseLiquidity(PbIncreaseLiquidityLayout {
                    liquidity_amount: Some(pb_liquidity_amount),
                    token_max_a: Some(token_max_a),
                    token_max_b: Some(token_max_b),
                })
            } else {
                arg::InstructionArgs::CollectFees(PbCollectFeesLayout {})
            }
        },
        "DecreaseLiquidity" => {
            if data.len() >= 32 {
                // Extract liquidity amount (16 bytes for u128)
                let liquidity_amount_bytes = data[0..16].try_into().unwrap();
                let liquidity_amount = u128::from_le_bytes(liquidity_amount_bytes);
                
                // Extract token min A (8 bytes for u64)
                let token_min_a = u64::from_le_bytes(data[16..24].try_into().unwrap());
                
                // Extract token min B (8 bytes for u64)
                let token_min_b = u64::from_le_bytes(data[24..32].try_into().unwrap());
                
                // Create a PbUint128 with just the string value
                let pb_liquidity_amount = PbUint128 {
                    value: Some(u128_to_string(liquidity_amount)),
                };

                arg::InstructionArgs::DecreaseLiquidity(PbDecreaseLiquidityLayout {
                    liquidity_amount: Some(pb_liquidity_amount),
                    token_min_a: Some(token_min_a),
                    token_min_b: Some(token_min_b),
                })
            } else {
                arg::InstructionArgs::CollectFees(PbCollectFeesLayout {})
            }
        },
        "CollectReward" => {
            if data.len() >= 1 {
                // Extract reward index (1 byte)
                let reward_index = data[0];
                
                arg::InstructionArgs::CollectReward(PbCollectRewardLayout {
                    reward_index: Some(reward_index as u32),
                })
            } else {
                arg::InstructionArgs::CollectFees(PbCollectFeesLayout {})
            }
        },
        "TwoHopSwap" => {
            if data.len() >= 50 {
                // Extract amount (8 bytes for u64)
                let amount = u64::from_le_bytes(data[0..8].try_into().unwrap());
                
                // Extract other amount threshold (8 bytes for u64)
                let other_amount_threshold = u64::from_le_bytes(data[8..16].try_into().unwrap());
                
                // Extract flags (3 bytes for booleans)
                let amount_specified_is_input = data[16] != 0;
                let a_to_b_one = data[17] != 0;
                let a_to_b_two = data[18] != 0;
                
                // Extract sqrt price limit one (16 bytes for u128)
                let sqrt_price_limit_one_bytes = data[19..35].try_into().unwrap();
                let sqrt_price_limit_one = u128::from_le_bytes(sqrt_price_limit_one_bytes);
                
                // Extract sqrt price limit two (16 bytes for u128)
                let sqrt_price_limit_two_bytes = data[35..51].try_into().unwrap();
                let sqrt_price_limit_two = u128::from_le_bytes(sqrt_price_limit_two_bytes);
                
                // Create PbUint128 objects with just the string values
                let pb_sqrt_price_limit_one = PbUint128 {
                    value: Some(u128_to_string(sqrt_price_limit_one)),
                };
                
                let pb_sqrt_price_limit_two = PbUint128 {
                    value: Some(u128_to_string(sqrt_price_limit_two)),
                };

                arg::InstructionArgs::TwoHopSwap(PbTwoHopSwapLayout {
                    amount: Some(amount),
                    other_amount_threshold: Some(other_amount_threshold),
                    amount_specified_is_input: Some(amount_specified_is_input),
                    a_to_b_one: Some(a_to_b_one),
                    a_to_b_two: Some(a_to_b_two),
                    sqrt_price_limit_one: Some(pb_sqrt_price_limit_one),
                    sqrt_price_limit_two: Some(pb_sqrt_price_limit_two),
                })
            } else {
                arg::InstructionArgs::CollectFees(PbCollectFeesLayout {})
            }
        },
        "IncreaseLiquidityV2" => {
            if data.len() >= 32 {
                // Extract liquidity amount (16 bytes for u128)
                let liquidity_amount_bytes = data[0..16].try_into().unwrap();
                let liquidity_amount = u128::from_le_bytes(liquidity_amount_bytes);
                
                // Extract token max A (8 bytes for u64)
                let token_max_a = u64::from_le_bytes(data[16..24].try_into().unwrap());
                
                // Extract token max B (8 bytes for u64)
                let token_max_b = u64::from_le_bytes(data[24..32].try_into().unwrap());
                
                // Create a PbUint128 with just the string value
                let pb_liquidity_amount = PbUint128 {
                    value: Some(u128_to_string(liquidity_amount)),
                };

                arg::InstructionArgs::IncreaseLiquidityV2(PbIncreaseLiquidityV2Layout {
                    liquidity_amount: Some(pb_liquidity_amount),
                    token_max_a: Some(token_max_a),
                    token_max_b: Some(token_max_b),
                    remaining_accounts_info: None,
                })
            } else {
                arg::InstructionArgs::CollectFees(PbCollectFeesLayout {})
            }
        },
        "DecreaseLiquidityV2" => {
            if data.len() >= 32 {
                // Extract liquidity amount (16 bytes for u128)
                let liquidity_amount_bytes = data[0..16].try_into().unwrap();
                let liquidity_amount = u128::from_le_bytes(liquidity_amount_bytes);
                
                // Extract token min A (8 bytes for u64)
                let token_min_a = u64::from_le_bytes(data[16..24].try_into().unwrap());
                
                // Extract token min B (8 bytes for u64)
                let token_min_b = u64::from_le_bytes(data[24..32].try_into().unwrap());
                
                // Create a PbUint128 with just the string value
                let pb_liquidity_amount = PbUint128 {
                    value: Some(u128_to_string(liquidity_amount)),
                };

                arg::InstructionArgs::DecreaseLiquidityV2(PbDecreaseLiquidityV2Layout {
                    liquidity_amount: Some(pb_liquidity_amount),
                    token_min_a: Some(token_min_a),
                    token_min_b: Some(token_min_b),
                    remaining_accounts_info: None,
                })
            } else {
                arg::InstructionArgs::CollectFees(PbCollectFeesLayout {})
            }
        },
        "SwapV2" => {
            if data.len() >= 34 {
                // Enough data to decode all fields
                
                let amount = u64::from_le_bytes(data[0..8].try_into().unwrap());
                let other_amount_threshold = u64::from_le_bytes(data[8..16].try_into().unwrap());
                let sqrt_price_limit_bytes = data[16..32].try_into().unwrap();
                let sqrt_price_limit = u128::from_le_bytes(sqrt_price_limit_bytes);
                let amount_specified_is_input = data[32] != 0;
                let a_to_b = data[33] != 0;
                
                // Create a PbUint128 with just the string value
                let pb_sqrt_price_limit = PbUint128 {
                    value: Some(u128_to_string(sqrt_price_limit)),
                };

                arg::InstructionArgs::SwapV2(PbSwapV2Layout {
                    amount: Some(amount),
                    other_amount_threshold: Some(other_amount_threshold),
                    sqrt_price_limit: Some(pb_sqrt_price_limit),
                    amount_specified_is_input: Some(amount_specified_is_input),
                    a_to_b: Some(a_to_b),
                    remaining_accounts_info: None,
                })
            } else {
                // Not enough data, use default values to ensure fields are present in JSON
                
                // Default PbUint128 value
                let pb_sqrt_price_limit = PbUint128 {
                    value: Some("0".to_string()),
                };

                arg::InstructionArgs::SwapV2(PbSwapV2Layout {
                    amount: Some(0),
                    other_amount_threshold: Some(0),
                    sqrt_price_limit: Some(pb_sqrt_price_limit),
                    amount_specified_is_input: Some(false),
                    a_to_b: Some(false),
                    remaining_accounts_info: None,
                })
            }
        },
        "TwoHopSwapV2" => {
            if data.len() >= 50 {
                // Extract amount (8 bytes for u64)
                let amount = u64::from_le_bytes(data[0..8].try_into().unwrap());
                
                // Extract other amount threshold (8 bytes for u64)
                let other_amount_threshold = u64::from_le_bytes(data[8..16].try_into().unwrap());
                
                // Extract flags (3 bytes for booleans)
                let amount_specified_is_input = data[16] != 0;
                let a_to_b_one = data[17] != 0;
                let a_to_b_two = data[18] != 0;
                
                // Extract sqrt price limit one (16 bytes for u128)
                let sqrt_price_limit_one_bytes = data[19..35].try_into().unwrap();
                let sqrt_price_limit_one = u128::from_le_bytes(sqrt_price_limit_one_bytes);
                
                // Extract sqrt price limit two (16 bytes for u128)
                let sqrt_price_limit_two_bytes = data[35..51].try_into().unwrap();
                let sqrt_price_limit_two = u128::from_le_bytes(sqrt_price_limit_two_bytes);
                
                // Create PbUint128 objects with just the string values
                let pb_sqrt_price_limit_one = PbUint128 {
                    value: Some(u128_to_string(sqrt_price_limit_one)),
                };
                
                let pb_sqrt_price_limit_two = PbUint128 {
                    value: Some(u128_to_string(sqrt_price_limit_two)),
                };

                arg::InstructionArgs::TwoHopSwapV2(PbTwoHopSwapV2Layout {
                    amount: Some(amount),
                    other_amount_threshold: Some(other_amount_threshold),
                    amount_specified_is_input: Some(amount_specified_is_input),
                    a_to_b_one: Some(a_to_b_one),
                    a_to_b_two: Some(a_to_b_two),
                    sqrt_price_limit_one: Some(pb_sqrt_price_limit_one),
                    sqrt_price_limit_two: Some(pb_sqrt_price_limit_two),
                    remaining_accounts_info: None,
                })
            } else {
                arg::InstructionArgs::CollectFees(PbCollectFeesLayout {})
            }
        },
        "InitializeConfig" => {
            if data.len() >= 98 { // 3 * 32 bytes Pubkey + 2 bytes u16
                arg::InstructionArgs::InitializeConfig(PbInitializeConfigLayout {
                    fee_authority: Some(PbPubKey { pub_key: Some(data[0..32].to_vec()) }),
                    collect_protocol_fees_authority: Some(PbPubKey { pub_key: Some(data[32..64].to_vec()) }),
                    reward_emissions_super_authority: Some(PbPubKey { pub_key: Some(data[64..96].to_vec()) }),
                    default_protocol_fee_rate: Some(u16::from_le_bytes(data[96..98].try_into().unwrap()) as u32),
                })
            } else { arg::InstructionArgs::CollectFees(PbCollectFeesLayout {}) } // Default
        },
        "InitializeTickArray" => {
            if data.len() >= 4 { // 4 bytes i32
                 arg::InstructionArgs::InitializeTickArray(PbInitializeTickArrayLayout {
                    start_tick_index: Some(i32::from_le_bytes(data[0..4].try_into().unwrap())),
                })
            } else { arg::InstructionArgs::CollectFees(PbCollectFeesLayout {}) } // Default
        },
         "InitializeFeeTier" => {
            if data.len() >= 4 { // 2 bytes u16 + 2 bytes u16
                arg::InstructionArgs::InitializeFeeTier(PbInitializeFeeTierLayout {
                     tick_spacing: Some(u16::from_le_bytes(data[0..2].try_into().unwrap()) as u32),
                     default_fee_rate: Some(u16::from_le_bytes(data[2..4].try_into().unwrap()) as u32),
                })
            } else { arg::InstructionArgs::CollectFees(PbCollectFeesLayout {}) } // Default
        },
        "InitializeReward" => {
             if data.len() >= 1 { // 1 byte u8
                 arg::InstructionArgs::InitializeReward(PbInitializeRewardLayout {
                     reward_index: Some(data[0] as u32),
                })
            } else { arg::InstructionArgs::CollectFees(PbCollectFeesLayout {}) } // Default
        },
        "OpenPosition" => {
            if data.len() >= 9 { // 1 byte bump + 4 bytes i32 + 4 bytes i32
                let bumps = PbOpenPositionBumps { position_bump: Some(data[0] as u32) };
                arg::InstructionArgs::OpenPosition(PbOpenPositionLayout {
                    open_position_bumps: Some(bumps),
                    tick_lower_index: Some(i32::from_le_bytes(data[1..5].try_into().unwrap())),
                    tick_upper_index: Some(i32::from_le_bytes(data[5..9].try_into().unwrap())),
                })
            } else { arg::InstructionArgs::CollectFees(PbCollectFeesLayout {}) } // Default
        },
         "OpenPositionWithMetadata" => {
             if data.len() >= 10 { // 1 byte bump + 1 byte bump + 4 bytes i32 + 4 bytes i32
                 let bumps = PbOpenPositionWithMetadataBumps { 
                     position_bump: Some(data[0] as u32),
                     metadata_bump: Some(data[1] as u32),
                 };
                 arg::InstructionArgs::OpenPositionWithMetadata(PbOpenPositionWithMetadataLayout {
                     open_position_with_metadata_bumps: Some(bumps),
                     tick_lower_index: Some(i32::from_le_bytes(data[2..6].try_into().unwrap())),
                     tick_upper_index: Some(i32::from_le_bytes(data[6..10].try_into().unwrap())),
                 })
             } else { arg::InstructionArgs::CollectFees(PbCollectFeesLayout {}) } // Default
         }
         "SetDefaultFeeRate" => {
             if data.len() >= 2 { // 2 bytes u16
                 arg::InstructionArgs::SetDefaultFeeRate(PbSetDefaultFeeRateLayout {
                     default_fee_rate: Some(u16::from_le_bytes(data[0..2].try_into().unwrap()) as u32),
                 })
             } else { arg::InstructionArgs::CollectFees(PbCollectFeesLayout {}) } // Default
         },
         "SetDefaultProtocolFeeRate" => {
             if data.len() >= 2 { // 2 bytes u16
                 arg::InstructionArgs::SetDefaultProtocolFeeRate(PbSetDefaultProtocolFeeRateLayout {
                     default_protocol_fee_rate: Some(u16::from_le_bytes(data[0..2].try_into().unwrap()) as u32),
                 })
             } else { arg::InstructionArgs::CollectFees(PbCollectFeesLayout {}) } // Default
         },
         "SetFeeRate" => {
             if data.len() >= 2 { // 2 bytes u16
                 arg::InstructionArgs::SetFeeRate(PbSetFeeRateLayout {
                     fee_rate: Some(u16::from_le_bytes(data[0..2].try_into().unwrap()) as u32),
                 })
             } else { arg::InstructionArgs::CollectFees(PbCollectFeesLayout {}) } // Default
         },
         "SetProtocolFeeRate" => {
             if data.len() >= 2 { // 2 bytes u16
                 arg::InstructionArgs::SetProtocolFeeRate(PbSetProtocolFeeRateLayout {
                     protocol_fee_rate: Some(u16::from_le_bytes(data[0..2].try_into().unwrap()) as u32),
                 })
             } else { arg::InstructionArgs::CollectFees(PbCollectFeesLayout {}) } // Default
         },
         "SetRewardAuthority" => {
             if data.len() >= 1 { // 1 byte u8
                 arg::InstructionArgs::SetRewardAuthority(PbSetRewardAuthorityLayout {
                     reward_index: Some(data[0] as u32),
                 })
             } else { arg::InstructionArgs::CollectFees(PbCollectFeesLayout {}) } // Default
         },
         "SetRewardAuthorityBySuperAuthority" => {
             if data.len() >= 1 { // 1 byte u8
                 arg::InstructionArgs::SetRewardAuthorityBySuperAuthority(PbSetRewardAuthorityBySuperAuthorityLayout {
                     reward_index: Some(data[0] as u32),
                 })
             } else { arg::InstructionArgs::CollectFees(PbCollectFeesLayout {}) } // Default
         },
         "OpenBundledPosition" => {
             if data.len() >= 10 { // 2 bytes u16 + 4 bytes i32 + 4 bytes i32
                 arg::InstructionArgs::OpenBundledPosition(PbOpenBundledPositionLayout {
                     bundle_index: Some(u16::from_le_bytes(data[0..2].try_into().unwrap()) as u32),
                     tick_lower_index: Some(i32::from_le_bytes(data[2..6].try_into().unwrap())),
                     tick_upper_index: Some(i32::from_le_bytes(data[6..10].try_into().unwrap())),
                 })
             } else { arg::InstructionArgs::CollectFees(PbCollectFeesLayout {}) } // Default
         },
         "CloseBundledPosition" => {
             if data.len() >= 2 { // 2 bytes u16
                 arg::InstructionArgs::CloseBundledPosition(PbCloseBundledPositionLayout {
                     bundle_index: Some(u16::from_le_bytes(data[0..2].try_into().unwrap()) as u32),
                 })
             } else { arg::InstructionArgs::CollectFees(PbCollectFeesLayout {}) } // Default
         },
         "InitializeRewardV2" => {
              if data.len() >= 1 { // 1 byte u8
                 arg::InstructionArgs::InitializeRewardV2(PbInitializeRewardV2Layout {
                     reward_index: Some(data[0] as u32),
                })
            } else { arg::InstructionArgs::CollectFees(PbCollectFeesLayout {}) } // Default
        },
         "SetRewardEmissionsV2" => {
             if data.len() >= 17 { // 1 byte u8 + 16 bytes u128
                 let emissions = u128::from_le_bytes(data[1..17].try_into().unwrap());
                 let pb_emissions = PbUint128 { value: Some(u128_to_string(emissions)) };
                 arg::InstructionArgs::SetRewardEmissionsV2(PbSetRewardEmissionsV2Layout {
                     reward_index: Some(data[0] as u32),
                     emissions_per_second_x64: Some(pb_emissions),
                 })
             } else { arg::InstructionArgs::CollectFees(PbCollectFeesLayout {}) } // Default
         },
         "InitializePoolV2" => {
              if data.len() >= 18 { // 2 bytes u16 + 16 bytes u128
                 let initial_sqrt_price = u128::from_le_bytes(data[2..18].try_into().unwrap());
                 let pb_initial_sqrt_price = PbUint128 { value: Some(u128_to_string(initial_sqrt_price)) };
                 arg::InstructionArgs::InitializePoolV2(PbInitializePoolV2Layout {
                     tick_spacing: Some(u16::from_le_bytes(data[0..2].try_into().unwrap()) as u32),
                     initial_sqrt_price: Some(pb_initial_sqrt_price),
                })
            } else { arg::InstructionArgs::CollectFees(PbCollectFeesLayout {}) } // Default
        },
        "OpenPositionWithTokenExtensions" => {
            if data.len() >= 9 { // i32 (4) + i32 (4) + bool (1)
                arg::InstructionArgs::OpenPositionWithTokenExtensions(
                    PbOpenPositionWithTokenExtensionsLayout {
                        tick_lower_index: Some(i32::from_le_bytes(data[0..4].try_into().unwrap())),
                        tick_upper_index: Some(i32::from_le_bytes(data[4..8].try_into().unwrap())),
                        with_token_metadata_extension: Some(data[8] != 0),
                    }
                )
            } else {
                 arg::InstructionArgs::CollectFees(PbCollectFeesLayout {}) // Default if data too short
            }
        },
        _ => arg::InstructionArgs::CollectFees(PbCollectFeesLayout {}),
    }
} 