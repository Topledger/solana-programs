use crate::pb::sf::solana::block_meta::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> InputAccounts {
    let input_accounts = populate_input_accounts(account_indices, accounts);

    let mut result = InputAccounts::default();
    match instruction_type.as_str() {
        "EscrowNft" => {
            result.owner = get_account_with(&input_accounts, 0);
            result.helio_signature_wallet = get_account_with(&input_accounts, 1);
            result.owner_nft_account = get_account_with(&input_accounts, 2);
            result.escrow_account = get_account_with(&input_accounts, 3);
            result.escrow_nft_account = get_account_with(&input_accounts, 4);
            result.nft_metadata_account = get_account_with(&input_accounts, 5);
            result.escrow_pda = get_account_with(&input_accounts, 6);
            result.mint = get_account_with(&input_accounts, 7);
            result.currency = get_account_with(&input_accounts, 8);
            result.token_program = get_account_with(&input_accounts, 9);
            result.associated_token_program = get_account_with(&input_accounts, 10);
            result.sysvar_instructions = get_account_with(&input_accounts, 11);
            result.system_program = get_account_with(&input_accounts, 12);
            result.nft_master_edition = get_account_with(&input_accounts, 13);
            result.owner_token_record = get_account_with(&input_accounts, 14);
            result.destination_token_record = get_account_with(&input_accounts, 15);
            result.auth_rules_program = get_account_with(&input_accounts, 16);
            result.auth_rules = get_account_with(&input_accounts, 17);
            result.metaplex_metadata_program = get_account_with(&input_accounts, 18);
        }
        "CancelEscrow" => {
            result.sender = get_account_with(&input_accounts, 0);
            result.helio_signature_wallet = get_account_with(&input_accounts, 1);
            result.sender_nft_account = get_account_with(&input_accounts, 2);
            result.escrow_account = get_account_with(&input_accounts, 3);
            result.escrow_nft_account = get_account_with(&input_accounts, 4);
            result.nft_metadata_account = get_account_with(&input_accounts, 5);
            result.escrow_pda = get_account_with(&input_accounts, 6);
            result.mint = get_account_with(&input_accounts, 7);
            result.token_program = get_account_with(&input_accounts, 8);
            result.associated_token_program = get_account_with(&input_accounts, 9);
            result.sysvar_instructions = get_account_with(&input_accounts, 10);
            result.system_program = get_account_with(&input_accounts, 11);
            result.nft_master_edition = get_account_with(&input_accounts, 12);
            result.owner_token_record = get_account_with(&input_accounts, 13);
            result.destination_token_record = get_account_with(&input_accounts, 14);
            result.auth_rules_program = get_account_with(&input_accounts, 15);
            result.auth_rules = get_account_with(&input_accounts, 16);
            result.metaplex_metadata_program = get_account_with(&input_accounts, 17);
        }
        "SinglePaymentEscrow" => {
            result.sender = get_account_with(&input_accounts, 0);
            result.helio_signature_wallet = get_account_with(&input_accounts, 1);
            result.sender_token_account = get_account_with(&input_accounts, 2);
            result.sender_nft_account = get_account_with(&input_accounts, 3);
            result.recipient = get_account_with(&input_accounts, 4);
            result.recipient_token_account = get_account_with(&input_accounts, 5);
            result.escrow_account = get_account_with(&input_accounts, 6);
            result.escrow_nft_account = get_account_with(&input_accounts, 7);
            result.escrow_pda = get_account_with(&input_accounts, 8);
            result.nft_metadata_account = get_account_with(&input_accounts, 9);
            result.helio_fee_token_account = get_account_with(&input_accounts, 10);
            result.dao_fee_token_account = get_account_with(&input_accounts, 11);
            result.helio_fee_account = get_account_with(&input_accounts, 12);
            result.dao_fee_account = get_account_with(&input_accounts, 13);
            result.mint = get_account_with(&input_accounts, 14);
            result.currency = get_account_with(&input_accounts, 15);
            result.token_program = get_account_with(&input_accounts, 16);
            result.associated_token_program = get_account_with(&input_accounts, 17);
            result.sysvar_instructions = get_account_with(&input_accounts, 18);
            result.system_program = get_account_with(&input_accounts, 19);
            result.nft_master_edition = get_account_with(&input_accounts, 20);
            result.owner_token_record = get_account_with(&input_accounts, 21);
            result.destination_token_record = get_account_with(&input_accounts, 22);
            result.auth_rules_program = get_account_with(&input_accounts, 23);
            result.auth_rules = get_account_with(&input_accounts, 24);
            result.metaplex_metadata_program = get_account_with(&input_accounts, 25);
        }
        "SingleSolPaymentEscrow" => {
            result.sender = get_account_with(&input_accounts, 0);
            result.helio_signature_wallet = get_account_with(&input_accounts, 1);
            result.sender_nft_account = get_account_with(&input_accounts, 2);
            result.recipient = get_account_with(&input_accounts, 3);
            result.escrow_account = get_account_with(&input_accounts, 4);
            result.escrow_nft_account = get_account_with(&input_accounts, 5);
            result.escrow_pda = get_account_with(&input_accounts, 6);
            result.nft_metadata_account = get_account_with(&input_accounts, 7);
            result.helio_fee_account = get_account_with(&input_accounts, 8);
            result.dao_fee_account = get_account_with(&input_accounts, 9);
            result.mint = get_account_with(&input_accounts, 10);
            result.currency = get_account_with(&input_accounts, 11);
            result.token_program = get_account_with(&input_accounts, 12);
            result.associated_token_program = get_account_with(&input_accounts, 13);
            result.sysvar_instructions = get_account_with(&input_accounts, 14);
            result.system_program = get_account_with(&input_accounts, 15);
            result.nft_master_edition = get_account_with(&input_accounts, 16);
            result.owner_token_record = get_account_with(&input_accounts, 17);
            result.destination_token_record = get_account_with(&input_accounts, 18);
            result.auth_rules_program = get_account_with(&input_accounts, 19);
            result.auth_rules = get_account_with(&input_accounts, 20);
            result.metaplex_metadata_program = get_account_with(&input_accounts, 21);
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
