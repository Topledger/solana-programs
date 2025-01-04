extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::{PubKeyLayout, TransferLayout, TransferWithSeedLayout};

const TRANSFER_DISCRIMINATOR: u32 = 2;
const TRANSFER_WITH_SEED_DISCRIMINATOR: u32 = 11;

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub transfer: TransferLayout,
    pub transferWithSeed: TransferWithSeedLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    if bytes_stream.len() >= 4 {
        let (disc_bytes, rest) = bytes_stream.split_at(4);
        let discriminator: u32 = disc_bytes.clone().get_u32_le();
        let rest_bytes = &mut rest.clone();

        match discriminator {
            TRANSFER_DISCRIMINATOR => {
                result.instructionType = "Transfer".to_string();
                result.transfer = TransferLayout::deserialize(rest_bytes).unwrap_or_default();
            }
            TRANSFER_WITH_SEED_DISCRIMINATOR => {
                result.instructionType = "TransferWithSeed".to_string();

                let total_length = rest.len();
                let lamports = &rest[0..8];
                let from_seed = &rest[8..(total_length - 32)];
                let from_owner = &rest[(total_length - 32)..];
                let mut transferWithSeed = TransferWithSeedLayout::default();
                transferWithSeed.lamports = u64::from_le_bytes(lamports.try_into().unwrap());
                transferWithSeed.from_seed =
                    String::from_utf8(from_seed.to_vec()).unwrap_or_default();
                transferWithSeed.from_owner = PubKeyLayout {
                    value: from_owner.to_vec().as_slice().try_into().unwrap(),
                };

                result.transferWithSeed = transferWithSeed;
            }
            _ => {}
        }
    }

    return result;
}

fn get_b58_string(data: &[u8]) -> String {
    return bs58::encode(data).into_string();
}
