use crate::pb::sf::solana::block_meta::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> InputAccounts {
    let input_accounts = populate_input_accounts(account_indices, accounts);

    let mut result = InputAccounts::default();

    match instruction_type.as_str() {
        "Init" => {
            result.mint = get_account_with(&input_accounts, 0);
            result.reflection = get_account_with(&input_accounts, 1);
            result.vault = get_account_with(&input_accounts, 2);
            result.authority = get_account_with(&input_accounts, 3);
            result.system_program = get_account_with(&input_accounts, 4);
            result.token_program = get_account_with(&input_accounts, 5);
            result.rent = get_account_with(&input_accounts, 6);
        }
        "Enter" => {
            result.reflection = get_account_with(&input_accounts, 0);
            result.stake = get_account_with(&input_accounts, 1);
            result.reward = get_account_with(&input_accounts, 2);
            result.authority = get_account_with(&input_accounts, 3);
            result.system_program = get_account_with(&input_accounts, 4);
        }
        "AddFee" => {
            result.user = get_account_with(&input_accounts, 0);
            result.reflection = get_account_with(&input_accounts, 1);
            result.vault = get_account_with(&input_accounts, 2);
            result.authority = get_account_with(&input_accounts, 3);
            result.token_program = get_account_with(&input_accounts, 4);
        }
        "Claim" => {
            result.user = get_account_with(&input_accounts, 0);
            result.vault = get_account_with(&input_accounts, 1);
            result.reflection = get_account_with(&input_accounts, 2);
            result.reward = get_account_with(&input_accounts, 3);
            result.stake = get_account_with(&input_accounts, 4);
            result.authority = get_account_with(&input_accounts, 5);
            result.token_program = get_account_with(&input_accounts, 6);
        }
        "Sync" => {
            result.reward = get_account_with(&input_accounts, 0);
            result.stake = get_account_with(&input_accounts, 1);
            result.reflection = get_account_with(&input_accounts, 2);
        }
        "Close" => {
            result.reflection = get_account_with(&input_accounts, 0);
            result.reward = get_account_with(&input_accounts, 1);
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
