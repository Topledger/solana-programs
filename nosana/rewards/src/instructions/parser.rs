extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::AddFeeLayout;

const INIT_DISCRIMINATOR: u64 = u64::from_be_bytes([220, 59, 207, 236, 108, 250, 47, 100]);
const ENTER_DISCRIMINATOR: u64 = u64::from_be_bytes([139, 49, 209, 114, 88, 91, 77, 134]);
const ADD_FEE_DISCRIMINATOR: u64 = u64::from_be_bytes([67, 225, 189, 212, 253, 123, 76, 112]);
const CLAIM_DISCRIMINATOR: u64 = u64::from_be_bytes([62, 198, 214, 193, 213, 159, 108, 210]);
const SYNC_DISCRIMINATOR: u64 = u64::from_be_bytes([4, 219, 40, 164, 21, 157, 189, 88]);
const CLOSE_DISCRIMINATOR: u64 = u64::from_be_bytes([98, 165, 201, 177, 108, 65, 206, 96]);

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub addFee: AddFeeLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator: u64 = disc_bytes.clone().get_u64();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        INIT_DISCRIMINATOR => {
            result.instructionType = "Init".to_string();
        }
        ENTER_DISCRIMINATOR => {
            result.instructionType = "Enter".to_string();
        }
        ADD_FEE_DISCRIMINATOR => {
            result.instructionType = "AddFee".to_string();
            result.addFee = AddFeeLayout::deserialize(rest_bytes).unwrap();
        }
        CLAIM_DISCRIMINATOR => {
            result.instructionType = "Claim".to_string();
        }
        SYNC_DISCRIMINATOR => {
            result.instructionType = "Sync".to_string();
        }
        CLOSE_DISCRIMINATOR => {
            result.instructionType = "Close".to_string();
        }
        _ => {}
    }

    return result;
}
