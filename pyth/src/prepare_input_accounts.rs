use crate::pb::sf::solana::block_meta::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> InputAccounts {
    let input_accounts = populate_input_accounts(account_indices, accounts);

    let mut result = InputAccounts::default();
    match instruction_type.as_str() {
        "InitMapping" => {
            result.funding = get_account_with(&input_accounts, 0);
            result.mapping = get_account_with(&input_accounts, 1);
        }
        "AddMapping" => {
            result.funding = get_account_with(&input_accounts, 0);
            result.mapping = get_account_with(&input_accounts, 1);
            result.next_mapping = get_account_with(&input_accounts, 2);
        }
        "AddProduct" => {
            result.funding = get_account_with(&input_accounts, 0);
            result.mapping = get_account_with(&input_accounts, 1);
            result.product = get_account_with(&input_accounts, 2);
        }
        "AddPrice" => {
            result.funding = get_account_with(&input_accounts, 0);
            result.product = get_account_with(&input_accounts, 1);
            result.price = get_account_with(&input_accounts, 2);
        }
        "AddPublisher" => {
            result.signer = get_account_with(&input_accounts, 0);
            result.price = get_account_with(&input_accounts, 1);
        }
        "DeletePublisher" => {
            result.signer = get_account_with(&input_accounts, 0);
            result.price = get_account_with(&input_accounts, 1);
        }
        "UpdatePrice" => {
            result.publisher = get_account_with(&input_accounts, 0);
            result.price = get_account_with(&input_accounts, 1);
        }
        "AggregatePrice" => {
            result.funding = get_account_with(&input_accounts, 0);
            result.price = get_account_with(&input_accounts, 1);
        }
        "SetMinPublishers" => {
            result.funding = get_account_with(&input_accounts, 0);
            result.price = get_account_with(&input_accounts, 1);
        }
        "UpdatePriceNoFailOnError" => {
            result.publisher = get_account_with(&input_accounts, 0);
            result.price = get_account_with(&input_accounts, 1);
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
