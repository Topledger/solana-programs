use crate::pb::sf::solana::block_meta::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> InputAccounts {
    let input_accounts = populate_input_accounts(account_indices, accounts);

    let mut result = InputAccounts::default();
    let input_accounts_length = input_accounts.len();
    match instruction_type.as_str() {
        "Initialize" => {
            result.swap_info = get_account_with(&input_accounts, 0);
            result.authority = get_account_with(&input_accounts, 1);
            result.token_accounts =
                get_accounts_with(&input_accounts, 2, 2 + ((input_accounts_length - 4) / 2));

            result.token_mints =
                get_accounts_with(&input_accounts, 4, 4 + ((input_accounts_length - 4) / 2));
            result.pool_token_mint = get_account_with(&input_accounts, input_accounts_length - 2);
            result.admin_token_mint = get_account_with(&input_accounts, input_accounts_length - 1);
        }
        "AddLiquidity" => {
            result.swap_info = get_account_with(&input_accounts, 0);
            result.token_program_id = get_account_with(&input_accounts, 1);
            result.authority = get_account_with(&input_accounts, 2);
            result.user_transfer_authority = get_account_with(&input_accounts, 3);
            result.token_accounts = get_accounts_with(
                &input_accounts,
                4,
                4 + ((input_accounts_length - 5 - 1) / 2),
            );

            result.pool_token_mint =
                get_account_with(&input_accounts, 4 + ((input_accounts_length - 5 - 1) / 2));
            result.source_token_accounts = get_accounts_with(
                &input_accounts,
                (4 + ((input_accounts_length - 5 - 1) / 2)) + 1,
                (4 + ((input_accounts_length - 5 - 1) / 2))
                    + 1
                    + ((input_accounts_length - 5 - 1) / 2),
            );
            result.user_lp_token_account =
                get_account_with(&input_accounts, input_accounts_length - 1);
        }
        "RemoveLiquidity" => {
            result.swap_info = get_account_with(&input_accounts, 0);
            result.token_program_id = get_account_with(&input_accounts, 1);
            result.authority = get_account_with(&input_accounts, 2);
            result.user_transfer_authority = get_account_with(&input_accounts, 3);
            result.token_accounts = get_accounts_with(
                &input_accounts,
                4,
                4 + ((input_accounts_length - 5 - 1) / 2),
            );
            result.pool_token_mint =
                get_account_with(&input_accounts, 4 + ((input_accounts_length - 5 - 1) / 2));
            result.source_token_accounts = get_accounts_with(
                &input_accounts,
                (4 + ((input_accounts_length - 5 - 1) / 2)) + 1,
                (4 + ((input_accounts_length - 5 - 1) / 2))
                    + 1
                    + ((input_accounts_length - 5 - 1) / 2),
            );
            result.user_lp_token_account =
                get_account_with(&input_accounts, input_accounts_length - 1);
        }
        "RemoveLiquidityOneToken" => {
            result.swap_info = get_account_with(&input_accounts, 0);
            result.token_program_id = get_account_with(&input_accounts, 1);
            result.authority = get_account_with(&input_accounts, 2);
            result.user_transfer_authority = get_account_with(&input_accounts, 3);
            result.token_accounts = get_accounts_with(
                &input_accounts,
                4,
                4 + ((input_accounts_length - 5 - 1) / 2),
            );
            result.pool_token_mint =
                get_account_with(&input_accounts, 4 + ((input_accounts_length - 5 - 1) / 2));
            result.source_token_accounts = get_accounts_with(
                &input_accounts,
                (4 + ((input_accounts_length - 5 - 1) / 2)) + 1,
                (4 + ((input_accounts_length - 5 - 1) / 2))
                    + 1
                    + ((input_accounts_length - 5 - 1) / 2),
            );
            result.user_lp_token_account =
                get_account_with(&input_accounts, input_accounts_length - 1);
        }
        "Exchange" => {
            result.swap_info = get_account_with(&input_accounts, 0);
            result.token_program_id = get_account_with(&input_accounts, 1);
            result.authority = get_account_with(&input_accounts, 2);
            result.user_transfer_authority = get_account_with(&input_accounts, 3);
            result.token_accounts =
                get_accounts_with(&input_accounts, 4, input_accounts_length - 2);
            result.source_token_account =
                get_account_with(&input_accounts, input_accounts_length - 2);
            result.destination_token_account =
                get_account_with(&input_accounts, input_accounts_length - 1);
        }
        "GetVirtualPrice" => {
            result.swap_info = get_account_with(&input_accounts, 0);
            result.token_program_id = get_account_with(&input_accounts, 1);
            result.token_accounts =
                get_accounts_with(&input_accounts, 2, input_accounts_length - 1);
            result.pool_token_mint = get_account_with(&input_accounts, input_accounts_length - 1);
        }
        "SetAdminSetting" => {
            result.swap_info = get_account_with(&input_accounts, 0);
            result.admin_token_account = get_account_with(&input_accounts, 1);
            result.admin_nft_owner = get_account_with(&input_accounts, 2);
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

fn get_accounts_with(accounts: &Vec<String>, start_index: usize, end_index: usize) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for x in accounts[start_index..end_index].iter() {
        result.push(x.to_string());
    }
    result
}

fn populate_input_accounts(account_indices: &Vec<u8>, accounts: &Vec<String>) -> Vec<String> {
    let mut instruction_accounts: Vec<String> = vec![];
    for (index, &el) in account_indices.iter().enumerate() {
        instruction_accounts.push(accounts.as_slice()[el as usize].to_string());
    }
    return instruction_accounts;
}
