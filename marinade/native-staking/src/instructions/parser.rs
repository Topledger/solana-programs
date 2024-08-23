extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::{
    InitRootLayout, SetAdminLayout, SetAlternateStakerLayout, SetOperatorLayout, SplitLayout,
};

const INIT_ROOT_DISCRIMINATOR: u64 = u64::from_be_bytes([122, 166, 158, 104, 172, 147, 210, 225]);
const SET_OPERATOR_DISCRIMINATOR: u64 = u64::from_be_bytes([238, 153, 101, 169, 243, 131, 36, 1]);
const SET_ADMIN_DISCRIMINATOR: u64 = u64::from_be_bytes([251, 163, 0, 52, 91, 194, 187, 92]);
const SET_ALTERNATE_STAKER_DISCRIMINATOR: u64 =
    u64::from_be_bytes([147, 164, 23, 104, 247, 123, 49, 171]);
const MERGE_DISCRIMINATOR: u64 = u64::from_be_bytes([148, 141, 236, 47, 174, 126, 69, 111]);
const SPLIT_DISCRIMINATOR: u64 = u64::from_be_bytes([124, 189, 27, 43, 216, 40, 147, 66]);
const DEACTIVATE_DISCRIMINATOR: u64 = u64::from_be_bytes([44, 112, 33, 172, 113, 28, 142, 13]);
const DELEGATE_DISCRIMINATOR: u64 = u64::from_be_bytes([90, 147, 75, 178, 85, 88, 4, 137]);
const REDELEGATE_DISCRIMINATOR: u64 = u64::from_be_bytes([212, 82, 51, 160, 228, 80, 116, 35]);
const SWITCH_STAKER_DISCRIMINATOR: u64 = u64::from_be_bytes([81, 25, 144, 169, 142, 215, 199, 150]);

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub initRoot: InitRootLayout,
    pub setOperator: SetOperatorLayout,
    pub setAdmin: SetAdminLayout,
    pub setAlternateStaker: SetAlternateStakerLayout,
    pub split: SplitLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator: u64 = disc_bytes.clone().get_u64();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        INIT_ROOT_DISCRIMINATOR => {
            result.instructionType = "InitRoot".to_string();
            result.initRoot = InitRootLayout::deserialize(rest_bytes).unwrap();
        }
        SET_OPERATOR_DISCRIMINATOR => {
            result.instructionType = "SetOperator".to_string();
            result.setOperator = SetOperatorLayout::deserialize(rest_bytes).unwrap();
        }
        SET_ADMIN_DISCRIMINATOR => {
            result.instructionType = "SetAdmin".to_string();
            result.setAdmin = SetAdminLayout::deserialize(rest_bytes).unwrap();
        }
        SET_ALTERNATE_STAKER_DISCRIMINATOR => {
            result.instructionType = "SetAlternateStaker".to_string();
            result.setAlternateStaker = SetAlternateStakerLayout::deserialize(rest_bytes).unwrap();
        }
        MERGE_DISCRIMINATOR => {
            result.instructionType = "Merge".to_string();
        }
        SPLIT_DISCRIMINATOR => {
            result.instructionType = "Split".to_string();
            result.split = SplitLayout::deserialize(rest_bytes).unwrap();
        }
        DEACTIVATE_DISCRIMINATOR => {
            result.instructionType = "Deactivate".to_string();
        }
        DELEGATE_DISCRIMINATOR => {
            result.instructionType = "Delegate".to_string();
        }
        REDELEGATE_DISCRIMINATOR => {
            result.instructionType = "Redelegate".to_string();
        }
        SWITCH_STAKER_DISCRIMINATOR => {
            result.instructionType = "SwitchStaker".to_string();
        }
        _ => {}
    }

    return result;
}
