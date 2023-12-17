mod pb;
mod utils;

use pb::sf::solana::transactions;
use pb::sf::solana::transactions::v1::{Error, Output, TransactionStats};
use serde_json;
use std::collections::HashSet;
use substreams::log;
use substreams_solana::pb::sf::solana::r#type::v1::{
    Block, CompiledInstruction, InnerInstruction, Message, Transaction,
    TransactionStatusMeta,
};
use utils::{
    calculate_byte_size, calculate_instruction_size, compact_array_size, convert_to_date,
    parse_logs, LogContext, LogContextIterator,
};

const VOTE_ACCOUNT: &str = "Vote111111111111111111111111111111111111111";

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Output, substreams::errors::Error> {
    let block_date = convert_to_date(block.block_time.as_ref().unwrap().timestamp);
    let block_slot = block.slot;
    let mut data = Vec::new();

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

        let message = transaction.message.as_ref().expect("Message is missing");
        
        // Skip Vote Transactions
        if message.account_keys.contains(&decoded_vote_account) {
            continue;
        }

        let header = message.header.as_ref().expect("Header is missing");
        let num_required_signatures = header.num_required_signatures;
        let accounts = trx.resolved_accounts_as_strings();
        let parsed_logs = parse_logs(&meta.log_messages);

        let mut transaction_stats = TransactionStats::default();

        populate_transaction_stats(
            &mut transaction_stats,
            &transaction,
            &accounts,
            &meta,
            &parsed_logs,
            block_slot,
            &block_date,
            block.block_time.as_ref().unwrap().timestamp,
            index,
            meta.fee,
            num_required_signatures,
            &message,
        );

        data.push(transaction_stats);
    }

    Ok(Output { data })
}

fn populate_transaction_stats(
    transaction_stats: &mut TransactionStats,
    transaction: &Transaction,
    accounts: &Vec<String>,
    meta: &TransactionStatusMeta,
    parsed_logs: &Vec<LogContext>,
    block_slot: u64,
    block_date: &String,
    block_time: i64,
    index: usize,
    fees: u64,
    num_required_signatures: u32,
    message: &Message,
) {
    transaction_stats.block_slot = block_slot;
    transaction_stats.block_date = block_date.to_string();
    transaction_stats.block_time = block_time;
    transaction_stats.index = index as u32;
    transaction_stats.error = find_error(parsed_logs);
    transaction_stats.id = bs58::encode(&transaction.signatures[0]).into_string();
    transaction_stats.success = meta.err.is_none();
    transaction_stats.account_keys = accounts.clone();
    transaction_stats.log_messages = meta.log_messages.clone();
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
    transaction_stats.executing_accounts = Vec::from_iter(get_unique_program_ids(parsed_logs));
    transaction_stats.fee = fees;
    transaction_stats.base_fee = 5000 * num_required_signatures as u32;
    transaction_stats.priority_fee = fees.saturating_sub(transaction_stats.base_fee.into()) as u32;
    transaction_stats.byte_size = calculate_byte_size(transaction) as u32;
    transaction_stats.trx_accounts_size = message.account_keys.len() as u32;
    transaction_stats.readable_alt_accounts_size = meta.loaded_readonly_addresses.len() as u32;
    transaction_stats.writeable_alt_accounts_size = meta.loaded_writable_addresses.len() as u32;
    update_transaction_stats_compute_units(transaction_stats, parsed_logs);
    update_transaction_stats_instructions(transaction_stats, accounts, meta, message, parsed_logs);
}

fn get_unique_program_ids(log_contexts: &[LogContext]) -> HashSet<String> {
    let mut unique_ids = HashSet::new();
    for log_context in log_contexts {
        // Insert the program_id of the current LogContext
        unique_ids.insert(log_context.program_id.clone());

        // Recursively process child nodes
        let child_ids = get_unique_program_ids(&log_context.children_nodes);
        unique_ids.extend(child_ids);
    }

    unique_ids
}

fn find_error(log_contexts: &[LogContext]) -> Option<Error> {
    for log_context in log_contexts {
        if let Some(ref message) = log_context.failure_message {
            return Some(Error {
                program: log_context.program_id.clone(),
                message: message.clone(),
            });
        }

        // Recursively check child nodes
        if let Some(found) = find_error(&log_context.children_nodes) {
            return Some(found);
        }
    }
    None
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
        bytes: calculate_instruction_size(instruction) as u32,
        account_bytes: compact_array_size(instruction.accounts.len(), 1) as u32,
        data_bytes: compact_array_size(instruction.data.len(), 1) as u32,
        ..Default::default()
    }
}

// Function to update TransactionStats.instructions
fn update_transaction_stats_instructions(
    transaction_stats: &mut TransactionStats,
    accounts: &Vec<String>,
    meta: &TransactionStatusMeta,
    message: &Message,
    parsed_logs: &Vec<LogContext>
) {

    let mut instructions = message
    .instructions
    .iter()
    .enumerate()
    .map(|(index, compiled)| process_instruction(compiled, accounts, meta, index))
    .collect();

    assign_logs_to_instructions_dfs(&mut instructions, parsed_logs);
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

fn update_transaction_stats_compute_units(
    transaction_stats: &mut TransactionStats,
    parsed_logs: &Vec<LogContext>,
) {
    for log_context in parsed_logs {
        if log_context.depth == 1 {
            transaction_stats.compute_units_allocated += log_context.compute_units as u64;
            transaction_stats.compute_units_consumed += log_context.consumed_units as u64;
        }
    }
}


fn assign_logs_to_instructions_dfs(
    instructions: &mut Vec<transactions::v1::Instruction>,
    log_contexts: &Vec<LogContext>,
) {
    let mut iterator = LogContextIterator::new(log_contexts);
    
    for instruction in instructions.iter_mut() {
        if let Some(log_context) = iterator.next() {
          
            if log_context.program_id == instruction.executing_account {
                instruction.program_logs = log_context.program_logs.clone();
            }

            for inner_instruction in instruction.inner_instructions.iter_mut() {
                if let Some(inner_log_context) = iterator.next() {
                   
                    if inner_log_context.program_id == inner_instruction.executing_account {
                        inner_instruction.program_logs = inner_log_context.program_logs.clone();
                    }
                }
            }
        }
    }
}