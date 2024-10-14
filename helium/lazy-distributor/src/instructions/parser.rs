extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::{
    DistributeCompressionRewardsV0Layout, InitializeCompressionRecipientV0Layout,
    InitializeLazyDistributorV0Layout, SetCurrentRewardsV0Layout, UpdateLazyDistributorV0Layout,
};

const INITIALIZE_LAZY_DISTRIBUTOR_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([82, 55, 202, 203, 158, 198, 240, 103]);
const INITIALIZE_RECIPIENT_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([216, 35, 51, 171, 152, 41, 62, 106]);
const INITIALIZE_COMPRESSION_RECIPIENT_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([80, 144, 151, 21, 26, 144, 218, 174]);
const SET_CURRENT_REWARDS_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([251, 237, 190, 128, 228, 195, 93, 239]);
const DISTRIBUTE_REWARDS_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([235, 19, 42, 12, 113, 98, 214, 117]);
const DISTRIBUTE_COMPRESSION_REWARDS_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([160, 48, 184, 57, 5, 105, 233, 71]);
const UPDATE_LAZY_DISTRIBUTOR_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([73, 118, 167, 236, 211, 213, 158, 214]);

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub initializeLazyDistributorV0: InitializeLazyDistributorV0Layout,
    pub initializeCompressionRecipientV0: InitializeCompressionRecipientV0Layout,
    pub setCurrentRewardsV0: SetCurrentRewardsV0Layout,
    pub distributeCompressionRewardsV0: DistributeCompressionRewardsV0Layout,
    pub updateLazyDistributorV0Layout: UpdateLazyDistributorV0Layout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator: u64 = disc_bytes.clone().get_u64();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        INITIALIZE_LAZY_DISTRIBUTOR_V0_DISCRIMINATOR => {
            result.instructionType = "InitializeLazyDistributorV0".to_string();
            result.initializeLazyDistributorV0 =
                InitializeLazyDistributorV0Layout::deserialize(rest_bytes).unwrap();
        }
        INITIALIZE_RECIPIENT_V0_DISCRIMINATOR => {
            result.instructionType = "InitializeRecipientV0".to_string();
        }
        INITIALIZE_COMPRESSION_RECIPIENT_V0_DISCRIMINATOR => {
            result.instructionType = "InitializeCompressionRecipientV0".to_string();
            result.initializeCompressionRecipientV0 =
                InitializeCompressionRecipientV0Layout::deserialize(rest_bytes).unwrap();
        }
        SET_CURRENT_REWARDS_V0_DISCRIMINATOR => {
            result.instructionType = "SetCurrentRewardsV0".to_string();
            result.setCurrentRewardsV0 =
                SetCurrentRewardsV0Layout::deserialize(rest_bytes).unwrap();
        }
        DISTRIBUTE_REWARDS_V0_DISCRIMINATOR => {
            result.instructionType = "DistributeRewardsV0".to_string();
        }
        DISTRIBUTE_COMPRESSION_REWARDS_V0_DISCRIMINATOR => {
            result.instructionType = "DistributeCompressionRewardsV0".to_string();
            result.distributeCompressionRewardsV0 =
                DistributeCompressionRewardsV0Layout::deserialize(rest_bytes).unwrap();
        }
        UPDATE_LAZY_DISTRIBUTOR_V0_DISCRIMINATOR => {
            result.instructionType = "UpdateLazyDistributorV0".to_string();
            result.updateLazyDistributorV0Layout =
                UpdateLazyDistributorV0Layout::deserialize(rest_bytes).unwrap();
        }
        _ => {}
    }

    return result;
}
