use substreams_solana::pb::sf::solana::r#type::v1::CompiledInstruction;
use chrono; // Needed for DateTime

use sha2::{Digest, Sha256};

use crate::pb::sf::solana::orca_whirlpool::v1::{
    Meta,
    FlatArg,
    arg::InstructionArgs as EnumInstructionArgs, // Alias to avoid conflict
    PbSwapLayout, // Add other Pb...Layout types as needed for create_flat_arg
    PbInitializePoolLayout,
    PbSetRewardEmissionsLayout,
    PbIncreaseLiquidityLayout,
    PbDecreaseLiquidityLayout,
    PbCollectRewardLayout,
    PbTwoHopSwapLayout,
    PbInitializeConfigLayout,
    PbInitializeTickArrayLayout,
    PbInitializeFeeTierLayout,
    PbOpenPositionLayout,
    PbOpenPositionWithMetadataLayout,
    PbSetDefaultFeeRateLayout,
    PbSetDefaultProtocolFeeRateLayout,
    PbSetFeeRateLayout,
    PbSetProtocolFeeRateLayout,
    PbSetRewardAuthorityLayout,
    PbSetRewardAuthorityBySuperAuthorityLayout,
    PbOpenBundledPositionLayout,
    PbCloseBundledPositionLayout,
    PbInitializeRewardV2Layout,
    PbSetRewardEmissionsV2Layout,
    PbInitializePoolV2Layout,
    PbIncreaseLiquidityV2Layout,
    PbDecreaseLiquidityV2Layout,
    PbSwapV2Layout,
    PbTwoHopSwapV2Layout,
    PbOpenPositionWithTokenExtensionsLayout,
    PbUint128, // Import PbUint128 for create_flat_arg
    InputAccounts, // Import InputAccounts
    PbOpenPositionBumps,
    PbOpenPositionWithMetadataBumps,
    PbInitializeRewardLayout,
    PbCollectFeesLayout,
    PbBumps,
    PbPubKey,
    // Layouts for manual decoding
    PbUpdateFeesAndRewardsLayout,
    PbCollectFeesV2Layout,
    PbCollectProtocolFeesV2Layout,
    PbClosePositionLayout,
    PbSetFeeAuthorityLayout,
    PbSetCollectProtocolFeesAuthorityLayout,
    PbSetRewardEmissionsSuperAuthorityLayout,
    PbInitializePositionBundleLayout,
    PbInitializePositionBundleWithMetadataLayout,
    PbDeletePositionBundleLayout,
    PbInitializeConfigExtensionLayout,
    PbSetConfigExtensionAuthorityLayout,
    PbSetTokenBadgeAuthorityLayout,
    PbInitializeTokenBadgeLayout,
    PbDeleteTokenBadgeLayout,
    PbInitializeAccountLayout,
    PbIdlWriteLayout,
    PbCollectRewardV2Layout,
    PbCollectProtocolFeesLayout,
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

/// Process a single instruction into a Meta object (Restored Original Logic + Signer/OuterProgram)
pub fn process_instruction(
    instruction: &CompiledInstruction,
    account_keys: &[String], // Original name
    block_slot: u64,
    block_time: i64, // Original name
    tx_id: &str,
    instruction_index: u32,
    is_inner_instruction: bool,
    inner_instruction_index: Option<u32>,
    // Added parameters
    signer: Option<&str>,
    outer_program: Option<&str>,
) -> Option<Meta> {
    // --- Start of original logic --- 
    let program_id = match account_keys.get(instruction.program_id_index as usize) {
        Some(id) => id,
        None => return None,
    };

    if program_id != ORCA_WHIRLPOOL_PROGRAM_ID {
        return None;
    }

    let (inst_type, decoded_args) = match decode_instruction_data(&instruction.data) {
        Ok((t, a)) => (t, a),
        Err(e) => {
            // Log decoding error if desired
            // substreams::log::info!("Decode error: {}", e);
            return None; 
        }
    };

    // Correct DateTime handling - Option return type
    let dt_opt = chrono::DateTime::from_timestamp(block_time, 0);
    let block_date = match dt_opt {
        Some(t) => t.format("%Y-%m-%d").to_string(),
        None => "".to_string(), // Or return None if date is critical
    };
    let block_time_i64 = dt_opt.map_or(0, |t| t.timestamp()); // Use 0 if None

    let mut flat_args = FlatArg {
        ..Default::default()
    };
    
    // Populate flat_args based on the SPECIFIC decoded instruction type
    match decoded_args {
        EnumInstructionArgs::Swap(swap) => {
            flat_args.amount = swap.amount.map(|v| v.to_string());
            flat_args.other_amount_threshold = swap.other_amount_threshold.map(|v| v.to_string());
            flat_args.sqrt_price_limit = swap.sqrt_price_limit.and_then(|p| p.value); // Direct access
            flat_args.amount_specified_is_input = swap.amount_specified_is_input;
            flat_args.a_to_b = swap.a_to_b;
        }
        EnumInstructionArgs::SwapV2(swap) => { // Separate V2 handling
            flat_args.amount = swap.amount.map(|v| v.to_string());
            flat_args.other_amount_threshold = swap.other_amount_threshold.map(|v| v.to_string());
            flat_args.sqrt_price_limit = swap.sqrt_price_limit.and_then(|p| p.value); // Direct access
            flat_args.amount_specified_is_input = swap.amount_specified_is_input;
            flat_args.a_to_b = swap.a_to_b;
        }
        EnumInstructionArgs::TwoHopSwap(two_hop) => {
            flat_args.amount = two_hop.amount.map(|v| v.to_string());
            flat_args.other_amount_threshold = two_hop.other_amount_threshold.map(|v| v.to_string());
            flat_args.amount_specified_is_input = two_hop.amount_specified_is_input;
            flat_args.a_to_b_one = two_hop.a_to_b_one;
            flat_args.a_to_b_two = two_hop.a_to_b_two;
            flat_args.sqrt_price_limit_one = two_hop.sqrt_price_limit_one.and_then(|p| p.value); // Direct access
            flat_args.sqrt_price_limit_two = two_hop.sqrt_price_limit_two.and_then(|p| p.value); // Direct access
        }
         EnumInstructionArgs::TwoHopSwapV2(two_hop) => { // Separate V2 handling
            flat_args.amount = two_hop.amount.map(|v| v.to_string());
            flat_args.other_amount_threshold = two_hop.other_amount_threshold.map(|v| v.to_string());
            flat_args.amount_specified_is_input = two_hop.amount_specified_is_input;
            flat_args.a_to_b_one = two_hop.a_to_b_one;
            flat_args.a_to_b_two = two_hop.a_to_b_two;
            flat_args.sqrt_price_limit_one = two_hop.sqrt_price_limit_one.and_then(|p| p.value); // Direct access
            flat_args.sqrt_price_limit_two = two_hop.sqrt_price_limit_two.and_then(|p| p.value); // Direct access
        }
        EnumInstructionArgs::InitializePool(pool) => {
            flat_args.tick_spacing = pool.tick_spacing;
            flat_args.sqrt_price_limit = pool.initial_sqrt_price.and_then(|p| p.value); // Direct access
        }
         EnumInstructionArgs::InitializePoolV2(pool) => { // Separate V2 handling
             flat_args.tick_spacing = pool.tick_spacing;
             flat_args.sqrt_price_limit = pool.initial_sqrt_price.and_then(|p| p.value); // Direct access
         }
        EnumInstructionArgs::SetRewardEmissions(reward) => {
            flat_args.reward_index = reward.reward_index;
            flat_args.emissions_per_second_x64 = reward.emissions_per_second_x64.and_then(|p| p.value); // Direct access
        }
         EnumInstructionArgs::SetRewardEmissionsV2(reward) => { // Separate V2 handling
             flat_args.reward_index = reward.reward_index;
             flat_args.emissions_per_second_x64 = reward.emissions_per_second_x64.and_then(|p| p.value); // Direct access
         }
        EnumInstructionArgs::IncreaseLiquidity(liq) => {
            flat_args.liquidity_amount = liq.liquidity_amount.and_then(|p| p.value); // Direct access
            flat_args.token_max_a = liq.token_max_a.map(|v| v.to_string());
            flat_args.token_max_b = liq.token_max_b.map(|v| v.to_string());
        }
         EnumInstructionArgs::IncreaseLiquidityV2(liq) => { // Separate V2 handling
             flat_args.liquidity_amount = liq.liquidity_amount.and_then(|p| p.value); // Direct access
             flat_args.token_max_a = liq.token_max_a.map(|v| v.to_string());
             flat_args.token_max_b = liq.token_max_b.map(|v| v.to_string());
         }
        EnumInstructionArgs::DecreaseLiquidity(liq) => {
            flat_args.liquidity_amount = liq.liquidity_amount.and_then(|p| p.value); // Direct access
            flat_args.token_min_a = liq.token_min_a.map(|v| v.to_string());
            flat_args.token_min_b = liq.token_min_b.map(|v| v.to_string());
        }
         EnumInstructionArgs::DecreaseLiquidityV2(liq) => { // Separate V2 handling
             flat_args.liquidity_amount = liq.liquidity_amount.and_then(|p| p.value); // Direct access
             flat_args.token_min_a = liq.token_min_a.map(|v| v.to_string());
             flat_args.token_min_b = liq.token_min_b.map(|v| v.to_string());
         }
        EnumInstructionArgs::CollectReward(reward) => {
            flat_args.reward_index = reward.reward_index;
        }
         EnumInstructionArgs::CollectRewardV2(reward) => { // Separate V2 handling
            flat_args.reward_index = reward.reward_index;
         }
        EnumInstructionArgs::InitializeTickArray(tick) => {
            flat_args.start_tick_index = tick.start_tick_index;
        }
        EnumInstructionArgs::InitializeFeeTier(fee) => {
            flat_args.tick_spacing = fee.tick_spacing;
            flat_args.default_fee_rate = fee.default_fee_rate;
        }
        EnumInstructionArgs::OpenPosition(pos) => {
            flat_args.tick_lower_index = pos.tick_lower_index;
            flat_args.tick_upper_index = pos.tick_upper_index;
        }
        EnumInstructionArgs::OpenPositionWithMetadata(pos) => {
            flat_args.tick_lower_index = pos.tick_lower_index;
            flat_args.tick_upper_index = pos.tick_upper_index;
        }
        EnumInstructionArgs::SetDefaultFeeRate(fee) => {
            flat_args.default_fee_rate = fee.default_fee_rate;
        }
        EnumInstructionArgs::SetDefaultProtocolFeeRate(fee) => {
            flat_args.default_protocol_fee_rate = fee.default_protocol_fee_rate;
        }
        EnumInstructionArgs::SetFeeRate(fee) => {
            flat_args.fee_rate = fee.fee_rate;
        }
        EnumInstructionArgs::SetProtocolFeeRate(fee) => {
            flat_args.protocol_fee_rate = fee.protocol_fee_rate;
        }
        EnumInstructionArgs::SetRewardAuthority(reward) => {
            flat_args.reward_index = reward.reward_index;
        }
        EnumInstructionArgs::SetRewardAuthorityBySuperAuthority(reward) => {
            flat_args.reward_index = reward.reward_index;
        }
        EnumInstructionArgs::OpenBundledPosition(bundle) => {
            flat_args.bundle_index = bundle.bundle_index;
            flat_args.tick_lower_index = bundle.tick_lower_index;
            flat_args.tick_upper_index = bundle.tick_upper_index;
        }
        EnumInstructionArgs::CloseBundledPosition(bundle) => {
            flat_args.bundle_index = bundle.bundle_index;
        }
         EnumInstructionArgs::InitializeRewardV2(reward) => {
            flat_args.reward_index = reward.reward_index;
         }
        EnumInstructionArgs::InitializeConfig(config) => {
            flat_args.default_protocol_fee_rate = config.default_protocol_fee_rate;
        }
         EnumInstructionArgs::OpenPositionWithTokenExtensions(pos) => {
            flat_args.tick_lower_index = pos.tick_lower_index;
            flat_args.tick_upper_index = pos.tick_upper_index;
            flat_args.with_token_metadata_extension = pos.with_token_metadata_extension;
        }
        // Instructions with no specific args to map to FlatArg
        EnumInstructionArgs::UpdateFeesAndRewards(_) |
        EnumInstructionArgs::CollectFees(_) |
        EnumInstructionArgs::CollectProtocolFees(_) |
        EnumInstructionArgs::ClosePosition(_) |
        EnumInstructionArgs::SetFeeAuthority(_) |
        EnumInstructionArgs::SetCollectProtocolFeesAuthority(_) |
        EnumInstructionArgs::SetRewardEmissionsSuperAuthority(_) |
        EnumInstructionArgs::InitializePositionBundle(_) |
        EnumInstructionArgs::InitializePositionBundleWithMetadata(_) |
        EnumInstructionArgs::DeletePositionBundle(_) |
        EnumInstructionArgs::InitializeConfigExtension(_) |
        EnumInstructionArgs::SetConfigExtensionAuthority(_) |
        EnumInstructionArgs::SetTokenBadgeAuthority(_) |
        EnumInstructionArgs::InitializeTokenBadge(_) |
        EnumInstructionArgs::DeleteTokenBadge(_) |
        EnumInstructionArgs::InitializeAccount(_) |
        EnumInstructionArgs::IdlWrite(_) => {
             // No args to map for these instructions
        }
        // Default case added for safety, though decode_instruction_args should handle all valid types
        _ => { 
             // Potentially log an unhandled instruction type if needed
        }
    }

    let account_indices: Vec<u32> = instruction.accounts.iter().map(|&idx| idx as u32).collect();
    // Wrap result in Some()
    let input_accounts_opt: Option<InputAccounts> = Some(prepare_input_accounts::prepare_input_accounts(
        inst_type.clone(),
        &account_indices,
        account_keys, // Use original variable name
    ));
    // --- End of original logic --- 

    // Determine final_outer_program (logic added previously)
    let final_outer_program = if is_inner_instruction {
        outer_program.unwrap_or_default()
    } else {
        program_id // Use instruction's own ID if outer
    };

    // Create the Meta object, using original variables + added signer/outer_program
    let meta = Meta {
        block_date: Some(block_date),
        block_time: Some(block_time_i64), // Use extracted i64 timestamp
        block_slot: Some(block_slot),
        tx_id: Some(tx_id.to_string()),
        instruction_index: Some(instruction_index),
        is_inner_instruction: Some(is_inner_instruction),
        inner_instruction_index: Some(inner_instruction_index.unwrap_or(0)), // Use 0 if None for inner index
        dapp: Some(program_id.to_string()),
        instruction_type: Some(inst_type), // Use inst_type from decode_instruction_data
        args: Some(flat_args), // Use the populated flat_args
        input_accounts: input_accounts_opt,
        signer: Some(signer.unwrap_or_default().to_string()), // Added
        outer_program: Some(final_outer_program.to_string()), // Added
    };

    Some(meta)
}

/// Decode instruction data into instruction type and decoded args enum
pub fn decode_instruction_data(data: &[u8]) -> Result<(String, EnumInstructionArgs), String> {
    if data.len() < 8 {
        return Err("Instruction data too short".to_string());
    }
    let discriminator = &data[0..8];
    let inst_type = match INSTRUCTION_TYPES.iter().find(|(disc, _)| disc == &discriminator) {
        Some((_, inst_type)) => inst_type.to_string(),
        None => return Err(format!("Unknown instruction discriminator: {:?}", discriminator)),
    };

    // Use ? operator to handle Result from decode_instruction_args
    let instruction_args = decode_instruction_args(&inst_type, &data[8..])?;
    Ok((inst_type, instruction_args))
}

// Helper function to decode instruction arguments based on the instruction type
// Reverted to manual byte parsing
pub fn decode_instruction_args(inst_type: &str, data: &[u8]) -> Result<EnumInstructionArgs, String> {
    use crate::pb::sf::solana::orca_whirlpool::v1::arg; // Only need arg namespace here

    // Helper to convert u128 bytes to PbUint128
    fn bytes_to_pb_u128(bytes: &[u8]) -> Result<Option<PbUint128>, String> {
        if bytes.len() != 16 {
            return Err(format!("Invalid length for u128: {}", bytes.len()));
        }
        let val = u128::from_le_bytes(bytes.try_into().map_err(|_| "Failed to convert slice to u128 array")?);
        Ok(Some(PbUint128 { value: Some(val.to_string()) }))
    }

    // Helper to convert pubkey bytes to PbPubKey
    fn bytes_to_pb_pubkey(bytes: &[u8]) -> Result<Option<PbPubKey>, String> {
        if bytes.len() != 32 {
            return Err(format!("Invalid length for PubKey: {}", bytes.len()));
        }
        Ok(Some(PbPubKey { pub_key: Some(bytes.to_vec()) }))
    }

    // Simplified macro for checking length and handling errors
    macro_rules! check_len {
        ($data:expr, $expected:expr, $name:expr) => {
            if $data.len() < $expected {
                return Err(format!("{}: Insufficient data length. Expected {}, got {}", $name, $expected, $data.len()));
            }
        };
    }

    let args = match inst_type {
        "Swap" => {
            check_len!(data, 34, "Swap");
            let amount = u64::from_le_bytes(data[0..8].try_into().map_err(|e| format!("Swap amount: {}", e))?);
            let other_amount_threshold = u64::from_le_bytes(data[8..16].try_into().map_err(|e| format!("Swap threshold: {}", e))?);
            let sqrt_price_limit = bytes_to_pb_u128(&data[16..32])?;
            let amount_specified_is_input = data[32] != 0;
            let a_to_b = data[33] != 0;
            Ok(arg::InstructionArgs::Swap(PbSwapLayout {
                amount: Some(amount),
                other_amount_threshold: Some(other_amount_threshold),
                sqrt_price_limit,
                amount_specified_is_input: Some(amount_specified_is_input),
                a_to_b: Some(a_to_b),
            }))
        },
        "InitializePool" => {
            check_len!(data, 19, "InitializePool");
            let bumps = PbBumps { whirlpool_bump: Some(data[0] as u32) };
            let tick_spacing = u16::from_le_bytes(data[1..3].try_into().map_err(|e| format!("InitializePool tick_spacing: {}", e))?);
            let initial_sqrt_price = bytes_to_pb_u128(&data[3..19])?;
            Ok(arg::InstructionArgs::InitializePool(PbInitializePoolLayout {
                bumps: Some(bumps),
                tick_spacing: Some(tick_spacing as u32),
                initial_sqrt_price,
            }))
        },
        "SetRewardEmissions" => {
            check_len!(data, 17, "SetRewardEmissions");
            let reward_index = data[0];
            let emissions_per_second_x64 = bytes_to_pb_u128(&data[1..17])?;
            Ok(arg::InstructionArgs::SetRewardEmissions(PbSetRewardEmissionsLayout {
                reward_index: Some(reward_index as u32),
                emissions_per_second_x64,
            }))
        },
        "IncreaseLiquidity" => {
            check_len!(data, 32, "IncreaseLiquidity");
            let liquidity_amount = bytes_to_pb_u128(&data[0..16])?;
            let token_max_a = u64::from_le_bytes(data[16..24].try_into().map_err(|e| format!("IncreaseLiquidity token_max_a: {}", e))?);
            let token_max_b = u64::from_le_bytes(data[24..32].try_into().map_err(|e| format!("IncreaseLiquidity token_max_b: {}", e))?);
            Ok(arg::InstructionArgs::IncreaseLiquidity(PbIncreaseLiquidityLayout {
                liquidity_amount,
                token_max_a: Some(token_max_a),
                token_max_b: Some(token_max_b),
            }))
        },
        "DecreaseLiquidity" => {
             check_len!(data, 32, "DecreaseLiquidity");
            let liquidity_amount = bytes_to_pb_u128(&data[0..16])?;
            let token_min_a = u64::from_le_bytes(data[16..24].try_into().map_err(|e| format!("DecreaseLiquidity token_min_a: {}", e))?);
            let token_min_b = u64::from_le_bytes(data[24..32].try_into().map_err(|e| format!("DecreaseLiquidity token_min_b: {}", e))?);
            Ok(arg::InstructionArgs::DecreaseLiquidity(PbDecreaseLiquidityLayout {
                liquidity_amount,
                token_min_a: Some(token_min_a),
                token_min_b: Some(token_min_b),
            }))
        },
        "CollectReward" => {
            check_len!(data, 1, "CollectReward");
            let reward_index = data[0];
            Ok(arg::InstructionArgs::CollectReward(PbCollectRewardLayout {
                reward_index: Some(reward_index as u32),
            }))
        },
        "TwoHopSwap" => {
            check_len!(data, 51, "TwoHopSwap");
            let amount = u64::from_le_bytes(data[0..8].try_into().map_err(|e| format!("TwoHopSwap amount: {}", e))?);
            let other_amount_threshold = u64::from_le_bytes(data[8..16].try_into().map_err(|e| format!("TwoHopSwap threshold: {}", e))?);
            let amount_specified_is_input = data[16] != 0;
            let a_to_b_one = data[17] != 0;
            let a_to_b_two = data[18] != 0;
            let sqrt_price_limit_one = bytes_to_pb_u128(&data[19..35])?;
            let sqrt_price_limit_two = bytes_to_pb_u128(&data[35..51])?;
            Ok(arg::InstructionArgs::TwoHopSwap(PbTwoHopSwapLayout {
                amount: Some(amount),
                other_amount_threshold: Some(other_amount_threshold),
                amount_specified_is_input: Some(amount_specified_is_input),
                a_to_b_one: Some(a_to_b_one),
                a_to_b_two: Some(a_to_b_two),
                sqrt_price_limit_one,
                sqrt_price_limit_two,
            }))
        },
        "SwapV2" => {
            check_len!(data, 34, "SwapV2"); // Assuming same size, adjust if needed
            let amount = u64::from_le_bytes(data[0..8].try_into().map_err(|e| format!("SwapV2 amount: {}", e))?);
            let other_amount_threshold = u64::from_le_bytes(data[8..16].try_into().map_err(|e| format!("SwapV2 threshold: {}", e))?);
            let sqrt_price_limit = bytes_to_pb_u128(&data[16..32])?;
            let amount_specified_is_input = data[32] != 0;
            let a_to_b = data[33] != 0;
            // remaining_accounts_info not decoded here
            Ok(arg::InstructionArgs::SwapV2(PbSwapV2Layout {
                amount: Some(amount),
                other_amount_threshold: Some(other_amount_threshold),
                sqrt_price_limit,
                amount_specified_is_input: Some(amount_specified_is_input),
                a_to_b: Some(a_to_b),
                remaining_accounts_info: None, // Set to None or handle if defined
            }))
        },
         "TwoHopSwapV2" => {
            check_len!(data, 51, "TwoHopSwapV2"); // Assuming same size
            let amount = u64::from_le_bytes(data[0..8].try_into().map_err(|e| format!("TwoHopSwapV2 amount: {}", e))?);
            let other_amount_threshold = u64::from_le_bytes(data[8..16].try_into().map_err(|e| format!("TwoHopSwapV2 threshold: {}", e))?);
            let amount_specified_is_input = data[16] != 0;
            let a_to_b_one = data[17] != 0;
            let a_to_b_two = data[18] != 0;
            let sqrt_price_limit_one = bytes_to_pb_u128(&data[19..35])?;
            let sqrt_price_limit_two = bytes_to_pb_u128(&data[35..51])?;
             // remaining_accounts_info not decoded here
            Ok(arg::InstructionArgs::TwoHopSwapV2(PbTwoHopSwapV2Layout {
                amount: Some(amount),
                other_amount_threshold: Some(other_amount_threshold),
                amount_specified_is_input: Some(amount_specified_is_input),
                a_to_b_one: Some(a_to_b_one),
                a_to_b_two: Some(a_to_b_two),
                sqrt_price_limit_one,
                sqrt_price_limit_two,
                remaining_accounts_info: None, // Set to None or handle if defined
            }))
         },
        "InitializePoolV2" => {
            check_len!(data, 18, "InitializePoolV2"); // tick_spacing (u16) + sqrt_price (u128)
            let tick_spacing = u16::from_le_bytes(data[0..2].try_into().map_err(|e| format!("InitializePoolV2 tick_spacing: {}", e))?);
            let initial_sqrt_price = bytes_to_pb_u128(&data[2..18])?;
             // bumps/remaining_accounts_info not decoded here
            Ok(arg::InstructionArgs::InitializePoolV2(PbInitializePoolV2Layout {
                tick_spacing: Some(tick_spacing as u32),
                initial_sqrt_price,
                // bumps: None, // Set if defined
            }))
        },
         "InitializeRewardV2" => {
            check_len!(data, 1, "InitializeRewardV2");
            let reward_index = data[0];
             // remaining_accounts_info not decoded here
            Ok(arg::InstructionArgs::InitializeRewardV2(PbInitializeRewardV2Layout {
                reward_index: Some(reward_index as u32),
            }))
         },
         "SetRewardEmissionsV2" => {
             check_len!(data, 17, "SetRewardEmissionsV2");
            let reward_index = data[0];
            let emissions_per_second_x64 = bytes_to_pb_u128(&data[1..17])?;
             // remaining_accounts_info not decoded here
            Ok(arg::InstructionArgs::SetRewardEmissionsV2(PbSetRewardEmissionsV2Layout {
                reward_index: Some(reward_index as u32),
                emissions_per_second_x64,
            }))
         },
         "IncreaseLiquidityV2" => {
            check_len!(data, 32, "IncreaseLiquidityV2"); // Assuming same size
            let liquidity_amount = bytes_to_pb_u128(&data[0..16])?;
            let token_max_a = u64::from_le_bytes(data[16..24].try_into().map_err(|e| format!("IncreaseLiquidityV2 token_max_a: {}", e))?);
            let token_max_b = u64::from_le_bytes(data[24..32].try_into().map_err(|e| format!("IncreaseLiquidityV2 token_max_b: {}", e))?);
             // remaining_accounts_info not decoded here
            Ok(arg::InstructionArgs::IncreaseLiquidityV2(PbIncreaseLiquidityV2Layout {
                liquidity_amount,
                token_max_a: Some(token_max_a),
                token_max_b: Some(token_max_b),
                remaining_accounts_info: None, // Set if defined
            }))
         },
         "DecreaseLiquidityV2" => {
             check_len!(data, 32, "DecreaseLiquidityV2"); // Assuming same size
            let liquidity_amount = bytes_to_pb_u128(&data[0..16])?;
            let token_min_a = u64::from_le_bytes(data[16..24].try_into().map_err(|e| format!("DecreaseLiquidityV2 token_min_a: {}", e))?);
            let token_min_b = u64::from_le_bytes(data[24..32].try_into().map_err(|e| format!("DecreaseLiquidityV2 token_min_b: {}", e))?);
            // remaining_accounts_info not decoded here
            Ok(arg::InstructionArgs::DecreaseLiquidityV2(PbDecreaseLiquidityV2Layout {
                liquidity_amount,
                token_min_a: Some(token_min_a),
                token_min_b: Some(token_min_b),
                remaining_accounts_info: None, // Set if defined
            }))
         },
        "CollectRewardV2" => {
            check_len!(data, 1, "CollectRewardV2");
            let reward_index = data[0];
             // remaining_accounts_info not decoded here
            Ok(arg::InstructionArgs::CollectRewardV2(PbCollectRewardV2Layout {
                reward_index: Some(reward_index as u32),
                remaining_accounts_info: None, // Set if defined
            }))
        },
        "InitializeConfig" => {
            check_len!(data, 98, "InitializeConfig"); // 3 * Pubkey + u16
            let fee_authority = bytes_to_pb_pubkey(&data[0..32])?;
            let collect_protocol_fees_authority = bytes_to_pb_pubkey(&data[32..64])?;
            let reward_emissions_super_authority = bytes_to_pb_pubkey(&data[64..96])?;
            let default_protocol_fee_rate = u16::from_le_bytes(data[96..98].try_into().map_err(|e| format!("InitializeConfig fee_rate: {}", e))?);
            Ok(arg::InstructionArgs::InitializeConfig(PbInitializeConfigLayout {
                fee_authority,
                collect_protocol_fees_authority,
                reward_emissions_super_authority,
                default_protocol_fee_rate: Some(default_protocol_fee_rate as u32),
            }))
        },
        "InitializeTickArray" => {
            check_len!(data, 4, "InitializeTickArray");
            let start_tick_index = i32::from_le_bytes(data[0..4].try_into().map_err(|e| format!("InitializeTickArray index: {}", e))?);
            Ok(arg::InstructionArgs::InitializeTickArray(PbInitializeTickArrayLayout {
                start_tick_index: Some(start_tick_index),
            }))
        },
         "InitializeFeeTier" => {
            check_len!(data, 4, "InitializeFeeTier");
            let tick_spacing = u16::from_le_bytes(data[0..2].try_into().map_err(|e| format!("InitializeFeeTier spacing: {}", e))?);
            let default_fee_rate = u16::from_le_bytes(data[2..4].try_into().map_err(|e| format!("InitializeFeeTier rate: {}", e))?);
            Ok(arg::InstructionArgs::InitializeFeeTier(PbInitializeFeeTierLayout {
                tick_spacing: Some(tick_spacing as u32),
                default_fee_rate: Some(default_fee_rate as u32),
            }))
         },
        "InitializeReward" => { // Shares layout with CollectReward
            check_len!(data, 1, "InitializeReward");
            let reward_index = data[0];
            Ok(arg::InstructionArgs::InitializeReward(PbInitializeRewardLayout {
                reward_index: Some(reward_index as u32),
            }))
        },
        "OpenPosition" => {
            check_len!(data, 9, "OpenPosition");
            let bumps = PbOpenPositionBumps { position_bump: Some(data[0] as u32) };
            let tick_lower_index = i32::from_le_bytes(data[1..5].try_into().map_err(|e| format!("OpenPosition lower: {}", e))?);
            let tick_upper_index = i32::from_le_bytes(data[5..9].try_into().map_err(|e| format!("OpenPosition upper: {}", e))?);
            Ok(arg::InstructionArgs::OpenPosition(PbOpenPositionLayout {
                open_position_bumps: Some(bumps),
                tick_lower_index: Some(tick_lower_index),
                tick_upper_index: Some(tick_upper_index),
            }))
        },
         "OpenPositionWithMetadata" => {
            check_len!(data, 10, "OpenPositionWithMetadata");
            let bumps = PbOpenPositionWithMetadataBumps { 
                position_bump: Some(data[0] as u32),
                metadata_bump: Some(data[1] as u32),
            };
            let tick_lower_index = i32::from_le_bytes(data[2..6].try_into().map_err(|e| format!("OpenPositionWithMetadata lower: {}", e))?);
            let tick_upper_index = i32::from_le_bytes(data[6..10].try_into().map_err(|e| format!("OpenPositionWithMetadata upper: {}", e))?);
            Ok(arg::InstructionArgs::OpenPositionWithMetadata(PbOpenPositionWithMetadataLayout {
                open_position_with_metadata_bumps: Some(bumps),
                tick_lower_index: Some(tick_lower_index),
                tick_upper_index: Some(tick_upper_index),
            }))
         },
         "OpenPositionWithTokenExtensions" => {
            check_len!(data, 9, "OpenPositionWithTokenExtensions"); // i32 + i32 + bool
            let tick_lower_index = i32::from_le_bytes(data[0..4].try_into().map_err(|e| format!("OpenPositionWithTokenExtensions lower: {}", e))?);
            let tick_upper_index = i32::from_le_bytes(data[4..8].try_into().map_err(|e| format!("OpenPositionWithTokenExtensions upper: {}", e))?);
            let with_token_metadata_extension = data[8] != 0;
             // bumps not in this layout?
            Ok(arg::InstructionArgs::OpenPositionWithTokenExtensions(
                PbOpenPositionWithTokenExtensionsLayout {
                    tick_lower_index: Some(tick_lower_index),
                    tick_upper_index: Some(tick_upper_index),
                    with_token_metadata_extension: Some(with_token_metadata_extension),
                }
            ))
         },
         "SetDefaultFeeRate" => {
            check_len!(data, 2, "SetDefaultFeeRate");
            let default_fee_rate = u16::from_le_bytes(data[0..2].try_into().map_err(|e| format!("SetDefaultFeeRate rate: {}", e))?);
            Ok(arg::InstructionArgs::SetDefaultFeeRate(PbSetDefaultFeeRateLayout {
                default_fee_rate: Some(default_fee_rate as u32),
            }))
         },
         "SetDefaultProtocolFeeRate" => {
            check_len!(data, 2, "SetDefaultProtocolFeeRate");
            let default_protocol_fee_rate = u16::from_le_bytes(data[0..2].try_into().map_err(|e| format!("SetDefaultProtocolFeeRate rate: {}", e))?);
            Ok(arg::InstructionArgs::SetDefaultProtocolFeeRate(PbSetDefaultProtocolFeeRateLayout {
                default_protocol_fee_rate: Some(default_protocol_fee_rate as u32),
            }))
         },
         "SetFeeRate" => {
            check_len!(data, 2, "SetFeeRate");
            let fee_rate = u16::from_le_bytes(data[0..2].try_into().map_err(|e| format!("SetFeeRate rate: {}", e))?);
            Ok(arg::InstructionArgs::SetFeeRate(PbSetFeeRateLayout {
                fee_rate: Some(fee_rate as u32),
            }))
         },
         "SetProtocolFeeRate" => {
            check_len!(data, 2, "SetProtocolFeeRate");
            let protocol_fee_rate = u16::from_le_bytes(data[0..2].try_into().map_err(|e| format!("SetProtocolFeeRate rate: {}", e))?);
            Ok(arg::InstructionArgs::SetProtocolFeeRate(PbSetProtocolFeeRateLayout {
                protocol_fee_rate: Some(protocol_fee_rate as u32),
            }))
         },
         "SetRewardAuthority" => { // Shares layout with CollectReward
            check_len!(data, 1, "SetRewardAuthority");
            let reward_index = data[0];
            Ok(arg::InstructionArgs::SetRewardAuthority(PbSetRewardAuthorityLayout {
                reward_index: Some(reward_index as u32),
            }))
         },
         "SetRewardAuthorityBySuperAuthority" => { // Shares layout with CollectReward
            check_len!(data, 1, "SetRewardAuthorityBySuperAuthority");
            let reward_index = data[0];
            Ok(arg::InstructionArgs::SetRewardAuthorityBySuperAuthority(PbSetRewardAuthorityBySuperAuthorityLayout {
                reward_index: Some(reward_index as u32),
            }))
         },
         "OpenBundledPosition" => {
            check_len!(data, 10, "OpenBundledPosition");
            let bundle_index = u16::from_le_bytes(data[0..2].try_into().map_err(|e| format!("OpenBundledPosition index: {}", e))?);
            let tick_lower_index = i32::from_le_bytes(data[2..6].try_into().map_err(|e| format!("OpenBundledPosition lower: {}", e))?);
            let tick_upper_index = i32::from_le_bytes(data[6..10].try_into().map_err(|e| format!("OpenBundledPosition upper: {}", e))?);
            Ok(arg::InstructionArgs::OpenBundledPosition(PbOpenBundledPositionLayout {
                bundle_index: Some(bundle_index as u32),
                tick_lower_index: Some(tick_lower_index),
                tick_upper_index: Some(tick_upper_index),
            }))
         },
         "CloseBundledPosition" => {
            check_len!(data, 2, "CloseBundledPosition");
            let bundle_index = u16::from_le_bytes(data[0..2].try_into().map_err(|e| format!("CloseBundledPosition index: {}", e))?);
            Ok(arg::InstructionArgs::CloseBundledPosition(PbCloseBundledPositionLayout {
                bundle_index: Some(bundle_index as u32),
            }))
         },

        // Instructions with no arguments or args not mapped
        "UpdateFeesAndRewards" => Ok(arg::InstructionArgs::UpdateFeesAndRewards(PbUpdateFeesAndRewardsLayout::default())), 
        "CollectFees" => Ok(arg::InstructionArgs::CollectFees(PbCollectFeesLayout::default())), 
        "CollectProtocolFees" => Ok(arg::InstructionArgs::CollectProtocolFees(PbCollectProtocolFeesLayout::default())), 
        "ClosePosition" => Ok(arg::InstructionArgs::ClosePosition(PbClosePositionLayout::default())), 
        "SetFeeAuthority" => Ok(arg::InstructionArgs::SetFeeAuthority(PbSetFeeAuthorityLayout::default())), 
        "SetCollectProtocolFeesAuthority" => Ok(arg::InstructionArgs::SetCollectProtocolFeesAuthority(PbSetCollectProtocolFeesAuthorityLayout::default())), 
        "SetRewardEmissionsSuperAuthority" => Ok(arg::InstructionArgs::SetRewardEmissionsSuperAuthority(PbSetRewardEmissionsSuperAuthorityLayout::default())), 
        "InitializePositionBundle" => Ok(arg::InstructionArgs::InitializePositionBundle(PbInitializePositionBundleLayout::default())), 
        "InitializePositionBundleWithMetadata" => Ok(arg::InstructionArgs::InitializePositionBundleWithMetadata(PbInitializePositionBundleWithMetadataLayout::default())), 
        "DeletePositionBundle" => Ok(arg::InstructionArgs::DeletePositionBundle(PbDeletePositionBundleLayout::default())), 
        "InitializeConfigExtension" => Ok(arg::InstructionArgs::InitializeConfigExtension(PbInitializeConfigExtensionLayout::default())), 
        "SetConfigExtensionAuthority" => Ok(arg::InstructionArgs::SetConfigExtensionAuthority(PbSetConfigExtensionAuthorityLayout::default())), 
        "SetTokenBadgeAuthority" => Ok(arg::InstructionArgs::SetTokenBadgeAuthority(PbSetTokenBadgeAuthorityLayout::default())), 
        "InitializeTokenBadge" => Ok(arg::InstructionArgs::InitializeTokenBadge(PbInitializeTokenBadgeLayout::default())),
        "DeleteTokenBadge" => Ok(arg::InstructionArgs::DeleteTokenBadge(PbDeleteTokenBadgeLayout::default())), 
        "InitializeAccount" => Ok(arg::InstructionArgs::InitializeAccount(PbInitializeAccountLayout::default())), 
        "IdlWrite" => Ok(arg::InstructionArgs::IdlWrite(PbIdlWriteLayout::default())), // Args not decoded
        "CollectFeesV2" => Ok(arg::InstructionArgs::CollectFeesV2(PbCollectFeesV2Layout::default())), // Args not decoded
        "CollectProtocolFeesV2" => Ok(arg::InstructionArgs::CollectProtocolFeesV2(PbCollectProtocolFeesV2Layout::default())), // Args not decoded

        // Fallback for any unexpected instruction type
        _ => Err(format!("Unknown or unhandled instruction type for manual decoding: {}", inst_type)),
    };

    args
} 