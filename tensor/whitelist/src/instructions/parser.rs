extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::{
    InitUpdateAuthorityLayout, InitUpdateMintProofLayout, InitUpdateWhitelistLayout,
};

const INIT_UPDATE_AUTHORITY_DISCRIMINATOR: u64 =
    u64::from_be_bytes([53, 144, 79, 150, 196, 110, 22, 55]);
const INIT_UPDATE_WHITELIST_DISCRIMINATOR: u64 =
    u64::from_be_bytes([255, 1, 192, 134, 111, 49, 212, 131]);
const INIT_UPDATE_MINT_PROOF_DISCRIMINATOR: u64 =
    u64::from_be_bytes([30, 77, 123, 9, 191, 37, 52, 159]);
const REALLOC_AUTHORITY_DISCRIMINATOR: u64 = u64::from_be_bytes([128, 120, 16, 197, 85, 34, 2, 91]);
const REALLOC_WHITELIST_DISCRIMINATOR: u64 =
    u64::from_be_bytes([173, 147, 168, 152, 181, 46, 55, 60]);
const FREEZE_WHITELIST_DISCRIMINATOR: u64 =
    u64::from_be_bytes([248, 112, 12, 150, 175, 238, 38, 184]);
const UNFREEZE_WHITELIST_DISCRIMINATOR: u64 =
    u64::from_be_bytes([215, 119, 9, 92, 160, 139, 226, 253]);

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub initUpdateAuthority: InitUpdateAuthorityLayout,
    pub initUpdateWhitelist: InitUpdateWhitelistLayout,
    pub initUpdateMintProof: InitUpdateMintProofLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator: u64 = disc_bytes.clone().get_u64();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        INIT_UPDATE_AUTHORITY_DISCRIMINATOR => {
            result.instructionType = "InitUpdateAuthority".to_string();
            result.initUpdateAuthority =
                InitUpdateAuthorityLayout::deserialize(rest_bytes).unwrap();
        }
        INIT_UPDATE_WHITELIST_DISCRIMINATOR => {
            result.instructionType = "InitUpdateWhitelist".to_string();
            result.initUpdateWhitelist =
                InitUpdateWhitelistLayout::deserialize(rest_bytes).unwrap();
        }
        INIT_UPDATE_MINT_PROOF_DISCRIMINATOR => {
            result.instructionType = "InitUpdateMintProof".to_string();
            result.initUpdateMintProof =
                InitUpdateMintProofLayout::deserialize(rest_bytes).unwrap();
        }
        REALLOC_AUTHORITY_DISCRIMINATOR => {
            result.instructionType = "ReallocAuthority".to_string();
        }
        REALLOC_WHITELIST_DISCRIMINATOR => {
            result.instructionType = "ReallocWhitelist".to_string();
        }
        FREEZE_WHITELIST_DISCRIMINATOR => {
            result.instructionType = "FreezeWhitelist".to_string();
        }
        UNFREEZE_WHITELIST_DISCRIMINATOR => {
            result.instructionType = "UnfreezeWhitelist".to_string();
        }
        _ => {}
    }

    return result;
}
