use crate::pb::sf::solana::block_meta::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> InputAccounts {
    let input_accounts = populate_input_accounts(account_indices, accounts);

    let mut result = InputAccounts::default();
    match instruction_type.as_str() {
        "InitializeMarket" => {
            result.authority = get_account_with(&input_accounts, 0);
            result.underlying_asset_mint = get_account_with(&input_accounts, 1);
            result.quote_asset_mint = get_account_with(&input_accounts, 2);
            result.option_mint = get_account_with(&input_accounts, 3);
            result.writer_token_mint = get_account_with(&input_accounts, 4);
            result.quote_asset_pool = get_account_with(&input_accounts, 5);
            result.underlying_asset_pool = get_account_with(&input_accounts, 6);
            result.option_market = get_account_with(&input_accounts, 7);
            result.fee_owner = get_account_with(&input_accounts, 8);
            result.token_program = get_account_with(&input_accounts, 9);
            result.associated_token_program = get_account_with(&input_accounts, 10);
            result.rent = get_account_with(&input_accounts, 11);
            result.system_program = get_account_with(&input_accounts, 12);
            result.clock = get_account_with(&input_accounts, 13);
        }
        "MintOption" => {
            result.user_authority = get_account_with(&input_accounts, 0);
            result.underlying_asset_mint = get_account_with(&input_accounts, 1);
            result.underlying_asset_pool = get_account_with(&input_accounts, 2);
            result.underlying_asset_src = get_account_with(&input_accounts, 3);
            result.option_mint = get_account_with(&input_accounts, 4);
            result.minted_option_dest = get_account_with(&input_accounts, 5);
            result.writer_token_mint = get_account_with(&input_accounts, 6);
            result.minted_writer_token_dest = get_account_with(&input_accounts, 7);
            result.option_market = get_account_with(&input_accounts, 8);
            result.fee_owner = get_account_with(&input_accounts, 9);
            result.token_program = get_account_with(&input_accounts, 10);
            result.associated_token_program = get_account_with(&input_accounts, 11);
            result.clock = get_account_with(&input_accounts, 12);
            result.rent = get_account_with(&input_accounts, 13);
            result.system_program = get_account_with(&input_accounts, 14);
        }
        "MintOptionV2" => {
            result.user_authority = get_account_with(&input_accounts, 0);
            result.underlying_asset_mint = get_account_with(&input_accounts, 1);
            result.underlying_asset_pool = get_account_with(&input_accounts, 2);
            result.underlying_asset_src = get_account_with(&input_accounts, 3);
            result.option_mint = get_account_with(&input_accounts, 4);
            result.minted_option_dest = get_account_with(&input_accounts, 5);
            result.writer_token_mint = get_account_with(&input_accounts, 6);
            result.minted_writer_token_dest = get_account_with(&input_accounts, 7);
            result.option_market = get_account_with(&input_accounts, 8);
            result.token_program = get_account_with(&input_accounts, 9);
        }
        "ExerciseOption" => {
            result.user_authority = get_account_with(&input_accounts, 0);
            result.option_authority = get_account_with(&input_accounts, 1);
            result.option_market = get_account_with(&input_accounts, 2);
            result.option_mint = get_account_with(&input_accounts, 3);
            result.exerciser_option_token_src = get_account_with(&input_accounts, 4);
            result.underlying_asset_pool = get_account_with(&input_accounts, 5);
            result.underlying_asset_dest = get_account_with(&input_accounts, 6);
            result.quote_asset_pool = get_account_with(&input_accounts, 7);
            result.quote_asset_src = get_account_with(&input_accounts, 8);
            result.fee_owner = get_account_with(&input_accounts, 9);
            result.token_program = get_account_with(&input_accounts, 10);
            result.system_program = get_account_with(&input_accounts, 11);
            result.clock = get_account_with(&input_accounts, 12);
        }
        "ExerciseOptionV2" => {
            result.user_authority = get_account_with(&input_accounts, 0);
            result.option_authority = get_account_with(&input_accounts, 1);
            result.option_market = get_account_with(&input_accounts, 2);
            result.option_mint = get_account_with(&input_accounts, 3);
            result.exerciser_option_token_src = get_account_with(&input_accounts, 4);
            result.underlying_asset_pool = get_account_with(&input_accounts, 5);
            result.underlying_asset_dest = get_account_with(&input_accounts, 6);
            result.quote_asset_pool = get_account_with(&input_accounts, 7);
            result.quote_asset_src = get_account_with(&input_accounts, 8);
            result.token_program = get_account_with(&input_accounts, 9);
        }
        "ClosePostExpiration" => {
            result.user_authority = get_account_with(&input_accounts, 0);
            result.option_authority = get_account_with(&input_accounts, 1);
            result.writer_token_mint = get_account_with(&input_accounts, 2);
            result.writer_token_src = get_account_with(&input_accounts, 3);
            result.underlying_asset_pool = get_account_with(&input_accounts, 4);
            result.underlying_asset_dest = get_account_with(&input_accounts, 5);
            result.token_program = get_account_with(&input_accounts, 6);
            result.clock = get_account_with(&input_accounts, 7);
        }
        "CloseOptionPosition" => {
            result.user_authority = get_account_with(&input_accounts, 0);
            result.option_authority = get_account_with(&input_accounts, 1);
            result.writer_token_mint = get_account_with(&input_accounts, 2);
            result.writer_token_src = get_account_with(&input_accounts, 3);
            result.option_token_mint = get_account_with(&input_accounts, 4);
            result.option_token_src = get_account_with(&input_accounts, 5);
            result.underlying_asset_pool = get_account_with(&input_accounts, 6);
            result.underlying_asset_dest = get_account_with(&input_accounts, 7);
            result.token_program = get_account_with(&input_accounts, 8);
        }
        "BurnWriterForQuote" => {
            result.user_authority = get_account_with(&input_accounts, 0);
            result.option_authority = get_account_with(&input_accounts, 1);
            result.writer_token_mint = get_account_with(&input_accounts, 2);
            result.writer_token_src = get_account_with(&input_accounts, 3);
            result.quote_asset_pool = get_account_with(&input_accounts, 4);
            result.writer_quote_dest = get_account_with(&input_accounts, 5);
            result.token_program = get_account_with(&input_accounts, 6);
        }
        "InitSerumMarket" => {
            result.user_authority = get_account_with(&input_accounts, 0);
            result.option_market = get_account_with(&input_accounts, 1);
            result.serum_market = get_account_with(&input_accounts, 2);
            result.system_program = get_account_with(&input_accounts, 3);
            result.token_program = get_account_with(&input_accounts, 4);
            result.dex_program = get_account_with(&input_accounts, 5);
            result.rent = get_account_with(&input_accounts, 6);
            result.pc_mint = get_account_with(&input_accounts, 7);
            result.option_mint = get_account_with(&input_accounts, 8);
            result.request_queue = get_account_with(&input_accounts, 9);
            result.event_queue = get_account_with(&input_accounts, 10);
            result.bids = get_account_with(&input_accounts, 11);
            result.asks = get_account_with(&input_accounts, 12);
            result.coin_vault = get_account_with(&input_accounts, 13);
            result.pc_vault = get_account_with(&input_accounts, 14);
            result.vault_signer = get_account_with(&input_accounts, 15);
            result.market_authority = get_account_with(&input_accounts, 16);
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
