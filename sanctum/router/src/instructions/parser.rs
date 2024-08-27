extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::{
    PrefundSwapViaStakeLayout, PrefundWithdrawStakeLayout, StakeWrappedSolLayout,
    SwapViaStakeLayout,
};

const STAKE_WRAPPED_SOL_DISCRIMINATOR: u8 = 0;
const SWAP_VIA_STAKE_DISCRIMINATOR: u8 = 1;
const CREATE_FEE_TOKEN_ACCOUNT_DISCRIMINATOR: u8 = 2;
const CLOSE_FEE_TOKEN_ACCOUNT_DISCRIMINATOR: u8 = 3;
const WITHDRAW_FEES_DISCRIMINATOR: u8 = 4;
const DEPOSIT_STAKE_DISCRIMINATOR: u8 = 5;
const PREFUND_WITHDRAW_STAKE_DISCRIMINATOR: u8 = 6;
const PREFUND_SWAP_VIA_STAKE_DISCRIMINATOR: u8 = 7;

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub stakeWrappedSol: StakeWrappedSolLayout,
    pub swapViaStake: SwapViaStakeLayout,
    pub prefundWithdrawStake: PrefundWithdrawStakeLayout,
    pub prefundSwapViaStake: PrefundSwapViaStakeLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let discriminator: u8 = disc_bytes.clone().get_u8();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        STAKE_WRAPPED_SOL_DISCRIMINATOR => {
            result.instructionType = "StakeWrappedSol".to_string();
            result.stakeWrappedSol = StakeWrappedSolLayout::deserialize(rest_bytes).unwrap();
        }
        SWAP_VIA_STAKE_DISCRIMINATOR => {
            result.instructionType = "SwapViaStake".to_string();
            result.swapViaStake = SwapViaStakeLayout::deserialize(rest_bytes).unwrap();
        }
        CREATE_FEE_TOKEN_ACCOUNT_DISCRIMINATOR => {
            result.instructionType = "CreateFeeTokenAccount".to_string();
        }
        CLOSE_FEE_TOKEN_ACCOUNT_DISCRIMINATOR => {
            result.instructionType = "CloseFeeTokenAccount".to_string();
        }
        WITHDRAW_FEES_DISCRIMINATOR => {
            result.instructionType = "WithdrawFees".to_string();
        }
        DEPOSIT_STAKE_DISCRIMINATOR => {
            result.instructionType = "DepositStake".to_string();
        }
        PREFUND_WITHDRAW_STAKE_DISCRIMINATOR => {
            result.instructionType = "PrefundWithdrawStake".to_string();
            result.prefundWithdrawStake =
                PrefundWithdrawStakeLayout::deserialize(rest_bytes).unwrap();
        }
        PREFUND_SWAP_VIA_STAKE_DISCRIMINATOR => {
            result.instructionType = "PrefundSwapViaStake".to_string();
            result.prefundSwapViaStake =
                PrefundSwapViaStakeLayout::deserialize(rest_bytes).unwrap();
        }
        _ => {}
    }

    return result;
}
