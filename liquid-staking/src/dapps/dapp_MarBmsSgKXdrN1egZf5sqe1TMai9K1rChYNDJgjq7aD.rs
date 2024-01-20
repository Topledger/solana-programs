use borsh::{BorshDeserialize, BorshSerialize};
use substreams_solana::pb::sf::solana::r#type::v1::{InnerInstructions, TokenBalance};

use crate::{pb::sf::solana::liquid::staking::v1::TradeData, utils::prepare_input_accounts};

const DEPOSIT_DISCRIMINATOR: u64 = 13182846803881894898;
const DEPOSIT_STAKE_ACCOUNT_DISCRIMINATOR: u64 = 4252073853447275118;
const CLAIM_DISCRIMINATOR: u64 = 15162669785878545982;
const ORDER_UNSTAKE_DISCRIMINATOR: u64 = 2630311593909462881;
const UPDATE_DEACTIVATED_DISCRIMINATOR: u64 = 3670262844445943824;
const UPDATE_ACTIVE_DISCRIMINATOR: u64 = 10979201432142562052;
const LIQUID_UNSTAKE_DISCRIMINATOR: u64 = 1156549617839971870;

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct MintToLayout {
    amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct BurnLayout {
    amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct SystemProgramTransferLayout {
    amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct SPLTokenTransferLayout {
    amount: u64,
}

pub fn parse_logs(log_messages: &Vec<String>) -> Option<f64> {
    let mut result: Option<f64> = None;

    for log_message in log_messages {
        if log_message.starts_with("Program log: ") & log_message.contains("Staking rewards") {
            let staking_reward_value = log_message
                .replace("Program log: Staking rewards: ", "")
                .trim()
                .to_string();
            let staking_reward_numeric: f64 = staking_reward_value.to_string().parse().unwrap();
            result = Some(staking_reward_numeric);
        }
    }

    return result;
}

pub fn enrich_with_logs_data(trade_data: &mut TradeData, log_messages: &Vec<String>) -> () {
    let log_data = parse_logs(log_messages);
    if log_data.is_some() {
        let log_data_unwraped = log_data.unwrap();
        trade_data.staking_reward = log_data_unwraped;
    }
}

pub fn enrich_with_inner_instructions_data(
    trade_data: &mut TradeData,
    accounts: &Vec<String>,
    inner_instructions: &Vec<InnerInstructions>,
) -> () {
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
                        7 => {
                            let input_accounts =
                                prepare_input_accounts(&inner_inst.accounts, accounts);
                            if input_accounts
                                .get(0)
                                .unwrap()
                                .to_string()
                                .eq(&trade_data.pool_mint)
                            {
                                let mint_to_data =
                                    MintToLayout::deserialize(&mut rest.clone()).unwrap();
                                trade_data.mint_amount = mint_to_data.amount as f64;
                            }
                        }
                        8 => {
                            let input_accounts =
                                prepare_input_accounts(&inner_inst.accounts, accounts);
                            if input_accounts
                                .get(1)
                                .unwrap()
                                .to_string()
                                .eq(&trade_data.pool_mint)
                            {
                                let burn_data = BurnLayout::deserialize(&mut rest.clone()).unwrap();
                                trade_data.burn_amount = burn_data.amount as f64;
                            }
                        }
                        _ => {}
                    }
                }
            })
    });
}

pub fn enrich_with_ix_details(
    trade_data: &mut TradeData,
    is_inner_instruction: bool,
    outer_idx: usize,
    inner_idx: usize,
    outer_program: &String,
    inner_program: &String,
) -> () {
    trade_data.is_inner_instruction = is_inner_instruction;
    trade_data.instruction_index = outer_idx as u32;
    trade_data.inner_instruction_index = inner_idx as u32;
    trade_data.outer_program = outer_program.to_string();
    if is_inner_instruction {
        trade_data.inner_program = inner_program.to_string();
    }
}

pub fn get_system_sol_transfer(
    trade_data: &mut TradeData,
    accounts: &Vec<String>,
    inner_instructions: &Vec<InnerInstructions>,
) -> f64 {
    let mut result = 0 as f64;

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
                            let destination = input_accounts.get(1).unwrap().to_string();
                            if destination.eq(trade_data.reserve_stake.as_str())
                                || destination.eq(trade_data.liq_pool_sol_leg.as_str())
                            {
                                let transfer_data =
                                    SystemProgramTransferLayout::deserialize(&mut rest.clone())
                                        .unwrap();
                                result = transfer_data.amount as f64;
                            }
                        }
                        _ => {}
                    }
                }
            })
    });

    result
}

pub fn get_spl_token_transfer(
    trade_data: &mut TradeData,
    accounts: &Vec<String>,
    inner_instructions: &Vec<InnerInstructions>,
) -> f64 {
    let mut result = 0 as f64;

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
                    let disc_bytes_arr: [u8; 1] = discriminator_bytes.to_vec().try_into().unwrap();
                    let discriminator: u8 = u8::from_le_bytes(disc_bytes_arr);
                    match discriminator {
                        3 => {
                            let input_accounts =
                                prepare_input_accounts(&inner_inst.accounts, accounts);
                            let destination = input_accounts.get(1).unwrap().to_string();
                            if destination.eq(trade_data.fee_account.as_str()) {
                                let transfer_data =
                                    SPLTokenTransferLayout::deserialize(&mut rest.clone()).unwrap();
                                result = transfer_data.amount as f64;
                            }
                        }
                        _ => {}
                    }
                }
            })
    });

    result
}

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    input_accounts: Vec<String>,
    accounts: &Vec<String>,
    log_messages: &Vec<String>,
    pre_balances: &Vec<u64>,
    post_balances: &Vec<u64>,
    pre_token_balances: &Vec<TokenBalance>,
    post_token_balances: &Vec<TokenBalance>,
    inner_instructions: &Vec<InnerInstructions>,
    outer_program: &String,
    outer_idx: usize,
    inner_idx: usize,
    is_inner_instruction: bool,
) -> Option<TradeData> {
    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let disc_bytes_arr: [u8; 8] = disc_bytes.to_vec().try_into().unwrap();
    let discriminator: u64 = u64::from_le_bytes(disc_bytes_arr);

    let mut result = None;
    let mut trade_data: TradeData;

    let inner_program = "MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD".to_string();

    match discriminator {
        DEPOSIT_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "Deposit".to_string();

            trade_data.stake_pool = input_accounts.get(0).unwrap().to_string();
            trade_data.withdraw_authority = "".to_string();
            trade_data.reserve_stake = input_accounts.get(5).unwrap().to_string();
            trade_data.validator_stake = "".to_string();
            trade_data.pool_mint = input_accounts.get(1).unwrap().to_string();
            trade_data.fee_account = "".to_string();
            trade_data.liq_pool_sol_leg = input_accounts.get(2).unwrap().to_string();

            trade_data.staking_reward = 0.0;
            trade_data.burn_amount = 0.0;
            enrich_with_inner_instructions_data(&mut trade_data, accounts, inner_instructions);
            trade_data.fee_amount = 0.0;

            trade_data.amount =
                get_system_sol_transfer(&mut trade_data, accounts, inner_instructions);

            enrich_with_ix_details(
                &mut trade_data,
                is_inner_instruction,
                outer_idx,
                inner_idx,
                outer_program,
                &inner_program,
            );

            result = Some(trade_data);
        }
        DEPOSIT_STAKE_ACCOUNT_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "DepositStakeAccount".to_string();

            trade_data.stake_pool = input_accounts.get(0).unwrap().to_string();
            trade_data.withdraw_authority = "".to_string();
            trade_data.reserve_stake = "".to_string();
            trade_data.validator_stake = "".to_string();
            trade_data.pool_mint = input_accounts.get(7).unwrap().to_string();
            trade_data.fee_account = "".to_string();

            trade_data.staking_reward = 0.0;
            trade_data.burn_amount = 0.0;
            enrich_with_inner_instructions_data(&mut trade_data, accounts, inner_instructions);
            trade_data.fee_amount = 0.0;
            trade_data.amount = 0.0;

            enrich_with_ix_details(
                &mut trade_data,
                is_inner_instruction,
                outer_idx,
                inner_idx,
                outer_program,
                &inner_program,
            );

            result = Some(trade_data);
        }
        CLAIM_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "Claim".to_string();

            trade_data.stake_pool = input_accounts.get(0).unwrap().to_string();
            trade_data.withdraw_authority = "".to_string();
            trade_data.reserve_stake = input_accounts.get(1).unwrap().to_string();
            trade_data.validator_stake = "".to_string();
            trade_data.pool_mint = "".to_string();
            trade_data.fee_account = "".to_string();

            trade_data.staking_reward = 0.0;
            trade_data.mint_amount = 0.0;
            trade_data.burn_amount = 0.0;
            trade_data.fee_amount = 0.0;
            trade_data.amount = -1.0
                * get_sol_balance_change(
                    &trade_data.reserve_stake,
                    accounts,
                    pre_balances,
                    post_balances,
                );

            enrich_with_ix_details(
                &mut trade_data,
                is_inner_instruction,
                outer_idx,
                inner_idx,
                outer_program,
                &inner_program,
            );

            result = Some(trade_data);
        }
        ORDER_UNSTAKE_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "OrderUnstake".to_string();

            trade_data.stake_pool = input_accounts.get(0).unwrap().to_string();
            trade_data.withdraw_authority = "".to_string();
            trade_data.reserve_stake = "".to_string();
            trade_data.validator_stake = "".to_string();
            trade_data.pool_mint = input_accounts.get(1).unwrap().to_string();
            trade_data.fee_account = "".to_string();

            trade_data.staking_reward = 0.0;
            trade_data.mint_amount = 0.0;
            enrich_with_inner_instructions_data(&mut trade_data, accounts, inner_instructions);
            trade_data.fee_amount = 0.0;
            trade_data.amount = 0.0;

            enrich_with_ix_details(
                &mut trade_data,
                is_inner_instruction,
                outer_idx,
                inner_idx,
                outer_program,
                &inner_program,
            );

            result = Some(trade_data);
        }
        UPDATE_DEACTIVATED_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "UpdateDeactivated".to_string();

            trade_data.stake_pool = input_accounts.get(0).unwrap().to_string();
            trade_data.withdraw_authority = "".to_string();
            trade_data.reserve_stake = input_accounts.get(4).unwrap().to_string();
            trade_data.validator_stake = "".to_string();
            trade_data.pool_mint = input_accounts.get(5).unwrap().to_string();
            trade_data.fee_account = input_accounts.get(7).unwrap().to_string();

            enrich_with_logs_data(&mut trade_data, log_messages);
            trade_data.mint_amount = 0.0;
            enrich_with_inner_instructions_data(&mut trade_data, accounts, inner_instructions);
            trade_data.burn_amount = 0.0;
            trade_data.fee_amount =
                get_spl_token_transfer(&mut trade_data, accounts, inner_instructions);
            trade_data.amount = 0.0;

            enrich_with_ix_details(
                &mut trade_data,
                is_inner_instruction,
                outer_idx,
                inner_idx,
                outer_program,
                &inner_program,
            );

            result = Some(trade_data);
        }
        UPDATE_ACTIVE_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "UpdateActive".to_string();

            trade_data.stake_pool = input_accounts.get(0).unwrap().to_string();
            trade_data.withdraw_authority = "".to_string();
            trade_data.reserve_stake = input_accounts.get(4).unwrap().to_string();
            trade_data.validator_stake = "".to_string();
            trade_data.pool_mint = input_accounts.get(5).unwrap().to_string();
            trade_data.fee_account = input_accounts.get(7).unwrap().to_string();

            enrich_with_logs_data(&mut trade_data, log_messages);
            trade_data.mint_amount = 0.0;
            enrich_with_inner_instructions_data(&mut trade_data, accounts, inner_instructions);
            trade_data.burn_amount = 0.0;
            trade_data.fee_amount =
                get_spl_token_transfer(&mut trade_data, accounts, inner_instructions);
            trade_data.amount = 0.0;

            enrich_with_ix_details(
                &mut trade_data,
                is_inner_instruction,
                outer_idx,
                inner_idx,
                outer_program,
                &inner_program,
            );

            result = Some(trade_data);
        }
        LIQUID_UNSTAKE_DISCRIMINATOR => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "LiquidUnstake".to_string();

            trade_data.stake_pool = input_accounts.get(0).unwrap().to_string();
            trade_data.withdraw_authority = "".to_string();
            trade_data.liq_pool_sol_leg = input_accounts.get(2).unwrap().to_string();
            trade_data.reserve_stake = "".to_string();
            trade_data.validator_stake = "".to_string();
            trade_data.pool_mint = input_accounts.get(1).unwrap().to_string();
            trade_data.fee_account = input_accounts.get(4).unwrap().to_string();

            trade_data.staking_reward = 0.0;
            trade_data.mint_amount = 0.0;
            trade_data.burn_amount = 0.0;
            trade_data.fee_amount =
                get_spl_token_transfer(&mut trade_data, accounts, inner_instructions);
            trade_data.amount = -1.0
                * get_sol_balance_change(
                    &trade_data.liq_pool_sol_leg,
                    accounts,
                    pre_balances,
                    post_balances,
                );

            enrich_with_ix_details(
                &mut trade_data,
                is_inner_instruction,
                outer_idx,
                inner_idx,
                outer_program,
                &inner_program,
            );

            result = Some(trade_data);
        }
        _ => {}
    }

    return result;
}

fn get_sol_balance_change(
    address: &String,
    accounts: &Vec<String>,
    pre_balances: &Vec<u64>,
    post_balances: &Vec<u64>,
) -> f64 {
    let index = accounts.iter().position(|r| r == address).unwrap();
    let pre_balance = pre_balances[index];
    let post_balance = post_balances[index];
    return post_balance as f64 - pre_balance as f64;
}
