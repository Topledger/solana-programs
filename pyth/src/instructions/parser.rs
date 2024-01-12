extern crate bs58;

use borsh::BorshDeserialize;
use substreams::log;

use super::structs::{SetMinPublishersArgsLayout, UpdatePriceNoFailOnErrorArgsLayout};

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub updatePriceNoFailOnErrorArgs: UpdatePriceNoFailOnErrorArgsLayout,
    pub updatePriceArgs: UpdatePriceNoFailOnErrorArgsLayout,
    pub setMinPublishersArgs: SetMinPublishersArgsLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator: u8 = u8::from(disc_bytes[4]);
    let rest_bytes = &mut rest.clone();

    match discriminator {
        7 => {
            result.instructionType = "UpdatePrice".to_string();
            if rest_bytes.len() > 0 {
                result.updatePriceArgs =
                    UpdatePriceNoFailOnErrorArgsLayout::deserialize(rest_bytes).unwrap();
            }
        }
        8 => {
            result.instructionType = "AggregatePrice".to_string();
        }
        12 => {
            result.instructionType = "SetMinPublishers".to_string();
            if rest_bytes.len() > 0 {
                result.setMinPublishersArgs =
                    SetMinPublishersArgsLayout::deserialize(rest_bytes).unwrap();
            }
        }
        13 => {
            result.instructionType = "UpdatePriceNoFailOnError".to_string();
            if rest_bytes.len() > 0 {
                result.updatePriceNoFailOnErrorArgs =
                    UpdatePriceNoFailOnErrorArgsLayout::deserialize(rest_bytes).unwrap();
            }
        }
        _ => {}
    }

    return result;
}
