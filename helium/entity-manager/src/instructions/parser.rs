extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::InitializeRewardableEntityConfigV0Layout;

const INITIALIZE_REWARDABLE_ENTITY_CONFIG_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([174, 50, 32, 15, 143, 7, 11, 152]);
const APPROVE_MAKER_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([12, 230, 133, 92, 150, 229, 141, 142]);
const REVOKE_MAKER_V0_DISCRIMINATOR: u64 = u64::from_be_bytes([121, 23, 18, 110, 43, 111, 32, 254]);
const APPROVE_PROGRAM_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([5, 238, 152, 185, 72, 120, 3, 145]);
const REVOKE_PROGRAM_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([166, 120, 44, 143, 5, 63, 121, 210]);
const INITIALIZE_MAKER_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([204, 26, 62, 134, 112, 201, 162, 160]);
const ISSUE_ENTITY_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([232, 194, 122, 68, 167, 246, 119, 197]);
const ISSUE_PROGRAM_ENTITY_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([70, 97, 114, 226, 33, 150, 80, 217]);
const ISSUE_NOT_EMITTED_ENTITY_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([60, 112, 240, 206, 80, 101, 94, 210]);
const ISSUE_IOT_OPERATIONS_FUND_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([222, 131, 120, 163, 230, 187, 91, 209]);
const ONBOARD_IOT_HOTSPOT_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([45, 24, 129, 38, 14, 65, 7, 15]);
const ONBOARD_MOBILE_HOTSPOT_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([23, 144, 210, 155, 8, 203, 90, 207]);
const UPDATE_REWARDABLE_ENTITY_CONFIG_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([52, 85, 169, 134, 138, 116, 78, 18]);
const UPDATE_MAKER_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([94, 227, 240, 133, 67, 224, 25, 109]);
const SET_MAKER_TREE_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([226, 10, 222, 243, 102, 205, 13, 221]);
const UPDATE_MAKER_TREE_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([149, 158, 98, 186, 64, 211, 52, 17]);
const UPDATE_IOT_INFO_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([211, 235, 205, 29, 109, 86, 153, 39]);
const UPDATE_MOBILE_INFO_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([207, 104, 82, 157, 193, 229, 73, 153]);
const INITIALIZE_DATA_ONLY_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([19, 81, 219, 30, 10, 34, 12, 80]);
const ISSUE_DATA_ONLY_ENTITY_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([191, 96, 245, 46, 63, 73, 207, 17]);
const ONBOARD_DATA_ONLY_IOT_HOTSPOT_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([98, 179, 127, 51, 58, 191, 174, 188]);
const UPDATE_DATA_ONLY_TREE_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([214, 52, 30, 65, 215, 38, 122, 102]);
const SET_ENTITY_ACTIVE_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([234, 186, 204, 28, 91, 88, 246, 244]);
const TEMP_PAY_MOBILE_ONBOARDING_FEE_V0_DISCRIMINATOR: u64 =
    u64::from_be_bytes([117, 84, 64, 76, 137, 31, 188, 28]);

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub initializeRewardableEntityConfigV0: InitializeRewardableEntityConfigV0Layout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator: u64 = disc_bytes.clone().get_u64();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        INITIALIZE_REWARDABLE_ENTITY_CONFIG_V0_DISCRIMINATOR => {
            result.instructionType = "InitializeRewardableEntityConfigV0".to_string();
            result.initializeRewardableEntityConfigV0 =
                InitializeRewardableEntityConfigV0Layout::deserialize(rest_bytes).unwrap();
        }
        APPROVE_MAKER_V0_DISCRIMINATOR => {
            result.instructionType = "ApproveMakerV0".to_string();
        }
        REVOKE_MAKER_V0_DISCRIMINATOR => {
            result.instructionType = "RevokeMakerV0".to_string();
        }
        APPROVE_PROGRAM_V0_DISCRIMINATOR => {
            result.instructionType = "ApproveProgramV0".to_string();
        }
        REVOKE_PROGRAM_V0_DISCRIMINATOR => {
            result.instructionType = "RevokeProgramV0".to_string();
        }
        INITIALIZE_MAKER_V0_DISCRIMINATOR => {
            result.instructionType = "InitializeMakerV0".to_string();
        }
        ISSUE_ENTITY_V0_DISCRIMINATOR => {
            result.instructionType = "IssueEntityV0".to_string();
        }
        ISSUE_PROGRAM_ENTITY_V0_DISCRIMINATOR => {
            result.instructionType = "IssueProgramEntityV0".to_string();
        }
        ISSUE_NOT_EMITTED_ENTITY_V0_DISCRIMINATOR => {
            result.instructionType = "IssueNotEmittedEntityV0".to_string();
        }
        ISSUE_IOT_OPERATIONS_FUND_V0_DISCRIMINATOR => {
            result.instructionType = "IssueIotOperationsFundV0".to_string();
        }
        ONBOARD_IOT_HOTSPOT_V0_DISCRIMINATOR => {
            result.instructionType = "OnboardIotHotspotV0".to_string();
        }
        ONBOARD_MOBILE_HOTSPOT_V0_DISCRIMINATOR => {
            result.instructionType = "OnboardMobileHotspotV0".to_string();
        }
        UPDATE_REWARDABLE_ENTITY_CONFIG_V0_DISCRIMINATOR => {
            result.instructionType = "UpdateRewardableEntityConfigV0".to_string();
        }
        UPDATE_MAKER_V0_DISCRIMINATOR => {
            result.instructionType = "UpdateMakerV0".to_string();
        }
        SET_MAKER_TREE_V0_DISCRIMINATOR => {
            result.instructionType = "SetMakerTreeV0".to_string();
        }
        UPDATE_MAKER_TREE_V0_DISCRIMINATOR => {
            result.instructionType = "UpdateMakerTreeV0".to_string();
        }
        UPDATE_IOT_INFO_V0_DISCRIMINATOR => {
            result.instructionType = "UpdateIotInfoV0".to_string();
        }
        UPDATE_MOBILE_INFO_V0_DISCRIMINATOR => {
            result.instructionType = "UpdateMobileInfoV0".to_string();
        }
        INITIALIZE_DATA_ONLY_V0_DISCRIMINATOR => {
            result.instructionType = "InitializeDataOnlyV0".to_string();
        }
        ISSUE_DATA_ONLY_ENTITY_V0_DISCRIMINATOR => {
            result.instructionType = "IssueDataOnlyEntityV0".to_string();
        }
        ONBOARD_DATA_ONLY_IOT_HOTSPOT_V0_DISCRIMINATOR => {
            result.instructionType = "OnboardDataOnlyIotHotspotV0".to_string();
        }
        UPDATE_DATA_ONLY_TREE_V0_DISCRIMINATOR => {
            result.instructionType = "UpdateDataOnlyTreeV0".to_string();
        }
        SET_ENTITY_ACTIVE_V0_DISCRIMINATOR => {
            result.instructionType = "SetEntityActiveV0".to_string();
        }
        TEMP_PAY_MOBILE_ONBOARDING_FEE_V0_DISCRIMINATOR => {
            result.instructionType = "TempPayMobileOnboardingFeeV0".to_string();
        }
        _ => {}
    }

    return result;
}
