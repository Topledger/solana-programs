extern crate chrono;
use borsh::{BorshDeserialize, BorshSerialize};
use chrono::prelude::*;

use crate::pb::sf::solana::transactions::v1::{InnerInstruction, TokenBalance};

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

    let mut result: String = String::new();

    token_balances
        .iter()
        .filter(|token_balance| token_balance.account.eq(address))
        .for_each(|token_balance| {
            result = token_balance.mint.clone();
        });
    return result;
}

pub fn get_amt(
    address: &String,
    input_inner_idx: u32,
    inner_instructions: &Vec<InnerInstruction>,
    accounts: &Vec<String>,
    post_token_balances: &Vec<TokenBalance>,
    dapp_address: String,
    pre_balances: Vec<u64>,
    post_balances: Vec<u64>,
) -> f64 {
    let mut result: f64 = 0.0;

    let source_transfer_amt = get_token_transfer(
        address,
        input_inner_idx,
        inner_instructions,
        accounts,
        "source".to_string(),
        dapp_address.clone(),
        pre_balances.clone(),
        post_balances.clone(),
    );

    let destination_transfer_amt = get_token_transfer(
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
        post_token_balances
            .iter()
            .filter(|token_balance| token_balance.mint.eq(address))
            .for_each(|token_balance: &TokenBalance| {
                // TODO: uncomment if decimals is available in token_balance
                // result = result / (u64::pow(10, decimals)) as f64;
            });
    }

    result
}

pub fn get_token_transfer(
    address: &String,
    input_inner_idx: u32,
    inner_instructions: &Vec<InnerInstruction>,
    accounts: &Vec<String>,
    account_name_to_check: String,
    dapp_address: String,
    pre_balances: Vec<u64>,
    post_balances: Vec<u64>,
) -> f64 {
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
    let mut result_assigned = false;

    inner_instructions
        .iter()
        .enumerate()
        .for_each(|(inner_idx, inner_instruction)| {
            let inner_program = inner_instruction.executing_account.clone();
            if inner_program
                .as_str()
                .eq("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
            {
                let b58_data = bs58::decode(inner_instruction.data.clone())
                    .into_vec()
                    .unwrap();
                let (discriminator_bytes, rest) = b58_data.split_at(1);
                let discriminator: u8 = u8::from(discriminator_bytes[0]);

                match discriminator {
                    3 => {
                        let input_accounts = inner_instruction.account_arguments.clone();

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
                                result_assigned = true;
                            }
                        }

                        if condition && address.eq(&destination) {
                            let data = TransferLayout::deserialize(&mut rest.clone()).unwrap();
                            if !result_assigned {
                                result = data.amount as f64;
                                result_assigned = true;
                            }
                        }
                    }
                    12 => {
                        let input_accounts = inner_instruction.account_arguments.clone();

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
                                result_assigned = true;
                            }
                        }

                        if condition && address.eq(&destination) {
                            let data = TransferLayout::deserialize(&mut rest.clone()).unwrap();
                            if !result_assigned {
                                result = data.amount as f64;
                                result_assigned = true;
                            }
                        }
                    }
                    _ => {}
                }
            }
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
            result = _result.unwrap();
        }
    }

    result
}

pub fn get_token_22_transfer(
    address: &String,
    input_inner_idx: u32,
    inner_instructions: &Vec<InnerInstruction>,
    accounts: &Vec<String>,
    account_name_to_check: String,
) -> Option<f64> {
    let mut result = None;
    let mut result_assigned = false;

    inner_instructions
        .iter()
        .enumerate()
        .for_each(|(inner_idx, inner_instruction)| {
            let inner_program = inner_instruction.executing_account.clone();

            if inner_program
                .as_str()
                .eq("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb")
            {
                let b58_data = bs58::decode(inner_instruction.data.clone())
                    .into_vec()
                    .unwrap();
                let (discriminator_bytes, rest) = b58_data.split_at(1);
                let discriminator: u8 = u8::from(discriminator_bytes[0]);

                match discriminator {
                    3 => {
                        let input_accounts = inner_instruction.account_arguments.clone();

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
                                result = Some(-1.0 * data.amount as f64);
                                result_assigned = true;
                            }
                        }

                        if condition && address.eq(&destination) {
                            let data = TransferLayout::deserialize(&mut rest.clone()).unwrap();
                            if !result_assigned {
                                result = Some(data.amount as f64);
                                result_assigned = true;
                            }
                        }
                    }
                    12 => {
                        let input_accounts = inner_instruction.account_arguments.clone();

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
                                result = Some(-1.0 * data.amount as f64);
                                result_assigned = true;
                            }
                        }

                        if condition && address.eq(&destination) {
                            let data = TransferLayout::deserialize(&mut rest.clone()).unwrap();
                            if !result_assigned {
                                result = Some(data.amount as f64);
                                result_assigned = true;
                            }
                        }
                    }
                    _ => {}
                }
            }
        });

    result
}

fn get_system_program_transfer(
    address: &String,
    input_inner_idx: u32,
    inner_instructions: &Vec<InnerInstruction>,
    accounts: &Vec<String>,
    account_name_to_check: String,
    pre_balances: Vec<u64>,
    post_balances: Vec<u64>,
) -> f64 {
    let mut result = 0.0;
    let mut result_assigned = false;

    &inner_instructions
        .iter()
        .enumerate()
        .for_each(|(inner_idx, inner_instruction)| {
            let inner_program = inner_instruction.executing_account.clone();

            if inner_program
                .as_str()
                .eq("11111111111111111111111111111111")
            {
                let b58_data = bs58::decode(inner_instruction.data.clone())
                    .into_vec()
                    .unwrap();
                let (discriminator_bytes, rest) = b58_data.split_at(4);

                let disc_bytes_arr: [u8; 4] = discriminator_bytes.to_vec().try_into().unwrap();
                let discriminator: u32 = u32::from_le_bytes(disc_bytes_arr);

                match discriminator {
                    2 => {
                        let input_accounts = inner_instruction.account_arguments.clone();

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
                                result_assigned = true;
                            }
                        }

                        if condition && address.eq(&destination) {
                            let data = TransferLayout::deserialize(&mut rest.clone()).unwrap();
                            if !result_assigned {
                                result = 1.0 * data.amount as f64;
                                result /= 10f64.powi(9);
                                result_assigned = true;
                            }
                        }
                    }
                    _ => {}
                }
            }
        });

    if !result_assigned {
        let index = accounts.iter().position(|r| r == address).unwrap();
        let _result = post_balances[index] as f64 - pre_balances[index] as f64;
        result = 1.0 * _result as f64;
        result /= 10f64.powi(9);
    }

    result
}
