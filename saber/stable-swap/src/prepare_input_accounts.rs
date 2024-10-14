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
            result.new_stable_swap = get_account_with(&input_accounts, 0);
            result.authority = get_account_with(&input_accounts, 1);
            result.admin_account = get_account_with(&input_accounts, 2);
            result.admin_fee_aadmin_fee_account = get_account_with(&input_accounts, 3);
            result.admin_fee_badmin_fee_account = get_account_with(&input_accounts, 4);
            result.token_a_account = get_account_with(&input_accounts, 5);
            result.token_b_account = get_account_with(&input_accounts, 6);
            result.pool_token_mint = get_account_with(&input_accounts, 7);
        }
        "Swap" => {
            result.stable_swap = get_account_with(&input_accounts, 0);
            result.authority = get_account_with(&input_accounts, 1);
            result.user_authority = get_account_with(&input_accounts, 2);
            result.token_ab_source_account = get_account_with(&input_accounts, 3);
            result.token_ab_base_account_swap_into = get_account_with(&input_accounts, 4);
            result.token_ab_base_account_swap_from = get_account_with(&input_accounts, 5);
            result.token_ab_destination_account = get_account_with(&input_accounts, 6);
            result.token_ab_admin_fee_account = get_account_with(&input_accounts, 7);
            result.token_program_id = get_account_with(&input_accounts, 7);
        }
        "Deposit" => {
            result.stable_swap = get_account_with(&input_accounts, 0);
            result.authority = get_account_with(&input_accounts, 1);
            result.user_authority = get_account_with(&input_accounts, 2);
            result.token_a_authority = get_account_with(&input_accounts, 3);
            result.token_b_authority = get_account_with(&input_accounts, 4);
            result.token_a_base_account_deposit_into = get_account_with(&input_accounts, 5);
            result.token_b_base_account_deposit_into = get_account_with(&input_accounts, 6);
            result.pool_mint_account = get_account_with(&input_accounts, 7);
            result.pool_account_deposit = get_account_with(&input_accounts, 8);
            result.token_program_id = get_account_with(&input_accounts, 9);
        }
        "Withdraw" => {
            result.stable_swap = get_account_with(&input_accounts, 0);
            result.authority = get_account_with(&input_accounts, 1);
            result.user_authority = get_account_with(&input_accounts, 2);
            result.pool_mint_account = get_account_with(&input_accounts, 3);
            result.source_pool_account = get_account_with(&input_accounts, 4);
            result.token_a_swap_account_withdraw_from = get_account_with(&input_accounts, 5);
            result.token_b_swap_account_withdraw_from = get_account_with(&input_accounts, 6);
            result.token_a_user_account = get_account_with(&input_accounts, 7);
            result.token_b_user_account = get_account_with(&input_accounts, 8);
            result.admin_fee_a_account = get_account_with(&input_accounts, 9);
            result.admin_fee_b_account = get_account_with(&input_accounts, 10);
            result.token_program_id = get_account_with(&input_accounts, 11);
        }
        "WithdrawOne" => {
            result.stable_swap = get_account_with(&input_accounts, 0);
            result.authority = get_account_with(&input_accounts, 1);
            result.user_authority = get_account_with(&input_accounts, 2);
            result.pool_mint_account = get_account_with(&input_accounts, 3);
            result.source_pool_account = get_account_with(&input_accounts, 4);
            result.token_ab_base_swap_account_withdraw_from = get_account_with(&input_accounts, 5);
            result.token_ab_quote_swap_account_exchange = get_account_with(&input_accounts, 6);
            result.token_ab_base_user_account_credit = get_account_with(&input_accounts, 7);
            result.token_ab_admin_fee_account = get_account_with(&input_accounts, 8);
            result.token_program_id = get_account_with(&input_accounts, 9);
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
