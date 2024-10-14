use crate::pb::sf::solana::block_meta::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> InputAccounts {
    let input_accounts = populate_input_accounts(account_indices, accounts);

    let mut result = InputAccounts::default();
    match instruction_type.as_str() {
        "InitializeLazyTransactionsV0" => {
            result.payer = get_account_with(&input_accounts, 0);
            result.lazy_transactions = get_account_with(&input_accounts, 1);
            result.canopy = get_account_with(&input_accounts, 2);
            result.system_program = get_account_with(&input_accounts, 3);
        }
        "ExecuteTransactionV0" => {
            result.payer = get_account_with(&input_accounts, 0);
            result.lazy_transactions = get_account_with(&input_accounts, 1);
            result.canopy = get_account_with(&input_accounts, 2);
            result.lazy_signer = get_account_with(&input_accounts, 3);
            result.block = get_account_with(&input_accounts, 4);
            result.system_program = get_account_with(&input_accounts, 5);
        }
        "CloseMarkerV0" => {
            result.refund = get_account_with(&input_accounts, 0);
            result.lazy_transactions = get_account_with(&input_accounts, 1);
            result.authority = get_account_with(&input_accounts, 2);
            result.block = get_account_with(&input_accounts, 3);
        }
        "CloseCanopyV0" => {
            result.refund = get_account_with(&input_accounts, 0);
            result.authority = get_account_with(&input_accounts, 1);
            result.lazy_transactions = get_account_with(&input_accounts, 2);
            result.canopy = get_account_with(&input_accounts, 3);
        }
        "UpdateLazyTransactionsV0" => {
            result.authority = get_account_with(&input_accounts, 0);
            result.lazy_transactions = get_account_with(&input_accounts, 1);
            result.canopy = get_account_with(&input_accounts, 2);
        }
        "SetCanopyV0" => {
            result.authority = get_account_with(&input_accounts, 0);
            result.lazy_transactions = get_account_with(&input_accounts, 1);
            result.canopy = get_account_with(&input_accounts, 2);
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
