use crate::pb::sf::solana::block_meta::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> InputAccounts {
    let input_accounts = populate_input_accounts(account_indices, accounts);

    let mut result = InputAccounts::default();
    match instruction_type.as_str() {
        "InitUpdateAuthority" => {
            result.whitelist_authority = get_account_with(&input_accounts, 0);
            result.cosigner = get_account_with(&input_accounts, 1);
            result.owner = get_account_with(&input_accounts, 2);
            result.system_program = get_account_with(&input_accounts, 3);
        }
        "InitUpdateWhitelist" => {
            result.whitelist = get_account_with(&input_accounts, 0);
            result.whitelist = get_account_with(&input_accounts, 1);
            result.cosigner = get_account_with(&input_accounts, 2);
            result.system_program = get_account_with(&input_accounts, 3);
        }
        "InitUpdateMintProof" => {
            result.whitelist = get_account_with(&input_accounts, 0);
            result.mint = get_account_with(&input_accounts, 1);
            result.mint_proof = get_account_with(&input_accounts, 2);
            result.user = get_account_with(&input_accounts, 3);
            result.system_program = get_account_with(&input_accounts, 4);
        }
        "ReallocAuthority" => {
            result.whitelist_authority = get_account_with(&input_accounts, 0);
            result.cosigner = get_account_with(&input_accounts, 1);
            result.system_program = get_account_with(&input_accounts, 2);
        }
        "ReallocWhitelist" => {
            result.whitelist = get_account_with(&input_accounts, 0);
            result.whitelist_authority = get_account_with(&input_accounts, 1);
            result.cosigner = get_account_with(&input_accounts, 2);
            result.system_program = get_account_with(&input_accounts, 3);
        }
        "FreezeWhitelist" => {
            result.whitelist = get_account_with(&input_accounts, 0);
            result.whitelist_authority = get_account_with(&input_accounts, 1);
            result.cosigner = get_account_with(&input_accounts, 2);
            result.system_program = get_account_with(&input_accounts, 3);
        }
        "UnfreezeWhitelist" => {
            result.whitelist = get_account_with(&input_accounts, 0);
            result.whitelist_authority = get_account_with(&input_accounts, 1);
            result.owner = get_account_with(&input_accounts, 2);
            result.system_program = get_account_with(&input_accounts, 3);
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
