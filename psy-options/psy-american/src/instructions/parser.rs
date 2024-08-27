extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::{
    BurnWriterForQuoteLayout, CloseOptionPositionLayout, ClosePostExpirationLayout,
    ExerciseOptionLayout, ExerciseOptionV2Layout, InitSerumMarketLayout, InitializeMarketLayout,
    MintOptionLayout, MintOptionV2Layout,
};

const INITIALIZE_MARKET_DISCRIMINATOR: u64 =
    u64::from_be_bytes([35, 35, 189, 193, 155, 48, 170, 203]);
const MINT_OPTION_DISCRIMINATOR: u64 = u64::from_be_bytes([76, 112, 32, 89, 147, 85, 222, 43]);
const MINT_OPTION_V2_DISCRIMINATOR: u64 =
    u64::from_be_bytes([229, 204, 221, 145, 212, 231, 30, 76]);
const EXERCISE_OPTION_DISCRIMINATOR: u64 =
    u64::from_be_bytes([231, 98, 131, 183, 245, 93, 122, 48]);
const EXERCISE_OPTION_V2_DISCRIMINATOR: u64 =
    u64::from_be_bytes([122, 167, 194, 73, 152, 110, 30, 200]);
const CLOSE_POST_EXPIRATION_DISCRIMINATOR: u64 =
    u64::from_be_bytes([115, 203, 223, 108, 122, 150, 229, 92]);
const CLOSE_OPTION_POSITION_DISCRIMINATOR: u64 =
    u64::from_be_bytes([225, 84, 101, 195, 235, 136, 55, 95]);
const BURN_WRITER_FOR_QUOTE_DISCRIMINATOR: u64 =
    u64::from_be_bytes([217, 231, 245, 248, 97, 254, 198, 120]);
const INIT_SERUM_MARKET_DISCRIMINATOR: u64 =
    u64::from_be_bytes([55, 157, 44, 90, 69, 81, 148, 175]);

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub initializeMarket: InitializeMarketLayout,
    pub mintOption: MintOptionLayout,
    pub mintOptionV2: MintOptionV2Layout,
    pub exerciseOption: ExerciseOptionLayout,
    pub exerciseOptionV2: ExerciseOptionV2Layout,
    pub closePostExpiration: ClosePostExpirationLayout,
    pub closeOptionPosition: CloseOptionPositionLayout,
    pub burnWriterForQuote: BurnWriterForQuoteLayout,
    pub initSerumMarket: InitSerumMarketLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator: u64 = disc_bytes.clone().get_u64();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        INITIALIZE_MARKET_DISCRIMINATOR => {
            result.instructionType = "InitializeMarket".to_string();
            result.initializeMarket = InitializeMarketLayout::deserialize(rest_bytes).unwrap();
        }
        MINT_OPTION_DISCRIMINATOR => {
            result.instructionType = "MintOption".to_string();
            result.mintOption = MintOptionLayout::deserialize(rest_bytes).unwrap();
        }
        MINT_OPTION_V2_DISCRIMINATOR => {
            result.instructionType = "MintOptionV2".to_string();
            result.mintOptionV2 = MintOptionV2Layout::deserialize(rest_bytes).unwrap();
        }
        EXERCISE_OPTION_DISCRIMINATOR => {
            result.instructionType = "ExerciseOption".to_string();
            result.exerciseOption = ExerciseOptionLayout::deserialize(rest_bytes).unwrap();
        }
        EXERCISE_OPTION_V2_DISCRIMINATOR => {
            result.instructionType = "ExerciseOptionV2".to_string();
            result.exerciseOptionV2 = ExerciseOptionV2Layout::deserialize(rest_bytes).unwrap();
        }
        CLOSE_POST_EXPIRATION_DISCRIMINATOR => {
            result.instructionType = "ClosePostExpiration".to_string();
            result.closePostExpiration =
                ClosePostExpirationLayout::deserialize(rest_bytes).unwrap();
        }
        CLOSE_OPTION_POSITION_DISCRIMINATOR => {
            result.instructionType = "CloseOptionPosition".to_string();
            result.closeOptionPosition =
                CloseOptionPositionLayout::deserialize(rest_bytes).unwrap();
        }
        BURN_WRITER_FOR_QUOTE_DISCRIMINATOR => {
            result.instructionType = "BurnWriterForQuote".to_string();
            result.burnWriterForQuote = BurnWriterForQuoteLayout::deserialize(rest_bytes).unwrap();
        }
        INIT_SERUM_MARKET_DISCRIMINATOR => {
            result.instructionType = "InitSerumMarket".to_string();
            result.initSerumMarket = InitSerumMarketLayout::deserialize(rest_bytes).unwrap();
        }
        _ => {}
    }

    return result;
}
