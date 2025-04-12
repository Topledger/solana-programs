use substreams_solana::pb::sf::solana::r#type::v1::CompiledInstruction;

// Known discriminator for Raydium CPMM Initialize instruction
const INITIALIZE_DISCRIMINATOR: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
// Discriminator for CreateAmmConfig instruction
const CREATE_AMM_CONFIG_DISCRIMINATOR: [u8; 8] = [137, 52, 237, 212, 215, 117, 108, 104];
// Discriminator for UpdateAmmConfig instruction
const UPDATE_AMM_CONFIG_DISCRIMINATOR: [u8; 8] = [49, 60, 174, 136, 154, 28, 116, 200];

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
    
    // We use a combination of discriminator matching and instruction name extraction
    let instruction_name = extract_cpmm_instruction_name(instruction, data);
    let is_create_amm_config = instruction_name.as_ref().map_or(false, |name| name == "CreateAmmConfig");
    let is_update_amm_config = instruction_name.as_ref().map_or(false, |name| name == "UpdateAmmConfig");
    
    match discriminator {
        // Initialize instruction - identified by its discriminator
        disc if disc == &INITIALIZE_DISCRIMINATOR => {
            // Initialize accounts based on raydium/cpmm:
            // 0: creator (signer)
            // 1: amm_config
            // 2: authority
            // 3: pool_state (write)
            // 4: token0_mint
            // ... (other accounts)
            
            // Get pool address (pool_state at index 3) and amm_config (index 1)
            let pool_address = get_account_at_index(instruction, account_keys, 3);
            let amm_config = get_account_at_index(instruction, account_keys, 1);
            
            // Parse data from instruction if available
            // For Initialize, we should extract fee rates from the AMM config
            // Since this is just the Initialize call, use the default fee rates
            // that would be present in the AMM config
            let trade_fee_rate = Some(0);
            let protocol_fee_rate = Some(0);
            let fund_fee_rate = Some(0);
            
            let param = Some(0);
            let value = Some(0);
            let create_pool_fee = Some(0);
            
            Some((
                "Initialize".to_string(),
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
        // CreateAmmConfig instruction - identified by discriminator or parsed name
        disc if disc == &CREATE_AMM_CONFIG_DISCRIMINATOR || is_create_amm_config => {
            // CreateAmmConfig accounts based on raydium/cpmm:
            // 0: owner (signer)
            // 1: amm_config (write)
            // 2: system_program
            
            // Get account at index 1 which should be the AMM config
            let amm_config = get_account_at_index(instruction, account_keys, 1);
            
            // Parse fee rates from instruction data
            // According to the binary layout in raydium/cpmm:
            // - index: u16 at offset 8 
            // - trade_fee_rate: u64 at offset 10
            // - protocol_fee_rate: u64 at offset 18
            // - fund_fee_rate: u64 at offset 26
            // - create_pool_fee: u64 at offset 34
            
            // Skip index at offset 8 (u16)
            let _index = if data.len() >= 10 {
                Some(u16::from_le_bytes([data[8], data[9]]) as u64)
            } else {
                None
            };
            
            // Parse trade_fee_rate (u64) - offset 10
            let trade_fee_rate = parse_u64_from_data(data, 10);
            
            // Parse protocol_fee_rate (u64) - offset 18
            let protocol_fee_rate = parse_u64_from_data(data, 18);
            
            // Parse fund_fee_rate (u64) - offset 26
            let fund_fee_rate = parse_u64_from_data(data, 26);
            
            // Parse create_pool_fee (u64) - offset 34
            let create_pool_fee = parse_u64_from_data(data, 34);
            
            // Default values if parsing fails
            let trade_fee_rate = trade_fee_rate.or(Some(25));
            let protocol_fee_rate = protocol_fee_rate.or(Some(5));
            let fund_fee_rate = fund_fee_rate.or(Some(0));
            let create_pool_fee = create_pool_fee.or(Some(0));
            
            // For param and value, use default values for CreateAmmConfig
            let param = Some(0);
            let value = Some(0);
            
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
        // UpdateAmmConfig instruction - identified by discriminator or parsed name
        disc if disc == &UPDATE_AMM_CONFIG_DISCRIMINATOR || is_update_amm_config => {
            // UpdateAmmConfig accounts based on raydium/cpmm:
            // 0: owner (signer)
            // 1: amm_config (write)
            
            // Get account at index 1 which should be the AMM config
            let amm_config = get_account_at_index(instruction, account_keys, 1);
            
            // Extract parameters from instruction data if available
            // According to binary layout in raydium/cpmm:
            // - param: u8 at offset 8
            // - value: u64 at offset 9
            
            // Parse param (u8) - offset 8
            let param = if data.len() >= 9 {
                Some(data[8] as u64)
            } else {
                Some(0)
            };
            
            // Parse value (u64) - offset 9
            let value = parse_u64_from_data(data, 9);
            
            // Set default values if parsing fails
            let param = param.or(Some(0));
            let value = value.or(Some(0));
            
            // For UpdateAmmConfig, we don't have direct fee rates in the instruction
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
        // For other instructions, match by name
        _ => {
            None
        }
    }
}

// Helper function to extract CPMM instruction name (simplified implementation)
fn extract_cpmm_instruction_name(
    _instruction: &CompiledInstruction,
    data: &[u8],
) -> Option<String> {
    // This is a simplified implementation for illustration purposes
    // In a real implementation, you would need to more robustly identify instruction types
    
    if data.len() < 9 {
        return None;
    }
    
    // Simplified approach - in a real implementation, this would be more sophisticated
    // Would likely involve parsing log messages or other context in the transaction
    match data[8] {
        1 => Some("CreateAmmConfig".to_string()),
        2 => Some("UpdateAmmConfig".to_string()),
        _ => None
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