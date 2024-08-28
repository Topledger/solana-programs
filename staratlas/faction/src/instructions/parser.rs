extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::ProcessEnlistPlayerLayout;

const PROCESS_ENLIST_PLAYER_DISCRIMINATOR: u64 =
    u64::from_be_bytes([198, 106, 62, 167, 119, 46, 207, 81]);

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub processEnlistPlayer: ProcessEnlistPlayerLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator: u64 = disc_bytes.clone().get_u64();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        PROCESS_ENLIST_PLAYER_DISCRIMINATOR => {
            result.instructionType = "ProcessEnlistPlayer".to_string();
            result.processEnlistPlayer =
                ProcessEnlistPlayerLayout::deserialize(rest_bytes).unwrap();
        }
        _ => {}
    }

    return result;
}
