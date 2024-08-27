extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::MintTokenLayout;

const INITIALIZE_DISCRIMINATOR: u64 = u64::from_be_bytes([175, 175, 109, 31, 13, 152, 155, 237]);
const MINT_TOKEN_DISCRIMINATOR: u64 = u64::from_be_bytes([172, 137, 183, 14, 207, 110, 234, 56]);

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub mintToken: MintTokenLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator: u64 = disc_bytes.clone().get_u64();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        INITIALIZE_DISCRIMINATOR => {
            result.instructionType = "Initialize".to_string();
        }
        MINT_TOKEN_DISCRIMINATOR => {
            result.instructionType = "MintToken".to_string();
            result.mintToken = MintTokenLayout::deserialize(rest_bytes).unwrap();
        }
        _ => {}
    }

    return result;
}
