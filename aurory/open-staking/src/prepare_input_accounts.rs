use crate::pb::sf::solana::block_meta::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> InputAccounts {
    let input_accounts = populate_input_accounts(account_indices, accounts);

    let mut result = InputAccounts::default();
    match instruction_type.as_str() {
        "Initialize" => {
            result.token_mint = get_account_with(&input_accounts, 0);
            result.token_vault = get_account_with(&input_accounts, 1);
            result.initializer = get_account_with(&input_accounts, 2);
            result.system_program = get_account_with(&input_accounts, 3);
            result.token_program = get_account_with(&input_accounts, 4);
            result.rent = get_account_with(&input_accounts, 5);
        }
        "ReclaimMintAuthority" => {
            result.token_mint = get_account_with(&input_accounts, 0);
            result.x_token_mint = get_account_with(&input_accounts, 1);
            result.token_vault = get_account_with(&input_accounts, 2);
            result.authority = get_account_with(&input_accounts, 3);
            result.token_program = get_account_with(&input_accounts, 4);
        }
        "Stake" => {
            result.token_mint = get_account_with(&input_accounts, 0);
            result.x_token_mint = get_account_with(&input_accounts, 1);
            result.token_from = get_account_with(&input_accounts, 2);
            result.token_from_authority = get_account_with(&input_accounts, 3);
            result.token_vault = get_account_with(&input_accounts, 4);
            result.x_token_to = get_account_with(&input_accounts, 5);
            result.token_program = get_account_with(&input_accounts, 6);
        }
        "Unstake" => {
            result.token_mint = get_account_with(&input_accounts, 0);
            result.x_token_mint = get_account_with(&input_accounts, 1);
            result.x_token_from = get_account_with(&input_accounts, 2);
            result.x_token_from_authority = get_account_with(&input_accounts, 3);
            result.token_vault = get_account_with(&input_accounts, 4);
            result.token_to = get_account_with(&input_accounts, 5);
            result.token_program = get_account_with(&input_accounts, 6);
        }
        "EmitPrice" => {
            result.token_mint = get_account_with(&input_accounts, 0);
            result.x_token_mint = get_account_with(&input_accounts, 1);
            result.token_vault = get_account_with(&input_accounts, 2);
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
