extern crate bs58;

use std::u64;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::EscrowNftLayout;

const ESCROW_NFT_DISCRIMINATOR: u64 = u64::from_be_bytes([18, 168, 117, 140, 153, 25, 139, 220]);
const CANCEL_ESCROW_DISCRIMINATOR: u64 = u64::from_be_bytes([156, 203, 54, 179, 38, 72, 33, 21]);
const SINGLE_PAYMENT_ESCROW_DISCRIMINATOR: u64 =
    u64::from_be_bytes([1, 66, 128, 69, 3, 253, 41, 188]);
const SINGLE_SOL_PAYMENT_ESCROW_DISCRIMINATOR: u64 =
    u64::from_be_bytes([44, 189, 137, 173, 29, 112, 152, 127]);

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub escrowNft: EscrowNftLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator: u64 = disc_bytes.clone().get_u64();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        ESCROW_NFT_DISCRIMINATOR => {
            result.instructionType = "EscrowNft".to_string();
            result.escrowNft = EscrowNftLayout::deserialize(rest_bytes).unwrap();
        }
        CANCEL_ESCROW_DISCRIMINATOR => {
            result.instructionType = "CancelEscrow".to_string();
        }
        SINGLE_PAYMENT_ESCROW_DISCRIMINATOR => {
            result.instructionType = "SinglePaymentEscrow".to_string();
        }
        SINGLE_SOL_PAYMENT_ESCROW_DISCRIMINATOR => {
            result.instructionType = "SingleSolPaymentEscrow".to_string();
        }
        _ => {}
    }

    return result;
}
