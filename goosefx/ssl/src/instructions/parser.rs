extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::{BurnPtLayout, DepositLayout, MintPtLayout, SwapLayout, WithdrawLayout};

const CREATE_LIQUIDITY_ACCOUNT_DISCRIMINATOR: u64 =
    u64::from_be_bytes([112, 19, 213, 238, 68, 113, 146, 38]);
const DEPOSIT_DISCRIMINATOR: u64 = u64::from_be_bytes([242, 35, 198, 137, 82, 225, 242, 182]);
const WITHDRAW_DISCRIMINATOR: u64 = u64::from_be_bytes([183, 18, 70, 156, 148, 109, 161, 34]);
const MINT_PT_DISCRIMINATOR: u64 = u64::from_be_bytes([115, 247, 213, 190, 222, 163, 95, 229]);
const BURN_PT_DISCRIMINATOR: u64 = u64::from_be_bytes([72, 94, 156, 201, 183, 224, 52, 100]);
const SWAP_DISCRIMINATOR: u64 = u64::from_be_bytes([248, 198, 158, 145, 225, 117, 135, 200]);
const CRANK_LIABILITY_DISCRIMINATOR: u64 = u64::from_be_bytes([101, 44, 139, 128, 121, 197, 0, 47]);

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub deposit: DepositLayout,
    pub withdraw: WithdrawLayout,
    pub mintPt: MintPtLayout,
    pub burnPt: BurnPtLayout,
    pub swap: SwapLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator: u64 = disc_bytes.clone().get_u64();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        CREATE_LIQUIDITY_ACCOUNT_DISCRIMINATOR => {
            result.instructionType = "CreateLiquidityAccount".to_string();
        }
        DEPOSIT_DISCRIMINATOR => {
            result.instructionType = "Deposit".to_string();
            result.deposit = DepositLayout::deserialize(rest_bytes).unwrap();
        }
        WITHDRAW_DISCRIMINATOR => {
            result.instructionType = "Withdraw".to_string();
            result.withdraw = WithdrawLayout::deserialize(rest_bytes).unwrap();
        }
        MINT_PT_DISCRIMINATOR => {
            result.instructionType = "MintPt".to_string();
            result.mintPt = MintPtLayout::deserialize(rest_bytes).unwrap();
        }
        BURN_PT_DISCRIMINATOR => {
            result.instructionType = "BurnPt".to_string();
            result.burnPt = BurnPtLayout::deserialize(rest_bytes).unwrap();
        }
        SWAP_DISCRIMINATOR => {
            result.instructionType = "Swap".to_string();
            result.swap = SwapLayout::deserialize(rest_bytes).unwrap();
        }
        CRANK_LIABILITY_DISCRIMINATOR => {
            result.instructionType = "CrankLiability".to_string();
        }
        _ => {}
    }

    return result;
}
