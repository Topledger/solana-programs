extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::SwapLayout;
const SWAP_DISCRIMINATOR: u64 = u64::from_be_bytes([248, 198, 158, 145, 225, 117, 135, 200]);

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub swap: SwapLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator: u64 = disc_bytes.clone().get_u64();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        SWAP_DISCRIMINATOR => {
            result.instructionType = "Swap".to_string();
            result.swap = SwapLayout::deserialize(rest_bytes).unwrap();
        }
        _ => {}
    }

    return result;
}
