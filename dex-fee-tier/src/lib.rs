mod pb;
mod dapps;

use pb::sf::solana::dex::fee::tier::v1::*;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, CompiledInstruction};
use chrono::prelude::*;

const TARGET_PROGRAMS: [&str; 4] = [
    "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK", // Raydium CLMM program
    "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C", // Raydium CPMM program
    "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc", // Orca Whirlpool program
    "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP", // Orca v1 program
];

#[substreams::handlers::map]
fn dex_fee_tier_map(block: Block) -> Result<Output, substreams::errors::Error> {
    let slot = block.slot;
    let _parent_slot = block.parent_slot;
    let timestamp = block.block_time.as_ref().unwrap().timestamp;
    let block_date = Utc.timestamp_opt(timestamp, 0).unwrap().format("%Y-%m-%d").to_string();

    let mut trade_data = Vec::new();

    for trx in block.transactions_owned() {
        // Use the first signature as the transaction ID
        let tx_id = trx.transaction.as_ref()
            .and_then(|tx| tx.signatures.get(0))
            .map_or_else(|| String::from("missing_signature"), |sig| bs58::encode(sig).into_string());

        // Skip failed transactions
        if let Some(meta) = &trx.meta {
            if meta.err.is_some() {
                continue;
            }
        } else {
            continue;
        }

        // Convert resolved accounts to Base58 strings
        let account_keys: Vec<String> = trx.resolved_accounts().iter()
            .map(|acc_bytes| bs58::encode(acc_bytes).into_string())
            .collect();

        // Process outer and inner instructions
        let mut flat_instructions: Vec<(CompiledInstruction, u32, bool, Option<u32>)> = Vec::new();
        
        // Collect regular instructions
        if let Some(tx) = &trx.transaction {
            if let Some(msg) = &tx.message {
                for (idx, inst) in msg.instructions.iter().enumerate() {
                    flat_instructions.push((inst.clone(), idx as u32, false, None));
                }
            }
        }
        
        // Collect inner instructions
        if let Some(meta) = &trx.meta {
            for inner_insts in meta.inner_instructions.iter() {
                let outer_instruction_index = inner_insts.index;
                for (inner_idx, inst) in inner_insts.instructions.iter().enumerate() {
                    let compiled_inst = CompiledInstruction {
                        program_id_index: inst.program_id_index,
                        accounts: inst.accounts.clone(),
                        data: inst.data.clone(),
                    };
                    flat_instructions.push((compiled_inst, outer_instruction_index, true, Some(inner_idx as u32)));
                }
            }
        }

        // Get signer
        let _signer = trx.transaction.as_ref()
            .and_then(|tx| tx.message.as_ref())
            .and_then(|msg| msg.account_keys.first())
            .map(|key| bs58::encode(key).into_string())
            .unwrap_or_default();

        // Process each instruction
        for (inst, instruction_index, is_inner_instruction, inner_instruction_index) in flat_instructions {
            // Get the program ID from the instruction
            let program_id = match account_keys.get(inst.program_id_index as usize) {
                Some(id) => id,
                None => continue,
            };

            // Check if we're interested in this program
            if !TARGET_PROGRAMS.contains(&program_id.as_str()) {
                continue;
            }

            // Get outer program for inner instructions
            let _outer_program = if is_inner_instruction {
                trx.transaction.as_ref()
                    .and_then(|tx| tx.message.as_ref())
                    .and_then(|msg| msg.instructions.get(instruction_index as usize))
                    .and_then(|outer_inst| account_keys.get(outer_inst.program_id_index as usize))
            } else {
                None
            };

            // Process the instruction data
            let data = &inst.data;
            if data.len() < 8 {
                continue;
            }

            let result = match program_id.as_str() {
                "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK" => 
                    dapps::dapp_CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK::parse_instruction(
                        &inst, &account_keys, instruction_index, is_inner_instruction, inner_instruction_index),
                "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C" => 
                    dapps::dapp_CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C::parse_instruction(
                        &inst, &account_keys, instruction_index, is_inner_instruction, inner_instruction_index),
                "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc" => 
                    dapps::dapp_whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc::parse_instruction(
                        &inst, &account_keys, instruction_index, is_inner_instruction, inner_instruction_index),
                "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP" => 
                    dapps::dapp_9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP::parse_instruction(
                        &inst, &account_keys, instruction_index, is_inner_instruction, inner_instruction_index),
                _ => None,
            };

            if let Some((ins_type, pool_addr, amm_cfg, fee_rate, protocol_rate, fund_fee_rate, param, value, create_pool_fee)) = result {
                // Check if this instruction type is of interest for the dapp
                if !dapps::is_dapp_instruction_type_of_interest(program_id, &Some(ins_type.clone())) {
                    continue;
                }
                
                trade_data.push(TradeData {
                    block_date: block_date.clone(),
                    block_time: timestamp,
                    block_slot: slot,
                    tx_id: tx_id.clone(),
                    pool_address: pool_addr.unwrap_or_default(),
                    amm_config: amm_cfg.unwrap_or_default(),
                    instruction_type: ins_type,
                    dapp: program_id.clone(),
                    trade_fee_rate: fee_rate.unwrap_or_default(),
                    protocol_fee_rate: protocol_rate.unwrap_or_default(),
                    fund_fee_rate: fund_fee_rate.unwrap_or_default(),
                    param: param.unwrap_or_default(),
                    value: value.unwrap_or_default(),
                    create_pool_fee: create_pool_fee.unwrap_or_default(),
                });
            }
        }
    }

    Ok(Output {
        data: trade_data,
    })
}

fn is_program_of_interest(program_id: &str) -> bool {
    TARGET_PROGRAMS.contains(&program_id)
} 