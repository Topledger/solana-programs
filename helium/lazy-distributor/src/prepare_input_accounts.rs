use crate::pb::sf::solana::block_meta::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> InputAccounts {
    let input_accounts = populate_input_accounts(account_indices, accounts);

    let mut result = InputAccounts::default();
    match instruction_type.as_str() {
        "InitializeLazyDistributorV0" => {
            result.payer = get_account_with(&input_accounts, 0);
            result.lazy_distributor = get_account_with(&input_accounts, 1);
            result.rewards_mint = get_account_with(&input_accounts, 2);
            result.rewards_escrow = get_account_with(&input_accounts, 3);
            result.circuit_breaker = get_account_with(&input_accounts, 4);
            result.system_program = get_account_with(&input_accounts, 5);
            result.associated_token_program = get_account_with(&input_accounts, 6);
            result.circuit_breaker_program = get_account_with(&input_accounts, 7);
            result.token_program = get_account_with(&input_accounts, 8);
        }
        "InitializeRecipientV0" => {
            result.payer = get_account_with(&input_accounts, 0);
            result.lazy_distributor = get_account_with(&input_accounts, 1);
            result.recipient = get_account_with(&input_accounts, 2);
            result.mint = get_account_with(&input_accounts, 3);
            result.target_metadata = get_account_with(&input_accounts, 4);
            result.system_program = get_account_with(&input_accounts, 5);
        }
        "InitializeCompressionRecipientV0" => {
            result.payer = get_account_with(&input_accounts, 0);
            result.lazy_distributor = get_account_with(&input_accounts, 1);
            result.recipient = get_account_with(&input_accounts, 2);
            result.merkle_tree = get_account_with(&input_accounts, 3);
            result.owner = get_account_with(&input_accounts, 4);
            result.delegate = get_account_with(&input_accounts, 5);
            result.compression_program = get_account_with(&input_accounts, 6);
            result.system_program = get_account_with(&input_accounts, 7);
        }
        "SetCurrentRewardsV0" => {
            result.payer = get_account_with(&input_accounts, 0);
            result.lazy_distributor = get_account_with(&input_accounts, 1);
            result.recipient = get_account_with(&input_accounts, 2);
            result.oracle = get_account_with(&input_accounts, 3);
            result.system_program = get_account_with(&input_accounts, 4);
        }
        "DistributeRewardsV0" => {
            result.common_payer = get_account_with(&input_accounts, 0);
            result.common_lazy_distributor = get_account_with(&input_accounts, 1);
            result.common_recipient = get_account_with(&input_accounts, 2);
            result.common_rewards_mint = get_account_with(&input_accounts, 3);
            result.common_rewards_escrow = get_account_with(&input_accounts, 4);
            result.common_circuit_breaker = get_account_with(&input_accounts, 5);
            result.common_owner = get_account_with(&input_accounts, 6);
            result.common_destination_account = get_account_with(&input_accounts, 7);
            result.common_associated_token_program = get_account_with(&input_accounts, 8);
            result.common_circuit_breaker_program = get_account_with(&input_accounts, 9);
            result.common_system_program = get_account_with(&input_accounts, 10);
            result.common_token_program = get_account_with(&input_accounts, 11);
            result.recipient_mint_account = get_account_with(&input_accounts, 12);
        }
        "DistributeCompressionRewardsV0" => {
            result.common_payer = get_account_with(&input_accounts, 0);
            result.common_lazy_distributor = get_account_with(&input_accounts, 1);
            result.common_recipient = get_account_with(&input_accounts, 2);
            result.common_rewards_mint = get_account_with(&input_accounts, 3);
            result.common_rewards_escrow = get_account_with(&input_accounts, 4);
            result.common_circuit_breaker = get_account_with(&input_accounts, 5);
            result.common_owner = get_account_with(&input_accounts, 6);
            result.common_destination_account = get_account_with(&input_accounts, 7);
            result.common_associated_token_program = get_account_with(&input_accounts, 8);
            result.common_circuit_breaker_program = get_account_with(&input_accounts, 9);
            result.common_system_program = get_account_with(&input_accounts, 10);
            result.common_token_program = get_account_with(&input_accounts, 11);
            result.merkle_tree = get_account_with(&input_accounts, 12);
            result.compression_program = get_account_with(&input_accounts, 13);
            result.token_program = get_account_with(&input_accounts, 14);
        }
        "UpdateLazyDistributorV0" => {
            result.lazy_distributor = get_account_with(&input_accounts, 0);
            result.rewards_mint = get_account_with(&input_accounts, 1);
            result.authority = get_account_with(&input_accounts, 2);
        }
        _ => {}
    }

    return result;
}

fn get_account_with(accounts: &Vec<String>, index: usize) -> Option<String> {
    let mut result: Option<String> = None;
    let account = accounts.get(index);
    if account.is_some() {
        result = Some(account.unwrap().to_string());
    }
    return result;
}

fn populate_input_accounts(account_indices: &Vec<u8>, accounts: &Vec<String>) -> Vec<String> {
    let mut instruction_accounts: Vec<String> = vec![];
    for (index, &el) in account_indices.iter().enumerate() {
        instruction_accounts.push(accounts.as_slice()[el as usize].to_string());
    }
    return instruction_accounts;
}
