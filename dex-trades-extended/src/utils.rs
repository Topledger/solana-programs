extern crate chrono;
use borsh::{BorshDeserialize, BorshSerialize};
use chrono::prelude::*;
use std::collections::HashSet;
use substreams_solana::pb::sf::solana::r#type::v1::{
    CompiledInstruction, InnerInstructions, TokenBalance,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct TransferLayout {
    amount: u64,
}

pub fn convert_to_date(ts: i64) -> String {
    let nt = NaiveDateTime::from_timestamp_opt(ts, 0);
    let dt: DateTime<Utc> = DateTime::from_naive_utc_and_offset(nt.unwrap(), Utc);
    let res = dt.format("%Y-%m-%d");
    return res.to_string();
}

pub fn get_mint(
    address: &String,
    token_balances: &Vec<TokenBalance>,
    accounts: &Vec<String>,
    dapp_address: String,
) -> String {
    if dapp_address.eq("MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG")
        || dapp_address.eq("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P")
    {
        return "So11111111111111111111111111111111111111112".to_string();
    }

    let index = accounts.iter().position(|r| r == address).unwrap();
    let mut result: String = String::new();

    token_balances
        .iter()
        .filter(|token_balance| token_balance.account_index == index as u32)
        .for_each(|token_balance| {
            result = token_balance.mint.clone();
        });
    return result;
}

pub fn get_trader_account(
    valut_a: &String,
    valut_b: &String,
    input_inner_idx: u32,
    inner_instructions: &Vec<InnerInstructions>,
    accounts: &Vec<String>,
    post_token_balances: &Vec<TokenBalance>,
    dapp_address: String,
    pre_balances: Vec<u64>,
    post_balances: Vec<u64>,
) -> String {
    let (_, destination_token_account) = get_token_transfer(
        valut_a,
        input_inner_idx,
        inner_instructions,
        accounts,
        "source".to_string(),
        dapp_address.clone(),
        pre_balances.clone(),
        post_balances.clone(),
    );

    let (_, source_token_account) = get_token_transfer(
        valut_b,
        input_inner_idx,
        inner_instructions,
        accounts,
        "destination".to_string(),
        dapp_address.clone(),
        pre_balances.clone(),
        post_balances.clone(),
    );

    let mut owner_destination_token_account: String = String::new();
    let mut owner_source_token_account: String = String::new();

    if !destination_token_account.is_empty() {
        let index_destination_token_account = accounts
            .iter()
            .position(|r| r == &destination_token_account)
            .unwrap();
        post_token_balances
            .iter()
            .filter(|token_balance| {
                token_balance.account_index == index_destination_token_account as u32
            })
            .for_each(|token_balance| {
                owner_destination_token_account = token_balance.owner.clone();
            });
    }

    if !source_token_account.is_empty() {
        let index_source_token_account = accounts
            .iter()
            .position(|r| r == &source_token_account)
            .unwrap();
        post_token_balances
            .iter()
            .filter(|token_balance| {
                token_balance.account_index == index_source_token_account as u32
            })
            .for_each(|token_balance| {
                owner_source_token_account = token_balance.owner.clone();
            });
    }

    if !owner_destination_token_account.is_empty() {
        return owner_destination_token_account;
    }

    if !owner_source_token_account.is_empty() {
        return owner_source_token_account;
    }
    return accounts.get(0).unwrap().to_string();
}

pub fn get_amt(
    address: &String,
    input_inner_idx: u32,
    inner_instructions: &Vec<InnerInstructions>,
    accounts: &Vec<String>,
    post_token_balances: &Vec<TokenBalance>,
    dapp_address: String,
    pre_balances: Vec<u64>,
    post_balances: Vec<u64>,
) -> f64 {
    let mut result: f64 = 0.0;

    let (source_transfer_amt, _) = get_token_transfer(
        address,
        input_inner_idx,
        inner_instructions,
        accounts,
        "source".to_string(),
        dapp_address.clone(),
        pre_balances.clone(),
        post_balances.clone(),
    );

    let (destination_transfer_amt, _) = get_token_transfer(
        address,
        input_inner_idx,
        inner_instructions,
        accounts,
        "destination".to_string(),
        dapp_address.clone(),
        pre_balances.clone(),
        post_balances.clone(),
    );

    if source_transfer_amt != 0.0 {
        result = source_transfer_amt;
    } else if destination_transfer_amt != 0.0 {
        result = destination_transfer_amt;
    }

    if result != 0.0 {
        let index = accounts.iter().position(|r| r == address).unwrap();
        post_token_balances
            .iter()
            .filter(|token_balance| token_balance.account_index == index as u32)
            .for_each(|token_balance: &TokenBalance| {
                let decimals = token_balance.ui_token_amount.clone().unwrap().decimals;
                result = result / (u64::pow(10, decimals)) as f64;
            });
    }

    result
}

pub fn get_token_transfer(
    address: &String,
    input_inner_idx: u32,
    inner_instructions: &Vec<InnerInstructions>,
    accounts: &Vec<String>,
    account_name_to_check: String,
    dapp_address: String,
    pre_balances: Vec<u64>,
    post_balances: Vec<u64>,
) -> (f64, String) {
    if dapp_address.eq("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P") {
        return get_system_program_transfer(
            address,
            input_inner_idx,
            inner_instructions,
            accounts,
            account_name_to_check,
            pre_balances,
            post_balances,
        );
    }

    let mut result = 0.0;
    let mut token_account = "".to_string();
    let mut result_assigned = false;

    inner_instructions.iter().for_each(|inner_instruction| {
        inner_instruction
            .instructions
            .iter()
            .enumerate()
            .for_each(|(inner_idx, inner_inst)| {
                let inner_program = &accounts[inner_inst.program_id_index as usize];

                if inner_program
                    .as_str()
                    .eq("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
                {
                    let (discriminator_bytes, rest) = inner_inst.data.split_at(1);
                    let discriminator: u8 = u8::from(discriminator_bytes[0]);

                    match discriminator {
                        3 => {
                            let input_accounts =
                                prepare_input_accounts(&inner_inst.accounts, accounts);

                            let source = input_accounts.get(0).unwrap().to_string();
                            let destination = input_accounts.get(1).unwrap().to_string();

                            let condition = if input_inner_idx > 0 {
                                inner_idx as u32 > input_inner_idx
                            } else {
                                true
                            };

                            if condition && address.eq(&source) {
                                let data = TransferLayout::deserialize(&mut rest.clone()).unwrap();
                                if !result_assigned {
                                    result = -1.0 * data.amount as f64;
                                    token_account = destination.clone();
                                    result_assigned = true;
                                }
                            }

                            if condition && address.eq(&destination) {
                                let data = TransferLayout::deserialize(&mut rest.clone()).unwrap();
                                if !result_assigned {
                                    result = data.amount as f64;
                                    token_account = source.clone();
                                    result_assigned = true;
                                }
                            }
                        }
                        12 => {
                            let input_accounts =
                                prepare_input_accounts(&inner_inst.accounts, accounts);

                            let source = input_accounts.get(0).unwrap().to_string();
                            let destination = input_accounts.get(2).unwrap().to_string();

                            let condition = if input_inner_idx > 0 {
                                inner_idx as u32 > input_inner_idx
                            } else {
                                true
                            };

                            if condition && address.eq(&source) {
                                let data = TransferLayout::deserialize(&mut rest.clone()).unwrap();
                                if !result_assigned {
                                    result = -1.0 * data.amount as f64;
                                    token_account = destination.clone();
                                    result_assigned = true;
                                }
                            }

                            if condition && address.eq(&destination) {
                                let data = TransferLayout::deserialize(&mut rest.clone()).unwrap();
                                if !result_assigned {
                                    result = data.amount as f64;
                                    token_account = source.clone();
                                    result_assigned = true;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            })
    });

    if !result_assigned {
        let _result = get_token_22_transfer(
            address,
            input_inner_idx,
            inner_instructions,
            accounts,
            account_name_to_check,
        );
        if _result.is_some() {
            result = _result.clone().unwrap().0;
            token_account = _result.clone().unwrap().1;
        }
    }

    (result, token_account)
}

pub fn get_token_22_transfer(
    address: &String,
    input_inner_idx: u32,
    inner_instructions: &Vec<InnerInstructions>,
    accounts: &Vec<String>,
    account_name_to_check: String,
) -> Option<(f64, String)> {
    let mut result = 0.0;
    let mut token_account = "".to_string();
    let mut result_assigned = false;

    inner_instructions.iter().for_each(|inner_instruction| {
        inner_instruction
            .instructions
            .iter()
            .enumerate()
            .for_each(|(inner_idx, inner_inst)| {
                let inner_program = &accounts[inner_inst.program_id_index as usize];

                if inner_program
                    .as_str()
                    .eq("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb")
                {
                    let (discriminator_bytes, rest) = inner_inst.data.split_at(1);
                    let discriminator: u8 = u8::from(discriminator_bytes[0]);

                    match discriminator {
                        3 => {
                            let input_accounts =
                                prepare_input_accounts(&inner_inst.accounts, accounts);

                            let source = input_accounts.get(0).unwrap().to_string();
                            let destination = input_accounts.get(1).unwrap().to_string();

                            let condition = if input_inner_idx > 0 {
                                inner_idx as u32 > input_inner_idx
                            } else {
                                true
                            };

                            if condition && address.eq(&source) {
                                let data = TransferLayout::deserialize(&mut rest.clone()).unwrap();
                                if !result_assigned {
                                    result = -1.0 * data.amount as f64;
                                    token_account = destination.clone();
                                    result_assigned = true;
                                }
                            }

                            if condition && address.eq(&destination) {
                                let data = TransferLayout::deserialize(&mut rest.clone()).unwrap();
                                if !result_assigned {
                                    result = data.amount as f64;
                                    token_account = source.clone();
                                    result_assigned = true;
                                }
                            }
                        }
                        12 => {
                            let input_accounts =
                                prepare_input_accounts(&inner_inst.accounts, accounts);

                            let source = input_accounts.get(0).unwrap().to_string();
                            let destination = input_accounts.get(2).unwrap().to_string();

                            let condition = if input_inner_idx > 0 {
                                inner_idx as u32 > input_inner_idx
                            } else {
                                true
                            };

                            if condition && address.eq(&source) {
                                let data = TransferLayout::deserialize(&mut rest.clone()).unwrap();
                                if !result_assigned {
                                    result = -1.0 * data.amount as f64;
                                    token_account = destination.clone();
                                    result_assigned = true;
                                }
                            }

                            if condition && address.eq(&destination) {
                                let data = TransferLayout::deserialize(&mut rest.clone()).unwrap();
                                if !result_assigned {
                                    result = data.amount as f64;
                                    token_account = source.clone();
                                    result_assigned = true;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            })
    });

    Some((result, token_account))
}

fn get_system_program_transfer(
    address: &String,
    input_inner_idx: u32,
    inner_instructions: &Vec<InnerInstructions>,
    accounts: &Vec<String>,
    account_name_to_check: String,
    pre_balances: Vec<u64>,
    post_balances: Vec<u64>,
) -> (f64, String) {
    let mut result = 0.0;
    let mut token_account = "".to_string();
    let mut result_assigned = false;

    inner_instructions.iter().for_each(|inner_instruction| {
        inner_instruction
            .instructions
            .iter()
            .enumerate()
            .for_each(|(inner_idx, inner_inst)| {
                let inner_program = &accounts[inner_inst.program_id_index as usize];

                if inner_program
                    .as_str()
                    .eq("11111111111111111111111111111111")
                {
                    let (discriminator_bytes, rest) = inner_inst.data.split_at(4);

                    let disc_bytes_arr: [u8; 4] = discriminator_bytes.to_vec().try_into().unwrap();
                    let discriminator: u32 = u32::from_le_bytes(disc_bytes_arr);

                    match discriminator {
                        2 => {
                            let input_accounts =
                                prepare_input_accounts(&inner_inst.accounts, accounts);

                            let source = input_accounts.get(0).unwrap().to_string();
                            let destination = input_accounts.get(1).unwrap().to_string();

                            let condition = if input_inner_idx > 0 {
                                inner_idx as u32 > input_inner_idx
                            } else {
                                true
                            };

                            if condition && address.eq(&source) {
                                let data = TransferLayout::deserialize(&mut rest.clone()).unwrap();
                                if !result_assigned {
                                    result = -1.0 * data.amount as f64;
                                    result /= 10f64.powi(9);
                                    token_account = destination.clone();
                                    result_assigned = true;
                                }
                            }

                            if condition && address.eq(&destination) {
                                let data = TransferLayout::deserialize(&mut rest.clone()).unwrap();
                                if !result_assigned {
                                    result = 1.0 * data.amount as f64;
                                    result /= 10f64.powi(9);
                                    token_account = source.clone();
                                    result_assigned = true;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            })
    });

    if !result_assigned {
        let index = accounts.iter().position(|r| r == address).unwrap();
        let _result = post_balances[index] as f64 - pre_balances[index] as f64;
        result = 1.0 * _result as f64;
        result /= 10f64.powi(9);
    }

    (result, token_account)
}

fn prepare_input_accounts(account_indices: &Vec<u8>, accounts: &Vec<String>) -> Vec<String> {
    let mut instruction_accounts: Vec<String> = vec![];
    for (index, &el) in account_indices.iter().enumerate() {
        instruction_accounts.push(accounts.as_slice()[el as usize].to_string());
    }
    return instruction_accounts;
}

pub fn get_outer_executing_accounts(
    instructions: &Vec<CompiledInstruction>,
    accounts: &Vec<String>,
) -> Vec<String> {
    let mut unique_ids = HashSet::new();
    for instruction in instructions.iter() {
        let executing_account = &accounts[instruction.program_id_index as usize];
        unique_ids.insert(executing_account.to_string());
    }

    Vec::from_iter(unique_ids)
}
