use crate::pb::sf::solana::block_meta::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> InputAccounts {
    let input_accounts = populate_input_accounts(account_indices, accounts);

    let mut result = InputAccounts::default();
    match instruction_type.as_str() {
        "StakeWrappedSol" => {
            result.user = get_account_with(&input_accounts, 0);
            result.wsol_from = get_account_with(&input_accounts, 1);
            result.dest_token_to = get_account_with(&input_accounts, 2);
            result.wsol_bridge_in = get_account_with(&input_accounts, 3);
            result.sol_bridge_out = get_account_with(&input_accounts, 4);
            result.dest_token_fee_token_account = get_account_with(&input_accounts, 5);
            result.dest_token_mint = get_account_with(&input_accounts, 6);
            result.wsol_mint = get_account_with(&input_accounts, 7);
            result.token_program = get_account_with(&input_accounts, 8);
            result.system_program = get_account_with(&input_accounts, 9);
        }
        "SwapViaStake" => {
            result.user = get_account_with(&input_accounts, 0);
            result.src_token_from = get_account_with(&input_accounts, 1);
            result.dest_token_to = get_account_with(&input_accounts, 2);
            result.bridge_stake = get_account_with(&input_accounts, 3);
            result.dest_token_fee_token_account = get_account_with(&input_accounts, 4);
            result.src_token_mint = get_account_with(&input_accounts, 5);
            result.dest_token_mint = get_account_with(&input_accounts, 6);
        }
        "CreateFeeTokenAccount" => {
            result.payer = get_account_with(&input_accounts, 0);
            result.fee_token_account = get_account_with(&input_accounts, 1);
            result.mint = get_account_with(&input_accounts, 2);
            result.token_program = get_account_with(&input_accounts, 3);
            result.system_program = get_account_with(&input_accounts, 4);
        }
        "CloseFeeTokenAccount" => {
            result.admin = get_account_with(&input_accounts, 0);
            result.fee_token_account = get_account_with(&input_accounts, 1);
            result.close_to = get_account_with(&input_accounts, 2);
            result.mint = get_account_with(&input_accounts, 3);
            result.token_program = get_account_with(&input_accounts, 4);
        }
        "WithdrawFees" => {
            result.admin = get_account_with(&input_accounts, 0);
            result.fee_token_account = get_account_with(&input_accounts, 1);
            result.withdraw_to = get_account_with(&input_accounts, 2);
            result.mint = get_account_with(&input_accounts, 3);
            result.token_program = get_account_with(&input_accounts, 4);
        }
        "DepositStake" => {
            result.user = get_account_with(&input_accounts, 0);
            result.stake_account = get_account_with(&input_accounts, 1);
            result.dest_token_to = get_account_with(&input_accounts, 2);
            result.dest_token_fee_token_account = get_account_with(&input_accounts, 3);
            result.dest_token_mint = get_account_with(&input_accounts, 4);
        }
        "PrefundWithdrawStake" => {
            result.user = get_account_with(&input_accounts, 0);
            result.src_token_from = get_account_with(&input_accounts, 1);
            result.bridge_stake = get_account_with(&input_accounts, 2);
            result.src_token_mint = get_account_with(&input_accounts, 3);
            result.prefunder = get_account_with(&input_accounts, 4);
            result.slumdog_stake = get_account_with(&input_accounts, 5);
            result.unstakeit_program = get_account_with(&input_accounts, 6);
            result.unstake_pool = get_account_with(&input_accounts, 7);
            result.pool_sol_reserves = get_account_with(&input_accounts, 8);
            result.unstake_fee = get_account_with(&input_accounts, 9);
            result.slumdog_stake_acc_record = get_account_with(&input_accounts, 10);
            result.unstake_protocol_fee = get_account_with(&input_accounts, 11);
            result.unstake_protocol_fee_dest = get_account_with(&input_accounts, 12);
            result.clock = get_account_with(&input_accounts, 13);
            result.stake_program = get_account_with(&input_accounts, 14);
            result.system_program = get_account_with(&input_accounts, 15);
        }
        "PrefundSwapViaStake" => {
            result.user = get_account_with(&input_accounts, 0);
            result.src_token_from = get_account_with(&input_accounts, 1);
            result.dest_token_to = get_account_with(&input_accounts, 2);
            result.bridge_stake = get_account_with(&input_accounts, 3);
            result.dest_token_fee_token_account = get_account_with(&input_accounts, 4);
            result.src_token_mint = get_account_with(&input_accounts, 5);
            result.dest_token_mint = get_account_with(&input_accounts, 6);
            result.prefunder = get_account_with(&input_accounts, 7);
            result.slumdog_stake = get_account_with(&input_accounts, 8);
            result.unstakeit_program = get_account_with(&input_accounts, 9);
            result.unstake_pool = get_account_with(&input_accounts, 10);
            result.pool_sol_reserves = get_account_with(&input_accounts, 11);
            result.unstake_fee = get_account_with(&input_accounts, 12);
            result.slumdog_stake_acc_record = get_account_with(&input_accounts, 13);
            result.unstake_protocol_fee = get_account_with(&input_accounts, 14);
            result.unstake_protocol_fee_dest = get_account_with(&input_accounts, 15);
            result.clock = get_account_with(&input_accounts, 16);
            result.stake_program = get_account_with(&input_accounts, 17);
            result.system_program = get_account_with(&input_accounts, 18);
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
