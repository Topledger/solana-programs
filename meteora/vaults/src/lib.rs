#![allow(deprecated)] // Allow deprecated fields if needed

pub mod instructions; // Instruction processing logic
// pub mod prepare_input_accounts; // REMOVED: Account mapping logic handled by IDL
mod pb; // Declare the protobuf module (generated into src/pb)

// Use generated Protobuf types via the module
use pb::vault_instructions::VaultInstructions;
use pb::meta::Meta;

// Other necessary imports
use substreams_solana::pb::sf::solana::r#type::v1::{Block, ConfirmedTransaction, TransactionStatusMeta, InnerInstructions, CompiledInstruction, InnerInstruction};
use substreams::errors::Error;
use substreams::log;
use std::collections::HashSet;
use bs58;

// Program ID constant (ensure it's correct)
const METEORA_VAULTS_PROGRAM_ID: &str = "24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi";

// Discriminator for FundReward instruction (first 8 bytes)
pub const FUND_REWARD_DISCRIMINATOR: &[u8] = &[246, 228, 58, 130, 145, 170, 79, 204];

use anyhow::Result;
use borsh::BorshDeserialize;
use bs58::decode;
use anchor_lang::prelude::Pubkey;
use std::str::FromStr;

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<VaultInstructions, Error> {
    let mut meta_list = Vec::new();
    let block_slot = block.slot;
    let block_time = match block.block_time.as_ref() {
        Some(time) => time.timestamp,
        None => {
             log::error!("Block {} has no block time", block_slot);
             return Err(Error::msg(format!("Block {} has no block time", block_slot)));
        }
    };

    for (trx_idx, trx) in block.transactions.iter().enumerate() {
        let tx_id = if !trx.transaction.as_ref().unwrap().signatures.is_empty() {
            bs58::encode(&trx.transaction.as_ref().unwrap().signatures[0]).into_string()
        } else {
            format!("unknown_tx_id_slot_{}_index_{}", block_slot, trx_idx)
        };

        let tx_meta = match trx.meta.as_ref() {
            Some(meta) => meta,
            None => {
                log::debug!("Transaction {} has no meta", tx_id);
                continue; // Skip transactions without metadata
            }
        };

        if tx_meta.err.is_some() {
             log::debug!("Transaction {} failed: {:?}", tx_id, tx_meta.err);
            continue; // Skip failed transactions
        }

        let msg = match trx.transaction.as_ref().and_then(|t| t.message.as_ref()) {
            Some(m) => m,
            None => {
                 log::debug!("Transaction {} has no message", tx_id);
                 continue; // Skip if no message
            }
        };

        // Get account keys involved in the transaction
        // Use resolved_accounts() and bs58 encode
        let account_keys: Vec<String> = trx.resolved_accounts()
            .iter()
            .map(|acc_bytes| bs58::encode(acc_bytes).into_string())
            .collect();

        let signer_pubkey = account_keys.get(0).map(|s| s.as_str()); // First account is usually the fee payer/signer

        // Process outer instructions
        for (inst_idx, inst) in msg.instructions.iter().enumerate() {
            if let Some(meta) = instructions::process_instruction(
                inst, // This is already &CompiledInstruction
                &account_keys,
                block_slot,
                block_time,
                &tx_id,
                inst_idx as u32,
                false, // is_inner_instruction
                None, // actual_inner_index
                signer_pubkey,
                None // outer_program initially None for outer instructions
            ) {
                meta_list.push(meta);
            }
        }

        // Process inner instructions
        // Note: using tx_meta here which is confirmed not None
        for (outer_inst_idx, inner_instructions) in tx_meta.inner_instructions.iter().enumerate() {
            let outer_program_index = msg.instructions.get(outer_inst_idx)
                                        .map(|inst| inst.program_id_index as usize)
                                        .unwrap_or(usize::MAX); // Use MAX or handle error
            let outer_program = account_keys.get(outer_program_index).map(|s| s.as_str());

            for (inner_inst_idx, inner_inst) in inner_instructions.instructions.iter().enumerate() {
                    // Create a CompiledInstruction from InnerInstruction
                    let compiled_inner_inst = CompiledInstruction {
                        program_id_index: inner_inst.program_id_index,
                        accounts: inner_inst.accounts.clone(),
                        data: inner_inst.data.clone(),
                    };

                    if let Some(meta) = instructions::process_instruction(
                        &compiled_inner_inst, // Pass the created CompiledInstruction
                        &account_keys,
                        block_slot,
                        block_time,
                        &tx_id,
                        outer_inst_idx as u32, // outer_instruction_index
                        true, // is_inner_instruction
                        Some(inner_inst_idx as u32), // actual_inner_index
                        signer_pubkey,
                        outer_program
                    ) {
                        meta_list.push(meta);
                    }
            }
        }
    }

    Ok(VaultInstructions { meta: meta_list })
}
