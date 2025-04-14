use substreams_solana::pb::sf::solana::r#type::v1::CompiledInstruction;

// Initialize instruction has a discriminator of 0
const INITIALIZE_DISCRIMINATOR: u8 = 0;

pub fn parse_instruction(
    ix: &CompiledInstruction,
    account_keys: &[String],
    _instruction_index: u32,
    _is_inner_instruction: bool,
    _inner_instruction_index: Option<u32>,
) -> Option<(String, Option<String>, Option<String>, Option<u64>, Option<u64>, Option<u64>, Option<u64>, Option<u64>, Option<u64>)> {
    // Check if the instruction data is empty
    if ix.data.is_empty() {
        return None;
    }

    // Check if this is an Initialize instruction
    let discriminator = ix.data[0];
    if discriminator == INITIALIZE_DISCRIMINATOR {
        // For Orca v1, we only need to extract the trading fee rate
        // Trading fee is at position 2-5 (4 bytes, u32)
        
        let trade_fee_rate = if ix.data.len() >= 6 {
            let fee_numerator = u32::from_le_bytes([ix.data[2], ix.data[3], ix.data[4], ix.data[5]]);
            fee_numerator as u64
        } else {
            0
        };
        
        // Get pool address from account index 0
        let pool_address = get_account_at_index(ix, account_keys, 0);
        
        // Return type values for Orca v1
        return Some((
            "Initialize".to_string(),      // ins_type
            Some(pool_address),            // pool_addr
            None,                          // amm_cfg
            Some(trade_fee_rate),          // fee_rate
            Some(0),                       // protocol_rate
            Some(0),                       // fund_fee_rate
            Some(0),                       // param
            Some(0),                       // value
            Some(0),                       // create_pool_fee
        ));
    }
    
    None
}

// Helper function to get account at a specific index
fn get_account_at_index(
    ix: &CompiledInstruction,
    account_keys: &[String],
    index: usize,
) -> String {
    if index < ix.accounts.len() {
        let account_index = ix.accounts[index] as usize;
        if account_index < account_keys.len() {
            return account_keys[account_index].clone();
        }
    }
    String::new()
} 