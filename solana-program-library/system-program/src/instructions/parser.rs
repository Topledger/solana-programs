extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::{CreateAccountLayout, CreateAccountWithSeedLayout, PubKeyLayout};

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
        let discriminator: u32 = disc_bytes.clone().get_u32_le();
        let rest_bytes = &mut rest.clone();

        match discriminator {
            CREATE_ACCOUNT_DISCRIMINATOR => {
                result.instructionType = "CreateAccount".to_string();
                result.createAccount =
                    CreateAccountLayout::deserialize(rest_bytes).unwrap_or_default();
            }
            CREATE_ACCOUNT_WITH_SEED_DISCRIMINATOR => {
                result.instructionType = "CreateAccountWithSeed".to_string();

                let total_length = rest.len();

                let base = &rest[0..32];
                let owner = &rest[(total_length - 32)..];
                let space = &rest[(total_length - 32 - 8)..(total_length - 32)];
                let lamports = &rest[(total_length - 32 - 8 - 8)..(total_length - 32 - 8)];
                let seed = &rest[(32 + 8)..(total_length - 32 - 8 - 8)];

                let mut createAccountWithSeed = CreateAccountWithSeedLayout::default();
                createAccountWithSeed.base = PubKeyLayout {
                    value: base.to_vec().as_slice().try_into().unwrap(),
                };
                createAccountWithSeed.seed = String::from_utf8(seed.to_vec()).unwrap();
                createAccountWithSeed.lamports = u64::from_le_bytes(lamports.try_into().unwrap());
                createAccountWithSeed.space = u64::from_le_bytes(space.try_into().unwrap());
                createAccountWithSeed.owner = PubKeyLayout {
                    value: owner.to_vec().as_slice().try_into().unwrap(),
                };

                result.createAccountWithSeed = createAccountWithSeed;
            }
            _ => {}
        }
    }

    return result;
}

fn get_b58_string(data: &[u8]) -> String {
    return bs58::encode(data).into_string();
}
