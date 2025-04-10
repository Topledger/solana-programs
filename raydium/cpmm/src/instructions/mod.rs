use std::collections::HashMap;
use chrono;
use substreams_solana::pb::sf::solana::r#type::v1::{CompiledInstruction};
use crate::pb::sf::solana::raydium_cpmm::v1::{
    Meta, FlatArg, InstructionArgs, instruction_args::InstructionArgs as IArgs,
    PbCreateAmmConfigLayout, PbUpdateAmmConfigLayout, PbUpdatePoolStatusLayout,
    PbCollectProtocolFeeLayout, PbCollectFundFeeLayout, PbInitializeLayout,
    PbDepositLayout, PbWithdrawLayout, PbSwapBaseInputLayout, PbSwapBaseOutputLayout,
};
use std::convert::TryInto; 
use hex;

// Helper macro to decode or return a default value
#[macro_export]
macro_rules! decode_or_default {
    ($e:expr) => {
        $e.unwrap_or_default()
    };
}
use crate::decode_or_default;

/// Process a single instruction and convert it to a Meta object if it's from the CPMM program
pub fn process_instruction(
    instruction: &CompiledInstruction,
    accounts: &[String],
    block_slot: u64,
    block_timestamp: i64,
    tx_id: &str,
    instruction_index: u32,
    is_inner_instruction: bool,
    inner_instruction_index: Option<u32>,
    signer: Option<&str>,
    outer_program: Option<&str>,
) -> Option<Meta> {
    // The Raydium CPMM program ID
    const RAYDIUM_CPMM_PROGRAM_ID: &str = "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C";
    
    // Get the program ID executing this instruction
    let program_id = accounts.get(instruction.program_id_index as usize)?;
    
    // Only process instructions from the Raydium CPMM program
    if program_id != RAYDIUM_CPMM_PROGRAM_ID {
        return None;
    }
    
    // Parse instruction data
    let instruction_data = parse_instruction(&instruction.data);
    
    // Create a mapping of account indices to their addresses
    let input_accounts = prepare_input_accounts(instruction, accounts, &instruction_data.0);
    
    // Create FlatArg with values according to instruction type
    let flat_args = process_args(&instruction_data.0, &instruction_data.1);
    
    // Create the Meta object
    let block_date = chrono::DateTime::from_timestamp(block_timestamp, 0)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "1970-01-01".to_string());
    
    Some(Meta {
        block_date: block_date,
        block_time: block_timestamp,
        block_slot: block_slot,
        tx_id: tx_id.to_string(),
        instruction_index: Some(instruction_index),
        is_inner_instruction: Some(is_inner_instruction),
        inner_instruction_index: if is_inner_instruction { inner_instruction_index } else { Some(0) },
        dapp: RAYDIUM_CPMM_PROGRAM_ID.to_string(),
        instruction_type: instruction_data.0,
        args: Some(flat_args),
        input_accounts: input_accounts,
        signer: signer.unwrap_or_default().to_string(),
        outer_program: outer_program.unwrap_or_default().to_string(),
    })
}

/// Create a mapping of account roles to their addresses based on instruction type
fn prepare_input_accounts(
    instruction: &CompiledInstruction,
    accounts: &[String],
    instruction_type: &str,
) -> HashMap<String, String> {
    let mut result = HashMap::new();
    
    // Map the accounts based on instruction type
    match instruction_type {
        "CreateAmmConfig" => {
            result.insert("owner".to_string(), get_account_at_index(instruction, accounts, 0));
            result.insert("ammConfig".to_string(), get_account_at_index(instruction, accounts, 1));
            result.insert("systemProgram".to_string(), get_account_at_index(instruction, accounts, 2));
        },
        "UpdateAmmConfig" => {
            result.insert("owner".to_string(), get_account_at_index(instruction, accounts, 0));
            result.insert("ammConfig".to_string(), get_account_at_index(instruction, accounts, 1));
        },
        "UpdatePoolStatus" => {
            result.insert("authority".to_string(), get_account_at_index(instruction, accounts, 0));
            result.insert("poolState".to_string(), get_account_at_index(instruction, accounts, 1));
        },
        "CollectProtocolFee" | "CollectFundFee" => {
            result.insert("owner".to_string(), get_account_at_index(instruction, accounts, 0));
            result.insert("authority".to_string(), get_account_at_index(instruction, accounts, 1));
            result.insert("poolState".to_string(), get_account_at_index(instruction, accounts, 2));
            result.insert("ammConfig".to_string(), get_account_at_index(instruction, accounts, 3));
            result.insert("token0Vault".to_string(), get_account_at_index(instruction, accounts, 4));
            result.insert("token1Vault".to_string(), get_account_at_index(instruction, accounts, 5));
            result.insert("vault0Mint".to_string(), get_account_at_index(instruction, accounts, 6));
            result.insert("vault1Mint".to_string(), get_account_at_index(instruction, accounts, 7));
            result.insert("recipientToken0Account".to_string(), get_account_at_index(instruction, accounts, 8));
            result.insert("recipientToken1Account".to_string(), get_account_at_index(instruction, accounts, 9));
            result.insert("tokenProgram".to_string(), get_account_at_index(instruction, accounts, 10));
            result.insert("tokenProgram2022".to_string(), get_account_at_index(instruction, accounts, 11));
        },
        "Initialize" => {
            result.insert("creator".to_string(), get_account_at_index(instruction, accounts, 0));
            result.insert("ammConfig".to_string(), get_account_at_index(instruction, accounts, 1));
            result.insert("authority".to_string(), get_account_at_index(instruction, accounts, 2));
            result.insert("poolState".to_string(), get_account_at_index(instruction, accounts, 3));
            result.insert("token0Mint".to_string(), get_account_at_index(instruction, accounts, 4));
            result.insert("token1Mint".to_string(), get_account_at_index(instruction, accounts, 5));
            result.insert("lpMint".to_string(), get_account_at_index(instruction, accounts, 6));
            result.insert("creatorToken0".to_string(), get_account_at_index(instruction, accounts, 7));
            result.insert("creatorToken1".to_string(), get_account_at_index(instruction, accounts, 8));
            result.insert("creatorLpToken".to_string(), get_account_at_index(instruction, accounts, 9));
            result.insert("token0Vault".to_string(), get_account_at_index(instruction, accounts, 10));
            result.insert("token1Vault".to_string(), get_account_at_index(instruction, accounts, 11));
            result.insert("createPoolFee".to_string(), get_account_at_index(instruction, accounts, 12));
            result.insert("observationState".to_string(), get_account_at_index(instruction, accounts, 13));
            result.insert("tokenProgram".to_string(), get_account_at_index(instruction, accounts, 14));
            result.insert("token0Program".to_string(), get_account_at_index(instruction, accounts, 15));
            result.insert("token1Program".to_string(), get_account_at_index(instruction, accounts, 16));
            result.insert("associatedTokenProgram".to_string(), get_account_at_index(instruction, accounts, 17));
            result.insert("systemProgram".to_string(), get_account_at_index(instruction, accounts, 18));
            result.insert("rent".to_string(), get_account_at_index(instruction, accounts, 19));
        },
        "Deposit" => {
            result.insert("owner".to_string(), get_account_at_index(instruction, accounts, 0));
            result.insert("authority".to_string(), get_account_at_index(instruction, accounts, 1));
            result.insert("poolState".to_string(), get_account_at_index(instruction, accounts, 2));
            result.insert("ownerLpToken".to_string(), get_account_at_index(instruction, accounts, 3));
            result.insert("token0Account".to_string(), get_account_at_index(instruction, accounts, 4));
            result.insert("token1Account".to_string(), get_account_at_index(instruction, accounts, 5));
            result.insert("token0Vault".to_string(), get_account_at_index(instruction, accounts, 6));
            result.insert("token1Vault".to_string(), get_account_at_index(instruction, accounts, 7));
            result.insert("tokenProgram".to_string(), get_account_at_index(instruction, accounts, 8));
            result.insert("tokenProgram2022".to_string(), get_account_at_index(instruction, accounts, 9));
            result.insert("vault0Mint".to_string(), get_account_at_index(instruction, accounts, 10));
            result.insert("vault1Mint".to_string(), get_account_at_index(instruction, accounts, 11));
            result.insert("lpMint".to_string(), get_account_at_index(instruction, accounts, 12));
        },
        "Withdraw" => {
            result.insert("owner".to_string(), get_account_at_index(instruction, accounts, 0));
            result.insert("authority".to_string(), get_account_at_index(instruction, accounts, 1));
            result.insert("poolState".to_string(), get_account_at_index(instruction, accounts, 2));
            result.insert("ownerLpToken".to_string(), get_account_at_index(instruction, accounts, 3));
            result.insert("token0Account".to_string(), get_account_at_index(instruction, accounts, 4));
            result.insert("token1Account".to_string(), get_account_at_index(instruction, accounts, 5));
            result.insert("token0Vault".to_string(), get_account_at_index(instruction, accounts, 6));
            result.insert("token1Vault".to_string(), get_account_at_index(instruction, accounts, 7));
            result.insert("tokenProgram".to_string(), get_account_at_index(instruction, accounts, 8));
            result.insert("tokenProgram2022".to_string(), get_account_at_index(instruction, accounts, 9));
            result.insert("vault0Mint".to_string(), get_account_at_index(instruction, accounts, 10));
            result.insert("vault1Mint".to_string(), get_account_at_index(instruction, accounts, 11));
            result.insert("lpMint".to_string(), get_account_at_index(instruction, accounts, 12));
            result.insert("memoProgram".to_string(), get_account_at_index(instruction, accounts, 13));
        },
        "SwapBaseInput" | "SwapBaseOutput" => {
            result.insert("payer".to_string(), get_account_at_index(instruction, accounts, 0));
            result.insert("authority".to_string(), get_account_at_index(instruction, accounts, 1));
            result.insert("ammConfig".to_string(), get_account_at_index(instruction, accounts, 2));
            result.insert("poolState".to_string(), get_account_at_index(instruction, accounts, 3));
            result.insert("inputTokenAccount".to_string(), get_account_at_index(instruction, accounts, 4));
            result.insert("outputTokenAccount".to_string(), get_account_at_index(instruction, accounts, 5));
            result.insert("inputVault".to_string(), get_account_at_index(instruction, accounts, 6));
            result.insert("outputVault".to_string(), get_account_at_index(instruction, accounts, 7));
            result.insert("inputTokenProgram".to_string(), get_account_at_index(instruction, accounts, 8));
            result.insert("outputTokenProgram".to_string(), get_account_at_index(instruction, accounts, 9));
            result.insert("inputTokenMint".to_string(), get_account_at_index(instruction, accounts, 10));
            result.insert("outputTokenMint".to_string(), get_account_at_index(instruction, accounts, 11));
            result.insert("observationState".to_string(), get_account_at_index(instruction, accounts, 12));
        },
        _ => {}
    }
    
    result
}

/// Helper function to get account at an index or return empty string
fn get_account_at_index(instruction: &CompiledInstruction, accounts: &[String], index: usize) -> String {
    if let Some(acc_idx) = instruction.accounts.get(index) {
        if let Some(acc) = accounts.get(*acc_idx as usize) {
            return acc.clone();
        }
    }
    String::new()
}

/// Process args based on instruction type
fn process_args(instruction_type: &str, args: &InstructionArgs) -> FlatArg {
    let mut flat_args = FlatArg::default();
    
    match &args.instruction_args {
        Some(IArgs::CreateAmmConfig(args)) => {
            flat_args.index = Some(args.index);
            flat_args.trade_fee_rate = Some(args.trade_fee_rate);
            flat_args.protocol_fee_rate = Some(args.protocol_fee_rate);
            flat_args.fund_fee_rate = Some(args.fund_fee_rate);
            flat_args.create_pool_fee = Some(args.create_pool_fee);
        },
        Some(IArgs::UpdateAmmConfig(args)) => {
            flat_args.param = Some(args.param as u32);
            flat_args.value = Some(args.value);
        },
        Some(IArgs::UpdatePoolStatus(args)) => {
            flat_args.status = Some(args.status);
        },
        Some(IArgs::CollectProtocolFee(args)) => {
            flat_args.amount0_requested = Some(args.amount0_requested.to_string());
            flat_args.amount1_requested = Some(args.amount1_requested.to_string());
        },
        Some(IArgs::CollectFundFee(args)) => {
            flat_args.amount0_requested = Some(args.amount0_requested.to_string());
            flat_args.amount1_requested = Some(args.amount1_requested.to_string());
        },
        Some(IArgs::Initialize(args)) => {
            flat_args.init_amount0 = Some(args.init_amount0);
            flat_args.init_amount1 = Some(args.init_amount1);
            flat_args.open_time = Some(args.open_time);
        },
        Some(IArgs::Deposit(args)) => {
            flat_args.lp_token_amount = Some(args.lp_token_amount);
            flat_args.maximum_token0_amount = Some(args.maximum_token0_amount);
            flat_args.maximum_token1_amount = Some(args.maximum_token1_amount);
        },
        Some(IArgs::Withdraw(args)) => {
            flat_args.lp_token_amount = Some(args.lp_token_amount);
            flat_args.minimum_token0_amount = Some(args.minimum_token0_amount);
            flat_args.minimum_token1_amount = Some(args.minimum_token1_amount);
        },
        Some(IArgs::SwapBaseInput(args)) => {
            flat_args.amount_in = Some(args.amount_in);
            flat_args.minimum_amount_out = Some(args.minimum_amount_out);
        },
        Some(IArgs::SwapBaseOutput(args)) => {
            flat_args.max_amount_in = Some(args.max_amount_in);
            flat_args.amount_out = Some(args.amount_out);
        },
        _ => {}
    }
    
    flat_args
}

/// Parse instruction data and return the instruction type and arguments
fn parse_instruction(data: &[u8]) -> (String, InstructionArgs) {
    if data.len() < 8 {
        return ("Unknown".to_string(), InstructionArgs::default());
    }
    
    // Extract the 8-byte discriminator
    let discriminator = &data[0..8];
    
    // Define instruction discriminators
    const CREATE_AMM_CONFIG_DISCRIMINATOR: [u8; 8] = [137, 52, 237, 212, 215, 117, 108, 104];
    const UPDATE_AMM_CONFIG_DISCRIMINATOR: [u8; 8] = [49, 60, 174, 136, 154, 28, 116, 200];
    const UPDATE_POOL_STATUS_DISCRIMINATOR: [u8; 8] = [130, 87, 108, 6, 46, 224, 117, 123];
    const COLLECT_PROTOCOL_FEE_DISCRIMINATOR: [u8; 8] = [136, 136, 252, 221, 194, 66, 126, 89];
    const COLLECT_FUND_FEE_DISCRIMINATOR: [u8; 8] = [167, 138, 78, 149, 223, 194, 6, 126];
    const INITIALIZE_DISCRIMINATOR: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
    const DEPOSIT_DISCRIMINATOR: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
    const WITHDRAW_DISCRIMINATOR: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];
    const SWAP_BASE_INPUT_DISCRIMINATOR: [u8; 8] = [143, 190, 90, 218, 196, 30, 51, 222];
    const SWAP_BASE_OUTPUT_DISCRIMINATOR: [u8; 8] = [55, 217, 98, 86, 163, 74, 180, 173];

    // Match discriminator to instruction type
    let (instruction_type, args) = match discriminator {
        x if x == CREATE_AMM_CONFIG_DISCRIMINATOR => {
            if data.len() >= 16 {
                ("CreateAmmConfig".to_string(), InstructionArgs {
                    instruction_args: Some(IArgs::CreateAmmConfig(PbCreateAmmConfigLayout {
                        index: decode_or_default!(read_u16(data, 8)),
                        trade_fee_rate: decode_or_default!(read_u64(data, 10)),
                        protocol_fee_rate: decode_or_default!(read_u64(data, 18)),
                        fund_fee_rate: decode_or_default!(read_u64(data, 26)),
                        create_pool_fee: decode_or_default!(read_u64(data, 34)),
                    }))
                })
            } else {
                ("CreateAmmConfig".to_string(), InstructionArgs::default())
            }
        },
        x if x == UPDATE_AMM_CONFIG_DISCRIMINATOR => {
            if data.len() >= 17 {
                ("UpdateAmmConfig".to_string(), InstructionArgs {
                    instruction_args: Some(IArgs::UpdateAmmConfig(PbUpdateAmmConfigLayout {
                        param: read_u8(data, 8).map(|v| v as u32).unwrap_or_default(),
                        value: decode_or_default!(read_u64(data, 9)),
                    }))
                })
            } else {
                ("UpdateAmmConfig".to_string(), InstructionArgs::default())
            }
        },
        x if x == UPDATE_POOL_STATUS_DISCRIMINATOR => {
            if data.len() >= 9 {
                ("UpdatePoolStatus".to_string(), InstructionArgs {
                    instruction_args: Some(IArgs::UpdatePoolStatus(PbUpdatePoolStatusLayout {
                        status: read_u8(data, 8).map(|v| v as u32).unwrap_or_default(),
                    }))
                })
            } else {
                ("UpdatePoolStatus".to_string(), InstructionArgs::default())
            }
        },
        x if x == COLLECT_PROTOCOL_FEE_DISCRIMINATOR => {
            if data.len() >= 24 {
                ("CollectProtocolFee".to_string(), InstructionArgs {
                    instruction_args: Some(IArgs::CollectProtocolFee(PbCollectProtocolFeeLayout {
                        amount0_requested: decode_or_default!(read_u64(data, 8)),
                        amount1_requested: decode_or_default!(read_u64(data, 16)),
                    }))
                })
            } else {
                ("CollectProtocolFee".to_string(), InstructionArgs::default())
            }
        },
        x if x == COLLECT_FUND_FEE_DISCRIMINATOR => {
            if data.len() >= 24 {
                ("CollectFundFee".to_string(), InstructionArgs {
                    instruction_args: Some(IArgs::CollectFundFee(PbCollectFundFeeLayout {
                        amount0_requested: decode_or_default!(read_u64(data, 8)),
                        amount1_requested: decode_or_default!(read_u64(data, 16)),
                    }))
                })
            } else {
                ("CollectFundFee".to_string(), InstructionArgs::default())
            }
        },
        x if x == INITIALIZE_DISCRIMINATOR => {
            if data.len() >= 32 {
                ("Initialize".to_string(), InstructionArgs {
                    instruction_args: Some(IArgs::Initialize(PbInitializeLayout {
                        init_amount0: decode_or_default!(read_u64(data, 8)),
                        init_amount1: decode_or_default!(read_u64(data, 16)),
                        open_time: decode_or_default!(read_u64(data, 24)),
                    }))
                })
            } else {
                ("Initialize".to_string(), InstructionArgs::default())
            }
        },
        x if x == DEPOSIT_DISCRIMINATOR => {
            if data.len() >= 32 {
                ("Deposit".to_string(), InstructionArgs {
                    instruction_args: Some(IArgs::Deposit(PbDepositLayout {
                        lp_token_amount: decode_or_default!(read_u64(data, 8)),
                        maximum_token0_amount: decode_or_default!(read_u64(data, 16)),
                        maximum_token1_amount: decode_or_default!(read_u64(data, 24)),
                    }))
                })
            } else {
                ("Deposit".to_string(), InstructionArgs::default())
            }
        },
        x if x == WITHDRAW_DISCRIMINATOR => {
            if data.len() >= 32 {
                ("Withdraw".to_string(), InstructionArgs {
                    instruction_args: Some(IArgs::Withdraw(PbWithdrawLayout {
                        lp_token_amount: decode_or_default!(read_u64(data, 8)),
                        minimum_token0_amount: decode_or_default!(read_u64(data, 16)),
                        minimum_token1_amount: decode_or_default!(read_u64(data, 24)),
                    }))
                })
            } else {
                ("Withdraw".to_string(), InstructionArgs::default())
            }
        },
        x if x == SWAP_BASE_INPUT_DISCRIMINATOR => {
            if data.len() >= 24 {
                ("SwapBaseInput".to_string(), InstructionArgs {
                    instruction_args: Some(IArgs::SwapBaseInput(PbSwapBaseInputLayout {
                        amount_in: decode_or_default!(read_u64(data, 8)),
                        minimum_amount_out: decode_or_default!(read_u64(data, 16)),
                    }))
                })
            } else {
                ("SwapBaseInput".to_string(), InstructionArgs::default())
            }
        },
        x if x == SWAP_BASE_OUTPUT_DISCRIMINATOR => {
            if data.len() >= 24 {
                ("SwapBaseOutput".to_string(), InstructionArgs {
                    instruction_args: Some(IArgs::SwapBaseOutput(PbSwapBaseOutputLayout {
                        max_amount_in: decode_or_default!(read_u64(data, 8)),
                        amount_out: decode_or_default!(read_u64(data, 16)),
                    }))
                })
            } else {
                ("SwapBaseOutput".to_string(), InstructionArgs::default())
            }
        },
        _ => {
            let discriminator_hex = hex::encode(discriminator);
            ("Unknown".to_string(), InstructionArgs::default())
        }
    };
    
    (instruction_type, args)
}

// Helper functions for decoding different data types
fn read_u8(data: &[u8], offset: usize) -> Option<u8> {
    data.get(offset).copied()
}

fn read_u16(data: &[u8], offset: usize) -> Option<u32> {
    data.get(offset..offset+2)?.try_into().map(|b| u16::from_le_bytes(b) as u32).ok()
}

fn read_i32(data: &[u8], offset: usize) -> Option<i32> {
    data.get(offset..offset+4)?.try_into().map(i32::from_le_bytes).ok()
}

fn read_u32(data: &[u8], offset: usize) -> Option<u32> {
    data.get(offset..offset+4)?.try_into().map(u32::from_le_bytes).ok()
}

fn read_i64(data: &[u8], offset: usize) -> Option<i64> {
    data.get(offset..offset+8)?.try_into().map(i64::from_le_bytes).ok()
}

fn read_u64(data: &[u8], offset: usize) -> Option<u64> {
    data.get(offset..offset+8)?.try_into().map(u64::from_le_bytes).ok()
}

fn bytes_to_bool(byte_opt: Option<&u8>) -> bool {
    byte_opt.map_or(false, |&b| b != 0)
}

fn bytes_to_optional_bool(byte_opt: Option<&u8>) -> Option<bool> {
    byte_opt.map(|&b| b != 0)
}

// Function to convert bytes to PbInt128
fn bytes_to_pbint128(data: &[u8]) -> Option<crate::pb::sf::solana::raydium_cpmm::v1::PbInt128> {
    if data.len() < 16 {
        return None;
    }
    
    // Convert the bytes to a u128
    let value = u128::from_le_bytes(data.try_into().ok()?);
    
    // Convert the u128 to a string to preserve full precision
    Some(crate::pb::sf::solana::raydium_cpmm::v1::PbInt128 {
        value: value.to_string(),
    })
} 