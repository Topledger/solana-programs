extern crate bs58;

use bytes::Buf;

const CREATE_DISCRIMINATOR: u8 = 0;
const CREATE_IDEMPOTENT_DISCRIMINATOR: u8 = 1;
const RECOVER_NESTED_DISCRIMINATOR: u8 = 2;

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let mut discriminator: u8 = 0;

    if bytes_stream.len() > 0 {
        let (disc_bytes, _rest) = bytes_stream.split_at(1);
        discriminator = disc_bytes.clone().get_u8();
    }

    match discriminator {
        CREATE_DISCRIMINATOR => {
            result.instructionType = "Create".to_string();
        }
        CREATE_IDEMPOTENT_DISCRIMINATOR => {
            result.instructionType = "CreateIdempotent".to_string();
        }
        RECOVER_NESTED_DISCRIMINATOR => {
            result.instructionType = "RecoverNested".to_string();
        }
        _ => {}
    }

    return result;
}
