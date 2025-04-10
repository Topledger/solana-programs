// Simplify imports for instructions module
use substreams_solana::pb::sf::solana::r#type::v1::CompiledInstruction;
use crate::pb::sf::solana::raydium_amm::v1::{self as amm_v1, Fees, FlatArg, InstructionArgs, instruction_args, LastOrderDistance, Meta, NeedTake,
    PbAdminCancelOrdersLayout, PbCreateConfigAccountLayout, PbDepositLayout, PbInitialize2Layout, PbInitializeLayout, PbMigrateToOpenBookLayout,
    PbMonitorStepLayout, PbPreInitializeLayout, PbSetParamsLayout, PbSimulateInfoLayout, PbSwapBaseInLayout, PbSwapBaseOutLayout,
    PbUpdateConfigAccountLayout, PbWithdrawLayout, PbWithdrawPnlLayout, PbWithdrawSrmLayout, SwapInstructionBaseIn, SwapInstructionBaseOut};
use std::collections::HashMap;
use chrono;
use bs58;
use std::convert::TryInto;

// Define the AMM program ID
pub const RAYDIUM_AMM_PROGRAM_ID: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";

// Discriminators from Python parser (adapt as needed, IDL doesn't specify them)
const INITIALIZE_DISC: u8 = 0;
const INITIALIZE2_DISC: u8 = 1;
const MONITOR_STEP_DISC: u8 = 2; // Assuming from order, Reserved0 in Python parser
const DEPOSIT_DISC: u8 = 3;
const WITHDRAW_DISC: u8 = 4;
const MIGRATE_TO_OPEN_BOOK_DISC: u8 = 5; // Assuming from order, Reserved1 in Python parser
const SET_PARAMS_DISC: u8 = 6; // Assuming from order, Reserved2 in Python parser
const WITHDRAW_PNL_DISC: u8 = 7; // Assuming from order, Reserved3 in Python parser
const WITHDRAW_SRM_DISC: u8 = 8; // Assuming from order, Reserved4 in Python parser
const SWAP_BASE_IN_DISC: u8 = 9;
const PRE_INITIALIZE_DISC: u8 = 10;
const SWAP_BASE_OUT_DISC: u8 = 11;
const SIMULATE_INFO_DISC: u8 = 12; // Assuming from order, Reserved5 in Python parser
const ADMIN_CANCEL_ORDERS_DISC: u8 = 13; // Not in Python parser, assuming next index
const CREATE_CONFIG_ACCOUNT_DISC: u8 = 14; // Not in Python parser, assuming next index
const UPDATE_CONFIG_ACCOUNT_DISC: u8 = 15; // Not in Python parser, assuming next index


// Helper function to read various data types from byte slice
fn read_u8(data: &[u8], offset: usize) -> Option<u8> {
    data.get(offset).copied()
}

fn read_u16(data: &[u8], offset: usize) -> Option<u16> {
    data.get(offset..offset+2).and_then(|slice| {
        slice.try_into().ok().map(u16::from_le_bytes)
    })
}

fn read_u64(data: &[u8], offset: usize) -> Option<u64> {
    data.get(offset..offset+8).and_then(|slice| {
        slice.try_into().ok().map(u64::from_le_bytes)
    })
}

fn read_option_u64(data: &[u8], offset: usize) -> Option<u64> {
    data.get(offset..offset+8).and_then(|slice| {
        if slice.iter().all(|&x| x == 0) { 
            None
        } else { 
            slice.try_into().ok().map(u64::from_le_bytes)
        } 
    })
}

fn read_pubkey(data: &[u8], offset: usize) -> Option<String> {
    data.get(offset..offset+32).map(|slice| bs58::encode(slice).into_string())
}

fn read_option_pubkey(data: &[u8], offset: usize) -> Option<String> {
    data.get(offset..offset+32).and_then(|slice| {
        if slice.iter().all(|&x| x == 0) { None } else { Some(bs58::encode(slice).into_string()) }
    })
}

// Placeholder functions for nested structs - Assuming these are handled correctly
// based on the `create_flat_arg` logic
fn read_fees(_data: &[u8], _offset: usize) -> Option<Fees> {
    None // Placeholder
}

fn read_last_order_distance(_data: &[u8], _offset: usize) -> Option<LastOrderDistance> {
    None // Placeholder
}

fn read_need_take(_data: &[u8], _offset: usize) -> Option<NeedTake> {
    None // Placeholder
}

// Implement reading for SwapInstructionBaseIn (assuming layout: amount_in u64, minimum_amount_out u64)
fn read_swap_base_in(data: &[u8], offset: usize) -> Option<SwapInstructionBaseIn> {
    // Check if there are enough bytes (16 bytes for two u64)
    if data.len() < offset + 16 {
        return None;
    }
    let amount_in = read_u64(data, offset)?;
    let minimum_amount_out = read_u64(data, offset + 8)?;
    Some(SwapInstructionBaseIn {
        amount_in: Some(amount_in),
        minimum_amount_out: Some(minimum_amount_out),
    })
}

// Implement reading for SwapInstructionBaseOut (assuming layout: max_amount_in u64, amount_out u64)
fn read_swap_base_out(data: &[u8], offset: usize) -> Option<SwapInstructionBaseOut> {
    // Check if there are enough bytes (16 bytes for two u64)
    if data.len() < offset + 16 {
        return None;
    }
    let max_amount_in = read_u64(data, offset)?;
    let amount_out = read_u64(data, offset + 8)?;
    Some(SwapInstructionBaseOut {
        max_amount_in: Some(max_amount_in),
        amount_out: Some(amount_out),
    })
}


// Prepare input accounts map using instruction-specific names based on the provided IDL JSON
fn prepare_input_accounts(
    instruction: &CompiledInstruction,
    accounts: &[String],
    instruction_name: &str // Use this to determine expected names
) -> HashMap<String, String> {
    let mut input_accounts = HashMap::new();

    // Define expected account names based on instruction type (order matches IDL JSON!)
    let expected_names: &[&str] = match instruction_name {
        "initialize" => &[
            "tokenProgram",           // 0
            "systemProgram",          // 1
            "rent",                   // 2
            "amm",                    // 3
            "ammAuthority",           // 4
            "ammOpenOrders",          // 5
            "lpMintAddress",          // 6
            "coinMintAddress",        // 7
            "pcMintAddress",          // 8
            "poolCoinTokenAccount",   // 9
            "poolPcTokenAccount",     // 10
            "poolWithdrawQueue",      // 11
            "poolTargetOrdersAccount",// 12
            "userLpTokenAccount",     // 13
            "poolTempLpTokenAccount", // 14
            "serumProgram",           // 15
            "serumMarket",            // 16
            "userWallet"              // 17
        ],
        "initialize2" => &[
            "tokenProgram",              // 0
            "splAssociatedTokenAccount", // 1
            "systemProgram",             // 2
            "rent",                      // 3
            "amm",                       // 4
            "ammAuthority",              // 5
            "ammOpenOrders",             // 6
            "lpMint",                    // 7
            "coinMint",                  // 8
            "pcMint",                    // 9
            "poolCoinTokenAccount",      // 10
            "poolPcTokenAccount",        // 11
            "poolWithdrawQueue",         // 12
            "ammTargetOrders",           // 13
            "poolTempLp",                // 14
            "serumProgram",              // 15
            "serumMarket",               // 16
            "userWallet",                // 17
            "userTokenCoin",             // 18
            "userTokenPc",               // 19
            "userLpTokenAccount"         // 20
        ],
        "monitorStep" => &[
            "tokenProgram",          // 0
            "rent",                  // 1
            "clock",                 // 2
            "amm",                   // 3
            "ammAuthority",          // 4
            "ammOpenOrders",         // 5
            "ammTargetOrders",       // 6
            "poolCoinTokenAccount",  // 7
            "poolPcTokenAccount",    // 8
            "poolWithdrawQueue",     // 9
            "serumProgram",          // 10
            "serumMarket",           // 11
            "serumCoinVaultAccount", // 12
            "serumPcVaultAccount",   // 13
            "serumVaultSigner",      // 14
            "serumReqQ",             // 15
            "serumEventQ",           // 16
            "serumBids",             // 17
            "serumAsks"              // 18
        ],
        "deposit" => &[
            "tokenProgram",          // 0
            "amm",                   // 1
            "ammAuthority",          // 2
            "ammOpenOrders",         // 3
            "ammTargetOrders",       // 4
            "lpMintAddress",         // 5
            "poolCoinTokenAccount",  // 6
            "poolPcTokenAccount",    // 7
            "serumMarket",           // 8
            "userCoinTokenAccount",  // 9
            "userPcTokenAccount",    // 10
            "userLpTokenAccount",    // 11
            "userOwner",             // 12
            "serumEventQueue"        // 13
        ],
        "withdraw" => &[
            "tokenProgram",          // 0
            "amm",                   // 1
            "ammAuthority",          // 2
            "ammOpenOrders",         // 3
            "ammTargetOrders",       // 4
            "lpMintAddress",         // 5
            "poolCoinTokenAccount",  // 6
            "poolPcTokenAccount",    // 7
            "poolWithdrawQueue",     // 8
            "poolTempLpTokenAccount",// 9
            "serumProgram",          // 10
            "serumMarket",           // 11
            "serumCoinVaultAccount", // 12
            "serumPcVaultAccount",   // 13
            "serumVaultSigner",      // 14
            "userLpTokenAccount",    // 15
            "userCoinTokenAccount",   // 16 - Typo in IDL? Assuming userCoinTokenAccount
            "userPcTokenAccount",     // 17 - Typo in IDL? Assuming userPcTokenAccount
            "userOwner",             // 18
            "serumEventQ",           // 19
            "serumBids",             // 20
            "serumAsks"              // 21
        ],
        "migrateToOpenBook" => &[
            "tokenProgram",          // 0
            "systemProgram",         // 1
            "rent",                  // 2
            "amm",                   // 3
            "ammAuthority",          // 4
            "ammOpenOrders",         // 5
            "ammTokenCoin",          // 6
            "ammTokenPc",            // 7
            "ammTargetOrders",       // 8
            "serumProgram",          // 9
            "serumMarket",           // 10
            "serumBids",             // 11
            "serumAsks",             // 12
            "serumEventQueue",       // 13
            "serumCoinVault",        // 14
            "serumPcVault",          // 15
            "serumVaultSigner",      // 16
            "newAmmOpenOrders",      // 17
            "newSerumProgram",       // 18
            "newSerumMarket",        // 19
            "admin"                  // 20
        ],
        "setParams" => &[
            "tokenProgram",          // 0
            "amm",                   // 1
            "ammAuthority",          // 2
            "ammOpenOrders",         // 3
            "ammTargetOrders",       // 4
            "ammCoinVault",          // 5
            "ammPcVault",            // 6
            "serumProgram",          // 7
            "serumMarket",           // 8
            "serumCoinVault",        // 9
            "serumPcVault",          // 10
            "serumVaultSigner",      // 11
            "serumEventQueue",       // 12
            "serumBids",             // 13
            "serumAsks",             // 14
            "ammAdminAccount"        // 15
        ],
        "withdrawPnl" => &[
            "tokenProgram",          // 0
            "amm",                   // 1
            "ammConfig",             // 2
            "ammAuthority",          // 3
            "ammOpenOrders",         // 4
            "poolCoinTokenAccount",  // 5
            "poolPcTokenAccount",    // 6
            "coinPnlTokenAccount",   // 7
            "pcPnlTokenAccount",     // 8
            "pnlOwnerAccount",       // 9
            "ammTargetOrders",       // 10
            "serumProgram",          // 11
            "serumMarket",           // 12
            "serumEventQueue",       // 13
            "serumCoinVaultAccount", // 14
            "serumPcVaultAccount",   // 15
            "serumVaultSigner"       // 16
        ],
        "withdrawSrm" => &[
            "tokenProgram",  // 0
            "amm",           // 1
            "ammOwnerAccount", // 2
            "ammAuthority",  // 3
            "srmToken",      // 4
            "destSrmToken"   // 5
        ],
        "preInitialize" => &[
            "tokenProgram",           // 0
            "systemProgram",          // 1
            "rent",                   // 2
            "ammTargetOrders",        // 3
            "poolWithdrawQueue",      // 4
            "ammAuthority",           // 5
            "lpMintAddress",          // 6
            "coinMintAddress",        // 7
            "pcMintAddress",          // 8
            "poolCoinTokenAccount",   // 9
            "poolPcTokenAccount",     // 10
            "poolTempLpTokenAccount", // 11
            "serumMarket",            // 12
            "userWallet"              // 13
        ],
        "simulateInfo" => &[
            "amm",                  // 0
            "ammAuthority",         // 1
            "ammOpenOrders",        // 2
            "poolCoinTokenAccount", // 3
            "poolPcTokenAccount",   // 4
            "lpMintAddress",        // 5
            "serumMarket",          // 6
            "serumEventQueue"       // 7
        ],
        "adminCancelOrders" => &[
            "tokenProgram",          // 0
            "amm",                   // 1
            "ammAuthority",          // 2
            "ammOpenOrders",         // 3
            "ammTargetOrders",       // 4
            "poolCoinTokenAccount",  // 5
            "poolPcTokenAccount",    // 6
            "ammOwnerAccount",       // 7
            "ammConfig",             // 8
            "serumProgram",          // 9
            "serumMarket",           // 10
            "serumCoinVaultAccount", // 11
            "serumPcVaultAccount",   // 12
            "serumVaultSigner",      // 13
            "serumEventQ",           // 14
            "serumBids",             // 15
            "serumAsks"              // 16
        ],
        "createConfigAccount" => &[
            "admin",         // 0
            "ammConfig",     // 1
            "owner",         // 2
            "systemProgram", // 3
            "rent"           // 4
        ],
        "updateConfigAccount" => &[
            "admin",     // 0
            "ammConfig"  // 1
        ],
        // Keep conditional logic for swap instructions
        "swapBaseIn" | "swapBaseOut" => {
            if instruction.accounts.len() == 18 {
                &[
                    "tokenProgram", "amm", "ammAuthority", "ammOpenOrders", 
                    "ammTargetOrders", "poolCoinTokenAccount", "poolPcTokenAccount", 
                    "serumProgram", "serumMarket", "serumBids", "serumAsks", 
                    "serumEventQueue", "serumCoinVaultAccount", "serumPcVaultAccount", 
                    "serumVaultSigner", "userSourceTokenAccount", // Typo from IDL?
                    "userDestinationTokenAccount", // Typo from IDL?
                    "userSourceOwner"
                ]
            } else {
                 &[
                    "tokenProgram", "amm", "ammAuthority", "ammOpenOrders", 
                    /* "ammTargetOrders", */ "poolCoinTokenAccount", "poolPcTokenAccount", 
                    "serumProgram", "serumMarket", "serumBids", "serumAsks", 
                    "serumEventQueue", "serumCoinVaultAccount", "serumPcVaultAccount", 
                    "serumVaultSigner", "userSourceTokenAccount", // Typo from IDL?
                    "userDestinationTokenAccount", // Typo from IDL?
                    "userSourceOwner"
                ]
            }
        },
        // Default for any instruction not explicitly listed (e.g., if new ones are added)
        _ => &[]
    };

    // Mapping loop remains the same
    for (i, acc_idx_u8) in instruction.accounts.iter().enumerate() {
        let acc_idx = *acc_idx_u8 as usize;
        if let Some(name) = expected_names.get(i) {
            if let Some(address) = accounts.get(acc_idx) {
                // Handle potential typos found in IDL
                let final_name = match *name { 
                    "uerCoinTokenAccount" => "userCoinTokenAccount",
                    "uerPcTokenAccount" => "userPcTokenAccount",
                    "uerSourceTokenAccount" => "userSourceTokenAccount",
                    "uerDestinationTokenAccount" => "userDestinationTokenAccount",
                    other => other
                };
                input_accounts.insert(final_name.to_string(), address.clone());
            } else {
                 input_accounts.insert(format!("error_invalid_index_{}", i), acc_idx.to_string());
            }
        } else {
             input_accounts.insert(format!("unexpected_account_{}", i), 
                 accounts.get(acc_idx).cloned().unwrap_or_else(|| "invalid_index".to_string()));
        }
    }

    input_accounts
}


// Helper function to parse instruction arguments
fn parse_instruction_args(
    instruction: &CompiledInstruction,
    program_id: &str
) -> (&'static str, Option<InstructionArgs>) {
    if program_id != RAYDIUM_AMM_PROGRAM_ID { return ("unknown", None); }

    let data = &instruction.data;
    if data.is_empty() { return ("unknown", None); }

    let discriminator = data[0];

    match discriminator {
        INITIALIZE_DISC => {
            let args = PbInitializeLayout {
                nonce: read_u8(data, 1).map(|v| v as u32).unwrap_or_default(),
                open_time: read_u64(data, 2).unwrap_or_default(),
            };
            ("initialize", Some(InstructionArgs { instruction: Some(instruction_args::Instruction::Initialize(args)) }))
        }
        INITIALIZE2_DISC => {
            let args = PbInitialize2Layout {
                nonce: read_u8(data, 1).map(|v| v as u32).unwrap_or_default(),
                open_time: read_u64(data, 2).unwrap_or_default(),
                init_pc_amount: read_u64(data, 10).unwrap_or_default(),
                init_coin_amount: read_u64(data, 18).unwrap_or_default(),
            };
            ("initialize2", Some(InstructionArgs { instruction: Some(instruction_args::Instruction::Initialize2(args)) }))
        }
        MONITOR_STEP_DISC => {
            let args = PbMonitorStepLayout {
                plan_order_limit: read_u16(data, 1).map(|v| v as u32).unwrap_or_default(),
                place_order_limit: read_u16(data, 3).map(|v| v as u32).unwrap_or_default(),
                cancel_order_limit: read_u16(data, 5).map(|v| v as u32).unwrap_or_default(),
            };
            ("monitorStep", Some(InstructionArgs { instruction: Some(instruction_args::Instruction::MonitorStep(args)) }))
        }
        DEPOSIT_DISC => {
            let args = PbDepositLayout {
                max_coin_amount: read_u64(data, 1).unwrap_or_default(),
                max_pc_amount: read_u64(data, 9).unwrap_or_default(),
                base_side: read_u64(data, 17).unwrap_or_default(),
            };
            ("deposit", Some(InstructionArgs { instruction: Some(instruction_args::Instruction::Deposit(args)) }))
        }
        WITHDRAW_DISC => {
            let args = PbWithdrawLayout {
                amount: read_u64(data, 1).unwrap_or_default(),
            };
            ("withdraw", Some(InstructionArgs { instruction: Some(instruction_args::Instruction::Withdraw(args)) }))
        }
        MIGRATE_TO_OPEN_BOOK_DISC => {
            let args = PbMigrateToOpenBookLayout {};
            ("migrateToOpenBook", Some(InstructionArgs { instruction: Some(instruction_args::Instruction::MigrateToOpenBook(args)) }))
        }
        SET_PARAMS_DISC => {
            let args = create_set_params_args(data);
            ("setParams", Some(InstructionArgs { instruction: Some(instruction_args::Instruction::SetParams(args)) }))
        }
        WITHDRAW_PNL_DISC => {
            let args = PbWithdrawPnlLayout {};
            ("withdrawPnl", Some(InstructionArgs { instruction: Some(instruction_args::Instruction::WithdrawPnl(args)) }))
        }
        WITHDRAW_SRM_DISC => {
            let args = PbWithdrawSrmLayout {
                amount: read_u64(data, 1).unwrap_or_default(),
            };
            ("withdrawSrm", Some(InstructionArgs { instruction: Some(instruction_args::Instruction::WithdrawSrm(args)) }))
        }
        SWAP_BASE_IN_DISC => {
            let args = PbSwapBaseInLayout {
                amount_in: read_u64(data, 1).unwrap_or_default(),
                minimum_amount_out: read_u64(data, 9).unwrap_or_default(),
            };
            ("swapBaseIn", Some(InstructionArgs { instruction: Some(instruction_args::Instruction::SwapBaseIn(args)) }))
        }
        PRE_INITIALIZE_DISC => {
            let args = PbPreInitializeLayout {
                nonce: read_u8(data, 1).map(|v| v as u32).unwrap_or_default(),
            };
            ("preInitialize", Some(InstructionArgs { instruction: Some(instruction_args::Instruction::PreInitialize(args)) }))
        }
        SWAP_BASE_OUT_DISC => {
            let args = PbSwapBaseOutLayout {
                max_amount_in: read_u64(data, 1).unwrap_or_default(),
                amount_out: read_u64(data, 9).unwrap_or_default(),
            };
            ("swapBaseOut", Some(InstructionArgs { instruction: Some(instruction_args::Instruction::SwapBaseOut(args)) }))
        }
        SIMULATE_INFO_DISC => {
            substreams::log::info!("Parsing simulateInfo (DISC 12): data length = {}", data.len());
            
            let param_val = read_u8(data, 1).map(|v| v as u32).unwrap_or_default();
            let swap_in_opt = read_swap_base_in(data, 2);
            let swap_out_opt = read_swap_base_out(data, 18);

            substreams::log::info!(
                "simulateInfo (DISC 12) parsed values: param={}, swap_in_present={}, swap_out_present={}",
                param_val,
                swap_in_opt.is_some(),
                swap_out_opt.is_some()
            );
            
            let args = PbSimulateInfoLayout {
                param: param_val,
                swap_base_in_value: swap_in_opt, 
                swap_base_out_value: swap_out_opt, 
            };
            ("simulateInfo", Some(InstructionArgs { instruction: Some(instruction_args::Instruction::SimulateInfo(args)) }))
        }
        ADMIN_CANCEL_ORDERS_DISC => {
            let args = PbAdminCancelOrdersLayout {
                limit: read_u16(data, 1).map(|v| v as u32).unwrap_or_default(),
            };
            ("adminCancelOrders", Some(InstructionArgs { instruction: Some(instruction_args::Instruction::AdminCancelOrders(args)) }))
        }
        CREATE_CONFIG_ACCOUNT_DISC => {
            let args = PbCreateConfigAccountLayout {};
            ("createConfigAccount", Some(InstructionArgs { instruction: Some(instruction_args::Instruction::CreateConfigAccount(args)) }))
        }
        UPDATE_CONFIG_ACCOUNT_DISC => {
            let args = PbUpdateConfigAccountLayout {
                param: read_u8(data, 1).map(|v| v as u32).unwrap_or_default(),
                owner: read_pubkey(data, 2).unwrap_or_default(),
            };
            ("updateConfigAccount", Some(InstructionArgs { instruction: Some(instruction_args::Instruction::UpdateConfigAccount(args)) }))
        }
        // Any discriminator not explicitly handled falls into unknown
        _ => ("unknown", None),
    }
}


// Helper function to create FlatArg from the InstructionArgs oneof
fn create_flat_arg(_instruction_name: &str, args_opt: &Option<InstructionArgs>) -> Option<FlatArg> {
    let mut flat_arg = FlatArg::default();

    if let Some(instruction_args) = args_opt {
        if let Some(specific_args) = &instruction_args.instruction {
            match specific_args {
                instruction_args::Instruction::Initialize(init) => {
                    flat_arg.nonce = Some(init.nonce);
                    flat_arg.open_time = Some(init.open_time);
                },
                instruction_args::Instruction::Initialize2(init2) => {
                    flat_arg.nonce = Some(init2.nonce);
                    flat_arg.open_time = Some(init2.open_time);
                    flat_arg.init_pc_amount = Some(init2.init_pc_amount);
                    flat_arg.init_coin_amount = Some(init2.init_coin_amount);
                },
                instruction_args::Instruction::MonitorStep(step) => {
                    flat_arg.plan_order_limit = Some(step.plan_order_limit);
                    flat_arg.place_order_limit = Some(step.place_order_limit);
                    flat_arg.cancel_order_limit = Some(step.cancel_order_limit);
                },
                instruction_args::Instruction::Deposit(deposit) => {
                    flat_arg.max_coin_amount = Some(deposit.max_coin_amount);
                    flat_arg.max_pc_amount = Some(deposit.max_pc_amount);
                    flat_arg.base_side = Some(deposit.base_side);
                },
                instruction_args::Instruction::Withdraw(withdraw) => {
                    flat_arg.amount = Some(withdraw.amount);
                },
                instruction_args::Instruction::SetParams(params) => {
                    flat_arg.param = Some(params.param);
                    flat_arg.value = params.value;
                    flat_arg.new_pubkey = params.new_pubkey.clone();
                    flat_arg.fees = params.fees.clone();
                    flat_arg.last_order_distance = params.last_order_distance.clone();
                    flat_arg.need_take_amounts = params.need_take_amounts.clone();
                },
                instruction_args::Instruction::WithdrawSrm(srm) => {
                    flat_arg.amount = Some(srm.amount);
                },
                instruction_args::Instruction::SwapBaseIn(swap) => {
                    flat_arg.amount_in = Some(swap.amount_in);
                    flat_arg.minimum_amount_out = Some(swap.minimum_amount_out);
                },
                instruction_args::Instruction::PreInitialize(pre) => {
                    flat_arg.nonce = Some(pre.nonce);
                },
                instruction_args::Instruction::SwapBaseOut(swap) => {
                    flat_arg.max_amount_in = Some(swap.max_amount_in);
                    flat_arg.amount_out = Some(swap.amount_out);
                },
                instruction_args::Instruction::SimulateInfo(info) => {
                    flat_arg.param = Some(info.param);
                    flat_arg.swap_base_in_value = info.swap_base_in_value.clone();
                    flat_arg.swap_base_out_value = info.swap_base_out_value.clone();
                },
                instruction_args::Instruction::AdminCancelOrders(cancel) => {
                    flat_arg.limit = Some(cancel.limit);
                },
                instruction_args::Instruction::UpdateConfigAccount(update) => {
                    flat_arg.param = Some(update.param);
                    flat_arg.owner = Some(update.owner.clone());
                },
                _ => {}
            }
        } else {
            return None;
        }
    } else {
        return None;
    }

    Some(flat_arg)
}


// Main function to process an instruction and return a Meta object
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
    let program_id = accounts.get(instruction.program_id_index as usize)?;
    if program_id != RAYDIUM_AMM_PROGRAM_ID {
        return None;
    }

    let dt = chrono::DateTime::from_timestamp(block_timestamp, 0)?;
    let block_date = dt.format("%Y-%m-%d").to_string();

    let (instruction_name, instruction_args_opt) = parse_instruction_args(instruction, program_id);
    if instruction_name == "unknown" { return None; }

    let flat_arg_opt = create_flat_arg(instruction_name, &instruction_args_opt);
    let input_accounts = prepare_input_accounts(instruction, accounts, instruction_name);

    // Determine the correct value for the outer_program field
    let final_outer_program = if is_inner_instruction {
        outer_program.unwrap_or_default() // Use the provided outer program ID (or default to "")
    } else {
        program_id // For outer instructions, use its own program ID
    };

    // Create the Meta object
    let meta = Meta {
        block_date: Some(block_date),
        block_time: Some(block_timestamp),
        block_slot: Some(block_slot),
        tx_id: Some(tx_id.to_string()),
        instruction_index: Some(instruction_index),
        is_inner_instruction: Some(is_inner_instruction),
        inner_instruction_index: Some(inner_instruction_index.unwrap_or(0)),
        dapp: Some(program_id.to_string()),
        instruction_type: Some(instruction_name.to_string()),
        args: flat_arg_opt,
        input_accounts: input_accounts,
        signer: Some(signer.unwrap_or_default().to_string()),
        outer_program: Some(final_outer_program.to_string()),
    };

    Some(meta)
}

// Helper function to create PbSetParamsLayout from data (needed for parsing)
fn create_set_params_args(data: &[u8]) -> PbSetParamsLayout {
    PbSetParamsLayout {
        param: read_u8(data, 1).map(|v| v as u32).unwrap_or(0),
        value: read_option_u64(data, 2),
        new_pubkey: read_option_pubkey(data, 10),
        fees: read_fees(data, 42),
        last_order_distance: read_last_order_distance(data, 82),
        need_take_amounts: read_need_take(data, 90),
    }
} 