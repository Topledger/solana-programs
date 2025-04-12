use substreams_solana::pb::sf::solana::r#type::v1::CompiledInstruction;

// Known discriminators for Orca Whirlpool instructions
const INITIALIZE_CONFIG_DISCRIMINATOR: [u8; 8] = [208, 127, 21, 1, 194, 190, 196, 70];
const INITIALIZE_POOL_DISCRIMINATOR: [u8; 8] = [95, 180, 10, 172, 84, 174, 232, 40];
const INITIALIZE_POOL_V2_DISCRIMINATOR: [u8; 8] = [207, 45, 87, 242, 27, 63, 204, 67];
const INITIALIZE_FEE_TIER_DISCRIMINATOR: [u8; 8] = [183, 74, 156, 160, 112, 2, 42, 30];
const SET_DEFAULT_FEE_RATE_DISCRIMINATOR: [u8; 8] = [118, 215, 214, 157, 182, 229, 208, 228];
const SET_DEFAULT_PROTOCOL_FEE_RATE_DISCRIMINATOR: [u8; 8] = [107, 205, 249, 226, 151, 35, 86, 0];
const SET_FEE_RATE_DISCRIMINATOR: [u8; 8] = [53, 243, 137, 65, 8, 140, 158, 6];
const SET_PROTOCOL_FEE_RATE_DISCRIMINATOR: [u8; 8] = [95, 7, 4, 50, 154, 79, 156, 131];

pub fn parse_instruction(
    instruction: &CompiledInstruction,
    account_keys: &[String],
    _instruction_index: u32,
    _is_inner_instruction: bool,
    _inner_instruction_index: Option<u32>,
) -> Option<(String, Option<String>, Option<String>, Option<u64>, Option<u64>, Option<u64>, Option<u64>, Option<u64>, Option<u64>)> {
    let data = &instruction.data;
    if data.len() < 8 {
        return None;
    }

    let discriminator = &data[0..8];
    
    match discriminator {
        // InitializeConfig instruction
        disc if disc == &INITIALIZE_CONFIG_DISCRIMINATOR => {
            // InitializeConfig accounts:
            // 0: whirlpools_config (write)
            // 1: funder (signer)
            // 2: system_program
            // Additional data: fee_authority, collect_protocol_fees_authority, reward_emissions_super_authority, default_protocol_fee_rate
            
            let config = get_account_at_index(instruction, account_keys, 0);
            
            // Parse default_protocol_fee_rate from data
            // Data structure: [8 bytes discriminator][32 bytes fee_auth][32 bytes collect_fees_auth][32 bytes reward_auth][2 bytes default_protocol_fee_rate]
            let default_protocol_fee_rate = if data.len() >= 106 {
                parse_u16_from_data(data, 104)
            } else {
                Some(0)
            };
            
            let trade_fee_rate = Some(0); // Not defined in InitializeConfig
            let fund_fee_rate = Some(0); // Not defined in InitializeConfig
            let param = Some(0);
            let value = Some(0);
            let create_pool_fee = Some(0);
            
            Some((
                "InitializeConfig".to_string(),
                None, // No pool address
                config,
                trade_fee_rate,
                default_protocol_fee_rate,
                fund_fee_rate,
                param,
                value,
                create_pool_fee,
            ))
        }
        
        // InitializePool instruction
        disc if disc == &INITIALIZE_POOL_DISCRIMINATOR => {
            // InitializePool accounts:
            // 0: whirlpools_config
            // 1: token_mint_a
            // 2: token_mint_b
            // 3: funder (signer)
            // 4: whirlpool (write)
            // 5: token_vault_a (write)
            // 6: token_vault_b (write)
            // 7: fee_tier
            // 8: token_program
            // 9: system_program
            // 10: rent
            
            let config = get_account_at_index(instruction, account_keys, 0);
            let pool_address = get_account_at_index(instruction, account_keys, 4);
            let fee_tier = get_account_at_index(instruction, account_keys, 7);
            
            // We need to fetch the fee rate from the fee_tier, but we don't have direct access
            // Let's use default values based on common Orca fee tiers
            let trade_fee_rate = Some(0);
            let protocol_fee_rate = Some(0);
            let fund_fee_rate = Some(0);
            let param = Some(0);
            let value = Some(0);
            let create_pool_fee = Some(0);
            
            Some((
                "InitializePool".to_string(),
                pool_address,
                config,
                trade_fee_rate,
                protocol_fee_rate,
                fund_fee_rate,
                param,
                value,
                create_pool_fee,
            ))
        }
        
        // InitializePoolV2 instruction
        disc if disc == &INITIALIZE_POOL_V2_DISCRIMINATOR => {
            // InitializePoolV2 accounts similar to InitializePool but with slight variations
            // 0: whirlpools_config
            // 1: token_mint_a 
            // 2: token_mint_b
            // 3: funder (signer)
            // 4: whirlpool (write)
            // 5: token_vault_a (write)
            // 6: token_vault_b (write)
            // 7: fee_tier
            // 8: token_program_a
            // 9: token_program_b
            // 10: system_program
            // 11: rent
            
            let config = get_account_at_index(instruction, account_keys, 0);
            let pool_address = get_account_at_index(instruction, account_keys, 4);
            let fee_tier = get_account_at_index(instruction, account_keys, 7);
            
            // Similar to InitializePool
            let trade_fee_rate = Some(0);
            let protocol_fee_rate = Some(0);
            let fund_fee_rate = Some(0);
            let param = Some(0);
            let value = Some(0);
            let create_pool_fee = Some(0);
            
            Some((
                "InitializePoolV2".to_string(),
                pool_address,
                config,
                trade_fee_rate,
                protocol_fee_rate,
                fund_fee_rate,
                param,
                value,
                create_pool_fee,
            ))
        }
        
        // InitializeFeeTier instruction
        disc if disc == &INITIALIZE_FEE_TIER_DISCRIMINATOR => {
            // InitializeFeeTier accounts:
            // 0: config
            // 1: fee_tier (write)
            // 2: funder (signer)
            // 3: fee_authority
            // 4: system_program
            
            let config = get_account_at_index(instruction, account_keys, 0);
            let fee_tier = get_account_at_index(instruction, account_keys, 1);
            
            // Parse fee rate from data
            // Data structure: [8 bytes discriminator][2 bytes tick_spacing][2 bytes default_fee_rate]
            let default_fee_rate = if data.len() >= 12 {
                parse_u16_from_data(data, 10)
            } else {
                Some(0)
            };
            
            let protocol_fee_rate = Some(0); // Not defined in InitializeFeeTier
            let fund_fee_rate = Some(0); // Not defined in InitializeFeeTier
            let param = Some(0);
            let value = Some(0);
            let create_pool_fee = Some(0);
            
            Some((
                "InitializeFeeTier".to_string(),
                None, // No pool address
                config,
                default_fee_rate,
                protocol_fee_rate,
                fund_fee_rate,
                param,
                value,
                create_pool_fee,
            ))
        }
        
        // SetDefaultFeeRate instruction
        disc if disc == &SET_DEFAULT_FEE_RATE_DISCRIMINATOR => {
            // SetDefaultFeeRate accounts:
            // 0: whirlpools_config
            // 1: fee_tier
            // 2: fee_authority (signer)
            
            let config = get_account_at_index(instruction, account_keys, 0);
            let fee_tier = get_account_at_index(instruction, account_keys, 1);
            
            // Parse fee rate from data
            // Data structure: [8 bytes discriminator][2 bytes default_fee_rate]
            let default_fee_rate = if data.len() >= 10 {
                parse_u16_from_data(data, 8)
            } else {
                Some(0)
            };
            
            let protocol_fee_rate = Some(0); // Not defined in SetDefaultFeeRate
            let fund_fee_rate = Some(0); // Not defined in SetDefaultFeeRate
            let param = Some(0);
            let value = Some(0);
            let create_pool_fee = Some(0);
            
            Some((
                "SetDefaultFeeRate".to_string(),
                None, // No pool address
                config,
                default_fee_rate,
                protocol_fee_rate,
                fund_fee_rate,
                param,
                value,
                create_pool_fee,
            ))
        }
        
        // SetDefaultProtocolFeeRate instruction
        disc if disc == &SET_DEFAULT_PROTOCOL_FEE_RATE_DISCRIMINATOR => {
            // SetDefaultProtocolFeeRate accounts:
            // 0: whirlpools_config
            // 1: fee_authority (signer)
            
            let config = get_account_at_index(instruction, account_keys, 0);
            
            // Parse protocol fee rate from data
            // Data structure: [8 bytes discriminator][2 bytes default_protocol_fee_rate]
            let default_protocol_fee_rate = if data.len() >= 10 {
                parse_u16_from_data(data, 8)
            } else {
                Some(0)
            };
            
            let trade_fee_rate = Some(0); // Not defined in SetDefaultProtocolFeeRate
            let fund_fee_rate = Some(0); // Not defined in SetDefaultProtocolFeeRate
            let param = Some(0);
            let value = Some(0);
            let create_pool_fee = Some(0);
            
            Some((
                "SetDefaultProtocolFeeRate".to_string(),
                None, // No pool address
                config,
                trade_fee_rate,
                default_protocol_fee_rate,
                fund_fee_rate,
                param,
                value,
                create_pool_fee,
            ))
        }
        
        // SetFeeRate instruction
        disc if disc == &SET_FEE_RATE_DISCRIMINATOR => {
            // SetFeeRate accounts:
            // 0: whirlpools_config
            // 1: whirlpool (write)
            // 2: fee_authority (signer)
            
            let config = get_account_at_index(instruction, account_keys, 0);
            let pool_address = get_account_at_index(instruction, account_keys, 1);
            
            // Parse fee rate from data
            // Data structure: [8 bytes discriminator][2 bytes fee_rate]
            let fee_rate = if data.len() >= 10 {
                parse_u16_from_data(data, 8)
            } else {
                Some(0)
            };
            
            let protocol_fee_rate = Some(0); // Not modified in SetFeeRate
            let fund_fee_rate = Some(0); // Not defined in SetFeeRate
            let param = Some(0);
            let value = Some(0);
            let create_pool_fee = Some(0);
            
            Some((
                "SetFeeRate".to_string(),
                pool_address,
                config,
                fee_rate,
                protocol_fee_rate,
                fund_fee_rate,
                param,
                value,
                create_pool_fee,
            ))
        }
        
        // SetProtocolFeeRate instruction
        disc if disc == &SET_PROTOCOL_FEE_RATE_DISCRIMINATOR => {
            // SetProtocolFeeRate accounts:
            // 0: whirlpools_config
            // 1: whirlpool (write)
            // 2: fee_authority (signer)
            
            let config = get_account_at_index(instruction, account_keys, 0);
            let pool_address = get_account_at_index(instruction, account_keys, 1);
            
            // Parse protocol fee rate from data
            // Data structure: [8 bytes discriminator][2 bytes protocol_fee_rate]
            let protocol_fee_rate = if data.len() >= 10 {
                parse_u16_from_data(data, 8)
            } else {
                Some(0)
            };
            
            let trade_fee_rate = Some(0); // Not modified in SetProtocolFeeRate
            let fund_fee_rate = Some(0); // Not defined in SetProtocolFeeRate
            let param = Some(0);
            let value = Some(0);
            let create_pool_fee = Some(0);
            
            Some((
                "SetProtocolFeeRate".to_string(),
                pool_address,
                config,
                trade_fee_rate,
                protocol_fee_rate,
                fund_fee_rate,
                param,
                value,
                create_pool_fee,
            ))
        }
        
        _ => None,
    }
}

// Helper function to get an account at a specific index
fn get_account_at_index(
    instruction: &CompiledInstruction,
    account_keys: &[String],
    index: usize,
) -> Option<String> {
    if index >= instruction.accounts.len() {
        return None;
    }
    
    let account_index = instruction.accounts[index] as usize;
    if account_index >= account_keys.len() {
        return None;
    }
    
    Some(account_keys[account_index].clone())
}

// Helper function to parse a u16 from a byte slice
fn parse_u16_from_data(data: &[u8], offset: usize) -> Option<u64> {
    if data.len() >= offset + 2 {
        let bytes = &data[offset..offset + 2];
        Some(u16::from_le_bytes([bytes[0], bytes[1]]) as u64)
    } else {
        None
    }
}

// Helper function to parse a u64 from a byte slice
fn parse_u64_from_data(data: &[u8], offset: usize) -> Option<u64> {
    if data.len() >= offset + 8 {
        let bytes = &data[offset..offset + 8];
        Some(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7]
        ]))
    } else {
        None
    }
}

// Helper function to parse a u32 from a byte slice
fn parse_u32_from_data(data: &[u8], offset: usize) -> Option<u64> {
    if data.len() >= offset + 4 {
        let bytes = &data[offset..offset + 4];
        Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as u64)
    } else {
        None
    }
} 