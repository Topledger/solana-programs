extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::{CreateAccountLayout, CreateAccountWithSeedLayout};

const CREATE_ACCOUNT_DISCRIMINATOR: u32 = 0;
const CREATE_ACCOUNT_WITH_SEED_DISCRIMINATOR: u32 = 3;

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub createAccount: CreateAccountLayout,
    pub createAccountWithSeed: CreateAccountWithSeedLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    if bytes_stream.len() >= 4 {
        let (disc_bytes, rest) = bytes_stream.split_at(4);
        let discriminator: u32 = disc_bytes.clone().get_u32();
        let rest_bytes = &mut rest.clone();

        match discriminator {
            CREATE_ACCOUNT_DISCRIMINATOR => {
                result.instructionType = "CreateAccount".to_string();
                result.createAccount =
                    CreateAccountLayout::try_from_slice(rest_bytes).unwrap_or_default();
            }
            CREATE_ACCOUNT_WITH_SEED_DISCRIMINATOR => {
                result.instructionType = "CreateAccountWithSeed".to_string();
                result.createAccountWithSeed =
                    CreateAccountWithSeedLayout::deserialize(rest_bytes).unwrap();
            }
            _ => {}
        }
    }

    return result;
}
