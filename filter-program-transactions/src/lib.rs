mod pb;
mod programs;
mod utils;

use pb::sf::solana::transactions;
use pb::sf::solana::transactions::v1::{Output, TransactionStats};
use programs::create_programs_map;

use std::collections::HashSet;

use substreams_solana::pb::sf::solana::r#type::v1::{
    Block, CompiledInstruction, InnerInstruction, Message, MessageHeader, TokenBalance,
    Transaction, TransactionStatusMeta,
};
use utils::{convert_to_date, parse_logs, LogContext, LogContextIterator};

const VOTE_ACCOUNT: &str = "Vote111111111111111111111111111111111111111";

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Output, substreams::errors::Error> {
    let block_time = block.block_time.as_ref();
    let block_date = match block_time {
        Some(block_time) => match convert_to_date(block_time.timestamp) {
            Ok(date) => date,
            Err(_) => "Error converting block time to date".to_string(),
        },
        None => "Block time is not available".to_string(),
    };
    let block_slot = block.slot;
    let mut data = Vec::new();
    let programs_map = create_programs_map();
    let program_accounts: HashSet<_> = programs_map.keys().collect();

    let decoded_vote_account = bs58::decode(VOTE_ACCOUNT)
        .into_vec()
        .expect("Failed to decode vote account");

    for (index, trx) in block.transactions.iter().enumerate() {
        let meta = match trx.meta.as_ref() {
            Some(meta) => meta,
            None => continue,
        };

        let transaction = match trx.transaction.as_ref() {
            Some(transaction) => transaction,
            None => continue,
        };

        if meta.err.is_some() {
            continue;
        }

        let message = transaction.message.as_ref().expect("Message is missing");

        // Skip Vote Transactions
        if message.account_keys.contains(&decoded_vote_account) {
            continue;
        }

        let accounts = trx.resolved_accounts_as_strings();
        if accounts
            .iter()
            .all(|account| !program_accounts.contains(&account.as_str()))
        {
            continue;
        }

        let header = message.header.as_ref().expect("Header is missing");

        let parsed_logs = parse_logs(&meta.log_messages);

        let mut transaction_stats = TransactionStats::default();
        transaction_stats.block_slot = block_slot as u32;
        transaction_stats.block_date = block_date.to_string();
        transaction_stats.block_time = block_time.unwrap().timestamp as u64;

        populate_transaction_stats(
            &mut transaction_stats,
            &transaction,
            &accounts,
            &meta,
            &parsed_logs,
            index,
            meta.fee,
            header,
            &message,
        );

        for account in transaction_stats.executing_accounts.iter() {
            if let Some(&program_name) = programs_map.get(account.as_str()) {
                let mut updated_transaction = transaction_stats.clone(); // Clone the original transaction
                updated_transaction.program = program_name.to_string(); // Update the program
                data.push(updated_transaction); // Add to the vector
            }
        }
    }

    Ok(Output { data })
}

fn populate_transaction_stats(
    transaction_stats: &mut TransactionStats,
    transaction: &Transaction,
    accounts: &Vec<String>,
    meta: &TransactionStatusMeta,
    parsed_logs: &Vec<LogContext>,
    index: usize,
    fees: u64,
    header: &MessageHeader,
    message: &Message,
) {
    let num_required_signatures = header.num_required_signatures;

    transaction_stats.required_signatures = num_required_signatures;
    transaction_stats.readonly_signed_accounts = header.num_readonly_signed_accounts;
    transaction_stats.readonly_unsigned_accounts = header.num_readonly_unsigned_accounts;

    transaction_stats.index = index as u32;
    transaction_stats.id = bs58::encode(&transaction.signatures[0]).into_string();
    transaction_stats.fees = fees;
    transaction_stats.pre_balances = meta.pre_balances.clone();
    transaction_stats.post_balances = meta.post_balances.clone();
    transaction_stats.signatures = transaction
        .signatures
        .iter()
        .map(|sig| bs58::encode(sig).into_string())
        .collect();
    transaction_stats.signer = accounts[0].clone();
    transaction_stats.version = if message.versioned {
        "0".into()
    } else {
        "legacy".into()
    };
    transaction_stats.logs_truncated = contains_substring(&meta.log_messages, "Log truncated");
    transaction_stats.log_messages = meta.log_messages.clone();
    transaction_stats.account_keys = accounts.clone();

    update_transaction_stats_instructions(transaction_stats, accounts, meta, message, parsed_logs);
    update_transaction_stats_token_balances(transaction_stats, meta, accounts);
    update_transaction_stats_executing_accounts(transaction_stats);
}

fn get_unique_program_ids(instructions: &Vec<transactions::v1::Instruction>) -> HashSet<String> {
    let mut unique_ids = HashSet::new();
    for instruction in instructions.iter() {
        // Insert the program_id of the current LogContext
        unique_ids.insert(instruction.executing_account.clone());

        for inner_instruction in instruction.inner_instructions.iter() {
            unique_ids.insert(inner_instruction.executing_account.clone());
        }
    }

    unique_ids
}

fn process_instruction(
    instruction: &CompiledInstruction,
    accounts: &Vec<String>,
    meta: &TransactionStatusMeta,
    index: usize,
) -> transactions::v1::Instruction {
    let executing_account = &accounts[instruction.program_id_index as usize];
    let indices = byte_vector_to_indices(&instruction.accounts);
    let account_arguments = filter_accounts_by_indices(&accounts, indices);
    let data = bs58::encode(instruction.data.clone()).into_string();

    transactions::v1::Instruction {
        account_arguments,
        data,
        executing_account: executing_account.to_string(),
        inner_instructions: process_inner_instructions(index, meta, accounts),
        ..Default::default()
    }
}

// Function to update TransactionStats.instructions
fn update_transaction_stats_instructions(
    transaction_stats: &mut TransactionStats,
    accounts: &Vec<String>,
    meta: &TransactionStatusMeta,
    message: &Message,
    parsed_logs: &Vec<LogContext>,
) {
    let mut instructions = message
        .instructions
        .iter()
        .enumerate()
        .map(|(index, compiled)| process_instruction(compiled, accounts, meta, index))
        .collect();

    assign_logs_to_instructions(&mut instructions, parsed_logs);
    transaction_stats.instructions = instructions;
}

// Function to convert a byte vector representing account indices to a Vec<usize>
fn byte_vector_to_indices(byte_vec: &[u8]) -> Vec<usize> {
    byte_vec.iter().map(|&byte| byte as usize).collect()
}

// Function to filter accounts based on indices in the instruction's accounts field
fn filter_accounts_by_indices(accounts: &[String], indices: Vec<usize>) -> Vec<String> {
    indices
        .iter()
        .filter_map(|&index| accounts.get(index))
        .cloned()
        .collect()
}

fn process_inner_instructions(
    program_index: usize,
    meta: &TransactionStatusMeta,
    accounts: &Vec<String>,
) -> Vec<transactions::v1::InnerInstruction> {
    meta.inner_instructions
        .iter()
        .filter_map(|inner_inst| {
            if inner_inst.index as usize == program_index {
                Some(
                    inner_inst
                        .instructions
                        .iter()
                        .map(|inst| process_inner_instruction(inst, accounts)),
                )
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

fn process_inner_instruction(
    inner_inst: &InnerInstruction,
    accounts: &Vec<String>,
) -> transactions::v1::InnerInstruction {
    let executing_account = &accounts[inner_inst.program_id_index as usize];
    let data = bs58::encode(inner_inst.data.clone()).into_string();
    let indices = byte_vector_to_indices(&inner_inst.accounts);
    let account_arguments = filter_accounts_by_indices(&accounts, indices);

    return transactions::v1::InnerInstruction {
        account_arguments,
        data,
        executing_account: executing_account.to_string(),
        ..Default::default()
    };
}

fn assign_logs_to_instructions(
    instructions: &mut Vec<transactions::v1::Instruction>,
    log_contexts: &Vec<LogContext>,
) {
    let mut iterator = LogContextIterator::new(log_contexts);

    for instruction in instructions.iter_mut() {
        if let Some(log_context) = iterator.next() {
            if log_context.program_id == instruction.executing_account {
                instruction.program_logs = log_context.program_logs.clone();
                instruction.program_data = log_context.program_data.clone();
            }

            for inner_instruction in instruction.inner_instructions.iter_mut() {
                if let Some(inner_log_context) = iterator.next() {
                    if inner_log_context.program_id == inner_instruction.executing_account {
                        inner_instruction.program_logs = inner_log_context.program_logs.clone();
                        inner_instruction.program_data = inner_log_context.program_data.clone();
                    }
                }
            }
        }
    }
}

fn update_transaction_stats_token_balances(
    transaction_stats: &mut TransactionStats,
    meta: &TransactionStatusMeta,
    accounts: &[String],
) {
    for pre_token_balance in &meta.pre_token_balances {
        match process_token_balance(pre_token_balance, accounts) {
            Ok(balance) => transaction_stats.pre_token_balances.push(balance),
            Err(e) => eprintln!("Error processing pre-token balance: {}", e), // Error handling
        }
    }

    for post_token_balance in &meta.post_token_balances {
        match process_token_balance(post_token_balance, accounts) {
            Ok(balance) => transaction_stats.post_token_balances.push(balance),
            Err(e) => eprintln!("Error processing post-token balance: {}", e), // Error handling
        }
    }
}

fn process_token_balance(
    token_balance: &TokenBalance,
    accounts: &[String],
) -> Result<transactions::v1::TokenBalance, &'static str> {
    let account = match accounts.get(token_balance.account_index as usize) {
        Some(account) => account,
        None => return Err("Account index out of bounds"),
    };

    let amount = token_balance
        .ui_token_amount
        .as_ref()
        .map_or(0.0, |amount| amount.ui_amount as f64);

    Ok(transactions::v1::TokenBalance {
        account: account.to_string(),
        amount,
        mint: token_balance.mint.to_string(),
        owner: token_balance.owner.to_string(),
        program: token_balance.program_id.to_string(),
    })
}

fn update_transaction_stats_executing_accounts(transaction_stats: &mut TransactionStats) {
    transaction_stats.executing_accounts =
        Vec::from_iter(get_unique_program_ids(&transaction_stats.instructions));
}

fn contains_substring(log_messages: &Vec<String>, sub_str: &str) -> bool {
    let sub_str_lower = sub_str.to_lowercase();
    log_messages
        .iter()
        .any(|message| message.to_lowercase().contains(&sub_str_lower))
}
