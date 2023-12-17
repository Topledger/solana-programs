mod pb;
mod utils;

use pb::sf::solana::program_analysis::v1::Output;
use pb::sf::solana::program_analysis::v1::ProgramStats;
use std::collections::HashMap;
use std::collections::HashSet;
use substreams::log;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;

use substreams_solana::pb::sf::solana::r#type::v1::TransactionStatusMeta;
use utils::convert_to_date;
use utils::parse_logs;
use utils::LogContext;

const VOTE_ACCOUNT: &str = "Vote111111111111111111111111111111111111111";

#[derive(Debug, Clone)]
struct ProgramData {
    signers: HashSet<String>,
    fee_payers: HashSet<String>,
    fee_lamports: u32,
    base_fee_lamports: u32,
    priority_fee_lamports: u32,
    compute_units_consumed: u32,
    compute_units_allocated: u32,
    successful_txns_count: u32,
    failed_txns_count: u32,
    outer_invocation_count: u32,
    inner_invocation_count: u32,
    failed_invocation_count: u32,
    errors: HashMap<String, u32>,
}

impl ProgramData {
    fn new() -> Self {
        ProgramData {
            signers: HashSet::new(),
            fee_payers: HashSet::new(),
            fee_lamports: 0,
            base_fee_lamports: 0,
            priority_fee_lamports: 0,
            compute_units_consumed: 0,
            compute_units_allocated: 0,
            successful_txns_count: 0,
            failed_txns_count: 0,
            outer_invocation_count: 0,
            inner_invocation_count: 0,
            failed_invocation_count: 0,
            errors: HashMap::new(),
        }
    }

    fn update_with_log(&mut self, log: &LogContext) {
        self.compute_units_consumed += log.consumed_units as u32;
        self.compute_units_allocated += log.compute_units as u32;

        if let Some(ref error) = log.failure_message {
            self.failed_invocation_count += 1;
            *self.errors.entry(error.clone()).or_insert(0) += 1;
        }
    }

    fn new_from_log(log: &LogContext) -> Self {
        let mut new_data = ProgramData::new();

        new_data.compute_units_consumed = log.consumed_units as u32;
        new_data.compute_units_allocated = log.compute_units as u32;

        new_data.failed_invocation_count = if log.failure_message.is_some() { 1 } else { 0 };

        if let Some(ref error) = log.failure_message {
            new_data.errors.insert(error.clone(), 1);
        }

        new_data
    }

    fn update_fees_and_counts(
        &mut self,
        accounts: &[String],
        num_required_signatures: u32,
        fees: u64,
        meta: &TransactionStatusMeta,
    ) {
        self.fee_payers.insert(accounts[0].clone());

        if let Some(signers) = accounts.get(0..num_required_signatures as usize) {
            self.signers.extend(signers.iter().cloned());
        }

        self.fee_lamports = fees as u32;
        self.base_fee_lamports = 5000 * num_required_signatures;
        self.priority_fee_lamports =
            fees.saturating_sub(5000 * num_required_signatures as u64) as u32;
        self.successful_txns_count = if meta.err.is_none() { 1 } else { 0 };
        self.failed_txns_count = if meta.err.is_some() { 1 } else { 0 };
    }
}

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Output, substreams::errors::Error> {
    let block_date = match block.block_time.as_ref() {
        Some(block_time) => match convert_to_date(block_time.timestamp) {
            Ok(date) => date,
            Err(_) => "Error converting block time to date".to_string(),
        },
        None => "Block time is not available".to_string(),
    };

    let decoded_vote_account = bs58::decode(VOTE_ACCOUNT)
        .into_vec()
        .expect("Failed to decode vote account");

    let mut program_data_map: Vec<HashMap<String, ProgramData>> = vec![];

    for trx in block.transactions {
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

        let (accounts, num_required_signatures) = extract_transaction_info(&trx);

        let mut trx_programs: HashMap<String, ProgramData> = HashMap::new();


        // update invocation counts only for successful transactions
        if meta.err.is_none() {
            let update_invocation_count = |program_data: &mut ProgramData, is_outer: bool| {
                if is_outer {
                    program_data.outer_invocation_count += 1;
                } else {
                    program_data.inner_invocation_count += 1;
                }
            };

            for instruction in &message.instructions {
                if let Some(program) = accounts.get(instruction.program_id_index as usize) {
                    trx_programs
                        .entry(program.to_string())
                        .and_modify(|data| update_invocation_count(data, true))
                        .or_insert_with(|| {
                            let mut data = ProgramData::new();
                            update_invocation_count(&mut data, true);
                            data
                        });
                }
            }

            for inner_instruction in &meta.inner_instructions {
                for inner_inst in &inner_instruction.instructions {
                    if let Some(inner_program) = accounts.get(inner_inst.program_id_index as usize)
                    {
                        trx_programs
                            .entry(inner_program.to_string())
                            .and_modify(|data| update_invocation_count(data, false))
                            .or_insert_with(|| {
                                let mut data = ProgramData::new();
                                update_invocation_count(&mut data, false);
                                data
                            });
                    }
                }
            }
        }

        for log in &parse_logs(&meta.log_messages) {
            process_log(log, &mut trx_programs);
        }

        for data in trx_programs.values_mut() {
            data.update_fees_and_counts(&accounts, num_required_signatures, meta.fee, &meta);
        }

        program_data_map.push(trx_programs);
    }
    // print_program_data_map(&program_data_map);

    let data = convert_to_output(&program_data_map, &block_date);
    Ok(Output { data })
}

fn extract_transaction_info(trx: &ConfirmedTransaction) -> (Vec<String>, u32) {
    let accounts = trx.resolved_accounts_as_strings();
    let num_required_signatures = trx
        .transaction
        .clone()
        .unwrap()
        .message
        .unwrap()
        .header
        .unwrap()
        .num_required_signatures;
    (accounts, num_required_signatures)
}

fn process_log(log: &LogContext, program_data_map: &mut HashMap<String, ProgramData>) {
    let program_id = &log.program_id;
    program_data_map
        .entry(program_id.clone())
        .and_modify(|data| data.update_with_log(log))
        .or_insert_with(|| ProgramData::new_from_log(log));

    for child in &log.children_nodes {
        process_log(child, program_data_map);
    }
}

fn convert_to_output(
    program_data_maps: &Vec<HashMap<String, ProgramData>>,
    block_date: &str,
) -> Vec<ProgramStats> {
    let mut aggregated_data: HashMap<String, ProgramStats> = HashMap::new();

    for trx_programs in program_data_maps {
        for (program_id, program_data) in trx_programs {
            aggregated_data
                .entry(program_id.clone())
                .and_modify(|existing_stats| {
                    // Efficiently merge signers while maintaining uniqueness
                    let mut unique_signers: HashSet<String> =
                        existing_stats.signers.iter().cloned().collect();
                    unique_signers.extend(program_data.signers.iter().cloned());
                    existing_stats.signers = unique_signers.into_iter().collect();

                    // Efficiently merge fee payers while maintaining uniqueness
                    let mut unique_fee_payers: HashSet<String> =
                        existing_stats.fee_payers.iter().cloned().collect();
                    unique_fee_payers.extend(program_data.fee_payers.iter().cloned());
                    existing_stats.fee_payers = unique_fee_payers.into_iter().collect();

                    // Aggregate other numeric fields
                    existing_stats.fee_lamports += program_data.fee_lamports;
                    existing_stats.base_fee_lamports += program_data.base_fee_lamports;
                    existing_stats.priority_fee_lamports += program_data.priority_fee_lamports;
                    existing_stats.compute_units_consumed += program_data.compute_units_consumed;
                    existing_stats.compute_units_allocated += program_data.compute_units_allocated;
                    existing_stats.successful_txns_count += program_data.successful_txns_count;
                    existing_stats.failed_invocation_count += program_data.failed_invocation_count;
                    existing_stats.failed_txns_count += program_data.failed_txns_count;
                    existing_stats.total_outer_invocation_count +=
                        program_data.outer_invocation_count;
                    existing_stats.total_inner_invocation_count +=
                        program_data.inner_invocation_count;

                    // Merge error counts
                    for (error, count) in &program_data.errors {
                        *existing_stats.errors.entry(error.clone()).or_insert(0) += count;
                    }
                })
                .or_insert_with(|| ProgramStats {
                    block_date: block_date.to_string(),
                    program_id: program_id.clone(),
                    signers: program_data.signers.iter().cloned().collect(),
                    fee_payers: program_data.fee_payers.iter().cloned().collect(),

                    // ... initialize other fields from program_data ...
                    fee_lamports: program_data.fee_lamports,
                    base_fee_lamports: program_data.base_fee_lamports,
                    priority_fee_lamports: program_data.priority_fee_lamports,
                    compute_units_consumed: program_data.compute_units_consumed,
                    compute_units_allocated: program_data.compute_units_allocated,
                    successful_txns_count: program_data.successful_txns_count,
                    failed_txns_count: program_data.failed_txns_count,
                    total_outer_invocation_count: program_data.outer_invocation_count,
                    total_inner_invocation_count: program_data.inner_invocation_count,
                    failed_invocation_count: program_data.failed_invocation_count,

                    //handle errors
                    errors: program_data.errors.clone(),
                });
        }
    }

    // Convert the aggregated HashMap into a Vec<ProgramStats>
    aggregated_data
        .into_iter()
        .map(|(_id, stats)| stats)
        .collect()
}
