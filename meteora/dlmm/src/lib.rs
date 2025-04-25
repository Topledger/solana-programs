mod instructions;
mod prepare_input_accounts;

pub mod pb;
use pb::sf::solana::meteora_dlmm::v1::{Instructions, Meta};
use substreams_solana::pb::sf::solana::r#type::v1::{Block, CompiledInstruction};
use substreams::log;
use bs58;

const METEORA_DLMM_PROGRAM_ID: &str = "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo";

#[substreams::handlers::map]
fn map_block(
    block: Block,
) -> Result<Instructions, substreams::errors::Error> {
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
            // Skip if not a Meteora DLMM instruction
            let program_id_index = ix.program_id_index as usize;
            if program_id_index >= account_keys.len() {
                continue;
            }
            
            let program_id = &account_keys[program_id_index];
            if program_id != METEORA_DLMM_PROGRAM_ID {
                continue;
            }
            
            // Process Meteora DLMM instruction
            if let Some(meta) = instructions::process_instruction(
                ix,
                &account_keys,
                block_slot,
                block_time,
                &tx_id,
                ix_index as u32,
                false, // is_inner_instruction
                None,  // inner_instruction_index
                signer_opt.as_deref(),
                None,  // outer_program is None for outer instructions
            ) {
                processed_instructions.push(meta);
            }
        }
        
        // Process inner instructions
        if let Some(meta) = &transaction.meta {
            for inner_ix_group in &meta.inner_instructions {
                let outer_instruction_index = inner_ix_group.index as usize;
                if outer_instruction_index >= message.instructions.len() {
                    continue;
                }
                
                // Calculate outer_program ID for this set of inner instructions
                let outer_program_opt: Option<String> = message.instructions.get(outer_instruction_index)
                    .and_then(|parent_inst| account_keys.get(parent_inst.program_id_index as usize).cloned());
                
                for (inner_ix_index, inner_ix) in inner_ix_group.instructions.iter().enumerate() {
                    // Skip if not a Meteora DLMM inner instruction
                    let program_id_index = inner_ix.program_id_index as usize;
                    if program_id_index >= account_keys.len() {
                        continue;
                    }
                    
                    let program_id = &account_keys[program_id_index];
                    if program_id != METEORA_DLMM_PROGRAM_ID {
                        continue;
                    }
                    
                    // Convert InnerInstruction to CompiledInstruction
                    let compiled_inner_inst = CompiledInstruction {
                        program_id_index: inner_ix.program_id_index,
                        accounts: inner_ix.accounts.clone(),
                        data: inner_ix.data.clone(),
                    };
                    
                    // Process Meteora DLMM inner instruction
                    if let Some(meta) = instructions::process_instruction(
                        &compiled_inner_inst,
                        &account_keys,
                        block_slot,
                        block_time,
                        &tx_id,
                        outer_instruction_index as u32,
                        true, // is_inner_instruction
                        Some(inner_ix_index as u32), // inner_instruction_index
                        signer_opt.as_deref(),
                        outer_program_opt.as_deref(),
                    ) {
                        processed_instructions.push(meta);
                    }
                }
            }
        }
    }
    
    log::info!("BLOCK SUMMARY: Processed {} Meteora DLMM instructions in block {}", 
             processed_instructions.len(), block_slot);
    
    Ok(Instructions { instructions: processed_instructions })
}
