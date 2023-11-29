use borsh::{BorshDeserialize, BorshSerialize};
use substreams_solana::pb::sf::solana::r#type::v1::{InnerInstructions, TokenBalance};

use crate::{
    pb::sf::solana::liquid::staking::v1::TradeData,
    utils::{get_token_balance_change, prepare_input_accounts},
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct MintToLayout {
    amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
struct BurnLayout {
    amount: u64,
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
                            let mint_to_data =
                                MintToLayout::deserialize(&mut rest.clone()).unwrap();
                            trade_data.mint_amount = mint_to_data.amount as f64;
                        }
                        8 => {
                            let input_accounts =
                                prepare_input_accounts(&inner_inst.accounts, accounts);
                            let burn_data = BurnLayout::deserialize(&mut rest.clone()).unwrap();
                            trade_data.burn_amount = burn_data.amount as f64;
                        }
                        _ => {}
                    }
                }
            })
    });
}

fn enrich_with_ix_data(trade_data: &mut TradeData, idx: u32) {
    trade_data.is_inner_instruction = false;
    trade_data.instruction_index = idx;
    trade_data.outer_program = "SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy".to_string();
}

pub fn parse_trade_instruction(
    bytes_stream: Vec<u8>,
    input_accounts: Vec<String>,
    accounts: &Vec<String>,
    pre_balances: &Vec<u64>,
    post_balances: &Vec<u64>,
    pre_token_balances: &Vec<TokenBalance>,
    post_token_balances: &Vec<TokenBalance>,
    inner_instructions: &Vec<InnerInstructions>,
    idx: usize,
) -> Option<TradeData> {
    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let discriminator: u8 = u8::from(disc_bytes[0]);

    let mut result = None;
    let mut trade_data: TradeData;

    match discriminator {
        9 => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "DepositStake".to_string();

            trade_data.stake_pool = input_accounts.get(0).unwrap().to_string();
            trade_data.withdraw_authority = input_accounts.get(3).unwrap().to_string();
            trade_data.reserve_stake = input_accounts.get(6).unwrap().to_string();
            trade_data.validator_stake = input_accounts.get(5).unwrap().to_string();
            trade_data.pool_mint = input_accounts.get(10).unwrap().to_string();
            trade_data.fee_account = input_accounts.get(8).unwrap().to_string();

            trade_data.amount = get_sol_balance_change(
                &trade_data.validator_stake,
                accounts,
                pre_balances,
                post_balances,
            );
            enrich_with_inner_instructions_data(&mut trade_data, accounts, inner_instructions);
            enrich_with_ix_data(&mut trade_data, idx as u32);

            result = Some(trade_data);
        }
        10 => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "WithdrawStake".to_string();

            trade_data.stake_pool = input_accounts.get(0).unwrap().to_string();
            trade_data.withdraw_authority = input_accounts.get(2).unwrap().to_string();
            trade_data.reserve_stake = "".to_string();
            trade_data.validator_stake = input_accounts.get(3).unwrap().to_string();
            trade_data.pool_mint = input_accounts.get(9).unwrap().to_string();
            trade_data.fee_account = input_accounts.get(8).unwrap().to_string();

            trade_data.amount = -1.0
                * get_sol_balance_change(
                    &trade_data.validator_stake,
                    accounts,
                    pre_balances,
                    post_balances,
                );
            enrich_with_inner_instructions_data(&mut trade_data, accounts, inner_instructions);
            trade_data.fee_amount = get_token_balance_change(
                &trade_data.fee_account,
                pre_token_balances,
                post_token_balances,
                accounts,
            );
            enrich_with_ix_data(&mut trade_data, idx as u32);

            result = Some(trade_data);
        }
        14 => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "DepositSol".to_string();

            trade_data.stake_pool = input_accounts.get(0).unwrap().to_string();
            trade_data.withdraw_authority = input_accounts.get(1).unwrap().to_string();
            trade_data.reserve_stake = input_accounts.get(2).unwrap().to_string();
            trade_data.validator_stake = "".to_string();
            trade_data.pool_mint = input_accounts.get(7).unwrap().to_string();
            trade_data.fee_account = input_accounts.get(5).unwrap().to_string();

            trade_data.amount = get_sol_balance_change(
                &trade_data.reserve_stake,
                accounts,
                pre_balances,
                post_balances,
            );
            enrich_with_inner_instructions_data(&mut trade_data, accounts, inner_instructions);
            enrich_with_ix_data(&mut trade_data, idx as u32);

            result = Some(trade_data);
        }
        16 => {
            trade_data = TradeData::default();
            trade_data.instruction_type = "WithdrawSol".to_string();

            trade_data.stake_pool = input_accounts.get(0).unwrap().to_string();
            trade_data.withdraw_authority = input_accounts.get(1).unwrap().to_string();
            trade_data.reserve_stake = input_accounts.get(4).unwrap().to_string();
            trade_data.validator_stake = "".to_string();
            trade_data.pool_mint = input_accounts.get(7).unwrap().to_string();
            trade_data.fee_account = input_accounts.get(6).unwrap().to_string();

            trade_data.amount = -1.0
                * get_sol_balance_change(
                    &trade_data.reserve_stake,
                    accounts,
                    pre_balances,
                    post_balances,
                );
            enrich_with_inner_instructions_data(&mut trade_data, accounts, inner_instructions);
            trade_data.fee_amount = get_token_balance_change(
                &trade_data.fee_account,
                pre_token_balances,
                post_token_balances,
                accounts,
            );
            enrich_with_ix_data(&mut trade_data, idx as u32);

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
