mod instructions;
// Remove the non-existent module declaration
// mod prepare_input_accounts; 

pub mod pb;
// Add necessary imports from the generated pb module
use pb::sf::solana::raydium_amm::v1::{Instructions, Meta};
use substreams_solana::pb::sf::solana::r#type::v1::{Block, CompiledInstruction};
use bs58;

#[substreams::handlers::map]
fn raydium_amm_instructions( // Renamed function
    block: Block,
) -> Result<Instructions, substreams::errors::Error> {
    let mut meta_items: Vec<Meta> = Vec::new(); // Changed variable name for clarity
    let block_slot = block.slot;
    let block_timestamp = block.block_time.as_ref().map_or(0, |t| t.timestamp);

    for trx in block.transactions_owned() {
        let tx_id = trx.transaction.as_ref()
            .and_then(|tx| tx.signatures.get(0))
            .map_or_else(|| String::from("missing_signature"), |sig| bs58::encode(sig).into_string());

        let resolved_accounts: Vec<String> = trx.resolved_accounts().iter().map(|acc_bytes| bs58::encode(acc_bytes).into_string()).collect();

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

        for (inst, instruction_index, is_inner_instruction, inner_instruction_index) in flat_instructions {
            let signer = trx.transaction.as_ref()
                .and_then(|tx| tx.message.as_ref())
                .and_then(|msg| msg.account_keys.first().cloned())
                .map(|pubkey| bs58::encode(&pubkey).into_string()); // Keep as Option<String>
            
            let outer_program = if is_inner_instruction {
                Some(trx.transaction.as_ref()
                    .and_then(|tx| tx.message.as_ref())
                    .and_then(|msg| msg.instructions.get(instruction_index as usize))
                    .and_then(|parent_inst| resolved_accounts.get(parent_inst.program_id_index as usize).cloned())
                    .unwrap_or_default())
            } else {
                None
            };
            
            // process_instruction returns Option<Meta>, handle it
            if let Some(meta_result) = instructions::process_instruction(
                &inst,
                &resolved_accounts,
                block_slot,
                block_timestamp,
                &tx_id,
                instruction_index,
                is_inner_instruction,
                inner_instruction_index,
                signer.as_deref(), // Pass Option<&str>
                outer_program.as_deref(), // Pass Option<&str>
            ) {
                 // Compare Option<String> with &str using as_deref()
                 if meta_result.instruction_type.as_deref() == Some("swapBaseIn") { 
                     if let Some(args) = &meta_result.args { // args in Meta is Option<FlatArg>
                         // Log optional fields safely using unwrap_or_default or match
                         substreams::log::info!(
                             "Outputting SwapBaseIn: amount_in={:?}, minimum_amount_out={:?}", 
                             args.amount_in, // snake_case
                             args.minimum_amount_out // snake_case
                         );
                     }
                 }
                meta_items.push(meta_result);
            }
        }
    }

    Ok(Instructions { instructions: meta_items })
} 