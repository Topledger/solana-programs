mod instructions;
// No need for separate prepare_input_accounts module as it's implemented in instructions

pub mod pb;
use pb::sf::solana::raydium_cpmm::v1::{Instructions, Meta};
use substreams_solana::pb::sf::solana::r#type::v1::{Block, CompiledInstruction};

#[substreams::handlers::map]
fn raydium_cpmm_instructions(
    block: Block,
) -> Result<Instructions, substreams::errors::Error> {
    let mut instructions: Vec<Meta> = Vec::new();
    let block_slot = block.slot;
    let block_timestamp = block.block_time.as_ref().map_or(0, |t| t.timestamp);

    for trx in block.transactions_owned() {
        // Use the first signature as the transaction ID
        let tx_id = trx.transaction.as_ref()
            .and_then(|tx| tx.signatures.get(0))
            .map_or_else(|| String::from("missing_signature"), |sig| bs58::encode(sig).into_string());

        // Convert resolved accounts (Vec<u8>) to Base58 strings
        let resolved_accounts: Vec<String> = trx.resolved_accounts().iter().map(|acc_bytes| bs58::encode(acc_bytes).into_string()).collect();

        // Flatten all instructions, including inner ones
        let mut flat_instructions: Vec<(CompiledInstruction, u32, bool, Option<u32>)> = Vec::new();
        if let Some(tx) = &trx.transaction {
            if let Some(msg) = &tx.message {
                for (idx, inst) in msg.instructions.iter().enumerate() {
                    flat_instructions.push((inst.clone(), idx as u32, false, None));
                }
            }
        }
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

        // Process each instruction
        for (inst, instruction_index, is_inner_instruction, inner_instruction_index) in flat_instructions {
            // Get the first account as the signer
            let signer = trx.transaction.as_ref()
                .and_then(|tx| tx.message.as_ref())
                .and_then(|msg| msg.account_keys.first().cloned())
                .map(|pubkey| bs58::encode(&pubkey).into_string())
                .unwrap_or_default();
            
            // If this is an inner instruction
            let outer_program = if is_inner_instruction {
                // For inner instructions, find the parent instruction's program ID
                // The parent is the outer instruction with the same instruction_index
                Some(trx.transaction.as_ref()
                    .and_then(|tx| tx.message.as_ref())
                    .and_then(|msg| msg.instructions.get(instruction_index as usize))
                    .and_then(|parent_inst| resolved_accounts.get(parent_inst.program_id_index as usize).cloned())
                    .map(|id_str| {
                        // TEMPORARY LOGGING
                        // substreams::log::info!("Inner Outer Program ID fetched: {}", id_str);
                        id_str
                    })
                    .unwrap_or_default())
            } else {
                // For outer instructions, outer_program is the current program's ID
                let current_program_id = resolved_accounts.get(inst.program_id_index as usize)
                    .cloned()
                    .map(|id_str| {
                        // TEMPORARY LOGGING
                        // substreams::log::info!("Outer Outer Program ID fetched: {}", id_str);
                        id_str
                    })
                    .unwrap_or_default();
                Some(current_program_id)
            };
            
            if let Some(meta) = instructions::process_instruction(
                &inst,
                &resolved_accounts, // Use the Vec<String> of accounts
                block_slot, // Use extracted value
                block_timestamp, // Use extracted value
                &tx_id,
                instruction_index,
                is_inner_instruction,
                inner_instruction_index,
                Some(&signer),
                outer_program.as_deref(),
            ) {
                instructions.push(meta);
            }
        }
    }

    Ok(Instructions { instructions })
} 