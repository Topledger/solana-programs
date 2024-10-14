use crate::pb::sf::solana::block_meta::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> InputAccounts {
    let input_accounts = populate_input_accounts(account_indices, accounts);

    let mut result = InputAccounts::default();
    match instruction_type.as_str() {
        "CreateProgramManager" => {
            result.multisig = get_account_with(&input_accounts, 0);
            result.program_manager = get_account_with(&input_accounts, 1);
            result.creator = get_account_with(&input_accounts, 2);
            result.system_program = get_account_with(&input_accounts, 3);
        }
        "CreateManagedProgram" => {
            result.multisig = get_account_with(&input_accounts, 0);
            result.program_manager = get_account_with(&input_accounts, 1);
            result.managed_program = get_account_with(&input_accounts, 2);
            result.creator = get_account_with(&input_accounts, 3);
            result.system_program = get_account_with(&input_accounts, 4);
        }
        "CreateProgramUpgrade" => {
            result.multisig = get_account_with(&input_accounts, 0);
            result.program_manager = get_account_with(&input_accounts, 1);
            result.managed_program = get_account_with(&input_accounts, 2);
            result.program_upgrade = get_account_with(&input_accounts, 3);
            result.creator = get_account_with(&input_accounts, 4);
            result.system_program = get_account_with(&input_accounts, 5);
        }
        "CloseManagedProgramAccount" => {
            result.multisig = get_account_with(&input_accounts, 0);
            result.program_manager = get_account_with(&input_accounts, 1);
            result.managed_program = get_account_with(&input_accounts, 2);
            result.transaction = get_account_with(&input_accounts, 3);
            result.authority = get_account_with(&input_accounts, 4);
        }
        "CloseUpgradeAccount" => {
            result.multisig = get_account_with(&input_accounts, 0);
            result.program_manager = get_account_with(&input_accounts, 1);
            result.managed_program = get_account_with(&input_accounts, 2);
            result.program_upgrade = get_account_with(&input_accounts, 3);
            result.transaction = get_account_with(&input_accounts, 4);
            result.authority = get_account_with(&input_accounts, 5);
        }
        "SetAsExecuted" => {
            result.multisig = get_account_with(&input_accounts, 0);
            result.program_manager = get_account_with(&input_accounts, 1);
            result.managed_program = get_account_with(&input_accounts, 2);
            result.program_upgrade = get_account_with(&input_accounts, 3);
            result.transaction = get_account_with(&input_accounts, 4);
            result.instruction = get_account_with(&input_accounts, 5);
            result.authority = get_account_with(&input_accounts, 6);
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
