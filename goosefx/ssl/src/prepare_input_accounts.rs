use crate::pb::sf::solana::block_meta::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> InputAccounts {
    let input_accounts = populate_input_accounts(account_indices, accounts);

    let mut result = InputAccounts::default();
    match instruction_type.as_str() {
        "CreateLiquidityAccount" => {
            result.controller = get_account_with(&input_accounts, 0);
            result.ssl = get_account_with(&input_accounts, 1);
            result.liquidity_account = get_account_with(&input_accounts, 2);
            result.user_wallet = get_account_with(&input_accounts, 3);
            result.system_program = get_account_with(&input_accounts, 4);
            result.rent = get_account_with(&input_accounts, 5);

            // result.escrow_pda = get_account_with(&input_accounts, 6);
            // result.mint = get_account_with(&input_accounts, 7);
            // result.currency = get_account_with(&input_accounts, 8);
            // result.token_program = get_account_with(&input_accounts, 9);
            // result.associated_token_program = get_account_with(&input_accounts, 10);
            // result.sysvar_instructions = get_account_with(&input_accounts, 11);
        }
        "Deposit" => {
            result.controller = get_account_with(&input_accounts, 0);
            result.ssl = get_account_with(&input_accounts, 1);
            result.liquidity_account = get_account_with(&input_accounts, 2);
            result.rt_vault = get_account_with(&input_accounts, 3);
            result.user_rt_ata = get_account_with(&input_accounts, 4);
            result.user_wallet = get_account_with(&input_accounts, 5);
            result.token_program = get_account_with(&input_accounts, 6);
        }
        "Withdraw" => {
            result.controller = get_account_with(&input_accounts, 0);
            result.ssl = get_account_with(&input_accounts, 1);
            result.liquidity_account = get_account_with(&input_accounts, 2);
            result.rt_vault = get_account_with(&input_accounts, 3);
            result.user_rt_ata = get_account_with(&input_accounts, 4);
            result.user_wallet = get_account_with(&input_accounts, 5);
            result.token_program = get_account_with(&input_accounts, 6);
        }
        "MintPt" => {
            result.controller = get_account_with(&input_accounts, 0);
            result.ssl = get_account_with(&input_accounts, 1);
            result.rt_vault = get_account_with(&input_accounts, 2);
            result.liquidity_account = get_account_with(&input_accounts, 3);
            result.pt_mint = get_account_with(&input_accounts, 4);
            result.user_pt_ata = get_account_with(&input_accounts, 5);
            result.user_wallet = get_account_with(&input_accounts, 6);
            result.token_program = get_account_with(&input_accounts, 7);
        }
        "BurnPt" => {
            result.controller = get_account_with(&input_accounts, 0);
            result.ssl = get_account_with(&input_accounts, 1);
            result.liquidity_account = get_account_with(&input_accounts, 2);
            result.pt_mint = get_account_with(&input_accounts, 3);
            result.user_pt_ata = get_account_with(&input_accounts, 4);
            result.user_wallet = get_account_with(&input_accounts, 5);
            result.token_program = get_account_with(&input_accounts, 6);
        }
        "Swap" => {
            result.controller = get_account_with(&input_accounts, 0);
            result.pair = get_account_with(&input_accounts, 1);
            result.ssl_in = get_account_with(&input_accounts, 2);
            result.ssl_out = get_account_with(&input_accounts, 3);
            result.liability_vault_in = get_account_with(&input_accounts, 4);
            result.swapped_liability_vault_in = get_account_with(&input_accounts, 5);
            result.liability_vault_out = get_account_with(&input_accounts, 6);
            result.swapped_liability_vault_out = get_account_with(&input_accounts, 7);
            result.user_in_ata = get_account_with(&input_accounts, 8);
            result.user_out_ata = get_account_with(&input_accounts, 9);
            result.fee_collector_ata = get_account_with(&input_accounts, 10);
            result.user_wallet = get_account_with(&input_accounts, 11);
            result.fee_collector = get_account_with(&input_accounts, 12);
            result.token_program = get_account_with(&input_accounts, 13);
        }
        "CrankLiability" => {
            result.accounts = get_account_with(&input_accounts, 0);
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
