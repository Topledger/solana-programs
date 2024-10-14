use crate::pb::sf::solana::block_meta::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> InputAccounts {
    let input_accounts = populate_input_accounts(account_indices, accounts);

    let mut result = InputAccounts::default();
    match instruction_type.as_str() {
        "InitializeTreasuryManagementV0" => {
            result.payer = get_account_with(&input_accounts, 0);
            result.treasury_management = get_account_with(&input_accounts, 1);
            result.treasury_mint = get_account_with(&input_accounts, 2);
            result.supply_mint = get_account_with(&input_accounts, 3);
            result.mint_authority = get_account_with(&input_accounts, 4);
            result.circuit_breaker = get_account_with(&input_accounts, 5);
            result.treasury = get_account_with(&input_accounts, 6);
            result.system_program = get_account_with(&input_accounts, 7);
            result.circuit_breaker_program = get_account_with(&input_accounts, 8);
            result.associated_token_program = get_account_with(&input_accounts, 9);
            result.token_program = get_account_with(&input_accounts, 10);
        }
        "UpdateTreasuryManagementV0" => {
            result.treasury_management = get_account_with(&input_accounts, 0);
            result.authority = get_account_with(&input_accounts, 1);
        }
        "RedeemV0" => {
            result.treasury_management = get_account_with(&input_accounts, 0);
            result.treasury_mint = get_account_with(&input_accounts, 1);
            result.supply_mint = get_account_with(&input_accounts, 2);
            result.treasury = get_account_with(&input_accounts, 3);
            result.circuit_breaker = get_account_with(&input_accounts, 4);
            result.from = get_account_with(&input_accounts, 5);
            result.to = get_account_with(&input_accounts, 6);
            result.owner = get_account_with(&input_accounts, 7);
            result.circuit_breaker_program = get_account_with(&input_accounts, 8);
            result.token_program = get_account_with(&input_accounts, 9);
        }
        "CorrectTreasuriesV0" => {
            result.admin = get_account_with(&input_accounts, 0);
            result.treasury_management = get_account_with(&input_accounts, 1);
            result.treasury = get_account_with(&input_accounts, 2);
            result.dest_treasury = get_account_with(&input_accounts, 3);
            result.circuit_breaker = get_account_with(&input_accounts, 4);
            result.circuit_breaker_program = get_account_with(&input_accounts, 5);
            result.token_program = get_account_with(&input_accounts, 6);
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
