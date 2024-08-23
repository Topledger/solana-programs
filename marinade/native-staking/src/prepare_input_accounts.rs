use crate::pb::sf::solana::block_meta::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> InputAccounts {
    let input_accounts = populate_input_accounts(account_indices, accounts);

    let mut result = InputAccounts::default();
    match instruction_type.as_str() {
        "InitRoot" => {
            result.root = get_account_with(&input_accounts, 0);
        }
        "SetOperator" => {
            result.root = get_account_with(&input_accounts, 0);
            result.admin = get_account_with(&input_accounts, 1);
        }
        "SetAdmin" => {
            result.root = get_account_with(&input_accounts, 0);
            result.admin = get_account_with(&input_accounts, 1);
        }
        "SetAlternateStaker" => {
            result.root = get_account_with(&input_accounts, 0);
            result.admin = get_account_with(&input_accounts, 1);
        }
        "Merge" => {
            result.root = get_account_with(&input_accounts, 0);
            result.operator = get_account_with(&input_accounts, 1);
            result.destination_stake = get_account_with(&input_accounts, 2);
            result.source_stake = get_account_with(&input_accounts, 3);
            result.staker = get_account_with(&input_accounts, 4);
            result.stake_history = get_account_with(&input_accounts, 5);
            result.clock = get_account_with(&input_accounts, 6);
            result.stake_program = get_account_with(&input_accounts, 7);
        }
        "Split" => {
            result.root = get_account_with(&input_accounts, 0);
            result.operator = get_account_with(&input_accounts, 1);
            result.staker = get_account_with(&input_accounts, 2);
            result.stake = get_account_with(&input_accounts, 3);
            result.split_stake = get_account_with(&input_accounts, 4);
            result.system_program = get_account_with(&input_accounts, 5);
            result.stake_program = get_account_with(&input_accounts, 6);
        }
        "Deactivate" => {
            result.root = get_account_with(&input_accounts, 0);
            result.operator = get_account_with(&input_accounts, 1);
            result.staker = get_account_with(&input_accounts, 2);
            result.stake = get_account_with(&input_accounts, 3);
            result.clock = get_account_with(&input_accounts, 4);
            result.stake_program = get_account_with(&input_accounts, 5);
        }
        "Delegate" => {
            result.root = get_account_with(&input_accounts, 0);
            result.operator = get_account_with(&input_accounts, 1);
            result.staker = get_account_with(&input_accounts, 2);
            result.stake = get_account_with(&input_accounts, 3);
            result.validator_vote = get_account_with(&input_accounts, 4);
            result.stake_history = get_account_with(&input_accounts, 5);
            result.stake_config = get_account_with(&input_accounts, 6);
            result.clock = get_account_with(&input_accounts, 7);
            result.stake_program = get_account_with(&input_accounts, 8);
        }
        "Redelegate" => {
            result.root = get_account_with(&input_accounts, 0);
            result.operator = get_account_with(&input_accounts, 1);
            result.staker = get_account_with(&input_accounts, 2);
            result.stake = get_account_with(&input_accounts, 3);
            result.redelegate_stake = get_account_with(&input_accounts, 4);
            result.validator_vote = get_account_with(&input_accounts, 5);
            result.stake_history = get_account_with(&input_accounts, 6);
            result.stake_config = get_account_with(&input_accounts, 7);
            result.system_program = get_account_with(&input_accounts, 8);
            result.stake_program = get_account_with(&input_accounts, 9);
        }
        "SwitchStaker" => {
            result.root = get_account_with(&input_accounts, 0);
            result.operator = get_account_with(&input_accounts, 1);
            result.staker = get_account_with(&input_accounts, 2);
            result.alternate_staker = get_account_with(&input_accounts, 3);
            result.stake = get_account_with(&input_accounts, 4);
            result.clock = get_account_with(&input_accounts, 5);
            result.stake_program = get_account_with(&input_accounts, 6);
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
