extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::{CreateManagedProgramLayout, CreateProgramUpgradeLayout};

const CREATE_PROGRAM_MANAGER_DISCRIMINATOR: u64 =
    u64::from_be_bytes([217, 218, 165, 106, 217, 38, 91, 30]);
const CREATE_MANAGED_PROGRAM_DISCRIMINATOR: u64 =
    u64::from_be_bytes([137, 131, 161, 245, 223, 242, 98, 101]);
const CREATE_PROGRAM_UPGRADE_DISCRIMINATOR: u64 =
    u64::from_be_bytes([157, 14, 28, 100, 193, 16, 187, 103]);
const CLOSE_MANAGED_PROGRAM_ACCOUNT_DISCRIMINATOR: u64 =
    u64::from_be_bytes([193, 85, 61, 96, 0, 233, 43, 158]);
const CLOSE_UPGRADE_ACCOUNT_DISCRIMINATOR: u64 =
    u64::from_be_bytes([156, 100, 181, 138, 107, 193, 164, 223]);
const SET_AS_EXECUTED_DISCRIMINATOR: u64 = u64::from_be_bytes([141, 120, 20, 18, 106, 135, 25, 62]);

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub createManagedProgram: CreateManagedProgramLayout,
    pub createProgramUpgrade: CreateProgramUpgradeLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator: u64 = disc_bytes.clone().get_u64();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        CREATE_PROGRAM_MANAGER_DISCRIMINATOR => {
            result.instructionType = "CreateProgramManager".to_string();
        }
        CREATE_MANAGED_PROGRAM_DISCRIMINATOR => {
            result.instructionType = "CreateManagedProgram".to_string();
            result.createManagedProgram =
                CreateManagedProgramLayout::deserialize(rest_bytes).unwrap();
        }
        CREATE_PROGRAM_UPGRADE_DISCRIMINATOR => {
            result.instructionType = "CreateProgramUpgrade".to_string();
            result.createProgramUpgrade =
                CreateProgramUpgradeLayout::deserialize(rest_bytes).unwrap();
        }
        CLOSE_MANAGED_PROGRAM_ACCOUNT_DISCRIMINATOR => {
            result.instructionType = "CloseManagedProgramAccount".to_string();
        }
        CLOSE_UPGRADE_ACCOUNT_DISCRIMINATOR => {
            result.instructionType = "CloseUpgradeAccount".to_string();
        }
        SET_AS_EXECUTED_DISCRIMINATOR => {
            result.instructionType = "SetAsExecuted".to_string();
        }
        _ => {}
    }

    return result;
}
