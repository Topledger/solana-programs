mod pb;
mod utils;

use std::collections::HashSet;

use pb::sf::solana::transaction_stats::v1::{Instruction, Output, TransactionStats};

use substreams_solana::pb::sf::solana::r#type::v1::{
    Block, CompiledInstruction, InnerInstructions, Message, MessageHeader, Transaction,
    TransactionStatusMeta,
};
use utils::{
    calculate_byte_size, calculate_instruction_size, compact_array_size, convert_to_date,
    parse_logs, LogContext,
};

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

    let decoded_vote_account = bs58::decode(VOTE_ACCOUNT)
        .into_vec()
        .expect("Failed to decode vote account");

    for trx in block.transactions.iter() {
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

        if  meta.err.is_some() {
            continue
        }

        let header = message.header.as_ref().expect("Header is missing");
        let accounts = trx.resolved_accounts_as_strings();
        let parsed_logs = parse_logs(&meta.log_messages);
        let _num_required_signatures = header.num_required_signatures;

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
            meta.fee,
            header,
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
    fees: u64,
    header: &MessageHeader,
    message: &Message,
) {
    let num_required_signatures = header.num_required_signatures;
    transaction_stats.id = bs58::encode(&transaction.signatures[0]).into_string();
    transaction_stats.signatures_size = transaction.signatures.len() as u32;

    transaction_stats.version = if message.versioned {
        "0".into()
    } else {
        "legacy".into()
    };
    transaction_stats.fee = fees as u64;
    transaction_stats.base_fee = 5000 * num_required_signatures as u32;
    transaction_stats.priority_fee = fees.saturating_sub(transaction_stats.base_fee.into()) as u64;
    transaction_stats.byte_size = calculate_byte_size(transaction) as u32;
    transaction_stats.trx_accounts_size = message.account_keys.len() as u32;
    transaction_stats.readable_alt_accounts_size = meta.loaded_readonly_addresses.len() as u32;
    transaction_stats.writable_alt_accounts_size = meta.loaded_writable_addresses.len() as u32;
    transaction_stats.logs_truncated = contains_substring(&meta.log_messages, "Log truncated");
    transaction_stats.executing_accounts = Vec::from_iter(get_unique_program_ids(
        &message.instructions,
        &meta.inner_instructions,
        accounts,
    ));
    update_transaction_stats_compute_units(transaction_stats, parsed_logs, meta);
    update_transaction_stats_instructions(transaction_stats, accounts, meta, message, parsed_logs);
}

fn process_instruction(
    instruction: &CompiledInstruction,
    accounts: &Vec<String>,
    _meta: &TransactionStatusMeta,
    _index: usize,
) -> Instruction {
    let executing_account = &accounts[instruction.program_id_index as usize];
    Instruction {
        executing_account: executing_account.to_string(),
        bytes: calculate_instruction_size(instruction) as u32,
        account_bytes: compact_array_size(instruction.accounts.len(), 1) as u32,
        data_bytes: compact_array_size(instruction.data.len(), 1) as u32,
    }
}

// Function to update TransactionStats.instructions
fn update_transaction_stats_instructions(
    transaction_stats: &mut TransactionStats,
    accounts: &Vec<String>,
    meta: &TransactionStatusMeta,
    message: &Message,
    _parsed_logs: &Vec<LogContext>,
) {
    let instructions = message
        .instructions
        .iter()
        .enumerate()
        .map(|(index, compiled)| process_instruction(compiled, accounts, meta, index))
        .collect();

    transaction_stats.instructions = instructions;
}

fn update_transaction_stats_compute_units(
    transaction_stats: &mut TransactionStats,
    parsed_logs: &Vec<LogContext>,
    meta: &TransactionStatusMeta,
) {
    for log_context in parsed_logs {
        if log_context.depth == 1 {
            transaction_stats.compute_units_allocated += log_context.compute_units as u64;
            transaction_stats.compute_units_consumed += log_context.consumed_units as u64;
        }
    }

    if let Some(compute_units) = meta.compute_units_consumed {
        transaction_stats.compute_units_consumed = compute_units;
    }
}

fn contains_substring(log_messages: &Vec<String>, sub_str: &str) -> bool {
    let sub_str_lower = sub_str.to_lowercase();
    log_messages
        .iter()
        .any(|message| message.to_lowercase().contains(&sub_str_lower))
}

fn get_unique_program_ids(
    instructions: &Vec<CompiledInstruction>,
    inner_instructions: &Vec<InnerInstructions>,
    accounts: &Vec<String>,
) -> HashSet<String> {
    let mut unique_ids = HashSet::new();
    for instruction in instructions.iter() {
        let executing_account = &accounts[instruction.program_id_index as usize];
        unique_ids.insert(executing_account.to_string());
    }
    for inner_instruction in inner_instructions.iter() {
        for inner_inst in inner_instruction.instructions.iter() {
            let executing_account = &accounts[inner_inst.program_id_index as usize];
            unique_ids.insert(executing_account.to_string());
        }
    }

    unique_ids
}
