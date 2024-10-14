use crate::pb::sf::solana::block_meta::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> InputAccounts {
    let input_accounts = populate_input_accounts(account_indices, accounts);

    let mut result = InputAccounts::default();
    match instruction_type.as_str() {
        "Create" => {
            result.payer_account = get_account_with(&input_accounts, 0);
            result.name_owner = get_account_with(&input_accounts, 1);
            result.name_account = get_account_with(&input_accounts, 2);
            result.name_class_account = get_account_with(&input_accounts, 3);
            result.parent_name_account = get_account_with(&input_accounts, 4);
            result.parent_name_owner = get_account_with(&input_accounts, 5);
            result.system_program = get_account_with(&input_accounts, 6);
        }
        "Update" => {
            result.owner = get_account_with(&input_accounts, 0);
            result.name_account = get_account_with(&input_accounts, 1);
            result.name_class_account = get_account_with(&input_accounts, 2);
            result.parent_name_account = get_account_with(&input_accounts, 3);
        }
        "Transfer" => {
            result.owner = get_account_with(&input_accounts, 0);
            result.name_account = get_account_with(&input_accounts, 1);
            result.name_class_account = get_account_with(&input_accounts, 2);
            result.parent_name_account = get_account_with(&input_accounts, 3);
        }
        "Delete" => {
            result.owner = get_account_with(&input_accounts, 0);
            result.name_account = get_account_with(&input_accounts, 1);
            result.refund_target = get_account_with(&input_accounts, 2);
            result.name_class_account = get_account_with(&input_accounts, 3);
            result.parent_name_account = get_account_with(&input_accounts, 4);
        }
        "Resize" => {
            result.owner = get_account_with(&input_accounts, 0);
            result.name_account = get_account_with(&input_accounts, 1);
            result.name_class_account = get_account_with(&input_accounts, 2);
            result.parent_name_account = get_account_with(&input_accounts, 3);
            result.system_program = get_account_with(&input_accounts, 4);
        }
        "Extend" => {
            result.owner = get_account_with(&input_accounts, 0);
            result.parent_name_owner = get_account_with(&input_accounts, 1);
            result.name_account = get_account_with(&input_accounts, 2);
            result.name_class_account = get_account_with(&input_accounts, 3);
            result.parent_name_account = get_account_with(&input_accounts, 4);
        }
        "ImmutableOwner" => {
            result.owner = get_account_with(&input_accounts, 0);
            result.parent_name_owner = get_account_with(&input_accounts, 1);
            result.name_account = get_account_with(&input_accounts, 2);
            result.name_class_account = get_account_with(&input_accounts, 3);
            result.parent_name_account = get_account_with(&input_accounts, 4);
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
