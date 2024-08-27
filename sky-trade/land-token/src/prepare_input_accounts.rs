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
            result.fee_payer = get_account_with(&input_accounts, 0);
            result.data_account = get_account_with(&input_accounts, 1);
            result.merkle_tree = get_account_with(&input_accounts, 2);
            result.system_program = get_account_with(&input_accounts, 3);
        }
        "MintToken" => {
            result.fee_payer = get_account_with(&input_accounts, 0);
            result.data_account = get_account_with(&input_accounts, 1);
            result.merkle_tree = get_account_with(&input_accounts, 2);
            result.recipient = get_account_with(&input_accounts, 3);
            result.tree_config = get_account_with(&input_accounts, 4);
            result.bubblegum_program = get_account_with(&input_accounts, 5);
            result.log_wrapper = get_account_with(&input_accounts, 6);
            result.compression_program = get_account_with(&input_accounts, 7);
            result.system_program = get_account_with(&input_accounts, 8);
            result.collection_mint = get_account_with(&input_accounts, 9);
            result.collection_metadata = get_account_with(&input_accounts, 10);
            result.collection_edition = get_account_with(&input_accounts, 11);
            result.bubblegum_signer = get_account_with(&input_accounts, 12);
            result.token_metadata_program = get_account_with(&input_accounts, 13);
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
