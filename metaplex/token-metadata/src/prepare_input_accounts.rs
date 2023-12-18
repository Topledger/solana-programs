use crate::pb::sf::solana::block_meta::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> InputAccounts {
    let input_accounts = populate_input_accounts(account_indices, accounts);

    let mut result = InputAccounts::default();
    match instruction_type.as_str() {
        "CreateMetadataAccount" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.mint = get_account_with(&input_accounts, 1);
            result.mint_authority = get_account_with(&input_accounts, 2);
            result.payer = get_account_with(&input_accounts, 3);
            result.update_authority = get_account_with(&input_accounts, 4);
            result.system_program = get_account_with(&input_accounts, 5);
            result.rent = get_account_with(&input_accounts, 6);
        }
        "UpdateMetadataAccount" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.update_authority = get_account_with(&input_accounts, 1);
        }
        "DeprecatedCreateMasterEdition" => {
            result.edition = get_account_with(&input_accounts, 0);
            result.mint = get_account_with(&input_accounts, 1);
            result.printing_mint = get_account_with(&input_accounts, 2);
            result.one_time_printing_authorization_mint = get_account_with(&input_accounts, 3);
            result.update_authority = get_account_with(&input_accounts, 4);
            result.printing_mint_authority = get_account_with(&input_accounts, 5);
            result.mint_authority = get_account_with(&input_accounts, 6);
            result.metadata = get_account_with(&input_accounts, 7);
            result.payer = get_account_with(&input_accounts, 8);
            result.token_program = get_account_with(&input_accounts, 9);
            result.system_program = get_account_with(&input_accounts, 10);
            result.rent = get_account_with(&input_accounts, 11);
            result.one_time_printing_authorization_mint_authority =
                get_account_with(&input_accounts, 12);
        }
        "DeprecatedMintNewEditionFromMasterEditionViaPrintingToken" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.edition = get_account_with(&input_accounts, 1);
            result.master_edition = get_account_with(&input_accounts, 2);
            result.mint = get_account_with(&input_accounts, 3);
            result.mint_authority = get_account_with(&input_accounts, 4);
            result.printing_mint = get_account_with(&input_accounts, 5);
            result.master_token_account = get_account_with(&input_accounts, 6);
            result.edition_marker = get_account_with(&input_accounts, 7);
            result.burn_authority = get_account_with(&input_accounts, 8);
            result.payer = get_account_with(&input_accounts, 9);
            result.master_update_authority = get_account_with(&input_accounts, 10);
            result.master_metadata = get_account_with(&input_accounts, 11);
            result.token_program = get_account_with(&input_accounts, 12);
            result.system_program = get_account_with(&input_accounts, 13);
            result.rent = get_account_with(&input_accounts, 14);
            result.reservation_list = get_account_with(&input_accounts, 15);
        }
        "UpdatePrimarySaleHappenedViaToken" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.owner = get_account_with(&input_accounts, 1);
            result.token = get_account_with(&input_accounts, 2);
        }
        "DeprecatedSetReservationList" => {
            result.master_edition = get_account_with(&input_accounts, 0);
            result.reservation_list = get_account_with(&input_accounts, 1);
            result.resource = get_account_with(&input_accounts, 2);
        }
        "DeprecatedCreateReservationList" => {
            result.reservation_list = get_account_with(&input_accounts, 0);
            result.payer = get_account_with(&input_accounts, 1);
            result.update_authority = get_account_with(&input_accounts, 2);
            result.master_edition = get_account_with(&input_accounts, 3);
            result.resource = get_account_with(&input_accounts, 4);
            result.metadata = get_account_with(&input_accounts, 5);
            result.system_program = get_account_with(&input_accounts, 6);
            result.rent = get_account_with(&input_accounts, 7);
        }
        "SignMetadata" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.creator = get_account_with(&input_accounts, 1);
        }
        "DeprecatedMintPrintingTokensViaToken" => {
            result.destination = get_account_with(&input_accounts, 0);
            result.token = get_account_with(&input_accounts, 1);
            result.one_time_printing_authorization_mint = get_account_with(&input_accounts, 2);
            result.printing_mint = get_account_with(&input_accounts, 3);
            result.burn_authority = get_account_with(&input_accounts, 4);
            result.metadata = get_account_with(&input_accounts, 5);
            result.master_edition = get_account_with(&input_accounts, 6);
            result.token_program = get_account_with(&input_accounts, 7);
            result.rent = get_account_with(&input_accounts, 8);
        }
        "DeprecatedMintPrintingTokens" => {
            result.destination = get_account_with(&input_accounts, 0);
            result.printing_mint = get_account_with(&input_accounts, 1);
            result.update_authority = get_account_with(&input_accounts, 2);
            result.metadata = get_account_with(&input_accounts, 3);
            result.master_edition = get_account_with(&input_accounts, 4);
            result.token_program = get_account_with(&input_accounts, 5);
            result.rent = get_account_with(&input_accounts, 6);
        }
        "CreateMasterEdition" => {
            result.edition = get_account_with(&input_accounts, 0);
            result.mint = get_account_with(&input_accounts, 1);
            result.update_authority = get_account_with(&input_accounts, 2);
            result.mint_authority = get_account_with(&input_accounts, 3);
            result.payer = get_account_with(&input_accounts, 4);
            result.metadata = get_account_with(&input_accounts, 5);
            result.token_program = get_account_with(&input_accounts, 6);
            result.system_program = get_account_with(&input_accounts, 7);
            result.rent = get_account_with(&input_accounts, 8);
        }
        "MintNewEditionFromMasterEditionViaToken" => {
            result.new_metadata = get_account_with(&input_accounts, 0);
            result.new_edition = get_account_with(&input_accounts, 1);
            result.master_edition = get_account_with(&input_accounts, 2);
            result.new_mint = get_account_with(&input_accounts, 3);
            result.edition_mark_pda = get_account_with(&input_accounts, 4);
            result.new_mint_authority = get_account_with(&input_accounts, 5);
            result.payer = get_account_with(&input_accounts, 6);
            result.token_account_owner = get_account_with(&input_accounts, 7);
            result.token_account = get_account_with(&input_accounts, 8);
            result.new_metadata_update_authority = get_account_with(&input_accounts, 9);
            result.metadata = get_account_with(&input_accounts, 10);
            result.token_program = get_account_with(&input_accounts, 11);
            result.system_program = get_account_with(&input_accounts, 12);
            result.rent = get_account_with(&input_accounts, 13);
        }
        "ConvertMasterEditionV1ToV2" => {
            result.master_edition = get_account_with(&input_accounts, 0);
            result.one_time_auth = get_account_with(&input_accounts, 1);
            result.printing_mint = get_account_with(&input_accounts, 2);
        }
        "MintNewEditionFromMasterEditionViaVaultProxy" => {
            result.new_metadata = get_account_with(&input_accounts, 0);
            result.new_edition = get_account_with(&input_accounts, 1);
            result.master_edition = get_account_with(&input_accounts, 2);
            result.new_mint = get_account_with(&input_accounts, 3);
            result.edition_mark_pda = get_account_with(&input_accounts, 4);
            result.new_mint_authority = get_account_with(&input_accounts, 5);
            result.payer = get_account_with(&input_accounts, 6);
            result.vault_authority = get_account_with(&input_accounts, 7);
            result.safety_deposit_store = get_account_with(&input_accounts, 8);
            result.safety_deposit_box = get_account_with(&input_accounts, 9);
            result.vault = get_account_with(&input_accounts, 10);
            result.new_metadata_update_authority = get_account_with(&input_accounts, 11);
            result.metadata = get_account_with(&input_accounts, 12);
            result.token_program = get_account_with(&input_accounts, 13);
            result.token_vault_program = get_account_with(&input_accounts, 14);
            result.system_program = get_account_with(&input_accounts, 15);
            result.rent = get_account_with(&input_accounts, 16);
        }
        "PuffMetadata" => {
            result.metadata = get_account_with(&input_accounts, 0);
        }
        "UpdateMetadataAccountV2" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.update_authority = get_account_with(&input_accounts, 1);
        }
        "CreateMetadataAccountV2" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.mint = get_account_with(&input_accounts, 1);
            result.mint_authority = get_account_with(&input_accounts, 2);
            result.payer = get_account_with(&input_accounts, 3);
            result.update_authority = get_account_with(&input_accounts, 4);
            result.system_program = get_account_with(&input_accounts, 5);
            result.rent = get_account_with(&input_accounts, 6);
        }
        "CreateMasterEditionV3" => {
            result.edition = get_account_with(&input_accounts, 0);
            result.mint = get_account_with(&input_accounts, 1);
            result.update_authority = get_account_with(&input_accounts, 2);
            result.mint_authority = get_account_with(&input_accounts, 3);
            result.payer = get_account_with(&input_accounts, 4);
            result.metadata = get_account_with(&input_accounts, 5);
            result.token_program = get_account_with(&input_accounts, 6);
            result.system_program = get_account_with(&input_accounts, 7);
            result.rent = get_account_with(&input_accounts, 8);
        }
        "VerifyCollection" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.collection_authority = get_account_with(&input_accounts, 1);
            result.payer = get_account_with(&input_accounts, 2);
            result.collection_mint = get_account_with(&input_accounts, 3);
            result.collection = get_account_with(&input_accounts, 4);
            result.collection_master_edition_account = get_account_with(&input_accounts, 5);
        }
        "Utilize" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.token_account = get_account_with(&input_accounts, 1);
            result.mint = get_account_with(&input_accounts, 2);
            result.use_authority = get_account_with(&input_accounts, 3);
            result.owner = get_account_with(&input_accounts, 4);
            result.token_program = get_account_with(&input_accounts, 5);
            result.ata_program = get_account_with(&input_accounts, 6);
            result.system_program = get_account_with(&input_accounts, 7);
            result.rent = get_account_with(&input_accounts, 8);
            result.use_authority_record = get_account_with(&input_accounts, 9);
            result.burner = get_account_with(&input_accounts, 10);
        }
        "ApproveUseAuthority" => {
            result.use_authority_record = get_account_with(&input_accounts, 0);
            result.owner = get_account_with(&input_accounts, 1);
            result.payer = get_account_with(&input_accounts, 2);
            result.user = get_account_with(&input_accounts, 3);
            result.owner_token_account = get_account_with(&input_accounts, 4);
            result.metadata = get_account_with(&input_accounts, 5);
            result.mint = get_account_with(&input_accounts, 6);
            result.burner = get_account_with(&input_accounts, 7);
            result.token_program = get_account_with(&input_accounts, 8);
            result.system_program = get_account_with(&input_accounts, 9);
            result.rent = get_account_with(&input_accounts, 10);
        }
        "RevokeUseAuthority" => {
            result.use_authority_record = get_account_with(&input_accounts, 0);
            result.owner = get_account_with(&input_accounts, 1);
            result.user = get_account_with(&input_accounts, 2);
            result.owner_token_account = get_account_with(&input_accounts, 3);
            result.mint = get_account_with(&input_accounts, 4);
            result.metadata = get_account_with(&input_accounts, 5);
            result.token_program = get_account_with(&input_accounts, 6);
            result.system_program = get_account_with(&input_accounts, 7);
            result.rent = get_account_with(&input_accounts, 8);
        }
        "UnverifyCollection" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.collection_authority = get_account_with(&input_accounts, 1);
            result.collection_mint = get_account_with(&input_accounts, 2);
            result.collection = get_account_with(&input_accounts, 3);
            result.collection_master_edition_account = get_account_with(&input_accounts, 4);
            result.collection_authority_record = get_account_with(&input_accounts, 5);
        }
        "ApproveCollectionAuthority" => {
            result.collection_authority_record = get_account_with(&input_accounts, 0);
            result.new_collection_authority = get_account_with(&input_accounts, 1);
            result.update_authority = get_account_with(&input_accounts, 2);
            result.payer = get_account_with(&input_accounts, 3);
            result.metadata = get_account_with(&input_accounts, 4);
            result.mint = get_account_with(&input_accounts, 5);
            result.system_program = get_account_with(&input_accounts, 6);
            result.rent = get_account_with(&input_accounts, 7);
        }
        "RevokeCollectionAuthority" => {
            result.collection_authority_record = get_account_with(&input_accounts, 0);
            result.delegate_authority = get_account_with(&input_accounts, 1);
            result.revoke_authority = get_account_with(&input_accounts, 2);
            result.metadata = get_account_with(&input_accounts, 3);
            result.mint = get_account_with(&input_accounts, 4);
        }
        "SetAndVerifyCollection" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.collection_authority = get_account_with(&input_accounts, 1);
            result.payer = get_account_with(&input_accounts, 2);
            result.update_authority = get_account_with(&input_accounts, 3);
            result.collection_mint = get_account_with(&input_accounts, 4);
            result.collection = get_account_with(&input_accounts, 5);
            result.collection_master_edition_account = get_account_with(&input_accounts, 6);
            result.collection_authority_record = get_account_with(&input_accounts, 7);
        }
        "FreezeDelegatedAccount" => {
            result.delegate = get_account_with(&input_accounts, 0);
            result.token_account = get_account_with(&input_accounts, 1);
            result.edition = get_account_with(&input_accounts, 2);
            result.mint = get_account_with(&input_accounts, 3);
            result.token_program = get_account_with(&input_accounts, 4);
        }
        "ThawDelegatedAccount" => {
            result.delegate = get_account_with(&input_accounts, 0);
            result.token_account = get_account_with(&input_accounts, 1);
            result.edition = get_account_with(&input_accounts, 2);
            result.mint = get_account_with(&input_accounts, 3);
            result.token_program = get_account_with(&input_accounts, 4);
        }
        "RemoveCreatorVerification" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.creator = get_account_with(&input_accounts, 1);
        }
        "BurnNft" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.owner = get_account_with(&input_accounts, 1);
            result.mint = get_account_with(&input_accounts, 2);
            result.token_account = get_account_with(&input_accounts, 3);
            result.master_edition_account = get_account_with(&input_accounts, 4);
            result.spl_token_program = get_account_with(&input_accounts, 5);
            result.collection_metadata = get_account_with(&input_accounts, 6);
        }
        "VerifySizedCollectionItem" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.collection_authority = get_account_with(&input_accounts, 1);
            result.payer = get_account_with(&input_accounts, 2);
            result.collection_mint = get_account_with(&input_accounts, 3);
            result.collection = get_account_with(&input_accounts, 4);
            result.collection_master_edition_account = get_account_with(&input_accounts, 5);
            result.collection_authority_record = get_account_with(&input_accounts, 6);
        }
        "UnverifySizedCollectionItem" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.collection_authority = get_account_with(&input_accounts, 1);
            result.payer = get_account_with(&input_accounts, 2);
            result.collection_mint = get_account_with(&input_accounts, 3);
            result.collection = get_account_with(&input_accounts, 4);
            result.collection_master_edition_account = get_account_with(&input_accounts, 5);
            result.collection_authority_record = get_account_with(&input_accounts, 6);
        }
        "SetAndVerifySizedCollectionItem" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.collection_authority = get_account_with(&input_accounts, 1);
            result.payer = get_account_with(&input_accounts, 2);
            result.update_authority = get_account_with(&input_accounts, 3);
            result.collection_mint = get_account_with(&input_accounts, 4);
            result.collection = get_account_with(&input_accounts, 5);
            result.collection_master_edition_account = get_account_with(&input_accounts, 6);
            result.collection_authority_record = get_account_with(&input_accounts, 7);
        }
        "CreateMetadataAccountV3" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.mint = get_account_with(&input_accounts, 1);
            result.mint_authority = get_account_with(&input_accounts, 2);
            result.payer = get_account_with(&input_accounts, 3);
            result.update_authority = get_account_with(&input_accounts, 4);
            result.system_program = get_account_with(&input_accounts, 5);
            result.rent = get_account_with(&input_accounts, 6);
        }
        "SetCollectionSize" => {
            result.collection_metadata = get_account_with(&input_accounts, 0);
            result.collection_authority = get_account_with(&input_accounts, 1);
            result.collection_mint = get_account_with(&input_accounts, 2);
            result.collection_authority_record = get_account_with(&input_accounts, 3);
        }
        "SetTokenStandard" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.update_authority = get_account_with(&input_accounts, 1);
            result.mint = get_account_with(&input_accounts, 2);
            result.edition = get_account_with(&input_accounts, 3);
        }
        "BubblegumSetCollectionSize" => {
            result.collection_metadata = get_account_with(&input_accounts, 0);
            result.collection_authority = get_account_with(&input_accounts, 1);
            result.collection_mint = get_account_with(&input_accounts, 2);
            result.bubblegum_signer = get_account_with(&input_accounts, 3);
            result.collection_authority_record = get_account_with(&input_accounts, 4);
        }
        "BurnEditionNft" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.owner = get_account_with(&input_accounts, 1);
            result.print_edition_mint = get_account_with(&input_accounts, 2);
            result.master_edition_mint = get_account_with(&input_accounts, 3);
            result.print_edition_token_account = get_account_with(&input_accounts, 4);
            result.master_edition_token_account = get_account_with(&input_accounts, 5);
            result.master_edition_account = get_account_with(&input_accounts, 6);
            result.print_edition_account = get_account_with(&input_accounts, 7);
            result.edition_marker_account = get_account_with(&input_accounts, 8);
            result.spl_token_program = get_account_with(&input_accounts, 9);
        }
        "CreateEscrowAccount" => {
            result.escrow = get_account_with(&input_accounts, 0);
            result.metadata = get_account_with(&input_accounts, 1);
            result.mint = get_account_with(&input_accounts, 2);
            result.token_account = get_account_with(&input_accounts, 3);
            result.edition = get_account_with(&input_accounts, 4);
            result.payer = get_account_with(&input_accounts, 5);
            result.system_program = get_account_with(&input_accounts, 6);
            result.authority = get_account_with(&input_accounts, 7);
        }
        "CloseEscrowAccount" => {
            result.escrow = get_account_with(&input_accounts, 0);
            result.metadata = get_account_with(&input_accounts, 1);
            result.mint = get_account_with(&input_accounts, 2);
            result.token_account = get_account_with(&input_accounts, 3);
            result.edition = get_account_with(&input_accounts, 4);
            result.payer = get_account_with(&input_accounts, 5);
            result.system_program = get_account_with(&input_accounts, 6);
        }
        "TransferOutOfEscrow" => {
            result.escrow = get_account_with(&input_accounts, 0);
            result.payer = get_account_with(&input_accounts, 1);
            result.attribute_mint = get_account_with(&input_accounts, 2);
            result.attribute_src = get_account_with(&input_accounts, 3);
            result.attribute_dst = get_account_with(&input_accounts, 4);
            result.escrow_mint = get_account_with(&input_accounts, 5);
            result.escrow_account = get_account_with(&input_accounts, 6);
            result.system_program = get_account_with(&input_accounts, 7);
            result.ata_program = get_account_with(&input_accounts, 8);
            result.token_program = get_account_with(&input_accounts, 9);
            result.rent = get_account_with(&input_accounts, 10);
            result.authority = get_account_with(&input_accounts, 11);
        }
        "Burn" => {
            result.authority = get_account_with(&input_accounts, 0);
            result.collection_metadata = get_account_with(&input_accounts, 1);
            result.metadata = get_account_with(&input_accounts, 2);
            result.edition = get_account_with(&input_accounts, 3);
            result.mint = get_account_with(&input_accounts, 4);
            result.token = get_account_with(&input_accounts, 5);
            result.master_edition = get_account_with(&input_accounts, 6);
            result.master_edition_mint = get_account_with(&input_accounts, 7);
            result.master_edition_token = get_account_with(&input_accounts, 8);
            result.edition_marker = get_account_with(&input_accounts, 9);
            result.token_record = get_account_with(&input_accounts, 10);
            result.system_program = get_account_with(&input_accounts, 11);
            result.sysvar_instructions = get_account_with(&input_accounts, 12);
            result.spl_token_program = get_account_with(&input_accounts, 13);
        }
        "Create" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.master_edition = get_account_with(&input_accounts, 1);
            result.mint = get_account_with(&input_accounts, 2);
            result.authority = get_account_with(&input_accounts, 3);
            result.payer = get_account_with(&input_accounts, 4);
            result.update_authority = get_account_with(&input_accounts, 5);
            result.system_program = get_account_with(&input_accounts, 6);
            result.sysvar_instructions = get_account_with(&input_accounts, 7);
            result.spl_token_program = get_account_with(&input_accounts, 8);
        }
        "Mint" => {
            result.token = get_account_with(&input_accounts, 0);
            result.token_owner = get_account_with(&input_accounts, 1);
            result.metadata = get_account_with(&input_accounts, 2);
            result.master_edition = get_account_with(&input_accounts, 3);
            result.token_record = get_account_with(&input_accounts, 4);
            result.mint = get_account_with(&input_accounts, 5);
            result.authority = get_account_with(&input_accounts, 6);
            result.delegate_record = get_account_with(&input_accounts, 7);
            result.payer = get_account_with(&input_accounts, 8);
            result.system_program = get_account_with(&input_accounts, 9);
            result.sysvar_instructions = get_account_with(&input_accounts, 10);
            result.spl_token_program = get_account_with(&input_accounts, 11);
            result.spl_ata_program = get_account_with(&input_accounts, 12);
            result.authorization_rules_program = get_account_with(&input_accounts, 13);
            result.authorization_rules = get_account_with(&input_accounts, 14);
        }
        "Delegate" => {
            result.delegate_record = get_account_with(&input_accounts, 0);
            result.delegate = get_account_with(&input_accounts, 1);
            result.metadata = get_account_with(&input_accounts, 2);
            result.master_edition = get_account_with(&input_accounts, 3);
            result.token_record = get_account_with(&input_accounts, 4);
            result.mint = get_account_with(&input_accounts, 5);
            result.token = get_account_with(&input_accounts, 6);
            result.authority = get_account_with(&input_accounts, 7);
            result.payer = get_account_with(&input_accounts, 8);
            result.system_program = get_account_with(&input_accounts, 9);
            result.sysvar_instructions = get_account_with(&input_accounts, 10);
            result.spl_token_program = get_account_with(&input_accounts, 11);
            result.authorization_rules_program = get_account_with(&input_accounts, 12);
            result.authorization_rules = get_account_with(&input_accounts, 13);
        }
        "Revoke" => {
            result.delegate_record = get_account_with(&input_accounts, 0);
            result.delegate = get_account_with(&input_accounts, 1);
            result.metadata = get_account_with(&input_accounts, 2);
            result.master_edition = get_account_with(&input_accounts, 3);
            result.token_record = get_account_with(&input_accounts, 4);
            result.mint = get_account_with(&input_accounts, 5);
            result.token = get_account_with(&input_accounts, 6);
            result.authority = get_account_with(&input_accounts, 7);
            result.payer = get_account_with(&input_accounts, 8);
            result.system_program = get_account_with(&input_accounts, 9);
            result.sysvar_instructions = get_account_with(&input_accounts, 10);
            result.spl_token_program = get_account_with(&input_accounts, 11);
            result.authorization_rules_program = get_account_with(&input_accounts, 12);
            result.authorization_rules = get_account_with(&input_accounts, 13);
        }
        "Lock" => {
            result.authority = get_account_with(&input_accounts, 0);
            result.token_owner = get_account_with(&input_accounts, 1);
            result.token = get_account_with(&input_accounts, 2);
            result.mint = get_account_with(&input_accounts, 3);
            result.metadata = get_account_with(&input_accounts, 4);
            result.edition = get_account_with(&input_accounts, 5);
            result.token_record = get_account_with(&input_accounts, 6);
            result.payer = get_account_with(&input_accounts, 7);
            result.system_program = get_account_with(&input_accounts, 8);
            result.sysvar_instructions = get_account_with(&input_accounts, 9);
            result.spl_token_program = get_account_with(&input_accounts, 10);
            result.authorization_rules_program = get_account_with(&input_accounts, 11);
            result.authorization_rules = get_account_with(&input_accounts, 12);
        }
        "Unlock" => {
            result.authority = get_account_with(&input_accounts, 0);
            result.token_owner = get_account_with(&input_accounts, 1);
            result.token = get_account_with(&input_accounts, 2);
            result.mint = get_account_with(&input_accounts, 3);
            result.metadata = get_account_with(&input_accounts, 4);
            result.edition = get_account_with(&input_accounts, 5);
            result.token_record = get_account_with(&input_accounts, 6);
            result.payer = get_account_with(&input_accounts, 7);
            result.system_program = get_account_with(&input_accounts, 8);
            result.sysvar_instructions = get_account_with(&input_accounts, 9);
            result.spl_token_program = get_account_with(&input_accounts, 10);
            result.authorization_rules_program = get_account_with(&input_accounts, 11);
            result.authorization_rules = get_account_with(&input_accounts, 12);
        }
        "Migrate" => {
            result.metadata = get_account_with(&input_accounts, 0);
            result.edition = get_account_with(&input_accounts, 1);
            result.token = get_account_with(&input_accounts, 2);
            result.token_owner = get_account_with(&input_accounts, 3);
            result.mint = get_account_with(&input_accounts, 4);
            result.payer = get_account_with(&input_accounts, 5);
            result.authority = get_account_with(&input_accounts, 6);
            result.collection_metadata = get_account_with(&input_accounts, 7);
            result.delegate_record = get_account_with(&input_accounts, 8);
            result.token_record = get_account_with(&input_accounts, 9);
            result.system_program = get_account_with(&input_accounts, 10);
            result.sysvar_instructions = get_account_with(&input_accounts, 11);
            result.spl_token_program = get_account_with(&input_accounts, 12);
            result.authorization_rules_program = get_account_with(&input_accounts, 13);
            result.authorization_rules = get_account_with(&input_accounts, 14);
        }
        "Transfer" => {
            result.token = get_account_with(&input_accounts, 0);
            result.token_owner = get_account_with(&input_accounts, 1);
            result.destination = get_account_with(&input_accounts, 2);
            result.destination_owner = get_account_with(&input_accounts, 3);
            result.mint = get_account_with(&input_accounts, 4);
            result.metadata = get_account_with(&input_accounts, 5);
            result.edition = get_account_with(&input_accounts, 6);
            result.owner_token_record = get_account_with(&input_accounts, 7);
            result.destination_token_record = get_account_with(&input_accounts, 8);
            result.authority = get_account_with(&input_accounts, 9);
            result.payer = get_account_with(&input_accounts, 10);
            result.system_program = get_account_with(&input_accounts, 11);
            result.sysvar_instructions = get_account_with(&input_accounts, 12);
            result.spl_token_program = get_account_with(&input_accounts, 13);
            result.spl_ata_program = get_account_with(&input_accounts, 14);
            result.authorization_rules_program = get_account_with(&input_accounts, 15);
            result.authorization_rules = get_account_with(&input_accounts, 16);
        }
        "Update" => {
            result.authority = get_account_with(&input_accounts, 0);
            result.delegate_record = get_account_with(&input_accounts, 1);
            result.token = get_account_with(&input_accounts, 2);
            result.mint = get_account_with(&input_accounts, 3);
            result.metadata = get_account_with(&input_accounts, 4);
            result.edition = get_account_with(&input_accounts, 5);
            result.payer = get_account_with(&input_accounts, 6);
            result.system_program = get_account_with(&input_accounts, 7);
            result.sysvar_instructions = get_account_with(&input_accounts, 8);
            result.authorization_rules_program = get_account_with(&input_accounts, 9);
            result.authorization_rules = get_account_with(&input_accounts, 10);
        }
        "Use" => {
            result.authority = get_account_with(&input_accounts, 0);
            result.delegate_record = get_account_with(&input_accounts, 1);
            result.token = get_account_with(&input_accounts, 2);
            result.mint = get_account_with(&input_accounts, 3);
            result.metadata = get_account_with(&input_accounts, 4);
            result.edition = get_account_with(&input_accounts, 5);
            result.payer = get_account_with(&input_accounts, 6);
            result.system_program = get_account_with(&input_accounts, 7);
            result.sysvar_instructions = get_account_with(&input_accounts, 8);
            result.spl_token_program = get_account_with(&input_accounts, 9);
            result.authorization_rules_program = get_account_with(&input_accounts, 10);
            result.authorization_rules = get_account_with(&input_accounts, 11);
        }
        "Verify" => {
            result.authority = get_account_with(&input_accounts, 0);
            result.delegate_record = get_account_with(&input_accounts, 1);
            result.metadata = get_account_with(&input_accounts, 2);
            result.collection_mint = get_account_with(&input_accounts, 3);
            result.collection_metadata = get_account_with(&input_accounts, 4);
            result.collection_master_edition = get_account_with(&input_accounts, 5);
            result.system_program = get_account_with(&input_accounts, 6);
            result.sysvar_instructions = get_account_with(&input_accounts, 7);
        }
        "Unverify" => {
            result.authority = get_account_with(&input_accounts, 0);
            result.delegate_record = get_account_with(&input_accounts, 1);
            result.metadata = get_account_with(&input_accounts, 2);
            result.collection_mint = get_account_with(&input_accounts, 3);
            result.collection_metadata = get_account_with(&input_accounts, 4);
            result.system_program = get_account_with(&input_accounts, 5);
            result.sysvar_instructions = get_account_with(&input_accounts, 6);
        }
        "Collect" => {
            result.authority = get_account_with(&input_accounts, 0);
            result.pda_account = get_account_with(&input_accounts, 1);
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
