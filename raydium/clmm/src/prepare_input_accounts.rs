// use crate::pb::v1::{InputAccount, InputAccounts}; // Commented out unused InputAccount
// Remove unused imports
// use crate::pb::sf::solana::raydium_clmm::v1::{InputAccount, InputAccounts};
use substreams_solana::pb::sf::solana::r#type::v1::CompiledInstruction;
use std::collections::HashMap;

// Remove unused function get_account
/*
// Helper function to safely get account details
fn get_account(instruction: &CompiledInstruction, account_keys: &[String], index: usize) -> Option<InputAccount> {
    if let Some(&acc_index) = instruction.accounts.get(index) {
        if let Some(address) = account_keys.get(acc_index as usize) {
            // Check if this account is a signer
            let is_signer = acc_index == 0 || (instruction.accounts.get(0) == Some(&acc_index));
            
            // In this simplified model, we assume the account is writable
            // In a real implementation, you'd check the transaction message
            let is_writable = true;
            
            return Some(InputAccount {
                address: address.clone(),
                is_signer,
                is_writable,
            });
        }
    }
    None
}
*/

// Use this function to add a key-value pair to the accounts map
fn add_account_key(accounts_map: &mut HashMap<String, String>, key: &str, instruction: &CompiledInstruction, account_keys: &[String], index: usize) {
    if let Some(&acc_index) = instruction.accounts.get(index) {
        if let Some(address) = account_keys.get(acc_index as usize) {
            accounts_map.insert(key.to_string(), address.clone());
        }
    }
}

pub fn prepare_input_accounts(
    instruction: &CompiledInstruction,
    account_keys: &[String],
    instruction_type: &str,
    _instruction_index: u32, // Mark as unused
    _inner_instruction_index: u32, // Mark as unused
    _is_inner_instruction: bool, // Mark as unused
) -> HashMap<String, String> { // Change return type
    let mut accounts_map = HashMap::new();
    
    match instruction_type {
        "CreateAmmConfig" => {
            add_account_key(&mut accounts_map, "owner", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "amm_config", instruction, account_keys, 1);
            add_account_key(&mut accounts_map, "system_program", instruction, account_keys, 2);
        }
        "UpdateAmmConfig" => {
            add_account_key(&mut accounts_map, "owner", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "amm_config", instruction, account_keys, 1);
        }
        "CreatePool" => {
            add_account_key(&mut accounts_map, "pool_creator", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "amm_config", instruction, account_keys, 1);
            add_account_key(&mut accounts_map, "pool_state", instruction, account_keys, 2);
            add_account_key(&mut accounts_map, "token_mint0", instruction, account_keys, 3);
            add_account_key(&mut accounts_map, "token_mint1", instruction, account_keys, 4);
            add_account_key(&mut accounts_map, "token_vault0", instruction, account_keys, 5);
            add_account_key(&mut accounts_map, "token_vault1", instruction, account_keys, 6);
            add_account_key(&mut accounts_map, "observation_state", instruction, account_keys, 7);
            add_account_key(&mut accounts_map, "tick_array_bitmap", instruction, account_keys, 8);
            add_account_key(&mut accounts_map, "token_program0", instruction, account_keys, 9);
            add_account_key(&mut accounts_map, "token_program1", instruction, account_keys, 10);
            add_account_key(&mut accounts_map, "system_program", instruction, account_keys, 11);
            add_account_key(&mut accounts_map, "rent", instruction, account_keys, 12);
        }
        "UpdatePoolStatus" => {
            add_account_key(&mut accounts_map, "authority", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "pool_state", instruction, account_keys, 1);
        }
        "CreateOperationAccount" => {
            add_account_key(&mut accounts_map, "owner", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "operation_state", instruction, account_keys, 1);
            add_account_key(&mut accounts_map, "system_program", instruction, account_keys, 2);
        }
        "UpdateOperationAccount" => {
            add_account_key(&mut accounts_map, "owner", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "operation_state", instruction, account_keys, 1);
            add_account_key(&mut accounts_map, "system_program", instruction, account_keys, 2);
        }
        "TransferRewardOwner" => {
            add_account_key(&mut accounts_map, "authority", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "pool_state", instruction, account_keys, 1);
        }
        "InitializeReward" => {
            add_account_key(&mut accounts_map, "reward_funder", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "funder_token_account", instruction, account_keys, 1);
            add_account_key(&mut accounts_map, "amm_config", instruction, account_keys, 2);
            add_account_key(&mut accounts_map, "pool_state", instruction, account_keys, 3);
            add_account_key(&mut accounts_map, "operation_state", instruction, account_keys, 4);
            add_account_key(&mut accounts_map, "reward_token_mint", instruction, account_keys, 5);
            add_account_key(&mut accounts_map, "reward_token_vault", instruction, account_keys, 6);
            add_account_key(&mut accounts_map, "reward_token_program", instruction, account_keys, 7);
            add_account_key(&mut accounts_map, "system_program", instruction, account_keys, 8);
            add_account_key(&mut accounts_map, "rent", instruction, account_keys, 9);
        }
        "CollectRemainingRewards" => {
            add_account_key(&mut accounts_map, "reward_funder", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "funder_token_account", instruction, account_keys, 1);
            add_account_key(&mut accounts_map, "pool_state", instruction, account_keys, 2);
            add_account_key(&mut accounts_map, "reward_token_vault", instruction, account_keys, 3);
            add_account_key(&mut accounts_map, "reward_vault_mint", instruction, account_keys, 4);
            add_account_key(&mut accounts_map, "token_program", instruction, account_keys, 5);
            add_account_key(&mut accounts_map, "token_program2022", instruction, account_keys, 6);
            add_account_key(&mut accounts_map, "memo_program", instruction, account_keys, 7);
        }
        "UpdateRewardInfos" => {
            add_account_key(&mut accounts_map, "pool_state", instruction, account_keys, 0);
        }
        "SetRewardParams" => {
            add_account_key(&mut accounts_map, "authority", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "amm_config", instruction, account_keys, 1);
            add_account_key(&mut accounts_map, "pool_state", instruction, account_keys, 2);
            add_account_key(&mut accounts_map, "operation_state", instruction, account_keys, 3);
            add_account_key(&mut accounts_map, "token_program", instruction, account_keys, 4);
            add_account_key(&mut accounts_map, "token_program2022", instruction, account_keys, 5);
        }
        "CollectProtocolFee" => {
            add_account_key(&mut accounts_map, "owner", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "pool_state", instruction, account_keys, 1);
            add_account_key(&mut accounts_map, "amm_config", instruction, account_keys, 2);
            add_account_key(&mut accounts_map, "token_vault0", instruction, account_keys, 3);
            add_account_key(&mut accounts_map, "token_vault1", instruction, account_keys, 4);
            add_account_key(&mut accounts_map, "vault0_mint", instruction, account_keys, 5);
            add_account_key(&mut accounts_map, "vault1_mint", instruction, account_keys, 6);
            add_account_key(&mut accounts_map, "recipient_token_account0", instruction, account_keys, 7);
            add_account_key(&mut accounts_map, "recipient_token_account1", instruction, account_keys, 8);
            add_account_key(&mut accounts_map, "token_program", instruction, account_keys, 9);
            add_account_key(&mut accounts_map, "token_program2022", instruction, account_keys, 10);
        }
        "CollectFundFee" => {
            add_account_key(&mut accounts_map, "owner", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "pool_state", instruction, account_keys, 1);
            add_account_key(&mut accounts_map, "amm_config", instruction, account_keys, 2);
            add_account_key(&mut accounts_map, "token_vault0", instruction, account_keys, 3);
            add_account_key(&mut accounts_map, "token_vault1", instruction, account_keys, 4);
            add_account_key(&mut accounts_map, "vault0_mint", instruction, account_keys, 5);
            add_account_key(&mut accounts_map, "vault1_mint", instruction, account_keys, 6);
            add_account_key(&mut accounts_map, "recipient_token_account0", instruction, account_keys, 7);
            add_account_key(&mut accounts_map, "recipient_token_account1", instruction, account_keys, 8);
            add_account_key(&mut accounts_map, "token_program", instruction, account_keys, 9);
            add_account_key(&mut accounts_map, "token_program2022", instruction, account_keys, 10);
        }
        "OpenPosition" => {
            add_account_key(&mut accounts_map, "payer", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "position_nft_owner", instruction, account_keys, 1);
            add_account_key(&mut accounts_map, "position_nft_mint", instruction, account_keys, 2);
            add_account_key(&mut accounts_map, "position_nft_account", instruction, account_keys, 3);
            add_account_key(&mut accounts_map, "metadata_account", instruction, account_keys, 4);
            add_account_key(&mut accounts_map, "pool_state", instruction, account_keys, 5);
            add_account_key(&mut accounts_map, "protocol_position", instruction, account_keys, 6);
            add_account_key(&mut accounts_map, "tick_array_lower", instruction, account_keys, 7);
            add_account_key(&mut accounts_map, "tick_array_upper", instruction, account_keys, 8);
            add_account_key(&mut accounts_map, "personal_position", instruction, account_keys, 9);
            add_account_key(&mut accounts_map, "token_account0", instruction, account_keys, 10);
            add_account_key(&mut accounts_map, "token_account1", instruction, account_keys, 11);
            add_account_key(&mut accounts_map, "token_vault0", instruction, account_keys, 12);
            add_account_key(&mut accounts_map, "token_vault1", instruction, account_keys, 13);
            add_account_key(&mut accounts_map, "rent", instruction, account_keys, 14);
            add_account_key(&mut accounts_map, "system_program", instruction, account_keys, 15);
            add_account_key(&mut accounts_map, "token_program", instruction, account_keys, 16);
            add_account_key(&mut accounts_map, "associated_token_program", instruction, account_keys, 17);
            add_account_key(&mut accounts_map, "metadata_program", instruction, account_keys, 18);
        }
        "OpenPositionV2" => {
            add_account_key(&mut accounts_map, "payer", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "position_nft_owner", instruction, account_keys, 1);
            add_account_key(&mut accounts_map, "position_nft_mint", instruction, account_keys, 2);
            add_account_key(&mut accounts_map, "position_nft_account", instruction, account_keys, 3);
            add_account_key(&mut accounts_map, "metadata_account", instruction, account_keys, 4);
            add_account_key(&mut accounts_map, "pool_state", instruction, account_keys, 5);
            add_account_key(&mut accounts_map, "protocol_position", instruction, account_keys, 6);
            add_account_key(&mut accounts_map, "tick_array_lower", instruction, account_keys, 7);
            add_account_key(&mut accounts_map, "tick_array_upper", instruction, account_keys, 8);
            add_account_key(&mut accounts_map, "personal_position", instruction, account_keys, 9);
            add_account_key(&mut accounts_map, "token_account0", instruction, account_keys, 10);
            add_account_key(&mut accounts_map, "token_account1", instruction, account_keys, 11);
            add_account_key(&mut accounts_map, "token_vault0", instruction, account_keys, 12);
            add_account_key(&mut accounts_map, "token_vault1", instruction, account_keys, 13);
            add_account_key(&mut accounts_map, "rent", instruction, account_keys, 14);
            add_account_key(&mut accounts_map, "system_program", instruction, account_keys, 15);
            add_account_key(&mut accounts_map, "token_program", instruction, account_keys, 16);
            add_account_key(&mut accounts_map, "associated_token_program", instruction, account_keys, 17);
            add_account_key(&mut accounts_map, "metadata_program", instruction, account_keys, 18);
            add_account_key(&mut accounts_map, "token_program2022", instruction, account_keys, 19);
            add_account_key(&mut accounts_map, "vault0_mint", instruction, account_keys, 20);
            add_account_key(&mut accounts_map, "vault1_mint", instruction, account_keys, 21);
        }
        "OpenPositionWithToken22Nft" => {
            add_account_key(&mut accounts_map, "payer", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "position_nft_owner", instruction, account_keys, 1);
            add_account_key(&mut accounts_map, "position_nft_mint", instruction, account_keys, 2);
            add_account_key(&mut accounts_map, "position_nft_account", instruction, account_keys, 3);
            add_account_key(&mut accounts_map, "pool_state", instruction, account_keys, 4);
            add_account_key(&mut accounts_map, "protocol_position", instruction, account_keys, 5);
            add_account_key(&mut accounts_map, "tick_array_lower", instruction, account_keys, 6);
            add_account_key(&mut accounts_map, "tick_array_upper", instruction, account_keys, 7);
            add_account_key(&mut accounts_map, "personal_position", instruction, account_keys, 8);
            add_account_key(&mut accounts_map, "token_account0", instruction, account_keys, 9);
            add_account_key(&mut accounts_map, "token_account1", instruction, account_keys, 10);
            add_account_key(&mut accounts_map, "token_vault0", instruction, account_keys, 11);
            add_account_key(&mut accounts_map, "token_vault1", instruction, account_keys, 12);
            add_account_key(&mut accounts_map, "rent", instruction, account_keys, 13);
            add_account_key(&mut accounts_map, "system_program", instruction, account_keys, 14);
            add_account_key(&mut accounts_map, "token_program", instruction, account_keys, 15);
            add_account_key(&mut accounts_map, "associated_token_program", instruction, account_keys, 16);
            add_account_key(&mut accounts_map, "token_program2022", instruction, account_keys, 17);
            add_account_key(&mut accounts_map, "vault0_mint", instruction, account_keys, 18);
            add_account_key(&mut accounts_map, "vault1_mint", instruction, account_keys, 19);
        }
        "ClosePosition" => {
            add_account_key(&mut accounts_map, "nft_owner", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "position_nft_mint", instruction, account_keys, 1);
            add_account_key(&mut accounts_map, "position_nft_account", instruction, account_keys, 2);
            add_account_key(&mut accounts_map, "personal_position", instruction, account_keys, 3);
            add_account_key(&mut accounts_map, "system_program", instruction, account_keys, 4);
            add_account_key(&mut accounts_map, "token_program", instruction, account_keys, 5);
        }
        "IncreaseLiquidity" => {
            add_account_key(&mut accounts_map, "nft_owner", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "nft_account", instruction, account_keys, 1);
            add_account_key(&mut accounts_map, "pool_state", instruction, account_keys, 2);
            add_account_key(&mut accounts_map, "protocol_position", instruction, account_keys, 3);
            add_account_key(&mut accounts_map, "personal_position", instruction, account_keys, 4);
            add_account_key(&mut accounts_map, "tick_array_lower", instruction, account_keys, 5);
            add_account_key(&mut accounts_map, "tick_array_upper", instruction, account_keys, 6);
            add_account_key(&mut accounts_map, "token_account0", instruction, account_keys, 7);
            add_account_key(&mut accounts_map, "token_account1", instruction, account_keys, 8);
            add_account_key(&mut accounts_map, "token_vault0", instruction, account_keys, 9);
            add_account_key(&mut accounts_map, "token_vault1", instruction, account_keys, 10);
            add_account_key(&mut accounts_map, "token_program", instruction, account_keys, 11);
        }
        "IncreaseLiquidityV2" => {
            add_account_key(&mut accounts_map, "nft_owner", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "nft_account", instruction, account_keys, 1);
            add_account_key(&mut accounts_map, "pool_state", instruction, account_keys, 2);
            add_account_key(&mut accounts_map, "protocol_position", instruction, account_keys, 3);
            add_account_key(&mut accounts_map, "personal_position", instruction, account_keys, 4);
            add_account_key(&mut accounts_map, "tick_array_lower", instruction, account_keys, 5);
            add_account_key(&mut accounts_map, "tick_array_upper", instruction, account_keys, 6);
            add_account_key(&mut accounts_map, "token_account0", instruction, account_keys, 7);
            add_account_key(&mut accounts_map, "token_account1", instruction, account_keys, 8);
            add_account_key(&mut accounts_map, "token_vault0", instruction, account_keys, 9);
            add_account_key(&mut accounts_map, "token_vault1", instruction, account_keys, 10);
            add_account_key(&mut accounts_map, "token_program", instruction, account_keys, 11);
            add_account_key(&mut accounts_map, "token_program2022", instruction, account_keys, 12);
            add_account_key(&mut accounts_map, "vault0_mint", instruction, account_keys, 13);
            add_account_key(&mut accounts_map, "vault1_mint", instruction, account_keys, 14);
        }
        "DecreaseLiquidity" => {
            add_account_key(&mut accounts_map, "nft_owner", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "nft_account", instruction, account_keys, 1);
            add_account_key(&mut accounts_map, "personal_position", instruction, account_keys, 2);
            add_account_key(&mut accounts_map, "pool_state", instruction, account_keys, 3);
            add_account_key(&mut accounts_map, "protocol_position", instruction, account_keys, 4);
            add_account_key(&mut accounts_map, "token_vault0", instruction, account_keys, 5);
            add_account_key(&mut accounts_map, "token_vault1", instruction, account_keys, 6);
            add_account_key(&mut accounts_map, "tick_array_lower", instruction, account_keys, 7);
            add_account_key(&mut accounts_map, "tick_array_upper", instruction, account_keys, 8);
            add_account_key(&mut accounts_map, "recipient_token_account0", instruction, account_keys, 9);
            add_account_key(&mut accounts_map, "recipient_token_account1", instruction, account_keys, 10);
            add_account_key(&mut accounts_map, "token_program", instruction, account_keys, 11);
        }
        "DecreaseLiquidityV2" => {
            add_account_key(&mut accounts_map, "nft_owner", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "nft_account", instruction, account_keys, 1);
            add_account_key(&mut accounts_map, "personal_position", instruction, account_keys, 2);
            add_account_key(&mut accounts_map, "pool_state", instruction, account_keys, 3);
            add_account_key(&mut accounts_map, "protocol_position", instruction, account_keys, 4);
            add_account_key(&mut accounts_map, "token_vault0", instruction, account_keys, 5);
            add_account_key(&mut accounts_map, "token_vault1", instruction, account_keys, 6);
            add_account_key(&mut accounts_map, "tick_array_lower", instruction, account_keys, 7);
            add_account_key(&mut accounts_map, "tick_array_upper", instruction, account_keys, 8);
            add_account_key(&mut accounts_map, "recipient_token_account0", instruction, account_keys, 9);
            add_account_key(&mut accounts_map, "recipient_token_account1", instruction, account_keys, 10);
            add_account_key(&mut accounts_map, "token_program", instruction, account_keys, 11);
            add_account_key(&mut accounts_map, "token_program2022", instruction, account_keys, 12);
            add_account_key(&mut accounts_map, "memo_program", instruction, account_keys, 13);
            add_account_key(&mut accounts_map, "vault0_mint", instruction, account_keys, 14);
            add_account_key(&mut accounts_map, "vault1_mint", instruction, account_keys, 15);
        }
        "Swap" => {
            add_account_key(&mut accounts_map, "payer", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "amm_config", instruction, account_keys, 1);
            add_account_key(&mut accounts_map, "pool_state", instruction, account_keys, 2);
            add_account_key(&mut accounts_map, "input_token_account", instruction, account_keys, 3);
            add_account_key(&mut accounts_map, "output_token_account", instruction, account_keys, 4);
            add_account_key(&mut accounts_map, "input_vault", instruction, account_keys, 5);
            add_account_key(&mut accounts_map, "output_vault", instruction, account_keys, 6);
            add_account_key(&mut accounts_map, "observation_state", instruction, account_keys, 7);
            add_account_key(&mut accounts_map, "token_program", instruction, account_keys, 8);
            add_account_key(&mut accounts_map, "tick_array", instruction, account_keys, 9);
        }
        "SwapV2" => {
            add_account_key(&mut accounts_map, "payer", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "amm_config", instruction, account_keys, 1);
            add_account_key(&mut accounts_map, "pool_state", instruction, account_keys, 2);
            add_account_key(&mut accounts_map, "input_token_account", instruction, account_keys, 3);
            add_account_key(&mut accounts_map, "output_token_account", instruction, account_keys, 4);
            add_account_key(&mut accounts_map, "input_vault", instruction, account_keys, 5);
            add_account_key(&mut accounts_map, "output_vault", instruction, account_keys, 6);
            add_account_key(&mut accounts_map, "observation_state", instruction, account_keys, 7);
            add_account_key(&mut accounts_map, "token_program", instruction, account_keys, 8);
            add_account_key(&mut accounts_map, "token_program2022", instruction, account_keys, 9);
            add_account_key(&mut accounts_map, "memo_program", instruction, account_keys, 10);
            add_account_key(&mut accounts_map, "input_vault_mint", instruction, account_keys, 11);
            add_account_key(&mut accounts_map, "output_vault_mint", instruction, account_keys, 12);
        }
        "SwapRouterBaseIn" => {
            add_account_key(&mut accounts_map, "payer", instruction, account_keys, 0);
            add_account_key(&mut accounts_map, "input_token_account", instruction, account_keys, 1);
            add_account_key(&mut accounts_map, "input_token_mint", instruction, account_keys, 2);
            add_account_key(&mut accounts_map, "token_program", instruction, account_keys, 3);
            add_account_key(&mut accounts_map, "token_program2022", instruction, account_keys, 4);
            add_account_key(&mut accounts_map, "memo_program", instruction, account_keys, 5);
        }
        _ => {}
    }
    
    accounts_map // Return the map directly
} 