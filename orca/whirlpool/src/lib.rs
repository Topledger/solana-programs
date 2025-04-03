use substreams_solana::pb::sf::solana::r#type::v1::{Block, CompiledInstruction};
use substreams::errors::Error;
use bs58;

use crate::pb::sf::solana::orca_whirlpool::v1::{Output};

// Import the generated protobuf messages
pub mod pb;

// Import instruction processing module
pub mod instructions;

// Add prepare_arg and prepare_input_accounts modules
pub mod prepare_input_accounts;

// Orca Whirlpool Program ID
const ORCA_WHIRLPOOL_PROGRAM_ID: &str = "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc";

#[substreams::handlers::map]
pub fn map_block(block: Block) -> Result<Output, Error> {
    let block_slot = block.slot;
    let block_time = block.block_time.as_ref().unwrap().timestamp;

    let mut metadata_updates = Vec::new();

    // Filter transactions to exclude votes (similar to how dex-trades does it)
    for trx in block.transactions_owned() {
        // Extract transaction information
        let transaction = match &trx.transaction {
            Some(tx) => tx,
            None => continue,
        };
        
        // Skip vote transactions - they have a specific program ID
        // Vote program ID is "Vote111111111111111111111111111111111111111"
        let message = match &transaction.message {
            Some(msg) => msg,
            None => continue,
        };
        
        // Check if any of the instructions in this transaction is calling the vote program
        let is_vote_transaction = message.instructions.iter().any(|inst| {
            if (inst.program_id_index as usize) < message.account_keys.len() {
                let program_id_bytes = &message.account_keys[inst.program_id_index as usize];
                // Convert bytes to base58 for comparison
                let program_id = bs58::encode(program_id_bytes).into_string();
                program_id == "Vote111111111111111111111111111111111111111"
            } else {
                false
            }
        });
        
        // Skip vote transactions
        if is_vote_transaction {
            continue;
        }
        
        // Extract the first signature as transaction ID
        let tx_id = if !transaction.signatures.is_empty() {
            bs58::encode(&transaction.signatures[0]).into_string()
        } else {
            continue; // Skip transactions without signatures
        };

        // Get meta data
        let meta = match &trx.meta {
            Some(m) => m,
            None => continue,
        };
        
        // Get resolved accounts - these are Vec<Vec<u8>> so we need to encode each one to Base58
        let accounts = trx.resolved_accounts();
        let account_keys: Vec<String> = accounts.iter()
            .map(|acc| bs58::encode(acc).into_string())
            .collect();

        // Top-level instructions
        for (inst_idx, inst) in message.instructions.iter().enumerate() {
            // Skip instructions with invalid program ID index
            if (inst.program_id_index as usize) >= account_keys.len() {
                continue;
            }

            let program_id = &account_keys[inst.program_id_index as usize];

            if program_id == ORCA_WHIRLPOOL_PROGRAM_ID {
                if let Some(update) = instructions::process_instruction(
                    inst,
                    &account_keys,
                    block_slot,
                    block_time,
                    &tx_id,
                    inst_idx as u32,
                    false,
                    None,
                ) {
                    metadata_updates.push(update);
                }
            }
        }

        // Inner instructions - meta.inner_instructions is directly a Vec, not an Option<Vec>
        for inner_insts in &meta.inner_instructions {
            let outer_instruction_index = inner_insts.index as usize;

            for (inner_idx, inner_inst) in inner_insts.instructions.iter().enumerate() {
                // Skip instructions with invalid program ID index
                if (inner_inst.program_id_index as usize) >= account_keys.len() {
                    continue;
                }

                let inner_program_id = &account_keys[inner_inst.program_id_index as usize];

                if inner_program_id == ORCA_WHIRLPOOL_PROGRAM_ID {
                    // Convert InnerInstruction to CompiledInstruction
                    let compiled_inner_inst = CompiledInstruction {
                        program_id_index: inner_inst.program_id_index,
                        accounts: inner_inst.accounts.clone(),
                        data: inner_inst.data.clone(),
                    };

                    if let Some(update) = instructions::process_instruction(
                        &compiled_inner_inst,
                        &account_keys,
                        block_slot,
                        block_time,
                        &tx_id,
                        outer_instruction_index as u32,
                        true,
                        Some(inner_idx as u32),
                    ) {
                        metadata_updates.push(update);
                    }
                }
            }
        }
    }

    Ok(Output { data: metadata_updates })
}
