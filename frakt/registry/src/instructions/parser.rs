extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::{
    AddWhitelistToMarketLayout, InitializeOracleLayout, SetOracleAuthorityLayout,
    SetOracleFloorLayout,
};

const INITIALIZE_FRAKT_MARKET_DISCRIMINATOR: u64 =
    u64::from_be_bytes([67, 9, 216, 219, 92, 211, 97, 181]);
const ADD_WHITELIST_TO_MARKET_DISCRIMINATOR: u64 =
    u64::from_be_bytes([211, 244, 180, 185, 132, 111, 122, 170]);
const REMOVE_WHITELIST_FROM_MARKET_DISCRIMINATOR: u64 =
    u64::from_be_bytes([32, 68, 195, 57, 98, 33, 203, 186]);
const ACTIVATE_FRAKT_MARKET_DISCRIMINATOR: u64 =
    u64::from_be_bytes([223, 233, 247, 181, 214, 142, 79, 214]);
const INITIALIZE_ORACLE_DISCRIMINATOR: u64 =
    u64::from_be_bytes([144, 223, 131, 120, 196, 253, 181, 99]);
const SET_ORACLE_AUTHORITY_DISCRIMINATOR: u64 =
    u64::from_be_bytes([39, 155, 66, 106, 213, 226, 114, 174]);
const SET_ORACLE_FLOOR_DISCRIMINATOR: u64 =
    u64::from_be_bytes([82, 67, 252, 218, 96, 146, 203, 67]);

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub addWhitelistToMarket: AddWhitelistToMarketLayout,
    pub initializeOracle: InitializeOracleLayout,
    pub setOracleAuthority: SetOracleAuthorityLayout,
    pub setOracleFloor: SetOracleFloorLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator: u64 = disc_bytes.clone().get_u64();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        INITIALIZE_FRAKT_MARKET_DISCRIMINATOR => {
            result.instructionType = "InitializeFraktMarket".to_string();
        }
        ADD_WHITELIST_TO_MARKET_DISCRIMINATOR => {
            result.instructionType = "AddWhitelistToMarket".to_string();
            result.addWhitelistToMarket =
                AddWhitelistToMarketLayout::deserialize(rest_bytes).unwrap();
        }
        REMOVE_WHITELIST_FROM_MARKET_DISCRIMINATOR => {
            result.instructionType = "RemoveWhitelistFromMarket".to_string();
        }
        ACTIVATE_FRAKT_MARKET_DISCRIMINATOR => {
            result.instructionType = "ActivateFraktMarket".to_string();
        }
        INITIALIZE_ORACLE_DISCRIMINATOR => {
            result.instructionType = "InitializeOracle".to_string();
            result.initializeOracle = InitializeOracleLayout::deserialize(rest_bytes).unwrap();
        }
        SET_ORACLE_AUTHORITY_DISCRIMINATOR => {
            result.instructionType = "SetOracleAuthority".to_string();
            result.setOracleAuthority = SetOracleAuthorityLayout::deserialize(rest_bytes).unwrap();
        }
        SET_ORACLE_FLOOR_DISCRIMINATOR => {
            result.instructionType = "SetOracleFloor".to_string();
            result.setOracleFloor = SetOracleFloorLayout::deserialize(rest_bytes).unwrap();
        }
        _ => {}
    }

    return result;
}
