use crate::pb::sf::solana::block_meta::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> InputAccounts {
    let input_accounts = populate_input_accounts(account_indices, accounts);

    let mut result = InputAccounts::default();
    match instruction_type.as_str() {
        "CreateMint" => {
            result.mint = get_account_with(&input_accounts, 0);
            result.name_account = get_account_with(&input_accounts, 1);
            result.central_state = get_account_with(&input_accounts, 2);
            result.spl_token_program = get_account_with(&input_accounts, 3);
            result.system_program = get_account_with(&input_accounts, 4);
            result.rent_account = get_account_with(&input_accounts, 5);
            result.fee_payer = get_account_with(&input_accounts, 6);
        }
        "CreateCollection" => {
            result.collection_mint = get_account_with(&input_accounts, 0);
            result.edition = get_account_with(&input_accounts, 1);
            result.metadata_account = get_account_with(&input_accounts, 2);
            result.central_state = get_account_with(&input_accounts, 3);
            result.central_state_nft_ata = get_account_with(&input_accounts, 4);
            result.fee_payer = get_account_with(&input_accounts, 5);
            result.spl_token_program = get_account_with(&input_accounts, 6);
            result.metadata_account = get_account_with(&input_accounts, 7);
            result.system_program = get_account_with(&input_accounts, 8);
            result.spl_name_service_program = get_account_with(&input_accounts, 9);
            result.ata_program = get_account_with(&input_accounts, 10);
            result.rent_account = get_account_with(&input_accounts, 11);
        }
        "CreateNft" => {
            result.mint = get_account_with(&input_accounts, 0);
            result.nft_destination = get_account_with(&input_accounts, 1);
            result.name_account = get_account_with(&input_accounts, 2);
            result.nft_record = get_account_with(&input_accounts, 3);
            result.name_owner = get_account_with(&input_accounts, 4);
            result.metadata_account = get_account_with(&input_accounts, 5);
            result.edition_account = get_account_with(&input_accounts, 6);
            result.collection_metadata = get_account_with(&input_accounts, 7);
            result.collection_mint = get_account_with(&input_accounts, 8);
            result.central_state = get_account_with(&input_accounts, 9);
            result.fee_payer = get_account_with(&input_accounts, 10);
            result.spl_token_program = get_account_with(&input_accounts, 11);
            result.metadata_program = get_account_with(&input_accounts, 12);
            result.system_program = get_account_with(&input_accounts, 13);
            result.spl_name_service_program = get_account_with(&input_accounts, 14);
            result.rent_account = get_account_with(&input_accounts, 15);
            result.metadata_signer = get_account_with(&input_accounts, 16);
        }
        "RedeemNft" => {
            result.mint = get_account_with(&input_accounts, 0);
            result.nft_source = get_account_with(&input_accounts, 1);
            result.nft_owner = get_account_with(&input_accounts, 2);
            result.nft_record = get_account_with(&input_accounts, 3);
            result.name_account = get_account_with(&input_accounts, 4);
            result.spl_token_program = get_account_with(&input_accounts, 5);
            result.spl_name_service_program = get_account_with(&input_accounts, 6);
        }
        "WithdrawTokens" => {
            result.nft = get_account_with(&input_accounts, 0);
            result.nft_owner = get_account_with(&input_accounts, 1);
            result.nft_record = get_account_with(&input_accounts, 2);
            result.token_destination = get_account_with(&input_accounts, 3);
            result.token_source = get_account_with(&input_accounts, 4);
            result.spl_token_program = get_account_with(&input_accounts, 5);
            result.system_program = get_account_with(&input_accounts, 6);
        }
        "EditData" => {
            result.nft_owner = get_account_with(&input_accounts, 0);
            result.nft_account = get_account_with(&input_accounts, 1);
            result.nft_record = get_account_with(&input_accounts, 2);
            result.name_account = get_account_with(&input_accounts, 3);
            result.spl_token_program = get_account_with(&input_accounts, 4);
            result.spl_name_service_program = get_account_with(&input_accounts, 5);
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
