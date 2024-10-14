extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::{
    ChangePoolWeightLayout, ChangeTotalGeneAllocatedLayout, ClaimRewardsLayout,
    CreateGlobalStateLayout, CreateStakingPoolLayout, ReLockDepositLayout, StakeLayout,
    WithdrawLayout,
};

const CHANGE_STAKE_MASTER_AUTHORITY_DISCRIMINATOR: u64 =
    u64::from_be_bytes([233, 12, 101, 125, 9, 83, 229, 193]);
const CHANGE_STAKE_MASTER_PAUSED_STATE_DISCRIMINATOR: u64 =
    u64::from_be_bytes([151, 206, 175, 64, 38, 70, 205, 35]);
const CHANGE_TOTAL_GENE_ALLOCATED_DISCRIMINATOR: u64 =
    u64::from_be_bytes([34, 162, 160, 214, 124, 206, 60, 237]);
const CHANGE_POOL_WEIGHT_DISCRIMINATOR: u64 =
    u64::from_be_bytes([247, 119, 219, 164, 59, 150, 185, 82]);
const CREATE_GLOBAL_STATE_DISCRIMINATOR: u64 =
    u64::from_be_bytes([53, 127, 207, 143, 222, 244, 229, 115]);
const CREATE_STAKING_POOL_DISCRIMINATOR: u64 =
    u64::from_be_bytes([104, 58, 70, 37, 225, 212, 145, 93]);
const STAKE_DISCRIMINATOR: u64 = u64::from_be_bytes([206, 176, 202, 18, 200, 209, 179, 108]);
const RE_LOCK_DEPOSIT_DISCRIMINATOR: u64 =
    u64::from_be_bytes([121, 174, 183, 149, 178, 172, 100, 176]);
const WITHDRAW_DISCRIMINATOR: u64 = u64::from_be_bytes([183, 18, 70, 156, 148, 109, 161, 34]);
const WITHDRAW_AS_SGENE_DISCRIMINATOR: u64 =
    u64::from_be_bytes([103, 175, 210, 108, 236, 80, 197, 123]);
const CLAIM_REWARDS_DISCRIMINATOR: u64 = u64::from_be_bytes([4, 144, 132, 71, 116, 23, 151, 80]);
const PENDING_YIELD_REWARDS_DISCRIMINATOR: u64 =
    u64::from_be_bytes([11, 2, 231, 148, 66, 143, 145, 183]);
const CURRENT_GENES_PER_SECOND_DISCRIMINATOR: u64 =
    u64::from_be_bytes([121, 13, 142, 116, 187, 90, 142, 43]);
const GET_VOTING_WEIGHT_DISCRIMINATOR: u64 =
    u64::from_be_bytes([244, 61, 69, 129, 170, 248, 28, 40]);

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub changeTotalGeneAllocated: ChangeTotalGeneAllocatedLayout,
    pub changePoolWeight: ChangePoolWeightLayout,
    pub createGlobalState: CreateGlobalStateLayout,
    pub createStakingPool: CreateStakingPoolLayout,
    pub stake: StakeLayout,
    pub reLockDeposit: ReLockDepositLayout,
    pub withdraw: WithdrawLayout,
    pub claimRewards: ClaimRewardsLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator: u64 = disc_bytes.clone().get_u64();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        CHANGE_STAKE_MASTER_AUTHORITY_DISCRIMINATOR => {
            result.instructionType = "ChangeStakeMasterAuthority".to_string();
        }
        CHANGE_STAKE_MASTER_PAUSED_STATE_DISCRIMINATOR => {
            result.instructionType = "ChangeStakeMasterPausedState".to_string();
        }
        CHANGE_TOTAL_GENE_ALLOCATED_DISCRIMINATOR => {
            result.instructionType = "ChangeTotalGeneAllocated".to_string();
            result.changeTotalGeneAllocated =
                ChangeTotalGeneAllocatedLayout::deserialize(rest_bytes).unwrap();
        }
        CHANGE_POOL_WEIGHT_DISCRIMINATOR => {
            result.instructionType = "ChangePoolWeight".to_string();
            result.changePoolWeight = ChangePoolWeightLayout::deserialize(rest_bytes).unwrap();
        }
        CREATE_GLOBAL_STATE_DISCRIMINATOR => {
            result.instructionType = "CreateGlobalState".to_string();
            result.createGlobalState = CreateGlobalStateLayout::deserialize(rest_bytes).unwrap();
        }
        CREATE_STAKING_POOL_DISCRIMINATOR => {
            result.instructionType = "CreateStakingPool".to_string();
            result.createStakingPool = CreateStakingPoolLayout::deserialize(rest_bytes).unwrap();
        }
        STAKE_DISCRIMINATOR => {
            result.instructionType = "Stake".to_string();
            result.stake = StakeLayout::deserialize(rest_bytes).unwrap();
        }
        RE_LOCK_DEPOSIT_DISCRIMINATOR => {
            result.instructionType = "ReLockDeposit".to_string();
            result.reLockDeposit = ReLockDepositLayout::deserialize(rest_bytes).unwrap();
        }
        WITHDRAW_DISCRIMINATOR => {
            result.instructionType = "Withdraw".to_string();
            result.withdraw = WithdrawLayout::deserialize(rest_bytes).unwrap();
        }
        WITHDRAW_AS_SGENE_DISCRIMINATOR => {
            result.instructionType = "WithdrawAsSgene".to_string();
        }
        CLAIM_REWARDS_DISCRIMINATOR => {
            result.instructionType = "ClaimRewards".to_string();
            result.claimRewards = ClaimRewardsLayout::deserialize(rest_bytes).unwrap();
        }
        PENDING_YIELD_REWARDS_DISCRIMINATOR => {
            result.instructionType = "PendingYieldRewards".to_string();
        }
        CURRENT_GENES_PER_SECOND_DISCRIMINATOR => {
            result.instructionType = "CurrentGenesPerSecond".to_string();
        }
        GET_VOTING_WEIGHT_DISCRIMINATOR => {
            result.instructionType = "GetVotingWeight".to_string();
        }

        _ => {}
    }

    return result;
}
