use substreams_solana::pb::sf::solana::r#type::v1::CompiledInstruction;

// Discriminators for CLMM instructions
const CREATE_AMM_CONFIG_DISCRIMINATOR: [u8; 8] = [137, 52, 237, 212, 215, 117, 108, 104];
const UPDATE_AMM_CONFIG_DISCRIMINATOR: [u8; 8] = [49, 60, 174, 136, 154, 28, 116, 200];
const CREATE_POOL_DISCRIMINATOR: [u8; 8] = [233, 146, 209, 142, 207, 104, 64, 188];

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
        disc if disc == CREATE_AMM_CONFIG_DISCRIMINATOR => {
            // CreateAmmConfig accounts based on raydium/clmm:
            // 0: owner (signer)
            // 1: amm_config (write)
            // 2: system_program
            
            // We want the amm_config account which is at index 1
            let amm_config = if instruction.accounts.len() > 1 {
                get_account_at_index(instruction, account_keys, 1)
            } else {
                None
            };
            
            // Parse fee rates from instruction data
            // According to the binary layout in raydium/clmm:
            // index = u16 at offset 8 (after discriminator)
            // tick_spacing = u16 at offset 10
            // trade_fee_rate = u32 at offset 12
            // protocol_fee_rate = u32 at offset 16
            // fund_fee_rate = u32 at offset 20
            
            // Parse index (u16) - offset 8 (after discriminator)
            let _index = if data.len() >= 10 {
                Some(u16::from_le_bytes([data[8], data[9]]) as u64)
            } else {
                None
            };
            
            // Parse tick_spacing (u16) - offset 10
            let _tick_spacing = if data.len() >= 12 {
                Some(u16::from_le_bytes([data[10], data[11]]) as u64)
            } else {
                None
            };
            
            // Parse trade_fee_rate (u32) - offset 12
            let trade_fee_rate = if data.len() >= 16 {
                Some(u32::from_le_bytes([data[12], data[13], data[14], data[15]]) as u64)
            } else {
                Some(0) // Default if parsing fails
            };
            
            // Parse protocol_fee_rate (u32) - offset 16
            let protocol_fee_rate = if data.len() >= 20 {
                Some(u32::from_le_bytes([data[16], data[17], data[18], data[19]]) as u64)
            } else {
                Some(0) // Default if parsing fails
            };
            
            // Parse fund_fee_rate (u32) - offset 20
            let fund_fee_rate = if data.len() >= 24 {
                Some(u32::from_le_bytes([data[20], data[21], data[22], data[23]]) as u64)
            } else {
                Some(0) // Default if parsing fails
            };
            
            // Extract param and value
            let param = Some(0);
            let value = Some(0);
            // CLMM doesn't have create_pool_fee, so set to default
            let create_pool_fee = Some(0);

            Some((
                "CreateAmmConfig".to_string(),
                None,
                amm_config,
                trade_fee_rate,
                protocol_fee_rate,
                fund_fee_rate,
                param,
                value,
                create_pool_fee,
            ))
        }
        disc if disc == UPDATE_AMM_CONFIG_DISCRIMINATOR => {
            // UpdateAmmConfig accounts based on raydium/clmm:
            // 0: owner (signer)
            // 1: amm_config (write)
            
            // We want the amm_config account which is at index 1
            let amm_config = if instruction.accounts.len() > 1 {
                get_account_at_index(instruction, account_keys, 1)
            } else {
                None
            };
            
            // For UpdateAmmConfig, we only care about param and value
            // According to binary layout in raydium/clmm:
            // - param: u8 at offset 8
            // - value: u32 at offset 9
            
            // Parse param (u8) - offset 8
            let param = if data.len() >= 9 {
                Some(data[8] as u64)
            } else {
                Some(0)
            };
            
            // Parse value (u32) - offset 9
            let value = if data.len() >= 13 {
                Some(u32::from_le_bytes([data[9], data[10], data[11], data[12]]) as u64)
            } else {
                Some(0)
            };
            
            // For UpdateAmmConfig, we don't extract fee rates directly
            // Instead we only care about param and value
            let trade_fee_rate = None;
            let protocol_fee_rate = None;
            let fund_fee_rate = None;
            let create_pool_fee = None;

            Some((
                "UpdateAmmConfig".to_string(),
                None,
                amm_config,
                trade_fee_rate,
                protocol_fee_rate,
                fund_fee_rate,
                param,
                value,
                create_pool_fee,
            ))
        }
        disc if disc == CREATE_POOL_DISCRIMINATOR => {
            // CreatePool accounts based on raydium/clmm:
            // 0: pool_creator (signer)
            // 1: amm_config
            // 2: pool_state (write)
            // 3: token_mint0
            // 4: token_mint1
            // etc.
            
            // Get pool address (pool_state at index 2) and amm_config (index 1)
            let pool_address = if instruction.accounts.len() > 2 {
                get_account_at_index(instruction, account_keys, 2)
            } else {
                None
            };
            
            let amm_config = if instruction.accounts.len() > 1 {
                get_account_at_index(instruction, account_keys, 1)
            } else {
                None
            };

            // Default values for CreatePool
            let trade_fee_rate = Some(0);
            let protocol_fee_rate = Some(0);
            let fund_fee_rate = Some(0);
            let param = Some(0);
            let value = Some(0);
            let create_pool_fee = Some(0);

            Some((
                "CreatePool".to_string(),
                pool_address,
                amm_config,
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

// Helper function to parse a u32 from a byte slice
fn parse_u32_from_data(data: &[u8], offset: usize) -> Option<u64> {
    if data.len() >= offset + 4 {
        let bytes = &data[offset..offset + 4];
        Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as u64)
    } else {
        None
    }
} 