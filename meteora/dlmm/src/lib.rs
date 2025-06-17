mod instructions;
mod prepare_input_accounts;

pub mod pb;

use crate::pb::sf::solana::meteora_dlmm::v1::{Output, Meta};
use substreams_solana::pb::sf::solana::r#type::v1::{Block, CompiledInstruction};
use substreams::log;
use bs58;
use crate::instructions::process_instruction;

pub const METEORA_DLMM_PROGRAM_ID: &str = "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo";
// Discriminator for FundReward instruction (first 8 bytes)
pub const FUND_REWARD_DISCRIMINATOR: &[u8] = &[246, 228, 58, 130, 145, 170, 79, 204];

#[substreams::handlers::map]
fn map_block(
    block: Block,
) -> Result<Output, substreams::errors::Error> {
    let mut processed_instructions: Vec<Meta> = Vec::new();
    let block_slot = block.slot;
    let block_time = block.block_time.as_ref().map_or(0, |t| t.timestamp);
    
    log::info!("Processing block: {} at timestamp: {}", block_slot, block_time);
    
    for transaction in block.transactions_owned() {
        // Skip if transaction fails - check meta.err instead of meta.status
        if transaction.meta.as_ref().map_or(false, |m| m.err.is_some()) {
            continue;
        }
        
        // Get transaction
        let tx = match &transaction.transaction {
            Some(tx) => tx,
            None => continue,
        };
        
        // Get transaction message
        let message = match &tx.message {
            Some(msg) => msg,
            None => continue,
        };
        
        // Extract the first signature as transaction ID
        let tx_id = if !tx.signatures.is_empty() {
            bs58::encode(&tx.signatures[0]).into_string()
        } else {
            continue; // Skip transactions without signatures
        };
        
        let target_tx_id = "5yeyRkoLnshKGqPDoAmV71tEPwLThtfUSwYc5vFMaRMdjWo3upv3ErDtdYShQb9AbAWWKBxsh6qcF3KWtPBDqDyf";
        let is_target_tx = tx_id == target_tx_id;
        
        if is_target_tx {
            log::info!("TARGET TX [{}] FOUND.", target_tx_id);
        }
        
        // Get resolved accounts
        let accounts = transaction.resolved_accounts();
        let account_keys: Vec<String> = accounts.iter()
            .map(|acc| bs58::encode(acc).into_string())
            .collect();
        
        // Calculate signer (first account key in the message)
        let signer_opt: Option<String> = message.account_keys.first()
            .map(|pubkey| bs58::encode(pubkey).into_string());
        
        // Process outer instructions
        for (ix_index, ix) in message.instructions.iter().enumerate() {
            let program_id_index = ix.program_id_index as usize;
            if program_id_index >= account_keys.len() {
                log::info!("[{}] Outer Ix [{}] program_id_index {} out of bounds", tx_id, ix_index, program_id_index);
                continue;
            }
            let program_id = &account_keys[program_id_index];
            
            // Log ALL outer instruction details if it's the target transaction
            if is_target_tx {
                log::info!("  TARGET TX [{}]: Outer Ix [{}]: Program={}, Data={}",
                         target_tx_id, ix_index, program_id, hex::encode(&ix.data));
            }

            // --- REMOVED PROGRAM ID CHECK FOR OUTER --- 
            // Always process outer instructions regardless of program ID for now, 
            // filtering happens in process_instruction
            if let Some(meta) = instructions::process_instruction(
                ix,                     // Outer instruction
                &account_keys,
                block_slot,
                block_time,
                &tx_id,
                ix_index as u32,        // Pass outer index as outer_instruction_index
                false,                  // is_inner_instruction
                None,                   // actual_inner_index is None for outer
                signer_opt.as_deref(),
                None,                   // outer_program is None for outer instructions
            ) {
                processed_instructions.push(meta);
            }
        }
        
        // Process inner instructions
        if let Some(meta) = &transaction.meta {
            for inner_ins in &meta.inner_instructions {
                let outer_idx = inner_ins.index as usize;
                
                // Find the outer instruction corresponding to this inner instruction block
                let outer_instruction_opt = message.instructions.get(outer_idx);
                let outer_program_id_opt = outer_instruction_opt.and_then(|outer_ix| {
                    let outer_program_idx = outer_ix.program_id_index as usize;
                    account_keys.get(outer_program_idx).map(|s| s.as_str())
                });

                
                for (idx, ins) in inner_ins.instructions.iter().enumerate() {
                    // Get program ID from account keys using program_id_index
                    let program_id_index = ins.program_id_index as usize;
                    if program_id_index >= account_keys.len() {
                        continue;
                    }
                    
                    let program_id = &account_keys[program_id_index];

                    // Log inner instruction details if it's the target transaction
                    if is_target_tx {
                        log::info!("    TARGET TX [{}]: Inner Ix [{}][{}]: Program={}, Data={}",
                                 target_tx_id, outer_idx, idx, program_id, hex::encode(&ins.data));
                    }

                    // --- REMOVED PROGRAM ID CHECK FOR INNER --- 
                    // Create CompiledInstruction from InnerInstruction
                    let compiled_inst = CompiledInstruction {
                        program_id_index: ins.program_id_index,
                        accounts: ins.accounts.clone(),
                        data: ins.data.clone(),
                    };
                    
                    // Process Meteora DLMM instruction or EventLog
                    if let Some(meta) = instructions::process_instruction(
                        &compiled_inst,         // Inner instruction
                        &account_keys,
                        block_slot,
                        block_time,
                        &tx_id,
                        outer_idx as u32,       // Pass outer index as outer_instruction_index
                        true,                   // is_inner_instruction
                        Some(idx as u32),       // Pass inner index as actual_inner_index
                        signer_opt.as_deref(),
                        outer_program_id_opt,   // Pass the outer program ID
                    ) {
                        processed_instructions.push(meta);
                    }
                }
            }
        }
    }
    
    log::info!("BLOCK SUMMARY: Processed {} Meteora DLMM instructions in block {}", 
             processed_instructions.len(), block_slot);
    
    Ok(Output { data: processed_instructions })
}

