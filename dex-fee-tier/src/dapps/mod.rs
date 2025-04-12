pub mod dapp_CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK;
pub mod dapp_CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C;
pub mod dapp_whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc;
pub mod dapp_9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP;

pub fn is_dapp_instruction_type_of_interest(
    program_id: &str,
    instruction_type: &Option<String>,
) -> bool {
    match program_id {
        "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK" => {
            if let Some(ins_type) = instruction_type {
                matches!(
                    ins_type.as_str(),
                    "CreateAmmConfig" | "UpdateAmmConfig" | "CreatePool"
                )
            } else {
                false
            }
        }
        "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C" => {
            if let Some(ins_type) = instruction_type {
                matches!(
                    ins_type.as_str(),
                    "CreateAmmConfig" | "UpdateAmmConfig" | "Initialize"
                )
            } else {
                false
            }
        }
        "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc" => {
            if let Some(ins_type) = instruction_type {
                matches!(
                    ins_type.as_str(),
                    "InitializeConfig" | "InitializePool" | "InitializePoolV2" | 
                    "InitializeFeeTier" | "SetDefaultFeeRate" | "SetDefaultProtocolFeeRate" | 
                    "SetFeeRate" | "SetProtocolFeeRate"
                )
            } else {
                false
            }
        }
        "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP" => {
            if let Some(ins_type) = instruction_type {
                matches!(ins_type.as_str(), "Initialize")
            } else {
                false
            }
        }
        _ => false,
    }
}

pub fn get_dapp_name(program_id: &str) -> &'static str {
    match program_id {
        "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK" => "Raydium CLMM",
        "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C" => "Raydium CPMM",
        "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc" => "Orca Whirlpool",
        "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP" => "Orca v1",
        _ => "Unknown Dapp",
    }
} 