extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::{CreateNftLayout, EditDataLayout};

const CREATE_MINT_DISCRIMINATOR: u8 = 0;
const CREATE_COLLECTION_DISCRIMINATOR: u8 = 1;
const CREATE_NFT_DISCRIMINATOR: u8 = 2;
const REDEEM_NFT_DISCRIMINATOR: u8 = 3;
const WITHDRAW_TOKENS_DISCRIMINATOR: u8 = 4;
const EDIT_DATA_DISCRIMINATOR: u8 = 5;

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub createNft: CreateNftLayout,
    pub editData: EditDataLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let discriminator: u8 = disc_bytes.clone().get_u8();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        CREATE_MINT_DISCRIMINATOR => {
            result.instructionType = "CreateMint".to_string();
        }
        CREATE_COLLECTION_DISCRIMINATOR => {
            result.instructionType = "CreateCollection".to_string();
        }
        CREATE_NFT_DISCRIMINATOR => {
            result.instructionType = "CreateNft".to_string();
            result.createNft = CreateNftLayout::deserialize(rest_bytes).unwrap();
        }
        REDEEM_NFT_DISCRIMINATOR => {
            result.instructionType = "RedeemNft".to_string();
        }
        WITHDRAW_TOKENS_DISCRIMINATOR => {
            result.instructionType = "WithdrawTokens".to_string();
        }
        EDIT_DATA_DISCRIMINATOR => {
            result.instructionType = "EditData".to_string();
            result.editData = EditDataLayout::deserialize(rest_bytes).unwrap();
        }
        _ => {}
    }

    return result;
}
