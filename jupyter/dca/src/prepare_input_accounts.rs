use crate::pb::sf::solana::block_meta::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> InputAccounts {
    let input_accounts = populate_input_accounts(account_indices, accounts);

    let mut result = InputAccounts::default();
    match instruction_type.as_str() {
        "OpenDca" => {
            result.dca = get_account_with(&input_accounts, 0);
            result.user = get_account_with(&input_accounts, 1);
            result.input_mint = get_account_with(&input_accounts, 2);
            result.output_mint = get_account_with(&input_accounts, 3);
            result.user_ata = get_account_with(&input_accounts, 4);
            result.in_ata = get_account_with(&input_accounts, 5);
            result.out_ata = get_account_with(&input_accounts, 6);
            result.system_program = get_account_with(&input_accounts, 7);
            result.token_program = get_account_with(&input_accounts, 8);
            result.associated_token_program = get_account_with(&input_accounts, 9);
            result.event_authority = get_account_with(&input_accounts, 10);
            result.program = get_account_with(&input_accounts, 11);
        }
        "CloseDca" => {
            result.user = get_account_with(&input_accounts, 0);
            result.dca = get_account_with(&input_accounts, 1);
            result.input_mint = get_account_with(&input_accounts, 2);
            result.output_mint = get_account_with(&input_accounts, 3);
            result.in_ata = get_account_with(&input_accounts, 4);
            result.out_ata = get_account_with(&input_accounts, 5);
            result.user_in_ata = get_account_with(&input_accounts, 6);
            result.user_out_ata = get_account_with(&input_accounts, 7);
            result.system_program = get_account_with(&input_accounts, 8);
            result.token_program = get_account_with(&input_accounts, 9);
            result.associated_token_program = get_account_with(&input_accounts, 10);
            result.event_authority = get_account_with(&input_accounts, 11);
            result.program = get_account_with(&input_accounts, 12);
        }
        "Withdraw" => {
            result.user = get_account_with(&input_accounts, 0);
            result.dca = get_account_with(&input_accounts, 1);
            result.input_mint = get_account_with(&input_accounts, 2);
            result.output_mint = get_account_with(&input_accounts, 3);
            result.dca_ata = get_account_with(&input_accounts, 4);
            result.user_in_ata = get_account_with(&input_accounts, 5);
            result.user_out_ata = get_account_with(&input_accounts, 6);
            result.system_program = get_account_with(&input_accounts, 7);
            result.token_program = get_account_with(&input_accounts, 8);
            result.associated_token_program = get_account_with(&input_accounts, 9);
            result.event_authority = get_account_with(&input_accounts, 10);
            result.program = get_account_with(&input_accounts, 11);
        }
        "Deposit" => {
            result.user = get_account_with(&input_accounts, 0);
            result.dca = get_account_with(&input_accounts, 1);
            result.in_ata = get_account_with(&input_accounts, 2);
            result.user_in_ata = get_account_with(&input_accounts, 3);
            result.token_program = get_account_with(&input_accounts, 4);
            result.event_authority = get_account_with(&input_accounts, 5);
            result.program = get_account_with(&input_accounts, 6);
        }
        "WithdrawFees" => {
            result.admin = get_account_with(&input_accounts, 0);
            result.mint = get_account_with(&input_accounts, 1);
            result.fee_authority = get_account_with(&input_accounts, 2);
            result.program_fee_ata = get_account_with(&input_accounts, 3);
            result.admin_fee_ata = get_account_with(&input_accounts, 4);
            result.system_program = get_account_with(&input_accounts, 5);
            result.token_program = get_account_with(&input_accounts, 6);
            result.associated_token_program = get_account_with(&input_accounts, 7);
        }
        "InitiateFlashFill" => {
            result.keeper = get_account_with(&input_accounts, 0);
            result.dca = get_account_with(&input_accounts, 1);
            result.input_mint = get_account_with(&input_accounts, 2);
            result.keeper_in_ata = get_account_with(&input_accounts, 3);
            result.in_ata = get_account_with(&input_accounts, 4);
            result.out_ata = get_account_with(&input_accounts, 5);
            result.instructions_sysvar = get_account_with(&input_accounts, 6);
            result.system_program = get_account_with(&input_accounts, 7);
            result.token_program = get_account_with(&input_accounts, 8);
            result.associated_token_program = get_account_with(&input_accounts, 9);
        }
        "FulfillFlashFill" => {
            result.keeper = get_account_with(&input_accounts, 0);
            result.dca = get_account_with(&input_accounts, 1);
            result.input_mint = get_account_with(&input_accounts, 2);
            result.output_mint = get_account_with(&input_accounts, 3);
            result.keeper_in_ata = get_account_with(&input_accounts, 4);
            result.in_ata = get_account_with(&input_accounts, 5);
            result.out_ata = get_account_with(&input_accounts, 6);
            result.fee_authority = get_account_with(&input_accounts, 7);
            result.fee_ata = get_account_with(&input_accounts, 8);
            result.instructions_sysvar = get_account_with(&input_accounts, 9);
            result.system_program = get_account_with(&input_accounts, 10);
            result.token_program = get_account_with(&input_accounts, 11);
            result.associated_token_program = get_account_with(&input_accounts, 12);
            result.event_authority = get_account_with(&input_accounts, 13);
            result.program = get_account_with(&input_accounts, 14);
        }
        "Transfer" => {
            result.keeper = get_account_with(&input_accounts, 0);
            result.dca = get_account_with(&input_accounts, 1);
            result.user = get_account_with(&input_accounts, 2);
            result.output_mint = get_account_with(&input_accounts, 3);
            result.dca_out_ata = get_account_with(&input_accounts, 4);
            result.user_out_ata = get_account_with(&input_accounts, 5);
            result.intermediate_account = get_account_with(&input_accounts, 6);
            result.system_program = get_account_with(&input_accounts, 7);
            result.token_program = get_account_with(&input_accounts, 8);
            result.associated_token_program = get_account_with(&input_accounts, 9);
            result.event_authority = get_account_with(&input_accounts, 10);
            result.program = get_account_with(&input_accounts, 11);
        }
        "EndAndClose" => {
            result.keeper = get_account_with(&input_accounts, 0);
            result.dca = get_account_with(&input_accounts, 1);
            result.input_mint = get_account_with(&input_accounts, 2);
            result.output_mint = get_account_with(&input_accounts, 3);
            result.in_ata = get_account_with(&input_accounts, 4);
            result.out_ata = get_account_with(&input_accounts, 5);
            result.user = get_account_with(&input_accounts, 6);
            result.user_out_ata = get_account_with(&input_accounts, 7);
            result.init_user_out_ata = get_account_with(&input_accounts, 8);
            result.intermediate_account = get_account_with(&input_accounts, 9);
            result.system_program = get_account_with(&input_accounts, 10);
            result.token_program = get_account_with(&input_accounts, 11);
            result.associated_token_program = get_account_with(&input_accounts, 12);
            result.event_authority = get_account_with(&input_accounts, 13);
            result.program = get_account_with(&input_accounts, 14);
        }
        "OpenDcaV2" => {
            result.dca = get_account_with(&input_accounts, 0);
            result.user = get_account_with(&input_accounts, 1);
            result.payer = get_account_with(&input_accounts, 2);
            result.input_mint = get_account_with(&input_accounts, 3);
            result.output_mint = get_account_with(&input_accounts, 4);
            result.user_ata = get_account_with(&input_accounts, 5);
            result.in_ata = get_account_with(&input_accounts, 6);
            result.out_ata = get_account_with(&input_accounts, 7);
            result.system_program = get_account_with(&input_accounts, 8);
            result.token_program = get_account_with(&input_accounts, 9);
            result.associated_token_program = get_account_with(&input_accounts, 10);
            result.event_authority = get_account_with(&input_accounts, 11);
            result.program = get_account_with(&input_accounts, 12);
        }
        "InitiateDlmmFill" => {
            result.keeper = get_account_with(&input_accounts, 0);
            result.dca = get_account_with(&input_accounts, 1);
            result.input_mint = get_account_with(&input_accounts, 2);
            result.keeper_in_ata = get_account_with(&input_accounts, 3);
            result.in_ata = get_account_with(&input_accounts, 4);
            result.out_ata = get_account_with(&input_accounts, 5);
            result.instructions_sysvar = get_account_with(&input_accounts, 6);
            result.system_program = get_account_with(&input_accounts, 7);
            result.token_program = get_account_with(&input_accounts, 8);
            result.associated_token_program = get_account_with(&input_accounts, 9);
        }
        "FulfillDlmmFill" => {
            result.keeper = get_account_with(&input_accounts, 0);
            result.dca = get_account_with(&input_accounts, 1);
            result.input_mint = get_account_with(&input_accounts, 2);
            result.output_mint = get_account_with(&input_accounts, 3);
            result.keeper_in_ata = get_account_with(&input_accounts, 4);
            result.in_ata = get_account_with(&input_accounts, 5);
            result.out_ata = get_account_with(&input_accounts, 6);
            result.fee_authority = get_account_with(&input_accounts, 7);
            result.fee_ata = get_account_with(&input_accounts, 8);
            result.instructions_sysvar = get_account_with(&input_accounts, 9);
            result.system_program = get_account_with(&input_accounts, 10);
            result.token_program = get_account_with(&input_accounts, 11);
            result.associated_token_program = get_account_with(&input_accounts, 12);
            result.event_authority = get_account_with(&input_accounts, 13);
            result.program = get_account_with(&input_accounts, 14);
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
