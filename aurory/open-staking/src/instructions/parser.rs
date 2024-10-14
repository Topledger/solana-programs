extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::{InitializeLayout, ReclaimMintAuthorityLayout, StakeLayout, UnstakeLayout};

const INITIALIZE_DISCRIMINATOR: u64 = u64::from_be_bytes([175, 175, 109, 31, 13, 152, 155, 237]);
const RECLAIM_MINT_AUTHORITY_DISCRIMINATOR: u64 =
    u64::from_be_bytes([197, 200, 55, 220, 220, 179, 8, 111]);
const STAKE_DISCRIMINATOR: u64 = u64::from_be_bytes([206, 176, 202, 18, 200, 209, 179, 108]);
const UNSTAKE_DISCRIMINATOR: u64 = u64::from_be_bytes([90, 95, 107, 42, 205, 124, 50, 225]);
const EMIT_PRICE_DISCRIMINATOR: u64 = u64::from_be_bytes([225, 226, 97, 234, 16, 193, 77, 28]);

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub initialize: InitializeLayout,
    pub reclaimMintAuthority: ReclaimMintAuthorityLayout,
    pub stake: StakeLayout,
    pub unstake: UnstakeLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator: u64 = disc_bytes.clone().get_u64();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        INITIALIZE_DISCRIMINATOR => {
            result.instructionType = "Initialize".to_string();
            result.initialize = InitializeLayout::try_from_slice(rest_bytes).unwrap_or_default();
        }
        RECLAIM_MINT_AUTHORITY_DISCRIMINATOR => {
            result.instructionType = "ReclaimMintAuthority".to_string();
            result.reclaimMintAuthority =
                ReclaimMintAuthorityLayout::try_from_slice(rest_bytes).unwrap_or_default();
        }
        STAKE_DISCRIMINATOR => {
            result.instructionType = "Stake".to_string();
            result.stake = StakeLayout::try_from_slice(rest_bytes).unwrap_or_default();
        }
        UNSTAKE_DISCRIMINATOR => {
            result.instructionType = "Unstake".to_string();
            result.unstake = UnstakeLayout::try_from_slice(rest_bytes).unwrap_or_default();
        }
        EMIT_PRICE_DISCRIMINATOR => {
            result.instructionType = "EmitPrice".to_string();
        }
        _ => {}
    }

    return result;
}
