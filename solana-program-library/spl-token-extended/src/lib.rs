#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod constants;
mod instruction;
mod pb;
mod utils;

use instruction::{parse_instruction, Instruction};
use pb::sf::solana::spl::v1::{Accounts, Arg, Output, SplTokenMeta};
// use substreams::log;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams_solana::pb::sf::solana::r#type::v1::TokenBalance;
use utils::convert_to_date;

#[derive(Default)]
pub struct OuterArg {
    pub instruction_type: String,
    pub input_accounts: Accounts,
    pub arg: Arg,
}

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Output, substreams::errors::Error> {
    let slot = block.slot;
    let parent_slot = block.parent_slot;
    let timestamp = block.block_time.as_ref().unwrap().timestamp;

    let mut data: Vec<SplTokenMeta> = vec![];

    for (tx_index, trx) in block.transactions_owned().enumerate() {
        let accounts = trx.resolved_accounts_as_strings();
        if let Some(transaction) = trx.transaction {
            let meta = trx.meta.unwrap();
            let msg = transaction.message.unwrap();
            let pre_token_balances = meta.pre_token_balances;
            let post_token_balances = meta.post_token_balances;
            
            // Extract transaction-level info
            let tx_fee = meta.fee;
            let num_signers = transaction.signatures.len() as u32;
            let signer = if !accounts.is_empty() {
                Some(accounts[0].clone()) // First global account key (fee payer)
            } else {
                None
            };

            for (idx, inst) in msg.instructions.into_iter().enumerate() {
                let program = &accounts[inst.program_id_index as usize];

                if program == constants::PROGRAM_ADDRESS {
                    let outer_arg = get_outer_arg(inst.data, &inst.accounts, &accounts);
                    let obj: SplTokenMeta = SplTokenMeta {
                        block_date: convert_to_date(timestamp),
                        block_time: timestamp,
                        tx_id: bs58::encode(&transaction.signatures[0]).into_string(),
                        dapp: constants::PROGRAM_ADDRESS.to_string(),
                        block_slot: slot,
                        instruction_index: idx as u32,
                        is_inner_instruction: false,
                        inner_instruction_index: 0,
                        instruction_type: outer_arg.instruction_type.clone(),
                        input_accounts: outer_arg.input_accounts,
                        outer_program: program.to_string(),
                        args: outer_arg.arg,
                        signer: signer.clone(),
                        tx_index: Some(tx_index as u32),
                        fee: Some(tx_fee),
                        num_signers: Some(num_signers),
                    };

                    data.push(handle_mints_and_amounts(obj, &pre_token_balances, &post_token_balances, &accounts));
                }

                meta.inner_instructions
                    .iter()
                    .filter(|inner_instruction| inner_instruction.index == idx as u32)
                    .for_each(|inner_instruction| {
                        inner_instruction.instructions.iter().enumerate().for_each(
                            |(inner_idx, inner_inst)| {
                                let inner_program = &accounts[inner_inst.program_id_index as usize];
                                if inner_program == constants::PROGRAM_ADDRESS {
                                    let outer_arg = get_outer_arg(
                                        inner_inst.data.clone(),
                                        &inner_inst.accounts,
                                        &accounts,
                                    );

                                    let obj: SplTokenMeta = SplTokenMeta {
                                        block_date: convert_to_date(timestamp),
                                        block_time: timestamp,
                                        tx_id: bs58::encode(&transaction.signatures[0])
                                            .into_string(),
                                        dapp: constants::PROGRAM_ADDRESS.to_string(),
                                        block_slot: slot,
                                        instruction_index: idx as u32,
                                        is_inner_instruction: true,
                                        inner_instruction_index: inner_idx as u32,
                                        instruction_type: outer_arg.instruction_type.clone(),
                                        input_accounts: outer_arg.input_accounts,
                                        outer_program: program.to_string(),
                                        args: outer_arg.arg,
                                        signer: signer.clone(),
                                        tx_index: Some(tx_index as u32),
                                        fee: Some(tx_fee),
                                        num_signers: Some(num_signers),
                                    };

                                    data.push(handle_mints_and_amounts(obj, &pre_token_balances, &post_token_balances, &accounts));
                                }
                            },
                        )
                    });
            }
        }
    }

    Ok(Output { data })
}

fn handle_mints_and_amounts(
    mut obj: SplTokenMeta,
    pre_token_balances: &Vec<TokenBalance>,
    post_token_balances: &Vec<TokenBalance>,
    accounts: &Vec<String>,
) -> SplTokenMeta {
    match obj.instruction_type.as_str() {
        "Transfer" | "TransferChecked" => {
            // Search for mint and decimals from source or destination account
            let (mint, decimals) = find_mint_and_decimals(
                &obj.input_accounts.source,
                &obj.input_accounts.destination,
                pre_token_balances,
                post_token_balances,
                accounts,
            );
            obj.input_accounts.mint = mint;
            
            // Find destination owner
            if let Some(dest_owner) = find_destination_owner(
                &obj.input_accounts.destination,
                pre_token_balances,
                post_token_balances,
                accounts,
            ) {
                obj.input_accounts.destination_owner = Some(dest_owner);
            }
            
            // Calculate UI amount if we have amount and decimals
            if let (Some(amount), Some(decimals)) = (obj.args.amount, decimals) {
                obj.args.ui_amount = Some(calculate_ui_amount(amount, decimals));
            }
        }
        "Approve" | "ApproveChecked" => {
            // Search for mint and decimals from source account
            let (mint, decimals) = find_mint_and_decimals(
                &obj.input_accounts.source,
                &None,
                pre_token_balances,
                post_token_balances,
                accounts,
            );
            obj.input_accounts.mint = mint;
            
            // Calculate UI amount if we have amount and decimals
            if let (Some(amount), Some(decimals)) = (obj.args.amount, decimals) {
                obj.args.ui_amount = Some(calculate_ui_amount(amount, decimals));
            }
        }
        "MintTo" | "MintToChecked" => {
            // Search for mint and decimals from destination account (account field for MintTo)
            let (mint, decimals) = find_mint_and_decimals(
                &obj.input_accounts.account,
                &None,
                pre_token_balances,
                post_token_balances,
                accounts,
            );
            obj.input_accounts.mint = mint;
            
            // Find owner of the destination account (account receiving minted tokens)
            if let Some(account_owner) = find_destination_owner(
                &obj.input_accounts.account,
                pre_token_balances,
                post_token_balances,
                accounts,
            ) {
                obj.input_accounts.owner = Some(account_owner);
            }
            
            // Calculate UI amount if we have amount and decimals
            if let (Some(amount), Some(decimals)) = (obj.args.amount, decimals) {
                obj.args.ui_amount = Some(calculate_ui_amount(amount, decimals));
            }
        }
        "Burn" | "BurnChecked" => {
            // Search for mint and decimals from source account (account field for Burn)
            let (mint, decimals) = find_mint_and_decimals(
                &obj.input_accounts.account,
                &None,
                pre_token_balances,
                post_token_balances,
                accounts,
            );
            obj.input_accounts.mint = mint;
            
            // Calculate UI amount if we have amount and decimals
            if let (Some(amount), Some(decimals)) = (obj.args.amount, decimals) {
                obj.args.ui_amount = Some(calculate_ui_amount(amount, decimals));
            }
        }
        _ => {
            // For other instructions, no special handling needed
        }
    }
    return obj;
}

fn find_mint_and_decimals(
    primary_account: &Option<String>,
    fallback_account: &Option<String>,
    pre_token_balances: &Vec<TokenBalance>,
    post_token_balances: &Vec<TokenBalance>,
    accounts: &Vec<String>,
) -> (Option<String>, Option<u8>) {
    // Helper function to search in token balances
    let search_in_balances = |account_addr: &str, balances: &Vec<TokenBalance>| -> Option<(String, u8)> {
        if let Some(index) = accounts.iter().position(|r| r == account_addr) {
            for token_balance in balances.iter() {
                if token_balance.account_index == index as u32 {
                    // Extract decimals from ui_token_amount
                    let decimals = token_balance.ui_token_amount.as_ref()
                        .map(|ui_amount| ui_amount.decimals as u8)
                        .unwrap_or(0);
                    return Some((token_balance.mint.clone(), decimals));
                }
            }
        }
        None
    };

    // Search priority: primary in pre -> primary in post -> fallback in pre -> fallback in post
    if let Some(primary) = primary_account {
        if let Some((mint, decimals)) = search_in_balances(primary, pre_token_balances) {
            return (Some(mint), Some(decimals));
        }
        if let Some((mint, decimals)) = search_in_balances(primary, post_token_balances) {
            return (Some(mint), Some(decimals));
        }
    }

    if let Some(fallback) = fallback_account {
        if let Some((mint, decimals)) = search_in_balances(fallback, pre_token_balances) {
            return (Some(mint), Some(decimals));
        }
        if let Some((mint, decimals)) = search_in_balances(fallback, post_token_balances) {
            return (Some(mint), Some(decimals));
        }
    }

    (None, None)
}

fn find_destination_owner(
    destination_account: &Option<String>,
    pre_token_balances: &Vec<TokenBalance>,
    post_token_balances: &Vec<TokenBalance>,
    accounts: &Vec<String>,
) -> Option<String> {
    if let Some(dest_addr) = destination_account {
        if let Some(index) = accounts.iter().position(|r| r == dest_addr) {
            // Search in pre_token_balances first
            for token_balance in pre_token_balances.iter() {
                if token_balance.account_index == index as u32 {
                    return Some(token_balance.owner.clone());
                }
            }
            // Then search in post_token_balances
            for token_balance in post_token_balances.iter() {
                if token_balance.account_index == index as u32 {
                    return Some(token_balance.owner.clone());
                }
            }
        }
    }
    None
}

fn calculate_ui_amount(amount: u64, decimals: u8) -> String {
    if decimals == 0 {
        return amount.to_string();
    }
    
    let divisor = 10_u64.pow(decimals as u32);
    let integer_part = amount / divisor;
    let fractional_part = amount % divisor;
    
    if fractional_part == 0 {
        integer_part.to_string()
    } else {
        // Remove trailing zeros from fractional part
        let fractional_str = format!("{:0width$}", fractional_part, width = decimals as usize);
        let trimmed = fractional_str.trim_end_matches('0');
        if trimmed.is_empty() {
            integer_part.to_string()
        } else {
            format!("{}.{}", integer_part, trimmed)
        }
    }
}

fn get_outer_arg(
    instruction_data: Vec<u8>,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> OuterArg {
    let account_args = prepare_account_args(account_indices, accounts);
    let mut outerArg: OuterArg = OuterArg::default();
    let mut arg: Arg = Arg::default();
    let instruction: Instruction = parse_instruction(instruction_data, account_args);

    outerArg.input_accounts = Accounts {
        mint: Some(instruction.instruction_accounts.mint),
        rent_sysvar: Some(instruction.instruction_accounts.rent_sysvar),
        account: Some(instruction.instruction_accounts.account),
        owner: Some(instruction.instruction_accounts.owner),
        signer_accounts: instruction.instruction_accounts.signer_accounts,
        source: Some(instruction.instruction_accounts.source),
        destination: Some(instruction.instruction_accounts.destination),
        delegate: Some(instruction.instruction_accounts.delegate),
        authority: Some(instruction.instruction_accounts.authority),
        payer: Some(instruction.instruction_accounts.payer),
        fund_relocation_sys_program: Some(
            instruction.instruction_accounts.fund_relocation_sys_program,
        ),
        funding_account: Some(instruction.instruction_accounts.funding_account),
        mint_funding_sys_program: Some(instruction.instruction_accounts.mint_funding_sys_program),
        destination_owner: None, // Will be populated later in handle_mints_and_amounts
    };

    match instruction.name.as_str() {
        "InitializeMint" => {
            outerArg.instruction_type = String::from("InitializeMint");
            arg.decimals = Some(i32::from(instruction.initializeMintArgs.decimals));
            arg.mint_authority =
                get_b58_string(instruction.initializeMintArgs.mint_authority.value);
            arg.freeze_authority =
                get_b58_string(instruction.initializeMintArgs.freeze_authority.value);
        }
        "InitializeAccount" => {
            outerArg.instruction_type = String::from("InitializeAccount");
        }
        "InitializeMultisig" => {
            outerArg.instruction_type = String::from("InitializeMultisig");
            arg.status = Some(i32::from(instruction.initializeMultisigArgs.status));
        }
        "Transfer" => {
            outerArg.instruction_type = String::from("Transfer");
            arg.amount = Some(instruction.transferArgs.amount);
        }
        "Approve" => {
            outerArg.instruction_type = String::from("Approve");
            arg.amount = Some(instruction.approveArgs.amount);
        }
        "Revoke" => {
            outerArg.instruction_type = String::from("Revoke");
        }
        "SetAuthority" => {
            outerArg.instruction_type = String::from("SetAuthority");
            arg.authority_type = Some(instruction.setAuthorityArgs.authority_type.to_string());
            arg.new_authority_option =
                Some(i32::from(instruction.setAuthorityArgs.new_authority_option));
            arg.new_authority = get_b58_string(instruction.setAuthorityArgs.new_authority.value);
        }
        "MintTo" => {
            outerArg.instruction_type = String::from("MintTo");
            arg.amount = Some(instruction.mintToArgs.amount);
        }
        "Burn" => {
            outerArg.instruction_type = String::from("Burn");
            arg.amount = Some(instruction.burnArgs.amount);
        }
        "CloseAccount" => {
            outerArg.instruction_type = String::from("CloseAccount");
        }
        "FreezeAccount" => {
            outerArg.instruction_type = String::from("FreezeAccount");
        }
        "ThawAccount" => {
            outerArg.instruction_type = String::from("ThawAccount");
        }
        "TransferChecked" => {
            outerArg.instruction_type = String::from("TransferChecked");
            arg.amount = Some(instruction.transferCheckedArgs.amount);
            arg.decimals = Some(i32::from(instruction.transferCheckedArgs.decimals));
        }
        "ApproveChecked" => {
            outerArg.instruction_type = String::from("ApproveChecked");
            arg.amount = Some(instruction.approveCheckedArgs.amount);
            arg.decimals = Some(i32::from(instruction.approveCheckedArgs.decimals));
        }
        "MintToChecked" => {
            outerArg.instruction_type = String::from("MintToChecked");
            arg.amount = Some(instruction.mintToCheckedArgs.amount);
            arg.decimals = Some(i32::from(instruction.mintToCheckedArgs.decimals));
        }
        "BurnChecked" => {
            outerArg.instruction_type = String::from("BurnChecked");
            arg.amount = Some(instruction.burnCheckedArgs.amount);
            arg.decimals = Some(i32::from(instruction.burnCheckedArgs.decimals));
        }
        "InitializeAccount2" => {
            outerArg.instruction_type = String::from("InitializeAccount2");
            arg.owner = get_b58_string(instruction.initializeAccount2Args.owner.value);
        }
        "SyncNative" => {
            outerArg.instruction_type = String::from("SyncNative");
        }
        "InitializeAccount3" => {
            outerArg.instruction_type = String::from("InitializeAccount3");
            arg.owner = get_b58_string(instruction.initializeAccount3Args.owner.value);
        }
        "InitializeMultisig2" => {
            outerArg.instruction_type = String::from("InitializeMultisig2");
            arg.status = Some(i32::from(instruction.initializeMultisig2Args.status));
        }
        "InitializeMint2" => {
            outerArg.instruction_type = String::from("InitializeMint2");
            arg.decimals = Some(i32::from(instruction.initializeMint2Args.decimals));
            arg.mint_authority =
                get_b58_string(instruction.initializeMint2Args.mint_authority.value);
            arg.freeze_authority =
                get_b58_string(instruction.initializeMint2Args.freeze_authority.value);
        }
        "GetAccountDataSize" => {
            outerArg.instruction_type = String::from("GetAccountDataSize");
            arg.extension_type = Some(i32::from(instruction.getAccountDataSizeArgs.extension_type));
        }
        "InitializeImmutableOwner" => {
            outerArg.instruction_type = String::from("InitializeImmutableOwner");
        }
        "AmountToUiAmount" => {
            outerArg.instruction_type = String::from("AmountToUiAmount");
            arg.amount = Some(instruction.amountToUiAmountArgs.amount);
        }
        "UiAmountToAmount" => {
            outerArg.instruction_type = String::from("UiAmountToAmount");
            arg.ui_amount = Some(instruction.uiAmountToAmountArgs.ui_amount);
        }
        "InitializeMintCloseAuthority" => {
            outerArg.instruction_type = String::from("InitializeMintCloseAuthority");
            arg.owner = get_b58_string(instruction.initializeMintCloseAuthorityArgs.owner.value);
        }
        "TransferFeeExtension" => {
            outerArg.instruction_type = String::from("TransferFeeExtension");
        }
        "ConfidentialTransferExtension" => {
            outerArg.instruction_type = String::from("ConfidentialTransferExtension");
        }
        "DefaultAccountStateExtension" => {
            outerArg.instruction_type = String::from("DefaultAccountStateExtension");
        }
        "Reallocate" => {
            outerArg.instruction_type = String::from("Reallocate");
        }
        "MemoTransferExtension" => {
            outerArg.instruction_type = String::from("MemoTransferExtension");
        }
        "CreateNativeMint" => {
            outerArg.instruction_type = String::from("CreateNativeMint");
        }
        "InitializeNonTransferableMint" => {
            outerArg.instruction_type = String::from("InitializeNonTransferableMint");
        }
        "InterestBearingMintExtension" => {
            outerArg.instruction_type = String::from("InterestBearingMintExtension");
        }
        _ => {
            outerArg.instruction_type = String::from("Unknown Instruction");
        }
    }

    outerArg.arg = arg;

    return outerArg;
}

fn get_b58_string(data: [u8; 32]) -> Option<String> {
    return Some(bs58::encode(data).into_string());
}

fn prepare_account_args(account_indices: &Vec<u8>, accounts: &Vec<String>) -> Vec<String> {
    let mut instruction_accounts: Vec<String> = vec![];
    for (index, &el) in account_indices.iter().enumerate() {
        instruction_accounts.push(accounts.as_slice()[el as usize].to_string());
    }
    return instruction_accounts;
}
