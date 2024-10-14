use crate::pb::sf::solana::block_meta::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> InputAccounts {
    let input_accounts = populate_input_accounts(account_indices, accounts);

    let mut result = InputAccounts::default();
    match instruction_type.as_str() {
        "Swap" => {
            result.authority = get_account_with(&input_accounts, 0);
            result.amm = get_account_with(&input_accounts, 1);
            result.user_transfer_authority = get_account_with(&input_accounts, 2);
            result.source_info = get_account_with(&input_accounts, 3);
            result.destination_info = get_account_with(&input_accounts, 4);
            result.swap_source = get_account_with(&input_accounts, 5);
            result.swap_destination = get_account_with(&input_accounts, 6);
            result.pool_mint = get_account_with(&input_accounts, 7);
            result.fee_account = get_account_with(&input_accounts, 8);
            result.token_program = get_account_with(&input_accounts, 9);
            result.oracle_main_account = get_account_with(&input_accounts, 10);
            result.oracle_sub_account = get_account_with(&input_accounts, 11);
            result.oracle_pc_account = get_account_with(&input_accounts, 12);
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
