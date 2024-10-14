extern crate bs58;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::{
    DepositLayout, InitializeLayout, SwapLayout, WithdrawLayout, WithdrawOneLayout,
};

const INITIALIZE_DISCRIMINATOR: u8 = 0;
const SWAP_DISCRIMINATOR: u8 = 1;
const DEPOSIT_DISCRIMINATOR: u8 = 2;
const WITHDRAW_DISCRIMINATOR: u8 = 3;
const WITHDRAWONE_DISCRIMINATOR: u8 = 4;

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub initialize: InitializeLayout,
    pub swap: SwapLayout,
    pub deposit: DepositLayout,
    pub withdraw: WithdrawLayout,
    pub withdrawOne: WithdrawOneLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let discriminator: u8 = disc_bytes.clone().get_u8();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        INITIALIZE_DISCRIMINATOR => {
            result.instructionType = "Initialize".to_string();
            result.initialize = InitializeLayout::deserialize(rest_bytes).unwrap();
        }
        SWAP_DISCRIMINATOR => {
            result.instructionType = "Swap".to_string();
            result.swap = SwapLayout::deserialize(rest_bytes).unwrap();
        }
        DEPOSIT_DISCRIMINATOR => {
            result.instructionType = "Deposit".to_string();
            result.deposit = DepositLayout::deserialize(rest_bytes).unwrap();
        }
        WITHDRAW_DISCRIMINATOR => {
            result.instructionType = "Withdraw".to_string();
            result.withdraw = WithdrawLayout::deserialize(rest_bytes).unwrap();
        }
        WITHDRAWONE_DISCRIMINATOR => {
            result.instructionType = "WithdrawOne".to_string();
            result.withdrawOne = WithdrawOneLayout::deserialize(rest_bytes).unwrap();
        }
        _ => {}
    }

    return result;
}
