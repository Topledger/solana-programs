extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::{
    InitializeTreasuryManagementV0Layout, RedeemV0Layout, UpdateTreasuryManagementV0Layout,
};

const INITIALIZE_TREASURY_MANAGEMENT_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([149, 3, 201, 108, 130, 56, 56, 210]);
const UPDATE_TREASURY_MANAGEMENT_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([209, 139, 90, 226, 249, 149, 89, 217]);
const REDEEM_V0_DISCRIMINATOR: u64 = u64::from_be_bytes([235, 127, 171, 139, 119, 77, 235, 118]);
const CORRECT_TREASURIES_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([76, 80, 195, 124, 145, 230, 132, 110]);

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub initializeTreasuryManagementV0: InitializeTreasuryManagementV0Layout,
    pub updateTreasuryManagementV0: UpdateTreasuryManagementV0Layout,
    pub redeemV0: RedeemV0Layout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator: u64 = disc_bytes.clone().get_u64();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        INITIALIZE_TREASURY_MANAGEMENT_V0_DISCRIMINATOR => {
            result.instructionType = "InitializeTreasuryManagementV0".to_string();
            result.initializeTreasuryManagementV0 =
                InitializeTreasuryManagementV0Layout::deserialize(rest_bytes).unwrap();
        }
        UPDATE_TREASURY_MANAGEMENT_V0_DISCRIMINATOR => {
            result.instructionType = "UpdateTreasuryManagementV0".to_string();
            result.updateTreasuryManagementV0 =
                UpdateTreasuryManagementV0Layout::deserialize(rest_bytes).unwrap();
        }
        REDEEM_V0_DISCRIMINATOR => {
            result.instructionType = "RedeemV0".to_string();
            result.redeemV0 = RedeemV0Layout::deserialize(rest_bytes).unwrap();
        }
        CORRECT_TREASURIES_V0_DISCRIMINATOR => {
            result.instructionType = "CorrectTreasuriesV0".to_string();
        }
        _ => {}
    }

    return result;
}
